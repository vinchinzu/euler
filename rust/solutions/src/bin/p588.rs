// Project Euler 588 - Quintinomial Coefficients
//
// Let Q(k) be the number of odd coefficients in (x^4+x^3+x^2+x+1)^k.
// Find sum_{k=1}^{18} Q(10^k).

const KK: usize = 5; // quintinomial
const MAX_COUNTS: usize = 1 << (KK - 1); // 16

fn helper(n: i64, size: i64, counts: &mut [i64; MAX_COUNTS]) {
    let mut small_sections = [0usize; 2 * KK];

    if n == 0 {
        *counts = [0i64; MAX_COUNTS];
        counts[0] = 0;
        counts[1] = 1;
        small_sections[0] = 1;
    } else if n < size / 2 {
        let mut sub_counts = [0i64; MAX_COUNTS];
        helper(n, size / 2, &mut sub_counts);
        *counts = sub_counts;
        for i in 0..KK - 1 {
            small_sections[i] = 1 << i;
        }
    } else {
        let mut sub_counts = [0i64; MAX_COUNTS];
        helper(n - size / 2, size / 2, &mut sub_counts);
        *counts = sub_counts;
        for i in 0..KK - 1 {
            small_sections[i] = (1 << (i + 1)) - 1;
            small_sections[i + KK - 1] = (1 << (KK - 1)) - (1 << i);
        }
    }

    let mut new_counts = [0i64; MAX_COUNTS];

    for subset in 0..MAX_COUNTS {
        let mut left_half = 0usize;
        let mut right_half = 0usize;
        for i in 0..KK - 1 {
            if subset & (1 << i) != 0 {
                left_half ^= small_sections[2 * i];
                right_half ^= small_sections[2 * i + 1];
            }
        }
        new_counts[subset] = counts[left_half] + counts[right_half];
    }

    *counts = new_counts;
}

fn main() {
    let n_max = 18;
    let mut ans: i64 = 0;

    for k in 1..=n_max {
        let mut n: i64 = 1;
        for _ in 0..k {
            n *= 10;
        }

        let size: i64 = 1i64 << 62;
        let mut result = [0i64; MAX_COUNTS];
        helper(n, size, &mut result);

        ans += result[1];
    }

    println!("{}", ans);
}
