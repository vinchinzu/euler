// Project Euler 585 - Nested Square Roots
//
// Count representations of integers as nested square roots.

use rayon::prelude::*;

const N: usize = 5_000_000;
const LARGE: usize = 64;

#[derive(Clone, Copy)]
struct Row {
    small: u32,
    sum: u32,
    k: u32,
}

#[inline(always)]
fn coprime(mut u: u32, mut v: u32) -> bool {
    if u == 0 || v == 0 {
        return u | v == 1;
    }
    if (u | v) & 1 == 0 {
        return false;
    }
    u >>= u.trailing_zeros();
    while v != 0 {
        v >>= v.trailing_zeros();
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        v -= u;
    }
    u == 1
}

#[inline(always)]
fn gen_reps(
    i: usize,
    nfac: usize,
    ps: &[u32; 16],
    es: &[u32; 16],
    s: u32,
    t: u32,
    k: u32,
    out: &mut Vec<Row>,
) {
    if i == nfac {
        if t >= s {
            let ss = s as u64 * s as u64;
            let tt = t as u64 * t as u64;
            if (ss + tt) * k as u64 <= N as u64 {
                out.push(Row {
                    small: (ss * k as u64) as u32,
                    sum: ((ss + tt) * k as u64) as u32,
                    k,
                });
            }
        }
        return;
    }
    let p = ps[i];
    let e = es[i];
    let mut p_ek = 1u32;
    for ek in 0..=1u32.min(e) {
        let rem = e - ek;
        let mut p_pow = 1u32;
        for _ in 0..rem {
            p_pow *= p;
        }
        let mut p_es = 1u32;
        let mut p_et = p_pow;
        for es_ in 0..=rem {
            gen_reps(i + 1, nfac, ps, es, s * p_es, t * p_et, k * p_ek, out);
            if es_ < rem {
                p_es *= p;
                p_et /= p;
            }
        }
        p_ek *= p;
    }
}

#[inline]
fn pair_rows(rows: &[Row], len: usize, n: u32) -> i64 {
    let mut local = 0i64;
    for ad in 0..len {
        let ra = unsafe { *rows.get_unchecked(ad) };
        let rem = n - ra.sum;
        for bc in 0..len {
            let rb = unsafe { *rows.get_unchecked(bc) };
            if ra.small < rb.small && rb.sum <= rem && ra.k != rb.k {
                local += if rb.small == rb.sum - rb.small { 1 } else { 2 };
            }
        }
    }
    local
}

fn main() {
    let mut spf = vec![0u32; N + 1];
    let mut phi = vec![0i32; N + 1];
    let mut primes: Vec<u32> = Vec::with_capacity(348_513);
    phi[1] = 1;
    for i in 2..=N {
        if spf[i] == 0 {
            spf[i] = i as u32;
            primes.push(i as u32);
            phi[i] = i as i32 - 1;
        }
        let spfi = spf[i];
        for &p in &primes {
            if p > spfi || p as usize > N / i {
                break;
            }
            let ip = i * p as usize;
            spf[ip] = p;
            if p == spfi {
                phi[ip] = phi[i] * p as i32;
                break;
            } else {
                phi[ip] = phi[i] * (p as i32 - 1);
            }
        }
    }
    drop(primes);

    let mut is_sf = vec![true; N + 1];
    let mut i = 2usize;
    while i * i <= N {
        let sq = i * i;
        let mut j = sq;
        while j <= N {
            is_sf[j] = false;
            j += sq;
        }
        i += 1;
    }

    let mut sqfree = Vec::with_capacity((N * 3) / 5);
    for k in 1..=N / 2 {
        if is_sf[k] {
            sqfree.push(k as u32);
        }
    }
    drop(is_sf);

    let mut f = vec![0i32; N + 1];
    let mut fp = vec![0i32; N + 1];
    for n in 1..=N {
        f[n] = (n as i32 - 1) / 2;
        fp[n] = phi[n] / 2;
    }
    drop(phi);

    let s_lim = N.isqrt();
    for s in 1..=s_lim {
        let ss = s * s;
        let t_max = (N - ss).isqrt();
        let su = s as u32;
        for t in s + 1..=t_max {
            let r = ss + t * t;
            let kmax = N / r;
            f[r] -= 1;
            if coprime(su, t as u32) {
                fp[r] -= 1;
            }
            for &k in &sqfree[1..] {
                let k = k as usize;
                if k > kmax {
                    break;
                }
                f[r * k] -= 1;
            }
        }
    }
    drop(sqfree);

    let mut pref_f = vec![0i64; N + 1];
    let mut pref_fp = vec![0i64; N + 1];
    for i in 1..=N {
        pref_f[i] = pref_f[i - 1] + f[i] as i64;
        pref_fp[i] = pref_fp[i - 1] + fp[i] as i64;
    }
    drop(f);
    drop(fp);
    let ans = pref_f[N];

    // sum_g f[g] * pref_fp[N/g] via Dirichlet grouping
    let mut res = 0i64;
    let mut g = 1usize;
    while g <= N {
        let q = N / g;
        let g2 = N / q;
        res += (pref_f[g2] - pref_f[g - 1]) * pref_fp[q];
        g = g2 + 1;
    }
    drop(pref_f);
    drop(pref_fp);

    // t>=s => (s^2+t^2)*k >= 2n, so n > N/2 has no admissible reps.
    let n_u = N as u32;
    let s_lim_u = s_lim as u32;
    let sub: i64 = (1..N / 2 + 1)
        .into_par_iter()
        .with_min_len(32)
        .map_init(
            || (Vec::<Row>::with_capacity(8192), [0u32; 16], [0u32; 16]),
            |(rows, ps, es), n| {
                if n as u32 > s_lim_u && spf[n] == n as u32 {
                    return 0;
                }
                rows.clear();
                let mut x = n;
                let mut nfac = 0usize;
                while x > 1 {
                    let p = spf[x];
                    let mut e = 0u32;
                    loop {
                        x /= p as usize;
                        e += 1;
                        if x == 1 || spf[x] != p {
                            break;
                        }
                    }
                    ps[nfac] = p;
                    es[nfac] = e;
                    nfac += 1;
                }
                gen_reps(0, nfac, ps, es, 1, 1, 1, rows);
                let len = rows.len();
                if len < 2 {
                    return 0;
                }
                if len >= LARGE {
                    let owned = rows.clone();
                    (0..len)
                        .into_par_iter()
                        .with_min_len(4)
                        .map(|ad| {
                            let ra = owned[ad];
                            let rem = n_u - ra.sum;
                            let mut local = 0i64;
                            for bc in 0..len {
                                let rb = owned[bc];
                                if ra.small < rb.small && rb.sum <= rem && ra.k != rb.k {
                                    local += if rb.small == rb.sum - rb.small { 1 } else { 2 };
                                }
                            }
                            local
                        })
                        .sum()
                } else {
                    pair_rows(rows, len, n_u)
                }
            },
        )
        .sum();
    res -= sub;

    println!("{}", ans + res / 2);
}
