// Project Euler 888
// Nim-like game with nimber computation, periodicity detection, and DP.

const N_VAL: usize = 12_491_249;
const K: usize = 1249;
const M: i64 = 912_491_249;
const L: usize = 25_000;

fn power_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn main() {
    let ds = [1usize, 2, 4, 9];

    // Compute nimbers
    let mut nimbers = vec![0i32; L];
    for n in 0..L {
        let mut used = vec![false; L + 1];

        for &d in &ds {
            if d <= n && (nimbers[n - d] as usize) < used.len() {
                used[nimbers[n - d] as usize] = true;
            }
        }
        // Split: pile n into (i, n-i)
        for i in 1..=n / 2 {
            let v = (nimbers[i] ^ nimbers[n - i]) as usize;
            if v < used.len() { used[v] = true; }
        }
        if n > 0 && n % 2 == 1 {
            let i = (n + 1) / 2;
            let v = (nimbers[i] ^ nimbers[n - i]) as usize;
            if v < used.len() { used[v] = true; }
        }

        let mut nimber = 0;
        while nimber < used.len() && used[nimber] { nimber += 1; }
        nimbers[n] = nimber as i32;
    }

    // Find max nimber and cap
    let max_nimber = *nimbers.iter().max().unwrap() as usize;
    let mut cap = 1;
    while cap < max_nimber { cap *= 2; }

    // Find period
    let half = L / 2;
    let mut period = 0;
    for start in 0..half {
        let mut matched = true;
        for j in 0..half {
            if nimbers[start + j] != nimbers[half + j] { matched = false; break; }
        }
        if matched { period = half - start; break; }
    }

    // Compute counts
    let mut counts = vec![0i64; cap];
    for n in 1..period {
        counts[nimbers[n] as usize] += 1;
    }
    for n in 0..period {
        counts[nimbers[n + period] as usize] += (N_VAL - n) as i64 / period as i64;
    }

    // Precompute factorials and inverse factorials
    let mut max_val = 0i64;
    for i in 0..cap { if counts[i] > max_val { max_val = counts[i]; } }
    max_val += K as i64 + 10;

    let mut fact = vec![0i64; max_val as usize + 1];
    let mut inv_fact = vec![0i64; max_val as usize + 1];
    fact[0] = 1;
    for i in 1..=max_val as usize { fact[i] = fact[i - 1] * i as i64 % M; }
    inv_fact[max_val as usize] = power_mod(fact[max_val as usize], M - 2, M);
    for i in (0..max_val as usize).rev() { inv_fact[i] = inv_fact[i + 1] * (i as i64 + 1) % M; }

    // DP: dp[xor_val][num_piles]
    let mut prev_dp = vec![vec![0i64; K + 1]; cap];
    prev_dp[0][0] = 1;

    for nimber in 0..cap {
        let mut next_dp = vec![vec![0i64; K + 1]; cap];

        for d in 0..=K {
            let mult = if d == 0 {
                1i64
            } else {
                // nCr(counts[nimber] + d - 1, d)
                let cn = counts[nimber] + d as i64 - 1;
                if cn < d as i64 || cn < 0 {
                    0
                } else if cn <= max_val {
                    fact[cn as usize] * inv_fact[d] % M * inv_fact[(cn - d as i64) as usize] % M
                } else {
                    let mut m = 1i64;
                    for j in 0..d {
                        m = m * ((counts[nimber] + d as i64 - 1 - j as i64) % M) % M;
                    }
                    m * inv_fact[d] % M
                }
            };
            if mult == 0 { continue; }

            for n in 0..cap {
                let new_n_xor = if d % 2 == 0 { n } else { n ^ nimber };
                for k in 0..=K - d {
                    if prev_dp[n][k] != 0 {
                        next_dp[new_n_xor][k + d] = (next_dp[new_n_xor][k + d] + mult * prev_dp[n][k]) % M;
                    }
                }
            }
        }

        prev_dp = next_dp;
    }

    println!("{}", prev_dp[0][K]);
}
