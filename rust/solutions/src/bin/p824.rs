// Project Euler Problem 824: Chess Sliders
// Compute L(10^9, 10^15) mod (10^7+19)^2

use rayon::prelude::*;

#[inline(always)]
fn mm(a: u64, b: u64, md: u64) -> u64 {
    ((a as u128 * b as u128) % md as u128) as u64
}

#[inline(always)]
fn inv_mod_p2(a: u64, invp: &[u64], p: u64, m: u64) -> u64 {
    let a = a % m;
    let r = a % p;
    // SAFETY: r = a % p < p, invp.len() == p
    let x = unsafe { *invp.get_unchecked(r as usize) };
    let ax = mm(a, x, m);
    mm(x, (2 + m - ax) % m, m)
}

#[inline(always)]
fn fpow(u: u64, p: u64, m: u64, w: u64) -> u64 {
    let um = u % p;
    let mut tmp = (1 + m - mm(mm(um, p, m), w, m)) % m;
    if u & 1 == 1 {
        tmp = (m - tmp) % m;
    }
    tmp
}

#[inline(always)]
fn unit_factorial(mut n: u64, fac: &[u64], h: &[u64], p: u64, m: u64, w: u64) -> u64 {
    let mut res = 1u64;
    while n > 0 {
        let u = n / p;
        let v = n % p;
        let vu = v as usize;
        // SAFETY: v = n % p < p; fac/h have length p
        unsafe {
            res = mm(res, *fac.get_unchecked(vu), m);
            let corr = mm(u % p, *h.get_unchecked(vu), p);
            res = mm(res, 1 + corr * p, m);
        }
        res = mm(res, fpow(u, p, m, w), m);
        n = u;
    }
    res
}

#[inline(always)]
fn vp_fact(n: u64, p: u64) -> u64 {
    let q = n / p;
    q + q / p
}

fn binom_mod_p2(
    n: u64,
    k: u64,
    fac: &[u64],
    h: &[u64],
    invp: &[u64],
    p: u64,
    m: u64,
    w: u64,
) -> u64 {
    if k > n {
        return 0;
    }
    let nk = n - k;
    let e = vp_fact(n, p) - vp_fact(k, p) - vp_fact(nk, p);
    if e >= 2 {
        return 0;
    }

    let un = unit_factorial(n, fac, h, p, m, w);
    let uk = unit_factorial(k, fac, h, p, m, w);
    let unk = unit_factorial(nk, fac, h, p, m, w);

    let mut val = mm(un, inv_mod_p2(uk, invp, p, m), m);
    val = mm(val, inv_mod_p2(unk, invp, p, m), m);
    if e == 1 {
        val = mm(val, p, m);
    }
    val
}

fn coeff_alpha_power(
    mmv: u64,
    d: u64,
    fac: &[u64],
    h: &[u64],
    invp: &[u64],
    p: u64,
    m: u64,
    w: u64,
) -> u64 {
    if d == 0 {
        return 1;
    }
    let b = binom_mod_p2(mmv - d - 1, d - 1, fac, h, invp, p, m, w);
    mm(mm(mmv % m, inv_mod_p2(d % m, invp, p, m), m), b, m)
}

fn solve() -> u64 {
    let p: u64 = 10_000_019;
    let m: u64 = p * p;

    let n: u64 = 1_000_000_000;
    let k: u64 = 1_000_000_000_000_000;
    let t_max = k / n;
    let p_us = p as usize;

    // fac is independent of invp/h; compute them in parallel
    let (fac, (invp, h)) = rayon::join(
        || {
            let mut fac = vec![0u64; p_us];
            fac[0] = 1;
            let mut f = 1u64;
            for i in 1..p {
                f = mm(f, i, m);
                fac[i as usize] = f;
            }
            fac
        },
        || {
            let mut invp = vec![0u64; p_us];
            let mut h = vec![0u64; p_us];
            invp[1] = 1;
            let mut hh = 0u64;
            for i in 1..p {
                if i >= 2 {
                    invp[i as usize] = (p - mm(p / i, invp[(p % i) as usize], p)) % p;
                }
                hh = (hh + invp[i as usize]) % p;
                h[i as usize] = hh;
            }
            (invp, h)
        },
    );

    // Wilson quotient: (p-1)! ≡ -1 + p*w (mod p^2)
    let big_f = fac[(p - 1) as usize];
    let w = ((big_f + 1) / p) % p;

    // Prefix C(n, t) mod m — loop-carried, must stay serial
    let t_len = (t_max + 1) as usize;
    let mut comb = vec![0u64; t_len];
    comb[0] = 1;
    for t in 1..=t_max {
        let mut c = mm(comb[(t - 1) as usize], (n - t + 1) % m, m);
        c = mm(c, inv_mod_p2(t, &invp, p, m), m);
        comb[t as usize] = c;
    }

    let ans: u128 = (0..t_len)
        .into_par_iter()
        .map(|t| {
            let t = t as u64;
            let d = k - t * n;
            let mv = n * n - 2 * n * t;
            mm(comb[t as usize], coeff_alpha_power(mv, d, &fac, &h, &invp, p, m, w), m) as u128
        })
        .sum();

    (ans % m as u128) as u64
}

fn main() {
    println!("{}", solve());
}
