// Project Euler 374 - Maximum Integer Partition Product

const MOD: u64 = 982_451_653;

fn main() {
    let n_big: i64 = 100_000_000_000_000; // 10^14

    // Find K such that T_K <= N < T_{K+1}
    let mut lo: i64 = 1;
    let mut hi: i64 = 20_000_000;
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if (mid as i128) * (mid as i128 + 1) / 2 <= n_big as i128 {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    let k_val = lo;
    let max_k = k_val + 10;

    let sz = (max_k + 3) as usize;

    // Precompute factorials mod MOD
    let mut fact = vec![0u64; sz];
    fact[0] = 1;
    for i in 1..sz {
        fact[i] = fact[i - 1] * (i as u64) % MOD;
    }

    // Precompute modular inverses using linear sieve
    let mut inv = vec![0u64; sz];
    inv[1] = 1;
    for i in 2..sz {
        inv[i] = (MOD - MOD / (i as u64)) * inv[(MOD % (i as u64)) as usize] % MOD;
    }

    // Precompute harmonic sums
    let mut harmonic = vec![0u64; sz];
    for i in 2..sz {
        harmonic[i] = (harmonic[i - 1] + inv[i]) % MOD;
    }

    let mut total: u64 = 0;

    // Handle k=1: n=1,2
    if n_big >= 1 {
        total = (total + 1) % MOD;
    }
    if n_big >= 2 {
        total = (total + 2) % MOD;
    }

    // Handle k=2: n=3,4,5
    if n_big >= 3 {
        total = (total + 3) % MOD;
    }
    if n_big >= 4 {
        total = (total + 4) % MOD;
    }
    if n_big >= 5 {
        total = (total + 12) % MOD;
    }

    let inv2 = inv[2];

    for k in 3..=k_val {
        let t_k = k * (k + 1) / 2;
        let mut r_max = if k < k_val {
            k
        } else {
            n_big - t_k
        };
        if r_max > k {
            r_max = k;
        }

        // Case 1: r from 0 to min(k-2, r_max)
        let r1 = std::cmp::min(k - 2, r_max);
        if r1 >= 0 {
            let j_min = k - r1;
            let sum_inv = if j_min <= 1 {
                (1 + harmonic[k as usize]) % MOD
            } else {
                (harmonic[k as usize] + MOD - harmonic[(j_min - 1) as usize]) % MOD
            };
            let contrib = fact[(k + 1) as usize] % MOD
                * ((k - 1) as u64 % MOD) % MOD
                * sum_inv % MOD;
            total = (total + contrib) % MOD;
        }

        // Case 2: r = k-1
        if r_max >= k - 1 {
            let contrib = fact[(k + 2) as usize] % MOD
                * ((k - 1) as u64 % MOD) % MOD;
            let contrib = contrib * inv2 % MOD;
            let contrib = contrib * inv[(k + 1) as usize] % MOD;
            total = (total + contrib) % MOD;
        }

        // Case 3: r = k
        if r_max >= k {
            let contrib = fact[(k + 1) as usize] % MOD * (k as u64 % MOD) % MOD;
            total = (total + contrib) % MOD;
        }
    }

    println!("{}", total);
}
