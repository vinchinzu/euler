// Project Euler 900
// Compute ((4^N + 2) * 3^(-1) - 2^N) mod P
// where P = 900497239 and N = 10000

fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn main() {
    let p: i64 = 900497239;
    let n: i64 = 10000;

    let four_n = mod_pow(4, n, p);
    let two_n = mod_pow(2, n, p);
    let three_inv = mod_pow(3, p - 2, p); // Fermat's little theorem

    let s = ((four_n + 2) % p * three_inv % p - two_n) % p;
    let s = ((s % p) + p) % p; // Ensure positive

    println!("{}", s);
}
