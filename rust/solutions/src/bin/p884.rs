// Project Euler 884
// Recursive S(n) using cube root intervals and prefix sums.

use fxhash::FxHashMap;

#[inline(always)]
fn integer_cbrt(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let x = (n as f64).cbrt() as i64;
    let x3 = x * x * x;
    if x3 > n {
        return x - 1;
    }
    let nx = x + 1;
    let nx3 = nx * nx * nx;
    if nx3 <= n {
        return nx;
    }
    x
}

const MAX_K: usize = 470_000;

fn main() {
    let n: i64 = 100_000_000_000_000_000; // 10^17

    if n <= 1 {
        println!("0");
        return;
    }

    let k_max = integer_cbrt(n - 1) as usize;

    let mut prefix_t = vec![0i64; MAX_K + 1];
    let mut memo: FxHashMap<i64, i64> = FxHashMap::with_capacity_and_hasher(100_000, Default::default());

    fn recursive_s(n: i64, prefix_t: &[i64], memo: &mut FxHashMap<i64, i64>) -> i64 {
        if n <= 1 { return 0; }
        
        if let Some(&cached) = memo.get(&n) {
            return cached;
        }

        let k = integer_cbrt(n - 1);
        let full_intervals_sum = unsafe {
            *prefix_t.get_unchecked(k as usize - 1)
        };

        let l = n - k * k * k;
        let partial_sum = l + recursive_s(l, prefix_t, memo);
        let result = full_intervals_sum + partial_sum;
        
        memo.insert(n, result);
        result
    }

    for k in 1..=k_max {
        let k64 = k as i64;
        let l_k = 3 * k64 * k64 + 3 * k64 + 1;
        let val = recursive_s(l_k, &prefix_t, &mut memo);
        let term = l_k + val;
        unsafe {
            *prefix_t.get_unchecked_mut(k) = *prefix_t.get_unchecked(k - 1) + term;
        }
    }

    println!("{}", recursive_s(n, &prefix_t, &mut memo));
}
