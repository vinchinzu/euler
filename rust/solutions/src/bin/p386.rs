// Project Euler 386 - Antichain Counting
//
// N(n) is the max antichain size in the divisor lattice of n (middle rank).
// Sum N(n) for n = 1..=10^8.
//
// DFS over increasing-prime factorizations, but batch the last prime via π(x):
// once p^2 > N/n, remaining primes p <= N/n are leaves with the same exponent
// signature. Recursion only needs primes <= sqrt(N). π(N/n) comes from a
// Lucy–Hedgehog table. Small-n prime loops are rayon-split.

use fxhash::FxHashMap;
use rayon::prelude::*;

const NLIMIT: u64 = 100_000_000;
const MAX_EXP_SUM: usize = 26;
const MAX_OMEGA: usize = 12;

#[inline(always)]
fn isqrt(n: u64) -> u64 {
    let mut r = (n as f64).sqrt() as u64;
    while r.saturating_mul(r) > n {
        r -= 1;
    }
    while r.saturating_add(1).saturating_mul(r + 1) <= n {
        r += 1;
    }
    r
}

fn sieve_primes(limit: usize) -> Vec<u32> {
    if limit < 2 {
        return Vec::new();
    }
    let mut is_comp = vec![false; limit + 1];
    let mut p = 2usize;
    while p * p <= limit {
        if !is_comp[p] {
            let mut j = p * p;
            while j <= limit {
                is_comp[j] = true;
                j += p;
            }
        }
        p += 1;
    }
    let mut primes = Vec::with_capacity(limit / 10 + 10);
    for i in 2..=limit {
        if !is_comp[i] {
            primes.push(i as u32);
        }
    }
    primes
}

struct PiTable {
    n: u64,
    isqrt: u64,
    s_small: Vec<i64>,
    s_large: Vec<i64>,
}

impl PiTable {
    fn new(n: u64, primes: &[u32]) -> Self {
        let sq = isqrt(n);
        let m = sq as usize;
        let mut s_small = vec![0i64; m + 1];
        let mut s_large = vec![0i64; m + 1];
        for v in 0..=m {
            s_small[v] = v as i64 - 1;
        }
        for k in 1..=m {
            s_large[k] = (n / k as u64) as i64 - 1;
        }

        for &p32 in primes {
            let p = p32 as u64;
            if p > sq {
                break;
            }
            let p2 = p * p;
            if p2 > n {
                break;
            }
            let sp_1 = s_small[(p - 1) as usize];
            let mut k_limit = n / p2;
            if k_limit > sq {
                k_limit = sq;
            }
            for k in 1..=k_limit as usize {
                let target = (n / k as u64) / p;
                let s_target = if target <= sq {
                    s_small[target as usize]
                } else {
                    s_large[(n / target) as usize]
                };
                s_large[k] -= s_target - sp_1;
            }
            for v in (p2 as usize..=m).rev() {
                s_small[v] -= s_small[v / p as usize] - sp_1;
            }
        }

        Self {
            n,
            isqrt: sq,
            s_small,
            s_large,
        }
    }

    #[inline(always)]
    fn pi(&self, x: u64) -> i64 {
        if x <= 1 {
            return 0;
        }
        if x <= self.isqrt {
            self.s_small[x as usize]
        } else {
            self.s_large[(self.n / x) as usize]
        }
    }
}

#[inline(always)]
fn make_key(exps: &[i32]) -> u64 {
    let n = exps.len();
    let mut sorted = [0u8; MAX_OMEGA];
    for i in 0..n {
        sorted[i] = exps[i] as u8;
    }
    sorted[..n].sort_unstable();
    let mut key: u64 = (n as u64) << 60;
    for i in 0..n {
        key |= (sorted[i] as u64) << (i * 5);
    }
    key
}

struct Worker<'a> {
    primes: &'a [u32],
    pi: &'a PiTable,
    binom_half: &'a [i64; MAX_EXP_SUM + 1],
    memo: FxHashMap<u64, i64>,
    exponents: [i32; MAX_OMEGA],
    exp_len: usize,
    non_one_count: usize,
    dp: [i64; MAX_EXP_SUM + 2],
    new_dp: [i64; MAX_EXP_SUM + 2],
}

impl<'a> Worker<'a> {
    fn new(
        primes: &'a [u32],
        pi: &'a PiTable,
        binom_half: &'a [i64; MAX_EXP_SUM + 1],
    ) -> Self {
        Self {
            primes,
            pi,
            binom_half,
            memo: FxHashMap::with_capacity_and_hasher(4096, Default::default()),
            exponents: [0; MAX_OMEGA],
            exp_len: 0,
            non_one_count: 0,
            dp: [0; MAX_EXP_SUM + 2],
            new_dp: [0; MAX_EXP_SUM + 2],
        }
    }

