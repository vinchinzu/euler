// Project Euler 864 - C(n) = count of squarefree x^2+1 for 1 <= x <= n
// Part A: DFS over d <= D with CRT solutions
// Part B: Pell equation for d > D

const N_VAL: i64 = 123_567_101_113;
const D_LIM: i64 = 100_000_000;
const SIEVE_LIM: usize = 100_000_001;
const MAX_SOLS: usize = 1024;

fn mod_inv_gen(mut a: i64, m: i64) -> i64 {
    let mut m0 = m;
    let mut x0 = 0i64;
    let mut x1 = 1i64;
    a = a.rem_euclid(m);
    while a > 1 && m0 > 0 {
        let q = a / m0;
        let t = m0;
        m0 = a - q * m0;
        a = t;
        let t2 = x0;
        x0 = x1 - q * x0;
        x1 = t2;
    }
    (x1 % m + m) % m
}

fn mod_sqrt_neg1_p2(p: i32) -> (i64, i64) {
    let pl = p as i64;
    let mut r = 0i64;
    for g in 2..pl {
        let mut t = 1i64;
        let mut base = g;
        let mut e = (p - 1) / 4;
        while e > 0 {
            if e & 1 == 1 { t = t * base % pl; }
            base = base * base % pl;
            e >>= 1;
        }
        if t * t % pl == pl - 1 { r = t; break; }
    }
    let p2 = pl * pl;
    let inv2r = mod_inv_gen(2 * r, p2);
    let val = r * r + 1;
    let correction = ((val % p2) as i128 * inv2r as i128 % p2 as i128) as i64;
    r = ((r - correction) % p2 + p2) % p2;
    if r < p2 - r { (r, p2 - r) } else { (p2 - r, r) }
}

#[inline]
fn count_solutions(n: i64, m_sq: i64, sols: &[i64]) -> i64 {
    let mut total = 0i64;
    for &a in sols {
        if a == 0 {
            if n >= m_sq { total += n / m_sq; }
        } else if a <= n {
            total += (n - a) / m_sq + 1;
        }
    }
    total
}

fn part_a_dfs(
    idx: usize, d: i64, sols: &[i64], mu: i64,
    primes_1mod4: &[i32], roots: &[(i64, i64)], result: &mut i64,
) {
    let d_sq = d * d;
    let cnt = count_solutions(N_VAL, d_sq, sols);
    if cnt > 0 { *result += mu * cnt; }

    for i in idx..primes_1mod4.len() {
        let p = primes_1mod4[i] as i64;
        let new_d = d * p;
        if new_d > D_LIM { break; }

        let p2 = p * p;
        let (r0, r1) = roots[i];

        let d_sq_mod_p2 = d_sq % p2;
        let inv_d_sq = mod_inv_gen(d_sq_mod_p2, p2);

        let mut new_sols = [0i64; MAX_SOLS];
        let mut nsols = 0usize;

        for &s in sols {
            if nsols >= MAX_SOLS - 2 { break; }
            let s_mod_p2 = s % p2;

            let diff0 = ((r0 - s_mod_p2) % p2 + p2) % p2;
            let kv0 = (diff0 as i128 * inv_d_sq as i128 % p2 as i128) as i64;
            new_sols[nsols] = (s as i128 + d_sq as i128 * kv0 as i128) as i64;
            nsols += 1;

            let diff1 = ((r1 - s_mod_p2) % p2 + p2) % p2;
            let kv1 = (diff1 as i128 * inv_d_sq as i128 % p2 as i128) as i64;
            new_sols[nsols] = (s as i128 + d_sq as i128 * kv1 as i128) as i64;
            nsols += 1;
        }

        part_a_dfs(i + 1, new_d, &new_sols[..nsols], -mu, primes_1mod4, roots, result);
    }
}

fn is_squarefree_with_mu(n: i64, all_primes: &[i32]) -> Option<i32> {
    let mut mu = 1i32;
    let mut temp = n;
    for &p in all_primes {
        let pl = p as i64;
        if pl * pl > temp { break; }
        if temp % pl == 0 {
            temp /= pl;
            mu = -mu;
            if temp % pl == 0 { return None; }
        }
    }
    if temp > 1 { mu = -mu; }
    Some(mu)
}

