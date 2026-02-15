// Project Euler 745 - Sum of Squares
//
// Sum g(n) for n=1..N where g(n) is the largest perfect square dividing n.
// Uses sieve for square-free counts and Lucy-like DP.

const MOD: i64 = 1_000_000_007;

fn isqrt_ll(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

fn icbrt_ll(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut r = (n as f64).cbrt() as i64;
    while r > 0 && r * r * r > n { r -= 1; }
    while (r + 1) * (r + 1) * (r + 1) <= n { r += 1; }
    r
}

fn main() {
    let n: i64 = 100_000_000_000_000; // 10^14

    let mut l = (n as f64).powf(2.0 / 7.0) as i64;
    while (l + 1) * (l + 1) * (l + 1) <= n { l += 1; }

    let l2 = n / (l * l);

    // Sieve square-free numbers up to l2
    let mut is_sq_free = vec![true; l2 as usize + 1];
    let sq_lim = isqrt_ll(l2);
    for i in 2..=sq_lim {
        let isq = i * i;
        let mut j = isq;
        while j <= l2 {
            is_sq_free[j as usize] = false;
            j += isq;
        }
    }

    // Prefix sums of square-free counts
    let mut small = vec![0i64; l2 as usize + 1];
    for i in 1..=l2 as usize {
        small[i] = small[i - 1] + if is_sq_free[i] { 1 } else { 0 };
    }

    // big[i] = number of square-free numbers up to n/(i*i)
    let mut big = vec![0i64; l as usize + 1];
    for i in (1..=l as usize).rev() {
        let n_val = n / (i as i64 * i as i64);
        big[i] = n_val;
        let lim = icbrt_ll(n_val);
        let sq_lim2 = isqrt_ll(if lim > 0 { n_val / lim } else { n_val });
        for k in 2..=sq_lim2 {
            let k_sq = k * k;
            let ik = i as i64 * k;
            if ik <= l {
                big[i] -= big[ik as usize];
            } else {
                big[i] -= small[(n_val / k_sq) as usize];
            }
        }
        for t in 1..lim {
            big[i] -= (isqrt_ll(n_val / t) - isqrt_ll(n_val / (t + 1))) * small[t as usize];
        }
    }

    let mut ans: i64 = 0;
    let mut k: i64 = 1;
    while k * k <= n {
        let k_sq = k * k % MOD;
        if k <= l {
            ans = (ans as i128 + k_sq as i128 * (big[k as usize] % MOD) as i128 % MOD as i128) as i64 % MOD;
        } else {
            ans = (ans as i128 + k_sq as i128 * (small[(n / (k * k)) as usize] % MOD) as i128 % MOD as i128) as i64 % MOD;
        }
        k += 1;
    }

    println!("{}", ((ans % MOD) + MOD) % MOD);
}
