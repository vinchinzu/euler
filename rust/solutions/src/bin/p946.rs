// Project Euler 946
// Continued fraction of beta = (2*alpha+3)/(3*alpha+2)
// where alpha has CF [2;1,1,2,1,1,1,2,...] with primes-many 1's between 2's.
// Find sum of first 10^8 coefficients of CF of beta.

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn floor_div(a: i128, b: i128) -> i64 {
    if b == 0 {
        return 0;
    }
    let mut res = a / b;
    if (a ^ b) < 0 && (a % b != 0) {
        res -= 1;
    }
    res as i64
}

fn main() {
    let mut a: i128 = 2;
    let mut b: i128 = 3;
    let mut c: i128 = 3;
    let mut d: i128 = 2;

    let mut sum_beta: i64 = 0;
    let mut count_beta: i64 = 0;
    let target: i64 = 100_000_000;

    let mut state = 0;
    let mut current_prime = 2;
    let mut ones_left = 0;

    while count_beta < target {
        let denom1 = c + d;
        let denom_inf = c;

        let mut matched = false;
        let mut q: i64 = 0;

        if denom1 != 0 && denom_inf != 0 {
            let q1 = floor_div(a + b, denom1);
            let q_inf = floor_div(a, denom_inf);
            if q1 == q_inf {
                matched = true;
                q = q1;
            }
        }

        if matched {
            sum_beta += q;
            count_beta += 1;

            let next_a = c;
            let next_b = d;
            let next_c = a - q as i128 * c;
            let next_d = b - q as i128 * d;

            a = next_a;
            b = next_b;
            c = next_c;
            d = next_d;
        } else {
            let coeff;
            if state == 0 {
                coeff = 2;
                state = 1;
                ones_left = current_prime;
            } else if state == 1 {
                if ones_left > 0 {
                    coeff = 1;
                    ones_left -= 1;
                } else {
                    state = 2;
                    coeff = 2;
                }
            } else {
                // state == 2
                let mut next_p = current_prime + 1;
                while !is_prime(next_p) {
                    next_p += 1;
                }
                current_prime = next_p;

                state = 1;
                ones_left = current_prime;
                coeff = 1;
                ones_left -= 1;
            }

            let next_a = a * coeff as i128 + b;
            let next_b = a;
            let next_c = c * coeff as i128 + d;
            let next_d = c;

            a = next_a;
            b = next_b;
            c = next_c;
            d = next_d;
        }
    }

    println!("{}", sum_beta);
}