    fn count_combinations(&mut self) -> i64 {
        let k = self.exp_len;
        let key = make_key(&self.exponents[..k]);
        if let Some(&val) = self.memo.get(&key) {
            return val;
        }

        let mut total = 0i32;
        for i in 0..k {
            total += self.exponents[i];
        }
        let target = (total / 2) as usize;
        let needed = target + 1;

        self.dp[..needed].fill(0);
        self.dp[0] = 1;

        for i in 0..k {
            let e = self.exponents[i] as usize;
            self.new_dp[..needed].fill(0);
            for s in 0..needed {
                let d = self.dp[s];
                if d != 0 {
                    let max_k = e.min(target - s);
                    for t in 0..=max_k {
                        self.new_dp[s + t] += d;
                    }
                }
            }
            self.dp[..needed].copy_from_slice(&self.new_dp[..needed]);
        }

        let result = self.dp[target];
        self.memo.insert(key, result);
        result
    }

    #[inline(always)]
    fn n_of_current(&mut self) -> i64 {
        let k = self.exp_len;
        if k == 0 {
            1
        } else if self.non_one_count == 0 {
            self.binom_half[k]
        } else if k == 1 {
            1
        } else {
            self.count_combinations()
        }
    }

    fn n_of_plus_one(&mut self) -> i64 {
        let k = self.exp_len;
        if self.non_one_count == 0 {
            return self.binom_half[k + 1];
        }
        self.exponents[k] = 1;
        self.exp_len = k + 1;
        let v = self.count_combinations();
        self.exp_len = k;
        v
    }

    fn leaf_contrib(&mut self, min_index: usize, pmax: u64, sqrt_pmax: u64) -> i64 {
        let lower = if min_index < self.primes.len() {
            let p_min = unsafe { *self.primes.get_unchecked(min_index) } as u64;
            if p_min > sqrt_pmax {
                p_min - 1
            } else {
                sqrt_pmax
            }
        } else {
            let last = *self.primes.last().unwrap_or(&1) as u64;
            if last > sqrt_pmax { last } else { sqrt_pmax }
        };
        let cnt = self.pi.pi(pmax) - self.pi.pi(lower);
        if cnt > 0 {
            cnt * self.n_of_plus_one()
        } else {
            0
        }
    }

    fn recurse_one_prime(&mut self, index: usize, n: u64) -> i64 {
        let p = unsafe { *self.primes.get_unchecked(index) } as u64;
        let mut ans = 0i64;
        let mut prod = 1u64;
        let mut e = 1i32;
        loop {
            if p > NLIMIT / prod {
                break;
            }
            prod *= p;
            if n > NLIMIT / prod {
                break;
            }
            self.exponents[self.exp_len] = e;
            self.exp_len += 1;
            if e > 1 {
                self.non_one_count += 1;
            }
            ans += self.helper(index + 1, n * prod);
            if e > 1 {
                self.non_one_count -= 1;
            }
            self.exp_len -= 1;
            e += 1;
        }
        ans
    }

    fn helper(&mut self, min_index: usize, n: u64) -> i64 {
        let mut ans = self.n_of_current();
        let pmax = NLIMIT / n;
        if pmax < 2 {
            return ans;
        }
        let sqrt_pmax = isqrt(pmax);
        ans += self.leaf_contrib(min_index, pmax, sqrt_pmax);

        let last_rec = self.primes.partition_point(|&p| p as u64 <= sqrt_pmax);
        if min_index >= last_rec {
            return ans;
        }

        // Split only fat nodes so we do not rayon millions of empty leaves.
        if self.exp_len < 3 && n < 200 && last_rec - min_index > 16 {
            let primes = self.primes;
            let pi = self.pi;
            let binom_half = self.binom_half;
            let exponents = self.exponents;
            let exp_len = self.exp_len;
            let non_one_count = self.non_one_count;
            ans += (min_index..last_rec)
                .into_par_iter()
                .map(|index| {
                    let mut w = Worker {
                        primes,
                        pi,
                        binom_half,
                        memo: FxHashMap::with_capacity_and_hasher(512, Default::default()),
                        exponents,
                        exp_len,
                        non_one_count,
                        dp: [0; MAX_EXP_SUM + 2],
                        new_dp: [0; MAX_EXP_SUM + 2],
                    };
                    w.recurse_one_prime(index, n)
                })
                .sum::<i64>();
        } else {
            for index in min_index..last_rec {
                ans += self.recurse_one_prime(index, n);
            }
        }
        ans
    }
}

fn main() {
    let sq = isqrt(NLIMIT) as usize;
    let primes = sieve_primes(sq);
    let pi = PiTable::new(NLIMIT, &primes);

    let mut binom_half = [0i64; MAX_EXP_SUM + 1];
    binom_half[0] = 1;
    for k in 1..=MAX_EXP_SUM {
        let target = k / 2;
        let mut val = 1i64;
        for i in 0..target {
            val = val * (k - i) as i64 / (i + 1) as i64;
        }
        binom_half[k] = val;
    }

    let mut worker = Worker::new(&primes, &pi, &binom_half);
    let ans = worker.helper(0, 1);
    println!("{}", ans);
}
