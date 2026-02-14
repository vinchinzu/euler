// Project Euler 441 - The inverse summation of coprime pairs
//
// Compute sum_{i=2}^N R(i), using the formula:
// ans = (sum_{g=1}^N mu(g) * (H(N/g)/g)^2 + N - 3) / 2
// where H(k) is the k-th harmonic number.

const N: usize = 10_000_000;

fn main() {
    // Sieve Mobius function
    let mut mu = vec![1i8; N + 1];
    let mut is_prime = vec![true; N + 1];
    is_prime[0] = false;
    if N >= 1 {
        is_prime[1] = false;
    }

    for i in 2..=N {
        if is_prime[i] {
            for j in (i..=N).step_by(i) {
                if j > i {
                    is_prime[j] = false;
                }
                mu[j] = -mu[j];
            }
            let i2 = i * i;
            if i2 <= N {
                for j in (i2..=N).step_by(i2) {
                    mu[j] = 0;
                }
            }
        }
    }

    // Compute harmonic numbers
    let mut harmonics = vec![0.0f64; N + 1];
    for i in 1..=N {
        harmonics[i] = harmonics[i - 1] + 1.0 / i as f64;
    }

    // Main computation
    let mut ans = 0.0f64;
    for g in 1..=N {
        if mu[g] != 0 {
            let h_val = harmonics[N / g] / g as f64;
            ans += mu[g] as f64 * h_val * h_val;
        }
    }

    ans = (ans + N as f64 - 3.0) / 2.0;
    println!("{:.4}", ans);
}
