// Project Euler 681 - Maximal Area
// Enumerate (w,x,y,z) with wxyz = K^2, w<=x<=y<=z, z < w+x+y, even perimeter.
// Light K run in parallel chunks; highly-composite K are split into (w, x-index)
// pair chunks so they cannot starve the pool. Do not nest rayon.

use rayon::prelude::*;
use std::cell::RefCell;

const PAIR_ND: usize = 512;
const LIGHT_CHUNK: usize = 128;
const BIAS: u64 = 0x8080808080808080;

#[derive(Clone, Copy)]
struct Div {
    val: i64,
    exp: u64,
}

thread_local! {
    static BUF: RefCell<Vec<Div>> = RefCell::new(vec![Div { val: 0, exp: 0 }; 4096]);
}

#[inline(always)]
fn divides(sub_exp: u64, sup_exp: u64) -> bool {
    let diff = (sup_exp + BIAS) - sub_exp;
    (diff & BIAS) == BIAS
}

fn k2_nd(k: usize, spf: &[u32]) -> usize {
    let mut tmp = k;
    let mut nd = 1usize;
    while tmp > 1 {
        let p = spf[tmp];
        let mut e = 0u32;
        while tmp > 1 && spf[tmp] == p {
            tmp /= p as usize;
            e += 1;
        }
        nd *= (2 * e as usize) + 1;
    }
    nd
}

fn fill_divs(k: usize, spf: &[u32], out: &mut [Div]) -> (i64, usize, u64) {
    let mut prms = [0i64; 8];
    let mut exps = [0u32; 8];
    let mut np = 0usize;
    let mut tmp = k;
    while tmp > 1 {
        let p = spf[tmp];
        let mut e = 0u32;
        while tmp > 1 && spf[tmp] == p {
            tmp /= p as usize;
            e += 1;
        }
        prms[np] = p as i64;
        exps[np] = e * 2;
        np += 1;
    }
    let k2 = (k as i64) * (k as i64);
    let mut e_k2 = 0u64;
    for (i, &exp) in exps.iter().take(np).enumerate() {
        e_k2 |= (exp as u64) << (i * 8);
    }
    out[0] = Div { val: 1, exp: 0 };
    let mut nd = 1usize;
    for i in 0..np {
        let old = nd;
        let mut pp = 1i64;
        for e in 1..=exps[i] {
            pp *= prms[i];
            let exp_inc = (e as u64) << (i * 8);
            for j in 0..old {
                out[nd] = Div {
                    val: out[j].val * pp,
                    exp: out[j].exp + exp_inc,
                };
                nd += 1;
            }
        }
    }
    out[..nd].sort_unstable_by_key(|d| d.val);
    (k2, nd, e_k2)
}

fn process_pairs(k2: i64, e_k2: u64, ds: &[Div]) -> i64 {
    let nd = ds.len();
    let mut sum = 0i64;
    for i in 0..nd {
        let dw = unsafe { *ds.get_unchecked(i) };
        let w = dw.val;
        if w > 1000 || w * w * w * w > k2 {
            break;
        }
        let r1 = k2 / w;
        let e_r1 = e_k2 - dw.exp;
        for j in i..nd {
            let dx = unsafe { *ds.get_unchecked(j) };
            let x = dx.val;
            if x > 10_000 || x * x * x > r1 {
                break;
            }
            if !divides(dx.exp, e_r1) {
                continue;
            }
            let r2 = r1 / x;
            let b = w + x;
            if (r2 & 1) != 0 && (b & 1) != 0 {
                continue;
            }

            // Fast quadratic bound: y_min > x <=> r2 >= x * (b + x)
            let (l0, y_lo) = if r2 < x * (b + x) {
                (j, x)
            } else {
                let sdisc = ((b * b + 4 * r2) as f64).sqrt() as i64;
                let y_min = ((sdisc - b) >> 1) + 1;
                let l0 = j + ds[j..].partition_point(|d| d.val < y_min);
                (l0, y_min)
            };

            let y_hi = ((r2 - 1) as f64).sqrt() as i64;
            if y_lo > y_hi {
                continue;
            }

            let e_r2 = e_r1 - dx.exp;

            for l in l0..nd {
                let dy = unsafe { *ds.get_unchecked(l) };
                let y = dy.val;
                if y > y_hi {
                    break;
                }
                if !divides(dy.exp, e_r2) {
                    continue;
                }
                let z = r2 / y;
                let total = b + y + z;
                if total & 1 == 0 {
                    sum += total;
                }
            }
        }
    }
    sum
}

fn main() {
    let n: i64 = 1_000_000;
    let n2 = n * n;

    let maxn = (n as usize) + 1;
    let mut spf: Vec<u32> = (0..maxn as u32).collect();
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
    heavy_ks.sort_unstable_by_key(|a| std::cmp::Reverse(a.1));

    let (ans_heavy, ans_light) = rayon::join(
        || {
            heavy_ks
                .par_iter()
                .map(|&(k, _)| {
                    BUF.with(|cell| {
                        let mut buf = cell.borrow_mut();
                        let (k2, nd, e_k2) = fill_divs(k as usize, &spf, &mut buf);
                        process_pairs(k2, e_k2, &buf[..nd])
                    })
                })
                .sum::<i64>()
        },
        || {
            light_ks
                .par_chunks(LIGHT_CHUNK)
                .map(|chunk| {
                    BUF.with(|cell| {
                        let mut buf = cell.borrow_mut();
                        let mut local = 0i64;
                        for &k in chunk {
                            let (k2, nd, e_k2) = fill_divs(k as usize, &spf, &mut buf);
                            local += process_pairs(k2, e_k2, &buf[..nd]);
                        }
                        local
                    })
                })
                .sum::<i64>()
        },
    );

    println!("{}", ans + ans_heavy + ans_light);
}
