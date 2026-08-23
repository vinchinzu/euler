// Project Euler 681 - Maximal Area
// Enumerate (w,x,y,z) with wxyz = K^2, w<=x<=y<=z, z < w+x+y, even perimeter.
// Light K run in parallel chunks; highly-composite K are split into (w, x-index)
// pair chunks so they cannot starve the pool. Do not nest rayon.

use rayon::prelude::*;

const PAIR_ND: usize = 192;
const X_CHUNK: usize = 16;
const LIGHT_CHUNK: usize = 32;
const LIGHT_BUF: usize = 256;

struct KData {
    k2: i64,
    divs: Vec<i64>,
}

struct Work {
    ki: u32,
    i: u32,
    j_lo: u32,
    j_hi: u32,
}

#[inline(always)]
fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    // disc = (w+x)^2 + 4 r2 <= ~4e12; f64 mantissa is exact below 2^53.
    let mut s = (n as f64).sqrt() as i64;
    if s * s > n {
        s -= 1;
    } else if (s + 1) * (s + 1) <= n {
        s += 1;
    }
    s
}

fn k2_nd(k: usize, spf: &[u32]) -> usize {
    let mut tmp = k;
    let mut nd = 1usize;
    while tmp > 1 {
        let p = spf[tmp] as usize;
        let mut e = 0u32;
        while tmp % p == 0 {
            tmp /= p;
            e += 1;
        }
        nd *= (2 * e as usize) + 1;
    }
    nd
}

fn fill_divs(k: usize, spf: &[u32], out: &mut [i64]) -> (i64, usize) {
    let mut prms = [0i64; 16];
    let mut exps = [0u32; 16];
    let mut np = 0usize;
    let mut tmp = k;
    while tmp > 1 {
        let p = spf[tmp] as usize;
        let mut e = 0u32;
        while tmp % p == 0 {
            tmp /= p;
            e += 1;
        }
        prms[np] = p as i64;
        exps[np] = e * 2;
        np += 1;
    }
    let k2 = (k as i64) * (k as i64);
    out[0] = 1;
    let mut nd = 1usize;
    for i in 0..np {
        let old = nd;
        let mut pp = 1i64;
        for _ in 0..exps[i] {
            pp *= prms[i];
            for j in 0..old {
                out[nd] = out[j] * pp;
                nd += 1;
            }
        }
    }
    out[..nd].sort_unstable();
    (k2, nd)
}

fn k2_divisors(k: usize, spf: &[u32]) -> (i64, Vec<i64>) {
    let nd = k2_nd(k, spf);
    let mut divs = vec![0i64; nd];
    let (k2, got) = fill_divs(k, spf, &mut divs);
    debug_assert_eq!(got, nd);
    (k2, divs)
}

fn split_pair_units(ki: u32, k2: i64, ds: &[i64]) -> Vec<Work> {
    let nd = ds.len();
    let mut units = Vec::new();
    for i in 0..nd {
        let w = ds[i];
        // w^4 > k2 <=> w > k^{1/2} <= 1000 for k <= 1e6.
        if w > 1000 || w * w * w * w > k2 {
            break;
        }
        let r1 = k2 / w;
        let mut x_lim = i;
        while x_lim < nd {
            let x = ds[x_lim];
            if x > 10_000 || x * x * x > r1 {
                break;
            }
            x_lim += 1;
        }
        let mut j = i;
        while j < x_lim {
            let j_hi = (j + X_CHUNK).min(x_lim);
            units.push(Work {
                ki,
                i: i as u32,
                j_lo: j as u32,
                j_hi: j_hi as u32,
            });
            j = j_hi;
        }
    }
    units
}

