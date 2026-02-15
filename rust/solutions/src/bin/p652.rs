// Project Euler 652 - Distinct values of logarithms
// Count distinct log_m(n) for 2 <= m,n <= N = 10^18, mod 10^9.

const M: i64 = 1_000_000_000;

fn nth_root(n: i64, k: i32) -> i64 {
    if k == 1 { return n; }
    if n <= 1 { return n; }
    let x = (n as f64).powf(1.0 / k as f64) as i64;
    let x = x.max(1);
    for t in (x.saturating_sub(2)..=x+2).rev() {
        if t < 1 { continue; }
        let mut pw = 1i128;
        let mut ok = true;
        for _ in 0..k {
            pw *= t as i128;
            if pw > n as i128 { ok = false; break; }
        }
        if ok {
            let mut pw2 = 1i128;
            let mut over = false;
            for _ in 0..k {
                pw2 *= (t + 1) as i128;
                if pw2 > n as i128 { over = true; break; }
            }
            if over { return t; }
        }
    }
    x
}

fn pow_fits(b: i64, e: i32, limit: i64) -> bool {
    let mut pw = 1i128;
    for _ in 0..e {
        pw *= b as i128;
        if pw > limit as i128 { return false; }
    }
    pw <= limit as i128
}

fn ncr2_mod(n: i64) -> i64 {
    let a = n % (2 * M);
    let b = (n - 1) % (2 * M);
    let prod = ((a as i128 * b as i128) % (2 * M as i128)) as i64;
    (prod / 2) % M
}

fn main() {
    let n: i64 = 1_000_000_000_000_000_000;

    // Mobius sieve
    let max_k = 60usize;
    let mut mobius = vec![1i32; max_k + 1];
    mobius[0] = 0;
    let mut is_prime = vec![true; max_k + 1];
    is_prime[0] = false; is_prime[1] = false;
    for i in 2..=max_k {
        if !is_prime[i] { continue; }
        for j in (i*i..=max_k).step_by(i) { is_prime[j] = false; }
    }
    for p in 2..=max_k {
        if !is_prime[p] { continue; }
        for j in (p..=max_k).step_by(p) { mobius[j] = -mobius[j]; }
        let p2 = p * p;
        for j in (p2..=max_k).step_by(p2) { mobius[j] = 0; }
    }

    let mut ans = ((n - 1) % M * ((n - 1) % M) % M - (n - 2) % M + M) % M;

    for k in 2..=60i32 {
        if !pow_fits(3, k, n) { break; }
        let num_powers = nth_root(n, k) - 1;
        let contrib = (2 * mobius[k as usize] as i64 % M + M) % M * ncr2_mod(num_powers) % M;
        ans = (ans + contrib) % M;
    }

    for k in 2..=60i32 {
        if !pow_fits(3, k, n) { break; }
        ans = (ans - 2 * ((nth_root(n, k) - 2) % M) % M + 2 * M) % M;
        let mut e = 2i32;
        while pow_fits(2, k * e, n) {
            let val = (2 * mobius[e as usize] as i64 % M + M) % M * ((nth_root(n, k * e) - 1) % M) % M;
            ans = (ans - val % M + M) % M;
            e += 1;
        }
    }

    let max_b_val = nth_root(n, 3);
    let max_b = max_b_val.min(10_000_000) as usize;
    let mut is_perfect_power = vec![false; max_b + 1];

    for b in 2..=max_b {
        if !pow_fits(b as i64, 6, n) { break; }
        let mut e = 2;
        loop {
            let mut pw = 1i128;
            let mut ok = true;
            for _ in 0..3*e {
                pw *= b as i128;
                if pw > n as i128 { ok = false; break; }
            }
            if !ok { break; }
            let mut bpow = 1i128;
            for _ in 0..e { bpow *= b as i128; }
            if bpow <= max_b as i128 { is_perfect_power[bpow as usize] = true; }
            e += 1;
        }
    }

    for b in 3..=max_b {
        if !pow_fits(b as i64, 3, n) { break; }
        if is_perfect_power[b] { continue; }
        let mut largest_e = 1;
        while pow_fits(b as i64, largest_e + 1, n) { largest_e += 1; }
        for e1 in 2..=largest_e {
            for e2 in 2..=largest_e {
                let mut a = e1; let mut bb = e2;
                while bb != 0 { let t = bb; bb = a % bb; a = t; }
                if a == 1 { ans = (ans - 1 + M) % M; }
            }
        }
    }

    println!("{}", ans % M);
}
