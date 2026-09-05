// Project Euler 245: Coresilience
// Find sum of composite n <= N where (n - phi(n)) / (n - 1) is a unit fraction.

use rayon::prelude::*;

const N: i64 = 200_000_000_000;
const L: usize = 447_213;
const LIMIT: usize = 34_199_519;

/// p <= L so p^2 << 2^64.
#[inline(always)]
fn mul_mod_p(a: u64, b: u64, p: u64) -> u64 {
    a.wrapping_mul(b) % p
}

#[inline(always)]
fn power_mod(mut base: u64, mut exp: u64, p: u64) -> u64 {
    let mut result = 1u64;
    base %= p;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod_p(result, base, p);
        }
        base = mul_mod_p(base, base, p);
        exp >>= 1;
    }
    result
}

#[inline(always)]
fn jacobi(mut a: i64, n: u64) -> i32 {
    let mut nn = n;
    let mut t = 1i32;
    a %= nn as i64;
    if a < 0 {
        a += nn as i64;
    }
    let mut aa = a as u64;
    while aa != 0 {
        while aa & 1 == 0 {
            aa >>= 1;
            let r = nn & 7;
            if r == 3 || r == 5 {
                t = -t;
            }
        }
        std::mem::swap(&mut aa, &mut nn);
        if (aa & 3) == 3 && (nn & 3) == 3 {
            t = -t;
        }
        aa %= nn;
    }
    if nn == 1 { t } else { 0 }
}

#[inline(always)]
fn is_sq(n: i64, p: u64) -> bool {
    jacobi(n, p) >= 0
}

fn sqrt_mod(n: i64, p: u64) -> u64 {
    let n = ((n % p as i64) + p as i64) as u64 % p;
    if n == 0 {
        return 0;
    }
    if p & 3 == 3 {
        return power_mod(n, (p + 1) >> 2, p);
    }
    let s = (p - 1).trailing_zeros();
    let q = (p - 1) >> s;
    let mut z = 2u64;
    while power_mod(z, (p - 1) >> 1, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = power_mod(z, q, p);
    let mut t = power_mod(n, q, p);
    let mut r = power_mod(n, (q + 1) >> 1, p);
    loop {
        if t == 1 {
            return r;
        }
        let mut i = 1u32;
        let mut tmp = mul_mod_p(t, t, p);
        while tmp != 1 {
            tmp = mul_mod_p(tmp, tmp, p);
            i += 1;
        }
        let mut b = c;
        for _ in 0..m - i - 1 {
            b = mul_mod_p(b, b, p);
        }
        m = i;
        c = mul_mod_p(b, b, p);
        t = mul_mod_p(t, c, p);
        r = mul_mod_p(r, b, p);
    }
}

#[inline(always)]
fn mul_mod64(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

#[inline(always)]
fn pow_mod64(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = mul_mod64(r, base, m);
        }
        base = mul_mod64(base, base, m);
        exp >>= 1;
    }
    r
}

#[inline(always)]
fn pow_mod_small(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = r.wrapping_mul(base) % m;
        }
        base = base.wrapping_mul(base) % m;
        exp >>= 1;
    }
    r
}

