// Project Euler Problem 104: Pandigital Fibonacci Ends
// Find k where F(k) has pandigital (1-9) first 9 and last 9 digits.

fn is_pandigital_1_9(s: &[u8; 9]) -> bool {
    let mut cnt = [0u8; 10];
    for &d in s {
        if d < 1 || d > 9 {
            return false;
        }
        cnt[d as usize] += 1;
    }
    cnt[1..=9].iter().all(|&c| c == 1)
}

fn digits_of(mut n: i64) -> [u8; 9] {
    let mut d = [0u8; 9];
    for i in (0..9).rev() {
        d[i] = (n % 10) as u8;
        n /= 10;
    }
    d
}

fn main() {
    let modulus: i64 = 1_000_000_000;
    let mut a_last: i64 = 1;
    let mut b_last: i64 = 1;

    let log10_phi: f64 = ((1.0f64 + 5.0f64.sqrt()) / 2.0).log10();
    let log10_sqrt5: f64 = 5.0f64.sqrt().log10();

    let mut k = 2u32;
    loop {
        k += 1;
        let c_last = (a_last + b_last) % modulus;
        a_last = b_last;
        b_last = c_last;

        // Check last 9 digits
        let last9 = digits_of(b_last);
        if !is_pandigital_1_9(&last9) {
            continue;
        }

        // Check first 9 digits using log10 approximation
        let log10_fk = k as f64 * log10_phi - log10_sqrt5;
        let frac = log10_fk - log10_fk.floor();
        let first9d = 10.0f64.powf(frac + 8.0);
        let first9_val = first9d as i64;
        let first9 = digits_of(first9_val);

        if is_pandigital_1_9(&first9) {
            println!("{}", k);
            return;
        }
    }
}