fn process_pairs(k2: i64, ds: &[i64], i_lo: usize, i_hi: usize, j_lo: usize, j_hi: usize) -> i64 {
    let nd = ds.len();
    let i_hi = i_hi.min(nd);
    let mut sum = 0i64;
    for i in i_lo..i_hi {
        // SAFETY: i < nd by loop bound.
        let w = unsafe { *ds.get_unchecked(i) };
        if w > 1000 || w * w * w * w > k2 {
            break;
        }
        let r1 = k2 / w;
        let js = i.max(j_lo);
        let je = j_hi.min(nd);
        for j in js..je {
            // SAFETY: j < nd.
            let x = unsafe { *ds.get_unchecked(j) };
            if x > 10_000 || x * x * x > r1 {
                break;
            }
            if r1 % x != 0 {
                continue;
            }
            let r2 = r1 / x;
            let b = w + x;
            // z < w+x+y <=> y > (-b + sqrt(b^2 + 4 r2)) / 2.
            let sdisc = isqrt(b * b + 4 * r2);
            let y_min = ((sdisc - b) >> 1) + 1;
            let y_lo = if y_min > x { y_min } else { x };
            if y_lo > r2 / y_lo {
                continue;
            }
            let mut l0 = j;
            if y_lo > x {
                l0 = j + ds[j..].partition_point(|&yy| yy < y_lo);
            }
            for l in l0..nd {
                // SAFETY: l < nd.
                let y = unsafe { *ds.get_unchecked(l) };
                if y > r2 / y {
                    break;
                }
                if r2 % y != 0 {
                    continue;
                }
                let z = r2 / y;
                let s = b + y;
                if z <= y || z >= s {
                    continue;
                }
                let total = s + z;
                if total & 1 == 0 {
                    sum += total;
                }
            }
        }
    }
    sum
}

fn process_light_k(k: usize, spf: &[u32]) -> i64 {
    let mut buf = [0i64; LIGHT_BUF];
    let (k2, nd) = fill_divs(k, spf, &mut buf);
    process_pairs(k2, &buf[..nd], 0, nd, 0, nd)
}

fn main() {
    let n: i64 = 1_000_000;
    let n2 = n * n;

    let maxn = (n as usize) + 1;
    let mut spf = vec![0u32; maxn];
    for i in 0..maxn {
        spf[i] = i as u32;
    }
    let mut i = 2;
    while i * i < maxn {
        if spf[i] == i as u32 {
            let mut j = i * i;
            while j < maxn {
                if spf[j] == j as u32 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
        i += 1;
    }

    // Case 1: z == y, wx must be perfect square
    let mut ans: i64 = 0;
    let mut w = 1i64;
    while w * w * w * w <= n2 {
        let mut x = w;
        while w * x * x * x <= n2 {
            let wx = w * x;
            let swx = (wx as f64).sqrt() as i64;
            let sq = if swx * swx == wx {
                swx
            } else if (swx + 1) * (swx + 1) == wx {
                swx + 1
            } else {
                x += 1;
                continue;
            };
            if (w + x) & 1 != 0 {
                x += 1;
                continue;
            }
            let y_max = n / sq;
            if y_max < x {
                x += 1;
                continue;
            }
            let count = y_max - x + 1;
            ans += count * (w + x + x + y_max);
            x += 1;
        }
        w += 1;
    }

    let mut light_ks: Vec<u32> = Vec::new();
    let mut heavy_ks: Vec<(u32, u32)> = Vec::new();
    for k in 4..=n as usize {
        if spf[k] == k as u32 {
            continue;
        }
        let nd = k2_nd(k, &spf);
        if nd >= PAIR_ND {
            heavy_ks.push((k as u32, nd as u32));
        } else {
            light_ks.push(k as u32);
        }
    }
    // Start the longest heavy K first so stealers pick up remaining pair chunks.
    heavy_ks.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let kdatas: Vec<KData> = heavy_ks
        .par_iter()
        .map(|&(k, _)| {
            let (k2, divs) = k2_divisors(k as usize, &spf);
            KData { k2, divs }
        })
        .collect();

    let work: Vec<Work> = kdatas
        .par_iter()
        .enumerate()
        .flat_map(|(ki, kd)| split_pair_units(ki as u32, kd.k2, &kd.divs))
        .collect();

    let ans_heavy: i64 = work
        .par_iter()
        .map(|u| {
            let kd = &kdatas[u.ki as usize];
            process_pairs(
                kd.k2,
                &kd.divs,
                u.i as usize,
                u.i as usize + 1,
                u.j_lo as usize,
                u.j_hi as usize,
            )
        })
        .sum();

    let ans_light: i64 = light_ks
        .par_chunks(LIGHT_CHUNK)
        .map(|chunk| {
            let mut local = 0i64;
            for &k in chunk {
                local += process_light_k(k as usize, &spf);
            }
            local
        })
        .sum();

    println!("{}", ans + ans_heavy + ans_light);
}
