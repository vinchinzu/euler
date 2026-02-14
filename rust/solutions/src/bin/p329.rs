// Project Euler 329: Prime Frog
// A frog jumps on squares 1..500, croaking P or N.
// Compute probability as reduced fraction.

const NSQUARES: usize = 500;
const SEQLEN: usize = 15;

fn main() {
    // Sieve primes up to NSQUARES
    let mut is_p = vec![false; NSQUARES + 1];
    for i in 2..=NSQUARES {
        is_p[i] = true;
    }
    let mut i = 2;
    while i * i <= NSQUARES {
        if is_p[i] {
            let mut j = i * i;
            while j <= NSQUARES {
                is_p[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let seq = b"PPPPNNPPPNPPNPN";

    // dp[i] = numerator for square i (1-indexed)
    let mut dp = vec![1i128; NSQUARES + 2];

    // Process sequence backward
    for t in (0..SEQLEN).rev() {
        let ch = seq[t];
        let mut tmp = vec![0i128; NSQUARES + 2];
        for i in 1..=NSQUARES {
            // Movement sum (with factor of 2 for uniform treatment)
            let msum = if i == 1 {
                2 * dp[2]
            } else if i == NSQUARES {
                2 * dp[NSQUARES - 1]
            } else {
                dp[i - 1] + dp[i + 1]
            };

            let matched = if ch == b'P' { is_p[i] } else { !is_p[i] };
            tmp[i] = msum * if matched { 2 } else { 1 };
        }
        dp = tmp;
    }

    // Total numerator
    let mut num: i128 = 0;
    for i in 1..=NSQUARES {
        num += dp[i];
    }

    // Denominator = 500 * 6^15
    let mut den: i128 = 500;
    for _ in 0..SEQLEN {
        den *= 6;
    }

    // Reduce fraction
    let g = gcd128(num.unsigned_abs(), den.unsigned_abs());
    num /= g as i128;
    den /= g as i128;

    println!("{}/{}", num, den);
}

fn gcd128(a: u128, b: u128) -> u128 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
