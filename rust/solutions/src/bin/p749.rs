// Project Euler 749 - Near Power Sums
//
// Find numbers where the sum of kth powers of digits is 1 away from the
// number itself.

use std::collections::BTreeSet;

const MAXN: usize = 16;
const BASE: u64 = 10;

static mut POW_LIMIT: u64 = 0;
static mut POWS: [u64; 10] = [0; 10];

fn helper(d: u64, num_digits: usize, sum_powers: u64, k: usize, results: &mut BTreeSet<u64>) {
    let pow_limit = unsafe { POW_LIMIT };
    let pows = unsafe { &POWS };

    // Check sum_powers - 1 and sum_powers + 1
    for delta in [-1i64, 1i64] {
        let candidate = (sum_powers as i64 + delta) as u64;
        if candidate > 0 && candidate < pow_limit {
            let mut actual_sum: u64 = 0;
            let mut num = candidate;
            while num > 0 {
                let dig = (num % BASE) as usize;
                actual_sum += pows[dig];
                num /= BASE;
            }
            if actual_sum == sum_powers {
                results.insert(candidate);
            }
        }
    }

    if num_digits < MAXN && num_digits <= k + 1 {
        for new_d in d..BASE {
            let new_sum = sum_powers + pows[new_d as usize];
            if new_sum < pow_limit {
                helper(new_d, num_digits + 1, new_sum, k, results);
            }
        }
    }
}

fn main() {
    unsafe {
        POW_LIMIT = 1;
        for _ in 0..MAXN {
            POW_LIMIT *= BASE;
        }
    }
    let pow_limit = unsafe { POW_LIMIT };

    let mut results = BTreeSet::new();

    let mut k = 2;
    while k <= MAXN + 2 {
        // Compute kth powers of digits
        for d in 0..BASE as usize {
            let mut p: u64 = 1;
            for _ in 0..k {
                p = p.saturating_mul(d as u64);
                if p >= pow_limit {
                    p = pow_limit;
                    break;
                }
            }
            unsafe {
                POWS[d] = p;
            }
        }
        helper(1, 0, 0, k, &mut results);
        k += 2;
    }

    let total: u64 = results.iter().sum();
    println!("{}", total);
}
