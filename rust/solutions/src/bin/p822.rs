// Project Euler 822 - Square the Smallest

const K: usize = 10000;
const M: i64 = 1234567891;

#[inline]
fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result: i64 = 1;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Number {
    log_val: f64,
    mod_val: i64,
    original: i32,
}

fn main() {
    let n: i64 = 10_000_000_000_000_000;
    let sz = K - 1;

    let mut nums: Vec<Number> = (2..=K as i32)
        .map(|v| Number {
            log_val: (v as f64).ln(),
            mod_val: v as i64,
            original: v,
        })
        .collect();

    nums.sort_unstable_by(|a, b| {
        a.log_val
            .partial_cmp(&b.log_val)
            .unwrap()
            .then(a.original.cmp(&b.original))
    });

    let mut t = n;
    while t % sz as i64 != 0 || unsafe { nums.get_unchecked(0).log_val * 2.0 < nums.get_unchecked(sz - 1).log_val } {
        let first_log;
        let first_mod;
        let first_orig;
        unsafe {
            let first = nums.get_unchecked(0);
            first_log = first.log_val;
            first_mod = first.mod_val;
            first_orig = first.original;
        }

        let new_log = first_log * 2.0;
        let new_mod = pow_mod(first_mod, 2, M);

        let mut lo = 0usize;
        let mut hi = sz - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            let mid_next = mid + 1;
            unsafe {
                let next = nums.get_unchecked(mid_next);
                if next.log_val < new_log
                    || (next.log_val == new_log && next.original < first_orig)
                {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
        }

        nums.copy_within(1..=lo, 0);

        nums[lo] = Number {
            log_val: new_log,
            mod_val: new_mod,
            original: first_orig,
        };

        t -= 1;
    }

    let exp = pow_mod(2, t / sz as i64, M - 1);
    let mut ans: i64 = 0;
    for i in 0..sz {
        unsafe {
            ans = (ans + pow_mod(nums.get_unchecked(i).mod_val, exp, M)) % M;
        }
    }

    println!("{}", ans);
}
