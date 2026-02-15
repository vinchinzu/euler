// Project Euler 708 - Twos Are All You Need
//
// Count numbers up to N=10^14 that are "twos are all you need" using
// recursive enumeration over powerful numbers with Lucy DP for
// divisor count prefix sums.

fn sum_floor_quotients(n: i64) -> i64 {
    let mut result: i64 = 0;
    let mut i: i64 = 1;
    while i <= n {
        let q = n / i;
        let r = n / q;
        result += q * (r - i + 1);
        i = r + 1;
    }
    result
}

static mut ANS: i64 = 0;

fn helper(
    min_index: usize,
    d: i64,
    mult: i64,
    primes: &[i64],
    big_n: i64,
    l: i64,
    sum_fq_arr: &[i64],
) {
    let q = big_n / d;
    let sum_val = if q <= l {
        sum_fq_arr[q as usize]
    } else {
        sum_floor_quotients(q)
    };
    unsafe {
        ANS += sum_val * mult;
    }

    for index in min_index..primes.len() {
        let p = primes[index];
        if (d as f64) * (p as f64) * (p as f64) > big_n as f64 {
            break;
        }
        let mut new_d = d * p;
        let mut e = 2;
        while (new_d as f64) * (p as f64) <= big_n as f64 {
            new_d *= p;
            helper(index + 1, new_d, mult << (e - 2), primes, big_n, l, sum_fq_arr);
            e += 1;
        }
    }
}

fn main() {
    let big_n: i64 = 100_000_000_000_000; // 10^14
    let mut l = (big_n as f64).sqrt() as i64;
    while (l + 1) * (l + 1) <= big_n {
        l += 1;
    }
    while l * l > big_n {
        l -= 1;
    }

    // Sieve primes up to l
    let limit = l as usize;
    let mut is_composite = vec![false; limit + 1];
    let mut primes: Vec<i64> = Vec::new();
    for i in 2..=limit {
        if !is_composite[i] {
            primes.push(i as i64);
            if (i as u64) * (i as u64) <= limit as u64 {
                let mut j = i * i;
                while j <= limit {
                    is_composite[j] = true;
                    j += i;
                }
            }
        }
    }

    // Precompute num_divisors for small values
    let mut num_div = vec![0i32; limit + 1];
    for i in 1..=limit {
        let mut j = i;
        while j <= limit {
            num_div[j] += 1;
            j += i;
        }
    }

    // Precompute cumulative sum of divisor counts
    let mut sum_fq_arr = vec![0i64; limit + 1];
    for i in 1..=limit {
        sum_fq_arr[i] = sum_fq_arr[i - 1] + num_div[i] as i64;
    }

    helper(0, 1, 1, &primes, big_n, l, &sum_fq_arr);

    println!("{}", unsafe { ANS });
}