fn solve_pell_and_count(k: i64, part_b: &mut i64, all_primes: &[i32]) {
    let mut a0 = (k as f64).sqrt() as i64;
    while (a0 + 1) * (a0 + 1) <= k { a0 += 1; }
    while a0 * a0 > k { a0 -= 1; }
    if a0 * a0 == k { return; }

    let (mut m, mut d, mut a) = (0i64, 1i64, a0);
    let (mut num2, mut num1) = (0i128, 1i128);
    let (mut den2, mut den1) = (1i128, 0i128);
    let mut fund_x: i128 = 0;
    let mut fund_y: i128 = 0;

    for iter in 0..10000 {
        let num = a as i128 * num1 + num2;
        let den = a as i128 * den1 + den2;

        // If convergent exceeds N, fundamental solution too large for any valid x
        if num > N_VAL as i128 { return; }

        let val = num * num - k as i128 * den * den;

        if val == -1 { fund_x = num; fund_y = den; break; }
        if val == 1 && iter > 0 { return; }

        num2 = num1; num1 = num;
        den2 = den1; den1 = den;
        m = d * a - m;
        d = (k - m * m) / d;
        if d == 0 { return; }
        a = (a0 + m) / d;
    }

    if fund_x == 0 { return; }

    let mul_x: i128 = 2 * fund_x * fund_x + 1;
    let mul_y: i128 = 2 * fund_x * fund_y;

    let (mut cx, mut cy): (i128, i128) = (fund_x, fund_y);
    while cx <= N_VAL as i128 {
        let y = cy as i64;
        if y > D_LIM {
            if let Some(mu) = is_squarefree_with_mu(y, all_primes) {
                *part_b += mu as i64;
            }
        }
        let nx = cx * mul_x + cy * mul_y * k as i128;
        let ny = cx * mul_y + cy * mul_x;
        if nx > N_VAL as i128 { break; }
        cx = nx; cy = ny;
    }
}

fn part_b_dfs(
    idx: usize, k: i64, k_limit: i64,
    primes_1mod4: &[i32], all_primes: &[i32],
    part_b: &mut i64,
) {
    solve_pell_and_count(k, part_b, all_primes);
    for i in idx..primes_1mod4.len() {
        let p = primes_1mod4[i] as i64;
        let nk = k * p;
        if nk > k_limit { break; }
        part_b_dfs(i, nk, k_limit, primes_1mod4, all_primes, part_b);
    }
}

fn main() {
    // Sieve for odd numbers
    let half = SIEVE_LIM / 2 + 1;
    let mut sieve_bits = vec![0u8; half];
    sieve_bits[0] = 1;

    let lim = (SIEVE_LIM as f64).sqrt() as usize;
    for i in 1..half {
        let p = 2 * i + 1;
        if p > lim { break; }
        if sieve_bits[i] == 0 {
            let mut j = p * p;
            while j <= SIEVE_LIM {
                sieve_bits[(j - 1) / 2] = 1;
                j += 2 * p;
            }
        }
    }

    let mut all_primes: Vec<i32> = vec![2];
    let mut primes_1mod4: Vec<i32> = Vec::new();
    for i in 1..half {
        let p = (2 * i + 1) as i32;
        if p > SIEVE_LIM as i32 { break; }
        if sieve_bits[i] == 0 {
            all_primes.push(p);
            if p % 4 == 1 { primes_1mod4.push(p); }
        }
    }
    drop(sieve_bits);

    // Pre-compute Hensel-lifted roots for all 1-mod-4 primes
    let roots: Vec<(i64, i64)> = primes_1mod4.iter().map(|&p| mod_sqrt_neg1_p2(p)).collect();

    // Part A: DFS with stack arrays
    let mut part_a_result = 0i64;
    part_a_dfs(0, 1, &[0], 1, &primes_1mod4, &roots, &mut part_a_result);

    // Part B: Pell equation
    let mut part_b_result = 0i64;
    let n128: i128 = N_VAL as i128;
    let k_limit = ((n128 * n128 + 1) / (D_LIM as i128 * D_LIM as i128)) as i64 + 1;

    part_b_dfs(0, 1, k_limit, &primes_1mod4, &all_primes, &mut part_b_result);
    part_b_dfs(0, 2, k_limit, &primes_1mod4, &all_primes, &mut part_b_result);

    let total = part_a_result + part_b_result;
    println!("{}", total);
}
