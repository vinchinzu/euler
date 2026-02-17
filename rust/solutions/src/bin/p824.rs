// Project Euler Problem 824: Chess Sliders
// Compute L(10^9, 10^15) mod (10^7+19)^2

fn solve() -> u64 {
    let p: u64 = 10_000_019;
    let m: u64 = p * p;

    let n: u64 = 1_000_000_000;
    let k: u64 = 1_000_000_000_000_000;
    let t_max = k / n;

    // Modular multiply using u128 to avoid overflow
    #[inline(always)]
    fn mm(a: u64, b: u64, md: u64) -> u64 {
        ((a as u128 * b as u128) % md as u128) as u64
    }

    // Inverses mod p
    let mut invp = vec![0u64; p as usize];
    invp[1] = 1;
    for i in 2..p {
        invp[i as usize] = (p - mm(p / i, invp[(p % i) as usize], p)) % p;
    }

    // Factorial mod p^2
    let mut fac = vec![0u64; p as usize];
    fac[0] = 1;
    let mut f = 1u64;
    for i in 1..p {
        f = mm(f, i, m);
        fac[i as usize] = f;
    }

    // Harmonic numbers mod p
    let mut h = vec![0u64; p as usize];
    let mut hh = 0u64;
    for i in 1..p {
        hh = (hh + invp[i as usize]) % p;
        h[i as usize] = hh;
    }

    // Wilson quotient: (p-1)! ≡ -1 + p*w (mod p^2)
    let big_f = fac[(p - 1) as usize];
    let w = ((big_f + 1) / p) % p;

    let inv_mod_p2 = |a: u64| -> u64 {
        let a = a % m;
        let r = a % p;
        if r == 0 {
            panic!(
                "Cannot compute inverse of {} mod {}: value is divisible by {}",
                a, m, p
            );
        }
        let x = invp[r as usize];
        // Newton step: x * (2 - a*x) mod m
        let ax = mm(a, x, m);
        mm(x, (2 + m - ax) % m, m)
    };

    let fpow = |u: u64| -> u64 {
        let um = u % p;
        let mut tmp = (1 + m - mm(mm(um, p, m), w, m)) % m;
        if u & 1 == 1 {
            tmp = (m - tmp) % m;
        }
        tmp
    };

    let unit_factorial = |n: u64| -> u64 {
        let mut n = n;
        let mut res = 1u64;
        while n > 0 {
            let u = n / p;
            let v = n % p;
            res = mm(res, fac[v as usize], m);
            let corr = mm(u % p, h[v as usize], p);
            res = mm(res, 1 + corr * p, m);
            res = mm(res, fpow(u), m);
            n = u;
        }
        res
    };

    let vp_fact = |n: u64| -> u64 {
        let q = n / p;
        q + q / p
    };

    let binom_mod_p2 = |n: u64, k: u64| -> u64 {
        if k > n {
            return 0;
        }
        let nk = n - k;
        let e = vp_fact(n) - vp_fact(k) - vp_fact(nk);
        if e >= 2 {
            return 0;
        }

        let un = unit_factorial(n);
        let uk = unit_factorial(k);
        let unk = unit_factorial(nk);

        let mut val = mm(un, inv_mod_p2(uk), m);
        val = mm(val, inv_mod_p2(unk), m);
        if e == 1 {
            val = mm(val, p, m);
        }
        val
    };

    let coeff_alpha_power = |mmv: u64, d: u64| -> u64 {
        if d == 0 {
            return 1;
        }
        let b = binom_mod_p2(mmv - d - 1, d - 1);
        mm(mm(mmv % m, inv_mod_p2(d % m), m), b, m)
    };

    let mut comb = 1u64;
    let mut ans = 0u64;
    let mut d = k;
    let mut mv = n * n;

    for t in 0..=t_max {
        if t > 0 {
            comb = mm(comb, (n - t + 1) % m, m);
            comb = mm(comb, inv_mod_p2(t), m);
        }

        ans = (ans + mm(comb, coeff_alpha_power(mv, d), m)) % m;

        d -= n;
        mv -= 2 * n;
    }

    ans
}

fn main() {
    println!("{}", solve());
}
