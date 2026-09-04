// Project Euler 946
// Continued fraction of beta = (2*alpha+3)/(3*alpha+2)
// where alpha has CF [2;1,1,2,1,1,1,2,...] with primes-many 1's between 2's.
// Find sum of first 10^8 coefficients of CF of beta.
//
// Values of the homography matrix stay tiny (|a|,|b|,|c|,|d| <= 79 for 10^8
// coefficients), so plain i32 arithmetic replaces software i128.

fn main() {
    // Homography [[a,b],[c,d]] representing (a*alpha+b)/(c*alpha+d)
    let mut a: i32 = 2;
    let mut b: i32 = 3;
    let mut c: i32 = 3;
    let mut d: i32 = 2;

    let mut sum_beta: i64 = 0;
    let mut count_beta: i64 = 0;
    let target: i64 = 100_000_000;

    // Precompute primes
    const PRIME_LIMIT: usize = 90_000;
    let mut is_prime = vec![true; PRIME_LIMIT];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut p = 2;
    while p * p < PRIME_LIMIT {
        if is_prime[p] {
            let mut m = p * p;
            while m < PRIME_LIMIT {
                is_prime[m] = false;
                m += p;
            }
        }
        p += 1;
    }
    let mut primes = Vec::with_capacity(9000);
    for (i, &prime) in is_prime.iter().enumerate().skip(2) {
        if prime {
            primes.push(i as i32);
        }
    }

    let mut state = 0i32;
    let mut prime_idx = 0usize;
    let mut current_prime = primes[0];
    let mut ones_left = 0i32;

    while count_beta < target {
        // Fast-forward consecutive 1s when in periodic attractor states
        if state == 1 && ones_left > 0 {
            if a == 3 && b == 1 && c == 1 && d == 2 {
                let k = (ones_left as i64).min(target - count_beta) as i32;
                if k > 0 {
                    ones_left -= k;
                    count_beta += k as i64;
                    sum_beta += k as i64;
                    continue;
                }
            } else if a == 3 && b == 2 && c == 1 && d == -1 {
                let cycles = ((ones_left / 5) as i64).min(target - count_beta) as i32;
                if cycles > 0 {
                    ones_left -= cycles * 5;
                    count_beta += cycles as i64;
                    sum_beta += cycles as i64 * 11;
                    continue;
                }
            }
        }

        let denom1 = c + d;
        let mut matched = false;
        let mut q: i32 = 0;

        if c > 0 && denom1 > 0 {
            let q_cand = a / c;
            let rem = (a + b) - q_cand * denom1;
            if (rem as u32) < (denom1 as u32) {
                matched = true;
                q = q_cand;
            }
        }

        if matched {
            sum_beta += q as i64;
            count_beta += 1;

            let next_c = a - q * c;
            let next_d = b - q * d;
            a = c;
            b = d;
            c = next_c;
            d = next_d;
        } else {
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
                prime_idx += 1;
                current_prime = primes[prime_idx];
                state = 1;
                ones_left = current_prime - 1;
                coeff = 1;
            }

            if coeff == 1 {
                let next_a = a + b;
                let next_c = c + d;
                b = a;
                d = c;
                a = next_a;
                c = next_c;
            } else {
                let next_a = a * 2 + b;
                let next_c = c * 2 + d;
                b = a;
                d = c;
                a = next_a;
                c = next_c;
            }
        }
    }

    println!("{}", sum_beta);
}
