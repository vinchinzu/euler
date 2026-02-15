// Project Euler 902 - Permutation rank sum
// Build sigma/tau permutations for m=100, compose pi = tau_inv o sigma o tau,
// find order d, compute rank sum using Fenwick tree, then multiply by m!/d mod MOD

const MOD: u64 = 1_000_000_007;

fn power_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

fn triangular(k: usize) -> usize {
    k * (k + 1) / 2
}

fn compose_into(p1: &[usize], p2: &[usize], result: &mut [usize]) {
    for i in 0..p2.len() {
        result[i] = p1[p2[i] - 1];
    }
}

fn rank_perm(perm: &[usize], fact: &[u64], bit: &mut [i32]) -> u64 {
    let n = perm.len();
    // Reset BIT
    for v in bit.iter_mut() { *v = 0; }
    for i in 1..=n {
        let mut j = i;
        while j <= n {
            bit[j] += 1;
            j += j & j.wrapping_neg();
        }
    }

    let mut rank = 0u64;
    for i in 0..n {
        let val = perm[i] - 1;
        let mut count = 0i32;
        let mut j = val;
        while j > 0 {
            count += bit[j];
            j -= j & j.wrapping_neg();
        }
        rank = (rank + count as u64 % MOD * fact[n - 1 - i]) % MOD;
        let mut j = perm[i];
        while j <= n {
            bit[j] -= 1;
            j += j & j.wrapping_neg();
        }
    }
    (rank + 1) % MOD
}

fn main() {
    let m = 100usize;
    let n = triangular(m); // 5050

    // Build sigma
    let mut sigma: Vec<usize> = (1..=n).collect();
    for k in 1..=m {
        let pos = triangular(k);
        sigma[pos - 1] = triangular(k - 1) + 1;
    }

    // Build tau
    let a = 1_000_000_007u64;
    let tau: Vec<usize> = (1..=n as u64)
        .map(|i| ((a * i) % n as u64) as usize + 1)
        .collect();

    // Build tau inverse
    let mut tau_inv = vec![0usize; n];
    for i in 0..n {
        tau_inv[tau[i] - 1] = i + 1;
    }

    // pi = tau_inv o sigma o tau
    let mut temp = vec![0usize; n];
    let mut pi = vec![0usize; n];
    compose_into(&sigma, &tau, &mut temp);
    compose_into(&tau_inv, &temp, &mut pi);

    // Precompute factorials mod MOD
    let mut fact = vec![0u64; n + 1];
    fact[0] = 1;
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as u64 % MOD;
    }

    // Pre-allocate buffers
    let mut bit = vec![0i32; n + 2];
    let mut current = pi.clone();
    let mut next_perm = vec![0usize; n];
    let identity: Vec<usize> = (1..=n).collect();

    let mut sum_r = rank_perm(&current, &fact, &mut bit);
    let mut d = 1usize;

    loop {
        if current == identity {
            break;
        }
        compose_into(&current, &pi, &mut next_perm);
        std::mem::swap(&mut current, &mut next_perm);
        d += 1;
        sum_r = (sum_r + rank_perm(&current, &fact, &mut bit)) % MOD;
    }

    // q = m!/d mod MOD
    let factorial_m = fact[m];
    let q_mod = factorial_m * power_mod(d as u64, MOD - 2, MOD) % MOD;

    let total = q_mod * sum_r % MOD;
    println!("{}", total);
}
