// Project Euler 312: Cyclic paths on Sierpinski graphs
use euler_utils::{euler_phi, mod_mul, mod_pow};

const N: u64 = 10000;

fn ck(modulus: u64, k: i32) -> u64 {
    let mod1 = 2 * euler_phi(modulus);
    let mod2 = euler_phi(mod1);

    let n = if k == 1 { N } else { ck(mod2, k - 1) };

    let exp1 = mod_pow(3, n - 2, mod1);
    let term1 = mod_pow(2, exp1, modulus);
    let exp2 = (exp1 - 3) / 2;
    let term2 = mod_pow(3, exp2, modulus);

    mod_mul(term1, term2, modulus)
}

fn main() {
    // 13^8
    let mut m: u64 = 1;
    for _ in 0..8 { m *= 13; }

    let result = ck(m, 3);
    println!("{}", result);
}
