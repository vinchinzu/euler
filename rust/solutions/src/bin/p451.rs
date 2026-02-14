// Project Euler 451 - Modular inverses
// Find sum_{n=3}^N l(n), where l(n) is the largest x < n-1 with x^2 ≡ 1 (mod n).

const MAXN: usize = 20_000_000;

fn mod_pos(a: i64, m: i64) -> i64 {
    let r = a % m;
    if r < 0 { r + m } else { r }
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (mut t, mut new_t) = (0i64, 1i64);
    let (mut r, mut new_r) = (m, a);
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;
        let tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }
    if t < 0 { t + m } else { t }
}

fn main() {
    let n_limit = MAXN as i64;

    // Sieve odd primes
    let mut is_prime = vec![true; MAXN + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= MAXN {
        if is_prime[i] {
            let mut j = i * i;
            while j <= MAXN {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let primes: Vec<i64> = (3..=MAXN).filter(|&i| is_prime[i]).map(|i| i as i64).collect();

    let mut ans: i64 = 0;

    fn helper(
        min_index: usize,
        n: i64,
        sqrts: &[i64],
        primes: &[i64],
        n_limit: i64,
        ans: &mut i64,
    ) {
        // Find largest sqrt < n-1
        let mut l = 0i64;
        for &s in sqrts.iter() {
            if s < n - 1 && s > l {
                l = s;
            }
        }
        *ans += l;

        for index in min_index..primes.len() {
            let p = primes[index];
            if n * p > n_limit {
                break;
            }

            let mut pe = p;
            while n * pe <= n_limit {
                let pe_inv = mod_inv(pe, n);
                let n_inv = (1 - pe * pe_inv) / n;
                let npe = n * pe;

                let mut new_sqrts = Vec::with_capacity(sqrts.len() * 2);
                for &sv in sqrts.iter() {
                    let a = sv * pe % npe * pe_inv % npe;
                    let b = n % npe * n_inv.abs() % npe;
                    if n_inv >= 0 {
                        new_sqrts.push(mod_pos(a as i64 + b as i64, npe));
                        new_sqrts.push(mod_pos(a as i64 - b as i64, npe));
                    } else {
                        new_sqrts.push(mod_pos(a as i64 - b as i64, npe));
                        new_sqrts.push(mod_pos(a as i64 + b as i64, npe));
                    }
                }

                helper(index + 1, npe, &new_sqrts, primes, n_limit, ans);
                pe *= p;
            }
        }
    }

    // Recurrence matches the C code exactly
    // But the C code computes: a = sv * pe * pe_inv, b = n * n_inv
    // Let me replicate exactly.

    fn helper2(
        min_index: usize,
        n: i64,
        sqrts: &[i64],
        primes: &[i64],
        n_limit: i64,
        ans: &mut i64,
    ) {
        let mut l = 0i64;
        for &s in sqrts.iter() {
            if s < n - 1 && s > l {
                l = s;
            }
        }
        *ans += l;

        for index in min_index..primes.len() {
            let p = primes[index];
            if n * p > n_limit {
                break;
            }

            let mut pe = p;
            while n * pe <= n_limit {
                let pe_inv = mod_inv(pe, n);
                let n_inv = (1 - pe * pe_inv) / n;
                let npe = n * pe;

                let mut new_sqrts = Vec::with_capacity(sqrts.len() * 2);
                for &sv in sqrts.iter() {
                    let a = (sv as i128 * pe as i128 * pe_inv as i128) as i64;
                    let b = (n as i128 * n_inv as i128) as i64;
                    new_sqrts.push(mod_pos(a + b, npe));
                    new_sqrts.push(mod_pos(a - b, npe));
                }

                helper2(index + 1, npe, &new_sqrts, primes, n_limit, ans);
                pe *= p;
            }
        }
    }

    helper2(0, 1, &[0], &primes, n_limit, &mut ans);
    helper2(0, 2, &[1], &primes, n_limit, &mut ans);
    helper2(0, 4, &[1, 3], &primes, n_limit, &mut ans);

    let mut pow2 = 8i64;
    while pow2 <= n_limit {
        let sqrts = vec![1, pow2 / 2 - 1, pow2 / 2 + 1, pow2 - 1];
        helper2(0, pow2, &sqrts, &primes, n_limit, &mut ans);
        pow2 *= 2;
    }

    println!("{}", ans);
}
