// Project Euler 738 - Counting Ordered Factorisations

const MOD: i64 = 1_000_000_007;
const HBITS: usize = 21;
const HSIZE: usize = 1 << HBITS;
const HMASK: usize = HSIZE - 1;

struct Table {
    keys: Vec<u64>,
    vals: Vec<u32>,
}

impl Table {
    fn new() -> Self {
        Self {
            keys: vec![0u64; HSIZE],
            vals: vec![0u32; HSIZE],
        }
    }

    #[inline(always)]
    fn get(&self, key: u64) -> Option<i64> {
        let mut idx = (key.wrapping_mul(0x517cc1b727220a95) >> (64 - HBITS)) as usize;
        loop {
            let k = unsafe { *self.keys.get_unchecked(idx) };
            if k == key {
                return Some(unsafe { *self.vals.get_unchecked(idx) } as i64);
            }
            if k == 0 {
                return None;
            }
            idx = (idx + 1) & HMASK;
        }
    }

    #[inline(always)]
    fn insert(&mut self, key: u64, val: i64) {
        let mut idx = (key.wrapping_mul(0x517cc1b727220a95) >> (64 - HBITS)) as usize;
        loop {
            let k = unsafe { *self.keys.get_unchecked(idx) };
            if k == 0 {
                unsafe {
                    *self.keys.get_unchecked_mut(idx) = key;
                    *self.vals.get_unchecked_mut(idx) = val as u32;
                }
                return;
            }
            if k == key {
                return;
            }
            idx = (idx + 1) & HMASK;
        }
    }
}

#[inline(always)]
fn isqrt(n: i64) -> i64 {
    (n as f64).sqrt() as i64
}

#[inline(always)]
fn icbrt(n: i64) -> i64 {
    (n as f64).cbrt() as i64
}

#[inline(always)]
fn iroot(n: i64, k: i32) -> i64 {
    if k == 1 {
        return n;
    }
    if k == 2 {
        return isqrt(n);
    }
    if k == 3 {
        return icbrt(n);
    }
    let mut r = (n as f64).powf(1.0 / k as f64) as i64;
    #[inline(always)]
    fn pow(mut base: i64, mut exp: i32) -> i64 {
        let mut res = 1i64;
        while exp > 0 {
            if exp & 1 == 1 {
                match res.checked_mul(base) {
                    Some(v) => res = v,
                    None => return i64::MAX,
                }
            }
            exp >>= 1;
            if exp > 0 {
                match base.checked_mul(base) {
                    Some(v) => base = v,
                    None => base = i64::MAX,
                }
            }
        }
        res
    }
    while pow(r + 1, k) <= n {
        r += 1;
    }
    while r > 0 && pow(r, k) > n {
        r -= 1;
    }
    r
}

fn num_products(
    min_val: i64,
    count: i32,
    n_val: i64,
    memo: &mut Table,
) -> i64 {
    if count == 1 {
        let v = n_val - min_val + 1;
        if v < 0 {
            return 0;
        }
        return v % MOD;
    }

    if count == 2 {
        let m = isqrt(n_val);
        if min_val > m {
            return 0;
        }
        let key = (n_val as u64) | ((count as u64) << 34) | ((min_val as u64) << 40);
        if let Some(c) = memo.get(key) {
            return c;
        }
        let len = m - min_val + 1;
        let linear = len * (2 - min_val - m) / 2;
        let mut sum_div: i64 = 0;
        let mut i = min_val;
        while i + 4 <= m + 1 {
            sum_div += n_val / i + n_val / (i + 1) + n_val / (i + 2) + n_val / (i + 3);
            i += 4;
        }
        while i <= m {
            sum_div += n_val / i;
            i += 1;
        }
        let res = ((sum_div % MOD + linear % MOD) % MOD + MOD) % MOD;
        memo.insert(key, res);
        return res;
    }

    let max_i = iroot(n_val, count);
    if min_val > max_i {
        return 0;
    }

    let key = (n_val as u64) | ((count as u64) << 34) | ((min_val as u64) << 40);
    if let Some(c) = memo.get(key) {
        return c;
    }

    let mut res: i64 = 0;
    for i in min_val..=max_i {
        res += num_products(i, count - 1, n_val / i, memo);
        if res >= (MOD << 16) {
            res %= MOD;
        }
    }
    res %= MOD;
    memo.insert(key, res);
    res
}

fn main() {
    let n: i64 = 10_000_000_000; // 10^10

    let mut memo = Table::new();

    let mut ans = n % MOD;
    let mut k = 1;
    while (1i64 << k) <= n {
        let np = num_products(2, k, n, &mut memo);
        let mult = (n - k as i64 + 1) % MOD;
        ans = (ans + mult * np % MOD) % MOD;
        k += 1;
    }

    println!("{}", ans);
}
