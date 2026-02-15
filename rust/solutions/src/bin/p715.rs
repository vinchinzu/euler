// Project Euler 715 - Sextuplet Norms
//
// Uses Lucy DP with a modified Mobius-like function over Gaussian primes
// and sum of cubes formula.

const M: i64 = 1_000_000_007;

fn imod(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

fn sum_cubes(n: i64, modulus: i64) -> i64 {
    // (n*(n+1)/2)^2 mod m
    let mut s = (n % modulus) * ((n + 1) % modulus) % modulus;
    if s % 2 == 0 {
        s /= 2;
    } else {
        s = s * ((modulus + 1) / 2) % modulus;
    }
    s * s % modulus
}

fn main() {
    let big_n: i64 = 1_000_000_000_000;
    let l1 = (big_n as f64).cbrt() as usize;
    let l2 = big_n as usize / l1;

    // Sieve smallest prime factors
    let mut ff = vec![0usize; l2 + 1];
    for i in 0..=l2 {
        ff[i] = i;
    }
    for i in 2..=l2 {
        if (i as u64) * (i as u64) > l2 as u64 {
            break;
        }
        if ff[i] == i {
            let mut j = i * i;
            while j <= l2 {
                if ff[j] == j {
                    ff[j] = i;
                }
                j += i;
            }
        }
    }

    // Compute mu_prime
    let mut mu_prime = vec![0i64; l2 + 1];
    mu_prime[1] = 1;
    for i in 2..=l2 {
        let d = ff[i];
        if d != 2 && (i / d) % d != 0 {
            mu_prime[i] = mu_prime[i / d] * (if d % 4 == 1 { -1 } else { 1 });
        }
    }

    // Compute small = cumulative sum of mu_prime
    let mut small = vec![0i64; l2 + 1];
    for i in 1..=l2 {
        small[i] = (small[i - 1] + mu_prime[i]) % M;
    }

    let is_mod4_sq: [i64; 4] = [0, 1, 0, -1];
    let cum_is_mod4_sq: [i64; 4] = [0, 1, 1, 0];

    // Compute big
    let mut big = vec![0i64; l1 + 2];
    for i in (1..=l1).rev() {
        big[i] = 1;
        let ni = big_n / i as i64;
        let mut sqrtni = (ni as f64).sqrt() as i64;
        while (sqrtni + 1) * (sqrtni + 1) <= ni {
            sqrtni += 1;
        }
        while sqrtni * sqrtni > ni {
            sqrtni -= 1;
        }

        for k in 2..sqrtni as usize {
            if (i * k) < l1 {
                big[i] = (big[i] - is_mod4_sq[k % 4] * big[i * k]) % M;
            } else {
                let idx = (ni / k as i64) as usize;
                big[i] = (big[i] - is_mod4_sq[k % 4] * small[idx]) % M;
            }
        }

        let max_t = (ni / sqrtni) as usize;
        for t in 1..=max_t {
            let nit = ni / t as i64;
            let nit1 = ni / (t as i64 + 1);
            let diff = (cum_is_mod4_sq[imod(nit, 4) as usize]
                - cum_is_mod4_sq[imod(nit1, 4) as usize])
                % M;
            big[i] = (big[i] - diff * small[t]) % M;
        }
        big[i] = imod(big[i], M);
    }

    let mut ans: i64 = 0;

    // Sum over i <= l2
    for i in 1..=l2 {
        let sc = sum_cubes(big_n / i as i64, M);
        ans = (ans + sc as i128 * mu_prime[i] as i128 % M as i128) as i64 % M;
    }

    // Sum over t < l1
    for t in 1..l1 {
        let sc = sum_cubes(t as i64, M);
        let diff = (big[t] - big[t + 1]) % M;
        ans = (ans + sc as i128 * diff as i128 % M as i128) as i64 % M;
    }

    ans = imod(ans, M);
    println!("{}", ans);
}
