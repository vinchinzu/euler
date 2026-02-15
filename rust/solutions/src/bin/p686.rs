// Project Euler 686 - Powers of Two
// Find the Nth j such that 2^j starts with "123".

fn main() {
    let n = 678910;
    let log2 = 2.0f64.log10();
    let lo = 1.23f64.log10();
    let hi = 1.24f64.log10();
    let mut count = 0;
    let mut j = 0i64;
    while count < n {
        j += 1;
        let val = j as f64 * log2;
        let frac = val - val.floor();
        if frac >= lo && frac < hi {
            count += 1;
        }
    }
    println!("{}", j);
}
