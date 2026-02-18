// Problem 798 — Card Stacking Game
//
// C(n,s) = number of initial card sets where first player loses with optimal play.
// Uses XOR-convolution (Walsh–Hadamard transform) over single-suit Grundy distributions.
//
// Algorithm:
//   1. Build f[g] = number of initial visible sets for a single suit with Grundy value g.
//   2. WHT of f.
//   3. C(n,s) = (1/L) * sum_t (f_hat[t])^s  (mod MOD), L = next power of 2 >= n.

const MOD: u64 = 1_000_000_007;

/// Fast modular exponentiation using u64 only (MOD < 2^32, so a*b < 2^64).
#[inline]
fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

/// Binomial coefficient C(n, k) mod MOD using precomputed factorials.
#[inline]
fn nck(n: usize, k: usize, fact: &[u64], inv_fact: &[u64]) -> u64 {
    if k > n {
        return 0;
    }
    fact[n] * inv_fact[k] % MOD * inv_fact[n - k] % MOD
}

/// Build factorial and inverse factorial tables up to max_n.
fn build_factorials(max_n: usize) -> (Vec<u64>, Vec<u64>) {
    let mut fact = vec![1u64; max_n + 1];
    for i in 2..=max_n {
        fact[i] = fact[i - 1] * (i as u64) % MOD;
    }
    let mut inv_fact = vec![1u64; max_n + 1];
    inv_fact[max_n] = pow_mod(fact[max_n], MOD - 2, MOD);
    for i in (1..=max_n).rev() {
        inv_fact[i - 1] = inv_fact[i] * (i as u64) % MOD;
    }
    (fact, inv_fact)
}

/// In-place Walsh–Hadamard transform over XOR, modulo MOD.
fn fwht_xor_inplace(a: &mut [u64]) {
    let n = a.len();
    let mut h = 1usize;
    while h < n {
        let step = h << 1;
        let mut i = 0;
        while i < n {
            for j in i..i + h {
                // SAFETY: j < i + h, j + h < i + step <= n, indices are in bounds.
                let x = unsafe { *a.get_unchecked(j) };
                let y = unsafe { *a.get_unchecked(j + h) };
                let u = {
                    let s = x + y;
                    if s >= MOD { s - MOD } else { s }
                };
                let v = if x >= y { x - y } else { MOD - y + x };
                unsafe {
                    *a.get_unchecked_mut(j) = u;
                    *a.get_unchecked_mut(j + h) = v;
                }
            }
            i += step;
        }
        h = step;
    }
}

/// Q(X, k) = X*C(X+k+1, k+1) - (k+1)*C(X+k+1, k+2)
#[inline]
fn q_of(x: usize, k: usize, fact: &[u64], inv_fact: &[u64]) -> u64 {
    let n1 = x + k + 1;
    let c1 = nck(n1, k + 1, fact, inv_fact);
    let c2 = nck(n1, k + 2, fact, inv_fact);
    let term1 = (x as u64) % MOD * c1 % MOD;
    let term2 = ((k + 1) as u64) % MOD * c2 % MOD;
    (term1 + MOD - term2) % MOD
}

/// Build the single-suit Grundy distribution f[0..L-1], padded with zeros to length L.
fn build_single_suit_distribution(n: usize, len: usize) -> Vec<u64> {
    let mut a = vec![0u64; len];
    if n == 0 {
        return a;
    }
    if n == 1 {
        a[0] = 2; // empty set or {1}; both Grundy 0
        return a;
    }

    let (fact, inv_fact) = build_factorials(n);

    // Base cases
    let a0 = (pow_mod(2, (n - 2) as u64, MOD) + 2) % MOD;
    let a1 = (pow_mod(2, (n - 2) as u64, MOD) + (n as u64 - 2)) % MOD;
    a[0] = a0;
    if n > 1 {
        a[1] = a1;
    }
    if n > 2 {
        a[2] = (pow_mod(2, (n - 3) as u64, MOD) + (n as u64 - 3)) % MOD;
    }

    let inv4 = pow_mod(4, MOD - 2, MOD);

    // Odd Grundy values: g = 2k + 3, starting at (X0 = n-4, k = 0)
    if n > 3 {
        let x0 = n - 4;
        let mut k: usize = 0;
        let mut x = x0;
        // F(X, 0) = 2^(X+1) - 1
        let mut f_val = (pow_mod(2, (x + 1) as u64, MOD) + MOD - 1) % MOD;
        loop {
            let g = 2 * k + 3;
            if g >= n || x == usize::MAX {
                // x < 0 represented as usize::MAX after wrapping
                break;
            }
            a[g] = (f_val + q_of(x, k, &fact, &inv_fact)) % MOD;

            if x < 2 {
                break;
            }
            // Compute F(X-2, k)
            let c_xk_1 = nck(x + k - 1, k, &fact, &inv_fact);
            let c_xk = nck(x + k, k, &fact, &inv_fact);
            let tmp = (f_val + MOD - 2 * c_xk_1 % MOD + MOD - c_xk) % MOD;
            let tmp = tmp * inv4 % MOD;
            // Compute F(X-2, k+1)
            let c_next = nck(x + k - 1, k + 1, &fact, &inv_fact);
            f_val = (2 * tmp % MOD + MOD - c_next) % MOD;
            k += 1;
            x -= 2;
        }
    }

    // Even Grundy values: g = 2k + 4, starting at (X0 = n-5, k = 0)
    if n > 4 {
        let x0 = n - 5;
        let mut k: usize = 0;
        let mut x = x0;
        let mut f_val = (pow_mod(2, (x + 1) as u64, MOD) + MOD - 1) % MOD;
        loop {
            let g = 2 * k + 4;
            if g >= n || x == usize::MAX {
                break;
            }
            a[g] = (f_val + q_of(x, k, &fact, &inv_fact)) % MOD;

            if x < 2 {
                break;
            }
            let c_xk_1 = nck(x + k - 1, k, &fact, &inv_fact);
            let c_xk = nck(x + k, k, &fact, &inv_fact);
            let tmp = (f_val + MOD - 2 * c_xk_1 % MOD + MOD - c_xk) % MOD;
            let tmp = tmp * inv4 % MOD;
            let c_next = nck(x + k - 1, k + 1, &fact, &inv_fact);
            f_val = (2 * tmp % MOD + MOD - c_next) % MOD;
            k += 1;
            x -= 2;
        }
    }

    // Drop fact/inv_fact before returning to free memory
    drop(fact);
    drop(inv_fact);

    a
}

fn compute_c(n: usize, s: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    // L = next power of 2 >= n
    let l = if n.is_power_of_two() {
        n
    } else {
        n.next_power_of_two()
    };

    let mut f = build_single_suit_distribution(n, l);

    fwht_xor_inplace(&mut f);

    // Pointwise exponentiation and sum
    let mut total = 0u64;
    for (i, &v) in f.iter().enumerate() {
        total += pow_mod(v, s, MOD);
        if i & 8191 == 0 {
            total %= MOD;
        }
    }
    total %= MOD;

    let inv_l = pow_mod(l as u64, MOD - 2, MOD);
    total * inv_l % MOD
}

fn main() {
    // Self-test with problem statement examples
    debug_assert_eq!(compute_c(3, 2), 26);
    debug_assert_eq!(compute_c(13, 4), 540318329);

    let n = 10_000_000;
    let s = 10_000_000u64;
    println!("{}", compute_c(n, s));
}
