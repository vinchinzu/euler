// Project Euler 214: Totient Chains
// Find sum of all primes p < 40,000,000 such that the totient chain
// p -> phi(p) -> phi(phi(p)) -> ... -> 1 has exactly 25 terms.

const LIMIT: usize = 40_000_000;

fn main() {
    // Euler's totient via sieve
    let mut phi = vec![0u32; LIMIT];
    for i in 0..LIMIT {
        phi[i] = i as u32;
    }
    for i in 2..LIMIT {
        if phi[i] == i as u32 {
            // i is prime
            let mut j = i;
            while j < LIMIT {
                phi[j] = phi[j] / i as u32 * (i as u32 - 1);
                j += i;
            }
        }
    }

    // Chain lengths
    let mut chain = vec![0u32; LIMIT];
    chain[1] = 1;

    let mut ans: i64 = 0;
    for i in 2..LIMIT {
        chain[i] = chain[phi[i] as usize] + 1;
        if phi[i] == (i as u32 - 1) && chain[i] == 25 {
            ans += i as i64;
        }
    }

    println!("{ans}");
}
