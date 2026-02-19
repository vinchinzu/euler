// Project Euler 344

use euler_utils::mod_pow;

fn w(n: usize, c: usize, modulus: i64) -> i64 {
    let k = c - c / 2;

    let mut fact = vec![0i64; n + 1];
    let mut inv_fact = vec![0i64; n + 1];
    fact[0] = 1;
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as i64 % modulus;
    }
    inv_fact[n] = mod_pow(fact[n] as u64, (modulus - 2) as u64, modulus as u64) as i64;
    for i in (0..n).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i + 1) as i64 % modulus;
    }

    let ncr = |a: usize, b: usize| -> i64 {
        if b > a { return 0; }
        fact[a] % modulus * inv_fact[b] % modulus * inv_fact[a - b] % modulus
    };

    let mut ncrs = vec![vec![0i64; k + 1]; k + 2];
    for i in 0..k + 2 {
        for j in 0..=k.min(i) {
            ncrs[i][j] = ncr(i, j);
        }
    }

    let max_sum = n;
    let num_piles = c / 2 + 1;

    let mut xz = vec![0i64; max_sum + 1];
    xz[0] = 1;
    for i in (2..=max_sum).step_by(2) {
        let lim = (num_piles / 2).min(i / 2);
        for np in 0..=lim {
            let idx = i / 2 - np;
            let prev = xz[idx];
            if 2 * np < ncrs.len() && 2 * np < ncrs[num_piles].len() {
                xz[i] = (xz[i] + prev % modulus * ncrs[num_piles][2 * np]) % modulus;
            }
        }
    }

    let num_piles2 = c / 2;
    let mut xz_minus = vec![0i64; max_sum + 1];
    xz_minus[0] = 1;
    for i in (2..=max_sum).step_by(2) {
        let lim = (num_piles2 / 2).min(i / 2);
        for np in 0..=lim {
            let idx = i / 2 - np;
            let prev = xz_minus[idx];
            if 2 * np < ncrs.len() && 2 * np < ncrs[num_piles2].len() {
                xz_minus[i] = (xz_minus[i] + prev % modulus * ncrs[num_piles2][2 * np]) % modulus;
            }
        }
    }

    let mut res = (c as i64 + 1) % modulus * ncr(n, c + 1) % modulus;
    for i in 0..(n - c) {
        res = (res - xz[i] % modulus * ncr(n - c - 1 - i + k, k) % modulus + modulus) % modulus;
    }

    let mut num_losing_late = 0i64;
    for i in 0..=(n - c) {
        num_losing_late = (num_losing_late + xz[i] % modulus * ncr(n - c - i + k, k)) % modulus;
    }
    for i in 0..=(n - c) {
        num_losing_late = (num_losing_late - xz_minus[i] % modulus * ncr(n - c - i + k, k) % modulus + modulus) % modulus;
    }

    res = (res - (c as i64 - 1) % modulus * num_losing_late % modulus + modulus) % modulus;
    res
}

fn main() {
    let n = 1_000_000;
    let c = 100;
    let m1: i64 = 1_000_003;
    let m2: i64 = 1_000_033;

    let w1 = w(n, c, m1);
    let w2 = w(n, c, m2);

    let m = m1 * m2;
    let x = ((w1 as i128 * m2 as i128 % m as i128 * mod_pow(m2 as u64, (m1 - 2) as u64, m1 as u64) as i128 % m as i128
        + w2 as i128 * m1 as i128 % m as i128 * mod_pow(m1 as u64, (m2 - 2) as u64, m2 as u64) as i128 % m as i128)
        % m as i128) as i64;

    println!("{}", x);
}
