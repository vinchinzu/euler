// Project Euler 63: Powerful digit counts
// Count n-digit numbers that are also an nth power.
// b^n has n digits when 10^(n-1) <= b^n < 10^n, so b < 10.
// For b=1..9, n digits when n*log10(b) >= n-1, i.e. n <= 1/(1-log10(b)).

fn main() {
    let mut count = 0u32;
    for b in 1u64..=9 {
        let mut n = 1u32;
        loop {
            // b^n has n digits when floor(n*log10(b)) + 1 == n
            let num_digits = (n as f64 * (b as f64).log10()).floor() as u32 + 1;
            if num_digits == n {
                count += 1;
            } else if num_digits < n {
                break;
            }
            n += 1;
        }
    }
    println!("{count}");
}
