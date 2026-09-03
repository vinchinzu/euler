// Project Euler 417: Reciprocal cycles II
//
// Compute sum of L(n) for n=3..10^8, where L(n) is the length of the
// repeating cycle in the decimal expansion of 1/n.

use rayon::prelude::*;

struct FastMod {
    m: u64,
    inv: u64,
}

impl FastMod {
    #[inline(always)]
    fn new(m: u32) -> Self {
        let m = m as u64;
        let inv = u64::MAX / m;
        Self { m, inv }
    }

    #[inline(always)]
    fn reduce(&self, x: u64) -> u64 {
        let q = ((x as u128 * self.inv as u128) >> 64) as u64;
        let mut r = x.wrapping_sub(q.wrapping_mul(self.m));
        if r >= self.m {
            r -= self.m;
        }
        r
    }

    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.reduce(a * b)
    }

    #[inline(always)]
    fn pow10(&self, mut exp: u32) -> u64 {
        let mut base = 10u64;
        while (exp & 1) == 0 {
            base = self.mul(base, base);
            exp >>= 1;
        }
        let mut res = base;
        exp >>= 1;
        while exp > 0 {
            base = self.mul(base, base);
            if exp & 1 == 1 {
                res = self.mul(res, base);
            }
            exp >>= 1;
        }
        res
    }
}

#[inline(always)]
fn gcd64(mut u: u64, mut v: u64) -> u64 {
    if u == 0 { return v; }
    if v == 0 { return u; }
    let shift = (u | v).trailing_zeros();
    u >>= u.trailing_zeros();
    loop {
        v >>= v.trailing_zeros();
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        v -= u;
        if v == 0 { break; }
    }
    u << shift
}

#[inline(always)]
fn lcm64(a: u64, b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    (a / gcd64(a, b)) * b
}

const HALF_N: usize = 50_000_001;

