// Project Euler 158: Exploring strings for exactly one adjacent decrease
fn main() {
    let mut max_val: i64 = 0;
    for n in 2..=26 {
        let euler = (1i64 << n) - n - 1;
        // C(26, n) * (2^n - n - 1)
        let binom = binom_val(26, n);
        let val = binom * euler;
        if val > max_val {
            max_val = val;
        }
    }
    println!("{}", max_val);
}

fn binom_val(n: i64, k: i64) -> i64 {
    let k = k.min(n - k);
    if k < 0 { return 0; }
    let mut num = 1i64;
    let mut den = 1i64;
    for i in 1..=k {
        num *= n - k + i;
        den *= i;
    }
    num / den
}
