// Project Euler 946
// Continued fraction of beta = (2*alpha+3)/(3*alpha+2)
// where alpha has CF [2;1,1,2,1,1,1,2,...] with primes-many 1's between 2's.
// Find sum of first 10^8 coefficients of CF of beta.
//
// Values of the homography matrix stay tiny (|a|,|b|,|c|,|d| <= 79 for 10^8
// coefficients), so plain i32 arithmetic replaces software i128.

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

/// Floor division for possibly-negative a, b. |a|,|b| tiny so i32 is fine.
#[inline(always)]
fn floor_div(a: i32, b: i32) -> i32 {
    // Hot path: both positive (vast majority of steps after initial)
    if a >= 0 && b > 0 {
        return a / b;
    }
    if b == 0 {
        return 0;
    }
    let mut res = a / b;
    if (a ^ b) < 0 && a % b != 0 {
        res -= 1;
    }
    res
}

fn main() {
    // Homography [[a,b],[c,d]] representing (a*alpha+b)/(c*alpha+d)
    let mut a: i32 = 2;
    let mut b: i32 = 3;
    let mut c: i32 = 3;
    let mut d: i32 = 2;

    let mut sum_beta: i64 = 0;
    let mut count_beta: i64 = 0;
    let target: i64 = 100_000_000;

    let mut state = 0i32;
    let mut current_prime = 2i32;
    let mut ones_left = 0i32;

    while count_beta < target {
        let denom1 = c + d;
        let denom_inf = c;

        let mut matched = false;
        let mut q: i32 = 0;

        if denom1 != 0 && denom_inf != 0 {
            let q1 = floor_div(a + b, denom1);
            let q_inf = floor_div(a, denom_inf);
            if q1 == q_inf {
                matched = true;
                q = q1;
            }
        }

        if matched {
            sum_beta += q as i64;
            count_beta += 1;

            // Output: [[0,1],[1,-q]] * M
            let next_a = c;
            let next_b = d;
            let next_c = a - q * c;
            let next_d = b - q * d;
            a = next_a;
            b = next_b;
            c = next_c;
            d = next_d;
        } else {
            // Input next alpha coefficient
            let coeff: i32;
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
                // state == 2: next prime block of ones
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

            // Input: M * [[coeff,1],[1,0]]
            // Specialize coeff==1 (≈99.997% of inputs): a' = a+b
            if coeff == 1 {
                let next_a = a + b;
                let next_b = a;
                let next_c = c + d;
                let next_d = c;
                a = next_a;
                b = next_b;
                c = next_c;
                d = next_d;
            } else {
                // coeff == 2
                let next_a = a * 2 + b;
                let next_b = a;
                let next_c = c * 2 + d;
                let next_d = c;
                a = next_a;
                b = next_b;
                c = next_c;
                d = next_d;
            }
        }
    }

    println!("{}", sum_beta);
}
