// Project Euler 554 - Centaurs on a Chessboard
//
// C(n) = 8*C(2n,n) - 3*(n-1)^2 - 8n - 4
// Find sum_{i=2}^{90} C(F_i) mod (10^8+7) using Lucas' theorem.
//
// n! for n > (p-1)/2 is recovered from Wilson's theorem so we only
// multiply through ~48e6 instead of allocating fact[0..p].

use rayon::prelude::*;

const M: u64 = 100_000_007; // 10^8 + 7, prime
const HALF: u64 = (M - 1) / 2;

#[inline(always)]
fn mulm(a: u64, b: u64) -> u64 {
    a.wrapping_mul(b) % M
}

fn powm(mut base: u64, mut exp: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = mulm(r, base);
        }
        base = mulm(base, base);
        exp >>= 1;
    }
    r
}

/// Product lo*...*hi (mod M). Independent streams hide mul-mod latency.
fn range_prod(lo: u64, hi: u64) -> u64 {
    if lo > hi {
        return 1;
    }
    let mut i = lo;
    let mut a0 = 1u64;
    let mut a1 = 1u64;
    let mut a2 = 1u64;
    let mut a3 = 1u64;
    let mut a4 = 1u64;
    let mut a5 = 1u64;
    let mut a6 = 1u64;
    let mut a7 = 1u64;
    while i + 7 <= hi {
        a0 = mulm(a0, i);
        a1 = mulm(a1, i + 1);
        a2 = mulm(a2, i + 2);
        a3 = mulm(a3, i + 3);
        a4 = mulm(a4, i + 4);
        a5 = mulm(a5, i + 5);
        a6 = mulm(a6, i + 6);
        a7 = mulm(a7, i + 7);
        i += 8;
    }
    while i <= hi {
        a0 = mulm(a0, i);
        i += 1;
    }
    mulm(
        mulm(mulm(a0, a1), mulm(a2, a3)),
        mulm(mulm(a4, a5), mulm(a6, a7)),
    )
}

fn reduced(n: u64) -> u64 {
    if n > HALF { M - 1 - n } else { n }
}

struct Facts {
    keys: Vec<u64>,
    fact: Vec<u64>,
    inv: Vec<u64>,
}

impl Facts {
    fn compute(mut keys: Vec<u64>) -> Self {
        keys.sort_unstable();
        keys.dedup();
        let nkeys = keys.len();
        let mut fact = vec![0u64; nkeys];
        if nkeys > 0 && keys[0] == 0 {
            fact[0] = 1;
        }
        let max_n = *keys.last().unwrap_or(&0);
        if max_n > 0 {
            let nthreads = rayon::current_num_threads().max(1);
            let chunk = (max_n + nthreads as u64 - 1) / nthreads as u64;
            let parts: Vec<(u64, Vec<(usize, u64)>)> = (0..nthreads)
                .into_par_iter()
                .map(|t| {
                    let lo = t as u64 * chunk + 1;
                    let hi = ((t as u64 + 1) * chunk).min(max_n);
                    if lo > hi {
                        return (1, Vec::new());
                    }
                    let s = keys.partition_point(|&x| x < lo);
                    let e = keys.partition_point(|&x| x <= hi);
                    let mut prod = 1u64;
                    let mut prev = lo - 1;
                    let mut locals = Vec::with_capacity(e - s);
                    for i in s..e {
                        let n = keys[i];
                        prod = mulm(prod, range_prod(prev + 1, n));
                        locals.push((i, prod));
                        prev = n;
                    }
                    prod = mulm(prod, range_prod(prev + 1, hi));
                    (prod, locals)
                })
                .collect();

            let mut scale = 1u64;
            for (full, locals) in &parts {
                for &(i, local) in locals {
                    fact[i] = mulm(scale, local);
                }
                scale = mulm(scale, *full);
            }
        }
        let inv: Vec<u64> = fact.par_iter().map(|&f| powm(f, M - 2)).collect();
        Facts { keys, fact, inv }
    }

    #[inline]
    fn pair(&self, n: u64) -> (u64, u64) {
        let i = self.keys.binary_search(&n).unwrap();
        (self.fact[i], self.inv[i])
    }

    /// n! mod M for n < M, via Wilson if n > (p-1)/2.
    #[inline]
    fn fact(&self, n: u64) -> u64 {
        if n > HALF {
            let inv_small = self.pair(M - 1 - n).1;
            if (M - n) & 1 == 0 {
                inv_small
            } else {
                M - inv_small
            }
        } else {
            self.pair(n).0
        }
    }

    #[inline]
    fn inv_fact(&self, n: u64) -> u64 {
        if n > HALF {
            let f_small = self.pair(M - 1 - n).0;
            if (M - n) & 1 == 0 {
                f_small
            } else {
                M - f_small
            }
        } else {
            self.pair(n).1
        }
    }

    #[inline]
    fn ncr(&self, n: u64, r: u64) -> u64 {
        if r > n {
            0
        } else {
            mulm(mulm(self.fact(n), self.inv_fact(r)), self.inv_fact(n - r))
        }
    }
}

fn ncr_lucas(n: u64, r: u64, facts: &Facts) -> u64 {
    if r > n {
        return 0;
    }
    // Reject any invalid base-M digit before touching the factorial table
    // (a later ri > ni would otherwise still evaluate an earlier C(ni, ri)).
    let mut nn = n;
    let mut rr = r;
    let mut digits = [(0u64, 0u64); 4];
    let mut nd = 0usize;
    while nn > 0 || rr > 0 {
        let ni = nn % M;
        let ri = rr % M;
        if ri > ni {
            return 0;
        }
        digits[nd] = (ni, ri);
        nd += 1;
        nn /= M;
        rr /= M;
    }
    let mut res = 1u64;
    for i in 0..nd {
        let (ni, ri) = digits[i];
        res = mulm(res, facts.ncr(ni, ri));
    }
    res
}

fn main() {
    let mut fibs = [0u64; 91];
    fibs[1] = 1;
    for i in 2..=90 {
        fibs[i] = fibs[i - 1] + fibs[i - 2];
    }

    let mut needed: Vec<u64> = vec![0, 1];
    for i in 2..=90 {
        let n = fibs[i];
        let mut nn = n * 2;
        let mut rr = n;
        let mut d = [(0u64, 0u64); 4];
        let mut nd = 0usize;
        let mut ok = true;
        while nn > 0 || rr > 0 {
            let ni = nn % M;
            let ri = rr % M;
            if ri > ni {
                ok = false;
                break;
            }
            d[nd] = (ni, ri);
            nd += 1;
            nn /= M;
            rr /= M;
        }
        if ok {
            for &(ni, ri) in d[..nd].iter() {
                needed.push(reduced(ni));
                needed.push(reduced(ri));
                needed.push(reduced(ni - ri));
            }
        }
    }

    let facts = Facts::compute(needed);

    let mut ans = 0u64;
    for i in 2..=90 {
        let n = fibs[i];
        let n_mod = n % M;
        let c2n_n = ncr_lucas(n * 2, n, &facts);
        let sq = mulm((n_mod + M - 1) % M, (n_mod + M - 1) % M);
        let mut val = mulm(8, c2n_n);
        val = (val + M - mulm(3, sq)) % M;
        val = (val + M - mulm(8, n_mod)) % M;
        val = (val + M - 4) % M;
        ans = (ans + val) % M;
    }

    println!("{ans}");
}
