// Project Euler 423 - Consecutive die throws
// C(n) = number of outcomes of n die throws where consecutive identical pairs <= pi(n).
// Uses sieve + modular arithmetic recurrence.

const N: usize = 50_000_000;
const K: u64 = 6;
const MOD: u64 = 1_000_000_007;

fn main() {
    // Sieve of Eratosthenes
    let mut is_prime = vec![true; N + 1];
    is_prime[0] = false;
    if N >= 1 { is_prime[1] = false; }
    {
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
    }

    // Modular inverses
    let mut inv = vec![0u64; N + 1];
    inv[1] = 1;
    for i in 2..=N {
        inv[i] = (MOD - MOD / i as u64 * inv[(MOD % i as u64) as usize] % MOD) % MOD;
    }

    let mut f: u64 = 1;
    let mut r: u64 = 1;
    let mut pi_n: u64 = 0;

    let mut ans = (K * f) % MOD; // C(1) = 6

    for n in 2..=N {
        if is_prime[n] {
            let r_new = r % MOD * ((n as u64 - 1) % MOD) % MOD * inv[(pi_n + 1) as usize] % MOD;
            f = (K * f % MOD + MOD - r % MOD + r_new) % MOD;
            r = r_new;
            pi_n += 1;
        } else {
            let f_new = (K * f % MOD + MOD - r % MOD) % MOD;
            let r_new = if (n as u64 - 1) > pi_n {
                r * ((n as u64 - 1) % MOD) % MOD * ((K - 1) % MOD) % MOD * inv[(n as u64 - 1 - pi_n) as usize] % MOD
            } else {
                1
            };
            f = f_new;
            r = r_new;
        }

        let c_n = K * f % MOD;
        ans = (ans + c_n) % MOD;
    }

    println!("{}", ans);
}
