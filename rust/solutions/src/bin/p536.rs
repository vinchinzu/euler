// Project Euler 536 - Modulo Power Identity
//
// Find sum of all m <= N such that a^{m+4} = a (mod m) for all a.
// m must be squarefree, and lambda(m) | m+3.

use euler_utils::{gcd, lcm};

const SIEVE_LIMIT: usize = 100_000_000;

fn sieve_spf(limit: usize) -> Vec<u32> {
    let mut spf = vec![0u32; limit + 1];
    for i in 2..=limit { if spf[i] == 0 { spf[i] = i as u32; } }
    let mut i = 2;
    while i * i <= limit {
        if spf[i] == i as u32 {
            let mut j = i * i;
            while j <= limit {
                if spf[j] == j as u32 { spf[j] = i as u32; }
                j += i;
            }
        }
        i += 1;
    }
    spf
}

fn sieve_primes(limit: usize) -> Vec<i32> {
    let mut is_p = vec![true; limit + 1];
    is_p[0] = false;
    if limit >= 1 { is_p[1] = false; }
    let mut i = 2;
    while i * i <= limit {
        if is_p[i] {
            let mut j = i * i;
            while j <= limit { is_p[j] = false; j += i; }
        }
        i += 1;
    }
    (2..=limit).filter(|&i| is_p[i]).map(|i| i as i32).collect()
}

fn mod_inv_fn(a: i64, m: i64) -> i64 {
    if m == 1 { return 0; }
    let (mut t, mut new_t, mut r, mut new_r) = (0i64, 1i64, m, a);
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t; new_t = t - q * new_t; t = tmp;
        let tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    if r != 1 { return -1; }
    if t < 0 { t += m; }
    t
}

fn imod(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

struct Solver {
    spf: Vec<u32>,
    primes: Vec<i32>,
    n_val: i64,
    ans: i64,
}

impl Solver {
    fn good(&self, m: i64, mut r: i64, max_p: i32) -> bool {
        while r > 1 {
            let p = if (r as usize) < SIEVE_LIMIT {
                self.spf[r as usize] as i32
            } else {
                r as i32
            };
            if (m + 3) % (p as i64 - 1) != 0 { return false; }
            if p >= max_p { return false; }
            r /= p as i64;
            if r % p as i64 == 0 { return false; }
        }
        true
    }

    fn helper(&mut self, max_index: usize, m: i64, carmichael: i64) {
        let g = gcd(m as u64, carmichael as u64) as i64;
        if 3 % g != 0 { return; }
        if (m + 3) % carmichael == 0 {
            self.ans += m;
        }

        if self.n_val / m < SIEVE_LIMIT as i64
            && self.n_val / m / carmichael < (1i64 << max_index)
        {
            let mod_val = carmichael / g;
            if mod_val > 0 {
                let inv = mod_inv_fn(m / g, mod_val);
                if inv >= 0 {
                    let r_start = imod((-3 / g) * inv, mod_val);
                    let mut r = r_start;
                    while m * r <= self.n_val {
                        if r > 1 {
                            let mp = if max_index < self.primes.len() {
                                self.primes[max_index]
                            } else {
                                (self.n_val + 1) as i32
                            };
                            if self.good(m * r, r, mp) {
                                self.ans += m * r;
                            }
                        }
                        r += mod_val;
                    }
                }
            }
            return;
        }

        for index in (0..max_index).rev() {
            let p = self.primes[index] as i64;
            if m * p > self.n_val { continue; }
            let new_lcm = lcm(carmichael as u64, (p - 1) as u64) as i64;
            self.helper(index, m * p, new_lcm);
        }
    }
}

fn main() {
    let n_val: i64 = 1_000_000_000_000;
    let sqrt_n = (n_val as f64).sqrt() as usize + 1;

    let spf = sieve_spf(SIEVE_LIMIT);
    let primes = sieve_primes(sqrt_n);
    let nprimes = primes.len();

    let mut solver = Solver {
        spf,
        primes,
        n_val,
        ans: 0,
    };

    solver.helper(nprimes, 1, 1);

    println!("{}", solver.ans);
}
