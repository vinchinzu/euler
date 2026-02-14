// Project Euler 542 - Geometric Progression with Alternating Sum
//
// S(n) = maximum sum of a geometric progression with >= 3 distinct terms <= n.
// Find sum_{k=4}^{10^17} (-1)^k S(k) using divide-and-conquer.

fn ipow(base: i64, exp: i32) -> i64 {
    let mut result: i64 = 1;
    for _ in 0..exp {
        result = result.checked_mul(base).unwrap_or(i64::MIN);
        if result < 0 { return result; } // overflow sentinel
    }
    result
}

fn ilog2(mut n: i64) -> i32 {
    if n <= 0 { return 0; }
    let mut r = 0;
    while n > 1 { n >>= 1; r += 1; }
    r
}

fn s(n: i64) -> i64 {
    if n < 3 { return 0; }
    let mut max_s: i64 = 0;
    let max_e = ilog2(n);

    for e in (2..=max_e).rev() {
        if (e as i64 + 1) * n < max_s { break; }
        let mut k: i64 = 2;
        loop {
            let ke = ipow(k, e);
            if ke > 2 * n || ke < 0 { break; }
            let r = n / ke;
            if r > 0 {
                let sum_val = (ipow(k, e + 1) - ipow(k - 1, e + 1)) * r;
                if sum_val > max_s { max_s = sum_val; }
            }
            k += 1;
        }
    }
    max_s
}

fn parity(n: i64) -> i64 {
    if n % 2 == 0 { 1 } else { -1 }
}

fn t(low: i64, high: i64) -> i64 {
    if low + 1 == high {
        return if (low + high) % 2 == 0 { 0 } else { parity(low) * s(low) };
    }

    let s_low = s(low);
    let s_high = s(high);

    if s_low == s_high {
        let count = high - low;
        if count % 2 == 0 { return 0; }
        return parity(low) * s_low;
    }

    let mid = (low + high) / 2;
    t(low, mid) + t(mid, high)
}

fn main() {
    let n: i64 = 100_000_000_000_000_000; // 10^17
    println!("{}", t(4, n + 1));
}
