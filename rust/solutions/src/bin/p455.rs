// Project Euler 455: Powers with trailing digits
// Independent n in 2..=N; modulus K=10^9 fits u64 mulmod.

use rayon::prelude::*;

fn pow_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn main() {
    let n = 1_000_000u64;
    let k: u64 = 1_000_000_000;
    let ans: u64 = (2..=n)
        .into_par_iter()
        .filter(|&n_val| n_val % 10 != 0)
        .map(|n_val| {
            let mut f = 2u64;
            loop {
                let nf = pow_mod(n_val, f, k);
                if nf == f {
                    break;
                }
                f = nf;
            }
            f
        })
        .sum();

    println!("{}", ans);
}
