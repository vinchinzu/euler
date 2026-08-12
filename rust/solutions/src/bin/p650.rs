// Project Euler 650 - Divisors of Binomial Product
// B(n) = product C(n,k). D(n) = sigma(B(n)). S(N) = sum D(n) mod 10^9+7.
// Optimization: pure u64 modular arithmetic (MOD < 2^30, products fit in u64).

const MAXN: usize = 20001;
const N_VAL: usize = 20000;
const MOD: u64 = 1_000_000_007;

#[inline(always)]
fn power(mut base: u64, mut exp: u64) -> u64 {
    let mut r = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            r = r * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    r
}

fn main() {
    // Sieve smallest prime factor
    let mut spf = vec![0i32; MAXN];
    let mut primes_arr: Vec<i32> = Vec::new();
    let mut pidx = vec![0usize; MAXN];

    for i in 2..=N_VAL {
        if spf[i] == 0 {
            spf[i] = i as i32;
            pidx[i] = primes_arr.len();
            primes_arr.push(i as i32);
        }
        for j in 0..primes_arr.len() {
            let pj = primes_arr[j];
            if pj > spf[i] || (i as i64) * (pj as i64) > N_VAL as i64 {
                break;
            }
            spf[i * pj as usize] = pj;
        }
    }

    let nprimes = primes_arr.len();
    let mut inv_pm1 = vec![0u64; nprimes];
    for i in 0..nprimes {
        inv_pm1[i] = power(primes_arr[i] as u64 - 1, MOD - 2);
    }

    let mut b_exp = vec![0i64; nprimes];
    let mut fact_exp = vec![0i64; nprimes];
    let mut d_factor = vec![1u64; nprimes];

    let mut answer = 0u64;

    for n in 1..=N_VAL {
        // Subtract previous factorial exponents from B exponents
        for i in 0..nprimes {
            if primes_arr[i] as usize >= n {
                break;
            }
            if fact_exp[i] > 0 {
                b_exp[i] -= fact_exp[i];
                if b_exp[i] > 0 {
                    d_factor[i] = (power(primes_arr[i] as u64, (b_exp[i] + 1) as u64) + MOD - 1) % MOD
                        * inv_pm1[i]
                        % MOD;
                } else {
                    d_factor[i] = 1;
                }
            }
        }

        // Factor n and update B exponents
        {
            let mut m = n;
            while m > 1 {
                let p = spf[m] as usize;
                let mut e = 0i64;
                while m % p == 0 {
                    m /= p;
                    e += 1;
                }
                let pi = pidx[p];
                b_exp[pi] += (n as i64 - 1) * e;
                fact_exp[pi] += e;
                if b_exp[pi] > 0 {
                    d_factor[pi] = (power(p as u64, (b_exp[pi] + 1) as u64) + MOD - 1) % MOD
                        * inv_pm1[pi]
                        % MOD;
                } else {
                    d_factor[pi] = 1;
                }
            }
        }

        // Compute D(n)
        let mut d = 1u64;
        for i in 0..nprimes {
            if primes_arr[i] as usize > n {
                break;
            }
            d = d * d_factor[i] % MOD;
        }

        answer = (answer + d) % MOD;
    }

    println!("{}", answer);
}
