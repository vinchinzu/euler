// Project Euler 495 - Writing n! as product of k distinct integers
// Inclusion-exclusion over partitions of K=30.

use euler_utils::mod_pow;

const N: usize = 10000;
const K: usize = 30;
const MOD: u64 = 1_000_000_007;

fn vp_factorial(n: usize, p: usize) -> usize {
    let mut count = 0;
    let mut pw = p as u64;
    while pw <= n as u64 {
        count += n / pw as usize;
        pw *= p as u64;
    }
    count
}

fn main() {
    // Sieve primes up to N
    let mut is_prime = vec![true; N + 1];
    is_prime[0] = false;
    if N >= 1 { is_prime[1] = false; }
    let mut i = 2;
    while i * i <= N {
        if is_prime[i] {
            let mut j = i * i;
            while j <= N {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let primes: Vec<usize> = (2..=N).filter(|&i| is_prime[i]).collect();
    let nprimes = primes.len();

    // Compute exponents[i] = v_p(N!) for each prime
    let exponents: Vec<usize> = primes.iter().map(|&p| vp_factorial(N, p)).collect();

    // Precompute modular inverses
    let mut inv_val = vec![0u64; K + 1];
    inv_val[1] = 1;
    for i in 2..=K {
        inv_val[i] = (MOD - MOD / i as u64) % MOD * inv_val[(MOD % i as u64) as usize] % MOD;
    }

    // Precompute inverse factorials
    let mut fact = 1u64;
    for i in 1..=K {
        fact = fact * i as u64 % MOD;
    }
    let mut inv_fact = vec![0u64; K + 1];
    inv_fact[K] = mod_pow(fact, MOD - 2, MOD);
    for i in (0..K).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i + 1) as u64 % MOD;
    }

    // DP over partitions
    // dp[depth][e] = number of ways to represent exponent e using coins so far
    let mut dp = vec![vec![0u64; N + 1]; K + 2];
    dp[0][0] = 1;

    let mut coins = vec![0usize; K + 1];
    let mut ans = 0u64;

    fn helper(
        min_val: usize,
        remaining: usize,
        depth: usize,
        dp: &mut Vec<Vec<u64>>,
        coins: &mut Vec<usize>,
        exponents: &[usize],
        nprimes: usize,
        inv_val: &[u64],
        inv_fact: &[u64],
        ans: &mut u64,
    ) {
        if remaining == 0 {
            let mut res = 1u64;
            for i in 0..nprimes {
                res = res * dp[depth][exponents[i]] % MOD;
            }
            for i in 0..depth {
                let c = coins[i];
                if c % 2 == 0 {
                    res = res * (MOD - 1) % MOD;
                }
                res = res * inv_val[c] % MOD;
            }
            let mut i = 0;
            while i < depth {
                let mut j = i;
                while j < depth && coins[j] == coins[i] {
                    j += 1;
                }
                let cnt = j - i;
                res = res * inv_fact[cnt] % MOD;
                i = j;
            }
            *ans = (*ans + res) % MOD;
            return;
        }

        for coeff in min_val..=remaining {
            let max_exp = N;
            let (left, right) = dp.split_at_mut(depth + 1);
            right[0][..=max_exp].copy_from_slice(&left[depth][..=max_exp]);
            for e in coeff..=max_exp {
                right[0][e] = (right[0][e] + right[0][e - coeff]) % MOD;
            }
            coins[depth] = coeff;
            helper(
                coeff,
                remaining - coeff,
                depth + 1,
                dp,
                coins,
                exponents,
                nprimes,
                inv_val,
                inv_fact,
                ans,
            );
        }
    }

    helper(
        1,
        K,
        0,
        &mut dp,
        &mut coins,
        &exponents,
        nprimes,
        &inv_val,
        &inv_fact,
        &mut ans,
    );

    println!("{}", ans);
}
