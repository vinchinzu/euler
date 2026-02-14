// Project Euler 188 - The hyperexponentiation of a number
// Compute 1777↑↑1855 mod 10^8 using iterated Euler's totient.

use euler_utils::{mod_pow, euler_phi, gcd};

fn tetration(a: u64, height: u32, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    if height == 1 {
        return a % modulus;
    }
    let phi_mod = euler_phi(modulus);
    let mut exponent = tetration(a, height - 1, phi_mod);
    if exponent < phi_mod && gcd(a, modulus) != 1 {
        exponent += phi_mod;
    }
    mod_pow(a, exponent, modulus)
}

fn main() {
    let result = tetration(1777, 1855, 100_000_000);
    println!("{}", result);
}
