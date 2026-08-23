// Project Euler 420 - 2x2 positive integer matrix M with trace < N
// where M = M^(-1) (involutory matrices)
// Sequential divisor-count sieve (indices only to n/4), then independent t1.

use rayon::prelude::*;

fn main() {
    let n: i64 = 10_000_000;
    // val = (g^2 - r^2)/4 <= t1^2/4 <= n/4
    let div_limit = (n / 4) as usize;

    // Sequential sieve: shared writes, not parallelized.
    let mut num_divs = vec![0u16; div_limit + 1];
    for i in 1..=div_limit {
        let mut j = i;
        while j <= div_limit {
            // SAFETY: j is in 1..=div_limit by the loop bound.
            unsafe {
                *num_divs.get_unchecked_mut(j) += 1;
            }
            j += i;
        }
    }
    let num_divs = num_divs;

    let sq_n = n.isqrt() as usize;
    // Exclusive range so with_min_len applies (RangeInclusive is not indexed).
    let ans: i64 = (1..sq_n + 1)
        .into_par_iter()
        .with_min_len(1)
        .map(|t1| contrib(t1 as u32, n, &num_divs))
        .sum();

    println!("{}", ans);
}

fn contrib(t1: u32, n: i64, num_divs: &[u16]) -> i64 {
    let t1_i = t1 as i64;
    // t1 <= isqrt(n) so 2n-1-t1^2 >= n-1 > 0.
    let t2_max = (2 * n - 1 - t1_i * t1_i).isqrt() as u32;

    let mut ans = 0i64;
    let mut t2 = t1 + 2;
    while t2 <= t2_max {
        let g = gcd(t1, t2);
        let r_max = (t1_i - 2) * g as i64 / t2 as i64;
        let mut r = (g & 1) as i64;
        if r == 0 {
            if r_max >= 0 {
                let val = (g as i64 * g as i64) >> 2;
                // SAFETY: r=0 < g so val = g^2/4 > 0; val <= t1^2/4 <= n/4 < len.
                ans += unsafe { *num_divs.get_unchecked(val as usize) } as i64;
            }
            r = 2;
        }
        // val' for r+2 equals val - (r+1); r and g have the same parity.
        let mut val = (g as i64 * g as i64 - r * r) >> 2;
        while r <= r_max {
            // SAFETY: 0 < r < g (r_max < g since t2 > t1-2) so 0 < val <= n/4 < len.
            ans += 2 * unsafe { *num_divs.get_unchecked(val as usize) } as i64;
            val -= r + 1;
            r += 2;
        }
        t2 += 2;
    }
    ans
}

#[inline(always)]
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
