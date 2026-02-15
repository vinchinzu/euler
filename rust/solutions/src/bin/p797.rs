// Project Euler 797 - Cyclotomic Polynomials
// Sieve division for F_n(2), then G_n(2) = product of (F_d(2)+1) for d|n.
// Total = sum_{n=1}^N Mertens(N/n) * G_n(2).

const N: usize = 10_000_000;
const MOD: i64 = 1_000_000_007;

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base %= m;
    if base < 0 { base += m; }
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn mod_inv(a: i64, m: i64) -> i64 {
    pow_mod(a, m - 2, m)
}

fn main() {
    // Mobius function
    let mut is_prime = vec![true; N + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut mu = vec![1i32; N + 1];

    for i in 2..=N {
        if is_prime[i] {
            for j in (i..=N).step_by(i) {
                if j > i { is_prime[j] = false; }
                if (j / i) % i == 0 {
                    mu[j] = 0;
                } else {
                    mu[j] = -mu[j];
                }
            }
        }
    }

    // Mertens function
    let mut mertens = vec![0i64; N + 1];
    for i in 1..=N {
        mertens[i] = mertens[i - 1] + mu[i] as i64;
    }

    // F[n] = cyclotomic polynomial at 2, computed by sieve division
    let mut f = vec![0i64; N + 1];
    for i in 0..=N {
        f[i] = (pow_mod(2, i as i64, MOD) - 1 + MOD) % MOD;
    }
    for i in 1..=N {
        let inv = mod_inv(f[i], MOD);
        let mut j = 2 * i;
        while j <= N {
            f[j] = f[j] * inv % MOD;
            j += i;
        }
    }

    // G[n] = product of (F[d]+1) for all d|n
    let mut g = vec![1i64; N + 1];
    for i in 1..=N {
        let factor = (f[i] + 1) % MOD;
        let mut j = i;
        while j <= N {
            g[j] = g[j] * factor % MOD;
            j += i;
        }
    }

    // Sum contributions
    let mut ans: i64 = 0;
    for i in 1..=N {
        ans = (ans + ((mertens[N / i] % MOD as i64 + MOD as i64) % MOD as i64) * g[i] % MOD) % MOD;
    }

    println!("{}", ans);
}