fn main() {
    let n: usize = 100_000_000;

    // Sieve base primes up to sqrt(N)
    let sqrt_n = 10000usize;
    let mut is_prime_base = vec![true; sqrt_n + 1];
    let mut base_primes = Vec::new();
    for p in 3..=sqrt_n {
        if p % 2 != 0 && is_prime_base[p] {
            base_primes.push(p);
            let mut j = p * p;
            while j <= sqrt_n {
                is_prime_base[j] = false;
                j += 2 * p;
            }
        }
    }

    // Sieve smallest prime factors for odd numbers in parallel chunks
    let mut spf = vec![0u32; HALF_N];
    let chunk_size = 128 * 1024;
    spf.par_chunks_mut(chunk_size).enumerate().for_each(|(chunk_idx, chunk)| {
        let start_idx = chunk_idx * chunk_size;
        let end_idx = start_idx + chunk.len() - 1;
        let start_num = 2 * start_idx + 1;
        let end_num = 2 * end_idx + 1;

        for &p in &base_primes {
            if p * p > end_num { break; }
            let m = start_num.max(p * p);
            let mut q = (m + p - 1) / p;
            if q % 2 == 0 { q += 1; }
            let first_num = q * p;
            if first_num <= end_num {
                let first_idx = first_num >> 1;
                let mut j = first_idx - start_idx;
                while j < chunk.len() {
                    if chunk[j] == 0 {
                        chunk[j] = p as u32;
                    }
                    j += p;
                }
            }
        }
    });

    // Compute ord_10(p) for all primes in parallel chunks
    const LEGENDRE_10: u64 = (1 << 1) | (1 << 3) | (1 << 9) | (1 << 13) | (1 << 27) | (1 << 31) | (1 << 37) | (1 << 39);
    let mut ord10 = vec![0u32; HALF_N];
    ord10.par_chunks_mut(chunk_size).enumerate().for_each(|(chunk_idx, chunk)| {
        let start_idx = chunk_idx * chunk_size;
        for (i, slot) in chunk.iter_mut().enumerate() {
            let idx = start_idx + i;
            let p = (idx << 1) | 1;
            if p > n { break; }
            if p < 3 || p == 5 { continue; }
            if unsafe { *spf.get_unchecked(idx) } == 0 {
                // p is prime
                let fm = FastMod::new(p as u32);
                let mut result = (p - 1) as u32;
                let mut temp = ((p - 1) >> (p - 1).trailing_zeros()) as u32;

                // Handle factor 2 using quadratic reciprocity
                let is_qr = ((LEGENDRE_10 >> (p % 40)) & 1) != 0;
                if is_qr {
                    result >>= 1;
                    while (result & 1) == 0 {
                        if fm.pow10(result >> 1) == 1 {
                            result >>= 1;
                        } else {
                            break;
                        }
                    }
                }

                while temp > 1 {
                    let s = unsafe { *spf.get_unchecked((temp >> 1) as usize) };
                    if s == 0 {
                        let q = temp;
                        while result % q == 0 {
                            if fm.pow10(result / q) == 1 {
                                result /= q;
                            } else {
                                break;
                            }
                        }
                        break;
                    }
                    let q = s;
                    while temp % q == 0 {
                        temp /= q;
                    }
                    while result % q == 0 {
                        if fm.pow10(result / q) == 1 {
                            result /= q;
                        } else {
                            break;
                        }
                    }
                }
                *slot = result;
            }
        }
    });

    // Precompute 2^a * 5^b Hamming numbers
    let mut hamming = Vec::new();
    let mut p2 = 1u64;
    while p2 <= n as u64 {
        let mut p10 = p2;
        while p10 <= n as u64 {
            hamming.push(p10 as u32);
            p10 *= 5;
        }
        p2 *= 2;
    }
    hamming.sort_unstable();

    // Small lookup table for count(K) for K <= 100_000
    const K_MAX: usize = 100_001;
    let mut count_table = vec![0u8; K_MAX];
    for k in 1..K_MAX {
        count_table[k] = hamming.partition_point(|&h| h <= k as u32) as u8;
    }

    #[inline(always)]
    fn get_count(m: u32, count_table: &[u8], hamming: &[u32]) -> i64 {
        if m > 50_000_000 {
            return 1;
        }
        let k = 100_000_000 / m;
        if (k as usize) < K_MAX {
            unsafe { *count_table.get_unchecked(k as usize) as i64 }
        } else {
            hamming.partition_point(|&h| h <= k) as i64
        }
    }

    // Compute sum of L(n) for n = 3..N in parallel chunks over odd indices
    let chunk_size_idx = 128 * 1024;
    let n_idx_chunks = (HALF_N - 1 + chunk_size_idx - 1) / chunk_size_idx;
    let total: i64 = (0..n_idx_chunks)
        .into_par_iter()
        .map(|ci| {
            let start_idx = 1 + ci * chunk_size_idx;
            let end_idx = (HALF_N - 1).min(start_idx + chunk_size_idx - 1);
            let mut local_total: i64 = 0;

            for idx in start_idx..=end_idx {
                if idx % 5 == 2 {
                    continue; // 2 * idx + 1 is divisible by 5
                }
                let m = (2 * idx + 1) as u32;
                if m > n as u32 {
                    break;
                }

                let cnt = get_count(m, &count_table, &hamming);

                let sp = unsafe { *spf.get_unchecked(idx) };
                if sp == 0 {
                    let lm = unsafe { *ord10.get_unchecked(idx) } as i64;
                    local_total += lm * cnt;
                    continue;
                }

                let mut t = m / sp;
                let mut e = 1u32;
                while t % sp == 0 {
                    t /= sp;
                    e += 1;
                }

                let lp = unsafe { *ord10.get_unchecked((sp >> 1) as usize) } as u64;
                let mut result = lp;
                if e >= 2 {
                    if sp == 3 {
                        result = 3u64.pow(e - 2);
                    } else if sp == 487 {
                        result = 486;
                    } else {
                        for _ in 1..e {
                            result *= sp as u64;
                        }
                    }
                }

                while t > 1 {
                    let s = unsafe { *spf.get_unchecked((t >> 1) as usize) };
                    if s == 0 {
                        let lp = unsafe { *ord10.get_unchecked((t >> 1) as usize) } as u64;
                        result = lcm64(result, lp);
                        break;
                    }
                    let mut e = 1u32;
                    t /= s;
                    while t % s == 0 {
                        t /= s;
                        e += 1;
                    }

                    let lp = unsafe { *ord10.get_unchecked((s >> 1) as usize) } as u64;
                    let mut lpe = lp;
                    if e >= 2 {
                        if s == 3 {
                            lpe = 3u64.pow(e - 2);
                        } else if s == 487 {
                            lpe = 486;
                        } else {
                            for _ in 1..e {
                                lpe *= s as u64;
                            }
                        }
                    }
                    result = lcm64(result, lpe);
                }
                local_total += result as i64 * cnt;
            }
            local_total
        })
        .sum();

    println!("{}", total);
}
