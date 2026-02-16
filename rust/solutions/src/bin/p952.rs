// Project Euler Problem 952
// Order Modulo Factorial.
// R(p, n) = multiplicative order of p mod n!.
// Find R(10^9+7, 10^7) mod (10^9+7).

fn main() {
    let n: usize = 10_000_000;
    let p: u64 = 1_000_000_007;
    let mod_ans: u64 = 1_000_000_007;

    // Compute smallest prime factor sieve
    let mut spf = vec![0u32; n + 1];
    for i in 2..=n {
        if spf[i] == 0 {
            spf[i] = i as u32;
            if (i as u64) * (i as u64) <= n as u64 {
                let mut j = i * i;
                while j <= n {
                    if spf[j] == 0 {
                        spf[j] = i as u32;
                    }
                    j += i;
                }
            }
        }
    }

    let mut exponents = vec![0i32; n + 1];

    // Inline mod_pow for performance (modulus can be up to ~2*10^18 for large q powers)
    let mod_pow = |mut base: u64, mut exp: u64, modulus: u64| -> u64 {
        if modulus == 1 {
            return 0;
        }
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
    };

    let get_ord = |p: u64, q: u32| -> u32 {
        if q == 2 {
            return 1;
        }
        let t = q - 1;
        let mut curr_ord = t;
        let mut temp = t;
        while temp > 1 {
            let f = spf[temp as usize];
            while curr_ord % f == 0 && mod_pow(p, curr_ord as u64 / f as u64, q as u64) == 1 {
                curr_ord /= f;
            }
            while temp % f == 0 {
                temp /= f;
            }
        }
        curr_ord
    };

    let get_legendre = |n: usize, p: usize| -> i32 {
        let mut count = 0i32;
        let mut pp = p as u64;
        while pp <= n as u64 {
            count += (n as u64 / pp) as i32;
            pp = pp.saturating_mul(p as u64);
            if pp == 0 {
                break;
            }
        }
        count
    };

    // Handle q = 2
    let k2 = get_legendre(n, 2);
    if k2 >= 4 {
        exponents[2] = k2 - 3;
    } else if k2 >= 2 {
        exponents[2] = 1;
    }

    // Iterate odd primes
    for q in 3..=n {
        if spf[q] != q as u32 {
            continue;
        }

        let d_q = get_ord(p, q as u32);

        // Update exponents from d_q's factorization
        let mut temp = d_q;
        while temp > 1 {
            let ell = spf[temp as usize];
            let mut cnt = 0i32;
            while temp % ell == 0 {
                cnt += 1;
                temp /= ell;
            }
            if exponents[ell as usize] < cnt {
                exponents[ell as usize] = cnt;
            }
        }

        // Compute v_q = q-adic val of p^{d_q} - 1
        let k_q = get_legendre(n, q);
        if k_q == 0 {
            continue;
        }

        let mut v_q = 1i32;
        let mut curr_mod = q as u64 * q as u64;
        let mut rem = mod_pow(p, d_q as u64, curr_mod);
        while rem == 1 {
            v_q += 1;
            if v_q >= k_q {
                break;
            }
            if curr_mod > 2_000_000_000_000_000_000u64 / q as u64 {
                break; // overflow guard
            }
            curr_mod *= q as u64;
            rem = mod_pow(p, d_q as u64, curr_mod);
        }

        let a_q = (k_q - v_q).max(0);
        if exponents[q] < a_q {
            exponents[q] = a_q;
        }
    }

    // Final answer
    let mut ans: u64 = 1;
    for ell in 2..=n {
        if exponents[ell] > 0 {
            let term = mod_pow(ell as u64, exponents[ell] as u64, mod_ans);
            ans = (ans as u128 * term as u128 % mod_ans as u128) as u64;
        }
    }

    println!("{}", ans);
}