/// Deterministic SPRP. n > LIMIT > 2^25, n <= N/3 < 7e10.
#[inline(always)]
fn sprp_small(n: u64, a: u64) -> bool {
    let tz = (n - 1).trailing_zeros();
    let d = (n - 1) >> tz;
    let mut x = pow_mod_small(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 1..tz {
        x = x.wrapping_mul(x) % n;
        if x == n - 1 {
            return true;
        }
    }
    false
}

#[inline(always)]
fn sprp64(n: u64, a: u64) -> bool {
    let tz = (n - 1).trailing_zeros();
    let d = (n - 1) >> tz;
    let mut x = pow_mod64(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 1..tz {
        x = mul_mod64(x, x, n);
        if x == n - 1 {
            return true;
        }
    }
    false
}

#[inline]
fn is_prime_mr(n: u64) -> bool {
    if n & 1 == 0 {
        return false;
    }
    if n % 3 == 0 || n % 5 == 0 || n % 7 == 0 || n % 11 == 0 || n % 13 == 0
        || n % 17 == 0 || n % 19 == 0 || n % 23 == 0
    {
        return false;
    }
    if n <= u32::MAX as u64 {
        // n < 2^32 < 4_759_123_141: witnesses 2, 7, 61.
        return sprp_small(n, 2) && sprp_small(n, 7) && sprp_small(n, 61);
    }
    if n < 4_759_123_141 {
        return sprp64(n, 2) && sprp64(n, 7) && sprp64(n, 61);
    }
    // n < 2.15e12: 2, 3, 5, 7, 11.
    sprp64(n, 2) && sprp64(n, 3) && sprp64(n, 5) && sprp64(n, 7) && sprp64(n, 11)
}

#[inline]
fn check_prime(n: i64, is_prime: &[u8]) -> bool {
    if n < 2 {
        return false;
    }
    if n <= LIMIT as i64 {
        // SAFETY: n >= 0 and n <= LIMIT; is_prime.len() == LIMIT+1.
        return unsafe { *is_prime.get_unchecked(n as usize) } != 0;
    }
    is_prime_mr(n as u64)
}

#[derive(Clone, Copy)]
struct Factors {
    data: [i64; 16],
    len: usize,
}

impl Factors {
    #[inline(always)]
    fn new() -> Self {
        Factors { data: [0; 16], len: 0 }
    }
    #[inline(always)]
    fn push(&mut self, val: i64) {
        self.data[self.len] = val;
        self.len += 1;
    }
    #[inline(always)]
    fn pop(&mut self) {
        self.len -= 1;
    }
    #[inline(always)]
    fn as_slice(&self) -> &[i64] {
        &self.data[..self.len]
    }
}

#[inline(always)]
fn all_divisors_stack(n: i64, prime_factors: &[i64], divs: &mut [i64; 512]) -> usize {
    divs[0] = 1;
    let mut len = 1;
    let mut temp = n;
    for &p in prime_factors {
        if temp % p == 0 {
            let size = len;
            let mut power = 1i64;
            while temp % p == 0 {
                temp /= p;
                power *= p;
                for i in 0..size {
                    divs[len] = divs[i] * power;
                    len += 1;
                }
            }
        }
    }
    if temp > 1 {
        let size = len;
        for i in 0..size {
            divs[len] = divs[i] * temp;
            len += 1;
        }
    }
    len
}

fn scan_k(big_p: i64, phi: i64, factors: &[i64], is_prime: &[u8]) -> i64 {
    let smallest = factors[0];
    let last = factors[factors.len() - 1];
    let delta = big_p - phi;
    let mut ans = 0i64;
    let mut k = 2i64;
    while k < smallest {
        let den = big_p - delta * k;
        if den <= 0 {
            break;
        }
        let num = phi * k + 1;
        if num % den == 0 {
            let q = num / den;
            if last < q && big_p <= N / q && check_prime(q, is_prime) {
                ans += big_p * q;
            }
        }
        k += 2;
    }
    ans
}

fn dfs(
    index: usize,
    big_p: i64,
    phi: i64,
    factors: &mut Factors,
    primes: &[i64],
    is_prime: &[u8],
) -> i64 {
    let mut ans = 0i64;
    if factors.len >= 2 {
        ans += scan_k(big_p, phi, factors.as_slice(), is_prime);
    }
    let lo = index;
    let mut hi = index;
    while hi < primes.len() {
        let q = primes[hi];
        let q2 = q * q;
        if big_p > N / q2 {
            break;
        }
        hi += 1;
    }
    ans += add_primes(lo, hi, big_p, phi, factors, primes, is_prime);
    ans
}

fn add_primes(
    lo: usize,
    hi: usize,
    big_p: i64,
    phi: i64,
    factors: &mut Factors,
    primes: &[i64],
    is_prime: &[u8],
) -> i64 {
    let n = hi - lo;
    if n == 0 {
        return 0;
    }
    if n >= 16 && factors.len <= 4 {
        let snap = *factors;
        let mid = lo + n / 2;
        let (a, b) = rayon::join(
            || {
                let mut f = snap;
                add_primes(lo, mid, big_p, phi, &mut f, primes, is_prime)
            },
            || {
                let mut f = snap;
                add_primes(mid, hi, big_p, phi, &mut f, primes, is_prime)
            },
        );
        return a + b;
    }
    let mut ans = 0i64;
    for idx in lo..hi {
        let q = primes[idx];
        factors.push(q);
        ans += dfs(idx + 1, big_p * q, phi * (q - 1), factors, primes, is_prime);
        factors.pop();
    }
    ans
}


fn main() {
    let mut is_prime = vec![1u8; LIMIT + 1];
    is_prime[0] = 0;
    is_prime[1] = 0;
    {
        let mut i = 2usize;
        while i * i <= LIMIT {
            if is_prime[i] != 0 {
                let mut j = i * i;
                while j <= LIMIT {
                    is_prime[j] = 0;
                    j += i;
                }
            }
            i += 1;
        }
    }

    let mut primes: Vec<i64> = Vec::with_capacity(38_000);
    for i in 3..=L {
        if is_prime[i] != 0 {
            primes.push(i as i64);
        }
    }

    // Pass 1: count occurrences per p
    let mut counts = vec![0u32; L + 1];
    for &q in &primes {
        let qu = q as u64;
        if is_sq(q - 3, qu) {
            let r1 = sqrt_mod(q - 3, qu) as i64;
            let inv2 = (q + 1) / 2;
            let s1_raw = (1 + r1) * inv2 % q;
            let s1 = if s1_raw == 0 { q } else { s1_raw };
            let mut p = s1;
            while p <= L as i64 {
                counts[p as usize] += 1;
                p += q;
            }
            let s2_raw = (1 - r1 + q) * inv2 % q;
            let s2 = if s2_raw == 0 { q } else { s2_raw };
            let mut p = s2;
            while p <= L as i64 {
                counts[p as usize] += 1;
                p += q;
            }
        }
    }

    let mut offsets = vec![0u32; L + 2];
    for i in 0..=L {
        offsets[i + 1] = offsets[i] + counts[i];
    }
    let total_elements = offsets[L + 1] as usize;
    let mut pf_data = vec![0i64; total_elements];
    let mut write_pos = offsets.clone();

    for &q in &primes {
        let qu = q as u64;
        if is_sq(q - 3, qu) {
            let r1 = sqrt_mod(q - 3, qu) as i64;
            let inv2 = (q + 1) / 2;
            let s1_raw = (1 + r1) * inv2 % q;
            let s1 = if s1_raw == 0 { q } else { s1_raw };
            let mut p = s1;
            while p <= L as i64 {
                let pu = p as usize;
                let pos = write_pos[pu] as usize;
                pf_data[pos] = q;
                write_pos[pu] += 1;
                p += q;
            }
            let s2_raw = (1 - r1 + q) * inv2 % q;
            let s2 = if s2_raw == 0 { q } else { s2_raw };
            let mut p = s2;
            while p <= L as i64 {
                let pu = p as usize;
                let pos = write_pos[pu] as usize;
                pf_data[pos] = q;
                write_pos[pu] += 1;
                p += q;
            }
        }
    }

    let (ans_two, ans_more) = rayon::join(
        || {
            let nt = rayon::current_num_threads().max(1);
            (0..nt)
                .into_par_iter()
                .map(|tid| {
                    let mut local = 0i64;
                    let mut divs = [0i64; 512];
                    let mut i = tid;
                    while i < primes.len() {
                        let p = primes[i];
                        let pu = p as usize;
                        let start = offsets[pu] as usize;
                        let end = offsets[pu + 1] as usize;
                        let pf = &pf_data[start..end];
                        let val = p * (p - 1) + 1;
                        let num_divs = all_divisors_stack(val, pf, &mut divs);
                        for &d in &divs[..num_divs] {
                            if d >= p {
                                let q = d - (p - 1);
                                if q > p && p <= N / q && check_prime(q, &is_prime) {
                                    local += p * q;
                                }
                            }
                        }
                        i += nt;
                    }
                    local
                })
                .sum::<i64>()
        },
        || {
            let mut factors = Factors::new();
            dfs(0, 1, 1, &mut factors, &primes, &is_prime)
        },
    );

    println!("{}", ans_two + ans_more);
}
