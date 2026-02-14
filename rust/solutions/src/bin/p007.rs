// Project Euler 7: 10001st prime
use euler_utils::primes_up_to;

fn main() {
    // Upper bound for the 10001st prime: ~115000 is safe
    let primes = primes_up_to(120_000);
    println!("{}", primes[10000]);
}
