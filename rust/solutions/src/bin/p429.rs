// Project Euler 429 - Sum of squares of unitary divisors of N!
// S(N) = prod_p (1 + p^{2*c(N,p)}) mod 1000000009
// where c(N,p) counts factors of p in N! (Legendre's formula).

const N_VAL: u64 = 100_000_000;
const MOD: u64 = 1_000_000_009;

fn pow_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % modulus as u128) as u64;
        }
        base = (base as u128 * base as u128 % modulus as u128) as u64;
        exp >>= 1;
    }
    result
}

fn main() {
    let n = N_VAL as usize;

    // Bit sieve for odd numbers
    let half = n / 2;
    let mut sieve = vec![0u8; half / 8 + 1];

    macro_rules! is_composite {
        ($i:expr) => { sieve[$i >> 3] & (1 << ($i & 7)) != 0 }
    }
    macro_rules! set_composite {
        ($i:expr) => { sieve[$i >> 3] |= 1 << ($i & 7) }
    }

    {
        let mut i = 3usize;
        while i * i <= n {
            if !is_composite!(i / 2) {
                let mut j = i * i;
                while j <= n {
                    set_composite!(j / 2);
                    j += 2 * i;
                }
            }
            i += 2;
        }
    }

    let mut ans: u64 = 1;

    // Handle p=2
    {
        let mut c: u64 = 0;
        let mut power = 2u64;
        while power <= N_VAL {
            c += N_VAL / power;
            power *= 2;
        }
        ans = ans * ((1 + pow_mod(2, 2 * c, MOD)) % MOD) % MOD;
    }

    // Handle odd primes
    {
        let mut p = 3u64;
        while p <= N_VAL {
            if !is_composite!(p as usize / 2) {
                let mut c: u64 = 0;
                let mut power = p;
                while power <= N_VAL {
                    c += N_VAL / power;
                    if power > N_VAL / p { break; } // prevent overflow
                    power *= p;
                }
                ans = ans * ((1 + pow_mod(p, 2 * c, MOD)) % MOD) % MOD;
            }
            p += 2;
        }
    }

    println!("{}", ans);
}
