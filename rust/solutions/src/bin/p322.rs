// Project Euler 322: Binomial coefficients divisible by 10
fn num_no_carries(limit: i64, k: i64, p: i64) -> i64 {
    if limit <= 0 { return 0; }
    let mut largest_pow: i64 = 1;
    while largest_pow * p <= limit { largest_pow *= p; }

    let mut num_remaining: i64 = 1;
    let mut pp: i64 = 1;
    while pp < largest_pow {
        num_remaining *= p - (k / pp % p);
        pp *= p;
    }

    let mut result: i64 = 0;
    let top_digit_limit = limit / largest_pow;
    let top_digit_k = k / largest_pow % p;

    for i in 0..p {
        if i + top_digit_k >= p { break; }
        if i == top_digit_limit {
            result += num_no_carries(limit % largest_pow, k, p);
        } else if i < top_digit_limit {
            result += num_remaining;
        }
    }
    result
}

fn to_base5(mut k: i64) -> Vec<i64> {
    let mut tmp = Vec::new();
    while k > 0 { tmp.push(k % 5); k /= 5; }
    tmp.reverse();
    tmp
}

fn main() {
    let big_n: i64 = 1_000_000_000_000_000_000; // 10^18
    let big_k: i64 = 1_000_000_000_000 - 10; // 10^12 - 10
    let limit = big_n - big_k;

    let no_carry_2 = num_no_carries(limit, big_k, 2);
    let no_carry_5 = num_no_carries(limit, big_k, 5);

    let k5_digits = to_base5(big_k);
    let ndigits = k5_digits.len();

    // Pre-compute total capacity: product of (5 - k5_digits[i]) for all digits
    let total_capacity: usize = k5_digits.iter().map(|&d| (5 - d) as usize).product();

    // Double-buffering approach to avoid repeated allocation
    let mut vals = Vec::with_capacity(total_capacity);
    let mut new_vals = Vec::with_capacity(total_capacity);
    vals.push(0i64);

    for di in 0..ndigits {
        let d = k5_digits[di];
        let allowed = 5 - d;
        new_vals.clear();
        for &v in &vals {
            for a in 0..allowed {
                new_vals.push(v * 5 + a);
            }
        }
        std::mem::swap(&mut vals, &mut new_vals);
    }

    let mut big_pow5: i64 = 1;
    for _ in 0..ndigits { big_pow5 *= 5; }

    // Use u64 throughout for the inner loop
    let big_k_u: u64 = big_k as u64;
    let limit_u: u64 = limit as u64;
    let big_pow5_u: u64 = big_pow5 as u64;
    let mut no_carry_both: i64 = 0;

    for &val in &vals {
        let val_u = val as u64;
        if val_u >= limit_u { continue; }
        let max_j = (limit_u - 1 - val_u) / big_pow5_u;
        for j in 0..=max_j {
            let d = val_u + j * big_pow5_u;
            if d & big_k_u == 0 {
                no_carry_both += 1;
            }
        }
    }

    let ans = limit - no_carry_2 - no_carry_5 + no_carry_both;
    println!("{ans}");
}
