// Project Euler 316: Numbers in decimal expansions
fn main() {
    let big_n: i64 = 10_000_000_000_000_000; // 10^16
    let kk = 999_999;

    let mut pow10 = [0u128; 20];
    pow10[0] = 1;
    for i in 1..20 { pow10[i] = pow10[i - 1] * 10; }

    let mut total: u128 = 0;

    for n in 2..=kk {
        let val = big_n / n;
        // Convert val to digits
        let s: Vec<u8> = if val == 0 {
            vec![0]
        } else {
            let mut digits = Vec::new();
            let mut v = val;
            while v > 0 { digits.push((v % 10) as u8); v /= 10; }
            digits.reverse();
            digits
        };
        let l = s.len();

        // For each i from 0 to L, check if prefix of length i == suffix of length i
        for i in 0..=l {
            let prefix = &s[..i];
            let suffix = &s[l - i..];
            if prefix == suffix {
                total += pow10[i];
            }
        }
        total -= l as u128;
    }

    println!("{}", total as i64);
}
