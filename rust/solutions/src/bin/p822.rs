// Project Euler 822 - Square the Smallest

const K: usize = 10000;
const M: i64 = 1234567891;

fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result: i64 = 1;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    result
}

#[derive(Clone)]
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

    nums.sort_by(|a, b| a.log_val.partial_cmp(&b.log_val).unwrap()
        .then(a.original.cmp(&b.original)));

    let mut t = n;
    while t % sz as i64 != 0 || nums[0].log_val * 2.0 < nums[sz - 1].log_val {
        let first = nums[0].clone();
        let new_log = first.log_val * 2.0;
        let new_mod = pow_mod(first.mod_val, 2, M);

        // Binary search for insertion point
        let mut lo = 0usize;
        let mut hi = sz - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid + 1].log_val < new_log
                || (nums[mid + 1].log_val == new_log && nums[mid + 1].original < first.original)
            {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        // Shift [1..lo] left
        for i in 0..lo {
            nums[i] = nums[i + 1].clone();
        }
        nums[lo] = Number {
            log_val: new_log,
            mod_val: new_mod,
            original: first.original,
        };

        t -= 1;
    }

    let exp = pow_mod(2, t / sz as i64, M - 1);
    let mut ans: i64 = 0;
    for num in &nums {
        ans = (ans + pow_mod(num.mod_val, exp, M)) % M;
    }

    println!("{}", ans);
}
