// Project Euler 590 - Sets with a Given Least Common Multiple
//
// H(n) = number of non-empty sets with lcm = n.
// Find HL(50000) mod 10^9 where L(n) = lcm(1,...,n).
//
// Uses CRT (mod 512 and mod 5^9) with DP over big primes
// and binomial coefficients for small primes.

use euler_utils::mod_pow;

const NMAX: usize = 50000;
const MOD: u64 = 1_000_000_000;
const MOD5: u64 = 1_953_125; // 5^9
const PHI5: usize = 1_562_500;

fn main() {
    // Sieve primes
    let mut is_prime_arr = vec![true; NMAX + 1];
    is_prime_arr[0] = false;
    if NMAX >= 1 { is_prime_arr[1] = false; }
    let mut i = 2;
    while i * i <= NMAX {
        if is_prime_arr[i] {
            let mut j = i * i;
            while j <= NMAX {
                is_prime_arr[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let prime_list: Vec<usize> = (2..=NMAX).filter(|&x| is_prime_arr[x]).collect();

    // Classify primes into big (e>=2) and small (e=1)
    let mut big_e: Vec<usize> = Vec::new();
    let mut m_small = 0usize;

    for &p in &prime_list {
        let mut e = 0;
        let mut pk = p as u64;
        while pk <= NMAX as u64 {
            e += 1;
            if pk > NMAX as u64 / p as u64 { break; }
            pk *= p as u64;
        }
        if e >= 2 {
            big_e.push(e);
        } else {
            m_small += 1;
        }
    }

    // Precompute pow2_tab[x] = 2^x mod 5^9
    let mut pow2_tab = vec![0u64; PHI5];
    pow2_tab[0] = 1;
    for i in 1..PHI5 {
        pow2_tab[i] = pow2_tab[i - 1] * 2 % MOD5;
    }

    // DP over big primes
    let mut dp = vec![0u64; PHI5];
    dp[1] = 1;

    for &ep in &big_e {
        let mut ndp = vec![0u64; PHI5];
        for x in 0..PHI5 {
            if dp[x] == 0 { continue; }
            let nx0 = (x as u64 * (ep as u64 + 1) % PHI5 as u64) as usize;
            let nx1 = (x as u64 * ep as u64 % PHI5 as u64) as usize;
            ndp[nx0] = (ndp[nx0] + dp[x]) % MOD5;
            ndp[nx1] = (ndp[nx1] + MOD5 - dp[x]) % MOD5;
        }
        dp = ndp;
    }

    // Compute coeff[k] = (-1)^(m-k) * C(m,k) mod 5^9
    let mut coeff = vec![0u64; m_small + 1];
    {
        let mut vv: i32 = 0;
        let mut unit: u64 = 1;
        for k in 0..=m_small {
            let bval: u64;
            if k == 0 {
                bval = 1;
            } else {
                let num = (m_small - k + 1) as i64;
                let den = k as i64;
                let mut nn = num;
                let mut dd = den;
                let mut v_num = 0i32;
                let mut v_den = 0i32;
                while nn % 5 == 0 { nn /= 5; v_num += 1; }
                while dd % 5 == 0 { dd /= 5; v_den += 1; }
                vv += v_num - v_den;
                unit = unit * (nn as u64 % MOD5) % MOD5;
                unit = unit * mod_pow(dd as u64 % MOD5, MOD5 - MOD5 / 5 - 1, MOD5) % MOD5;
                bval = if vv >= 9 {
                    0
                } else {
                    unit * mod_pow(5, vv as u64, MOD5) % MOD5
                };
            }
            let sign = if (m_small - k) % 2 == 0 { 1i64 } else { -1i64 };
            coeff[k] = ((sign * bval as i64 % MOD5 as i64 + MOD5 as i64) % MOD5 as i64) as u64;
        }
    }

    // Compute H5
    let mut h5: u64 = 0;
    for x in 0..PHI5 {
        if dp[x] == 0 { continue; }
        let mut s: u64 = 0;
        let mut ak = x;
        for k in 0..=m_small {
            if coeff[k] != 0 {
                s = (s + coeff[k] * pow2_tab[ak]) % MOD5;
            }
            ak = ak * 2 % PHI5;
        }
        h5 = (h5 + dp[x] * s) % MOD5;
    }
    h5 = (h5 % MOD5 + MOD5) % MOD5;

    // CRT: H mod 512 = 0
    let inv357 = mod_pow(357, 255, 512);
    let diff = ((0i64 - h5 as i64) % 512 + 512) % 512;
    let t = (diff as u64 * inv357) % 512;
    let result = (h5 + 1953125 * t) % MOD;

    println!("{}", result);
}
