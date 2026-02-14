// Project Euler 467: Superstring of prime and composite digital roots
//
// Build shortest common supersequence of first N=10000 digital roots
// of primes and composites. Return its digits interpreted as base-10
// number mod 10^9+7.

const NN: usize = 10000;
const MOD: i64 = 1_000_000_007;

fn digital_root(n: usize) -> i32 {
    if n == 0 {
        return 0;
    }
    1 + ((n - 1) % 9) as i32
}

fn main() {
    // Sieve
    let limit = 120000;
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let mut p = vec![0i32; NN];
    let mut c = vec![0i32; NN];
    let mut np = 0usize;
    let mut nc = 0usize;

    let mut n = 2;
    while np < NN || nc < NN {
        if is_prime[n] {
            if np < NN {
                p[np] = digital_root(n);
                np += 1;
            }
        } else if nc < NN {
            c[nc] = digital_root(n);
            nc += 1;
        }
        n += 1;
    }

    // DP arrays
    let stride = NN + 1;
    let mut dp = vec![0i32; stride * stride];
    let mut move_i = vec![0u8; stride * stride];

    // Base cases
    for j in 0..=NN {
        dp[NN * stride + j] = (NN - j) as i32;
    }
    for i in 0..=NN {
        dp[i * stride + NN] = (NN - i) as i32;
    }
    for i in 0..NN {
        move_i[i * stride + NN] = 1;
    }

    for i in (0..NN).rev() {
        let pi = p[i];
        for j in (0..NN).rev() {
            if pi == c[j] {
                dp[i * stride + j] = 1 + dp[(i + 1) * stride + (j + 1)];
            } else {
                let val_i = dp[(i + 1) * stride + j];
                let val_j = dp[i * stride + (j + 1)];
                if val_i <= val_j {
                    dp[i * stride + j] = 1 + val_i;
                    if val_i < val_j || pi < c[j] {
                        move_i[i * stride + j] = 1;
                    }
                } else {
                    dp[i * stride + j] = 1 + val_j;
                }
            }
        }
    }

    // Reconstruct
    let mut ans: i64 = 0;
    let mut ii = 0usize;
    let mut jj = 0usize;
    while ii < NN || jj < NN {
        let idx = ii * stride + jj;
        let digit;
        if ii < NN && jj < NN && p[ii] == c[jj] {
            digit = p[ii] as i64;
            ii += 1;
            jj += 1;
        } else if move_i[idx] != 0 {
            digit = p[ii] as i64;
            ii += 1;
        } else {
            digit = c[jj] as i64;
            jj += 1;
        }
        ans = (10 * ans + digit) % MOD;
    }

    println!("{}", ans);
}
