// Project Euler 386 - Antichain Counting
//
// N(n) is the max antichain size in divisor lattice of n.
// Sum of N(n) for n=1 to 10^8.
//
// Optimization: memoize by compact u64 key, fast paths for common cases,
// struct to reduce function parameter overhead in DFS.

use std::collections::HashMap;

const NLIMIT: u64 = 100_000_000;
const MAX_EXP_SUM: usize = 26;

#[inline(always)]
fn make_key(exps: &[i32]) -> u64 {
    let n = exps.len();
    let mut sorted = [0u8; 12];
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

struct Solver {
    primes: Vec<u64>,
    binom_half: [i64; MAX_EXP_SUM + 1],
    memo: HashMap<u64, i64>,
    exponents: Vec<i32>,
    non_one_count: usize,
    ans: i64,
    dp_buf: Vec<i64>,
    new_dp_buf: Vec<i64>,
}

impl Solver {
    fn count_combinations(&mut self, exps_len: usize) -> i64 {
        let key = make_key(&self.exponents[..exps_len]);

        if let Some(&val) = self.memo.get(&key) {
            return val;
        }

        let exps = &self.exponents[..exps_len];
        let total: i32 = exps.iter().sum();
        let target = (total / 2) as usize;

        let needed = target + 1;
        self.dp_buf.resize(needed, 0);
        self.dp_buf[..needed].fill(0);
        self.dp_buf[0] = 1;

        for &e in exps {
            self.new_dp_buf.resize(needed, 0);
            self.new_dp_buf[..needed].fill(0);
            for s in 0..needed {
                let d = self.dp_buf[s];
                if d > 0 {
                    let max_k = (e as usize).min(target - s);
                    for k in 0..=max_k {
                        unsafe {
                            *self.new_dp_buf.get_unchecked_mut(s + k) += d;
                        }
                    }
                }
            }
            std::mem::swap(&mut self.dp_buf, &mut self.new_dp_buf);
        }

        let result = self.dp_buf[target];
        self.memo.insert(key, result);
        result
    }

    fn helper(&mut self, min_index: usize, n: u64) {
        let k = self.exponents.len();
        if k == 0 {
            self.ans += 1;
        } else if self.non_one_count == 0 {
            self.ans += self.binom_half[k];
        } else if k == 1 {
            self.ans += 1;
        } else {
            let val = self.count_combinations(k);
            self.ans += val;
        }

        let primes_len = self.primes.len();
        for index in min_index..primes_len {
            let p = unsafe { *self.primes.get_unchecked(index) };
            if n * p > NLIMIT {
                break;
            }

            let mut prod = 1u64;
            let mut e = 1i32;
            loop {
                prod *= p;
                if n * prod > NLIMIT {
                    break;
                }
                self.exponents.push(e);
                if e > 1 { self.non_one_count += 1; }
                self.helper(index + 1, n * prod);
                if e > 1 { self.non_one_count -= 1; }
                self.exponents.pop();
                e += 1;
            }
        }
    }
}

fn main() {
    // Local bit-packed sieve
    let limit = NLIMIT as usize;
    let sieve_size = limit / 2 + 1;
    let mut is_composite = vec![0u8; (sieve_size + 7) / 8];

    {
        let mut i = 3usize;
        while i * i <= limit {
            let idx = i / 2;
            if is_composite[idx / 8] & (1 << (idx % 8)) == 0 {
                let mut j = i * i;
                while j <= limit {
                    let jdx = j / 2;
                    is_composite[jdx / 8] |= 1 << (jdx % 8);
                    j += 2 * i;
                }
            }
            i += 2;
        }
    }

    let mut primes: Vec<u64> = Vec::with_capacity(6_000_000);
    primes.push(2);
    for i in (3..=limit).step_by(2) {
        let idx = i / 2;
        if is_composite[idx / 8] & (1 << (idx % 8)) == 0 {
            primes.push(i as u64);
        }
    }

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

    let mut solver = Solver {
        primes,
        binom_half,
        memo: HashMap::with_capacity(4096),
        exponents: Vec::with_capacity(30),
        non_one_count: 0,
        ans: 0,
        dp_buf: vec![0i64; MAX_EXP_SUM / 2 + 2],
        new_dp_buf: vec![0i64; MAX_EXP_SUM / 2 + 2],
    };

    solver.helper(0, 1);
    println!("{}", solver.ans);
}
