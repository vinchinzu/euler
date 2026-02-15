// Project Euler 479: Roots on the Rise

const MODULUS: u64 = 1_000_000_007;

#[inline(always)]
fn mpow(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MODULUS;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % MODULUS; }
        base = base * base % MODULUS;
        exp >>= 1;
    }
    result
}

#[inline(always)]
fn minv(a: u64) -> u64 { mpow(a, MODULUS - 2) }

fn main() {
    let n: u64 = 1_000_000;
    let mut ans: u64 = 0;

    for k in 1..=n {
        let k_sq = k * k % MODULUS;
        let term = (1 + MODULUS - k_sq) % MODULUS;
        let geo = (1 + MODULUS - mpow(term, n)) % MODULUS;
        let val = term * geo % MODULUS * minv(k_sq) % MODULUS;
        ans = (ans + val) % MODULUS;
    }

    println!("{}", ans);
}
