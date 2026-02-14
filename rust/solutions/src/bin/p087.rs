// Project Euler 87: Prime power triples
// Count numbers < 50,000,000 expressible as p^2 + q^3 + r^4.

use euler_utils::primes_up_to;

fn main() {
    const LIMIT: usize = 50_000_000;
    let max_prime = (LIMIT as f64).sqrt() as usize + 1;
    let primes = primes_up_to(max_prime);

    let mut seen = vec![false; LIMIT];

    for &p in &primes {
        let sq = p * p;
        if sq >= LIMIT {
            break;
        }
        for &q in &primes {
            let cube = q * q * q;
            if sq + cube >= LIMIT {
                break;
            }
            for &r in &primes {
                let fourth = r * r * r * r;
                let s = sq + cube + fourth;
                if s >= LIMIT {
                    break;
                }
                seen[s] = true;
            }
        }
    }

    let count = seen.iter().filter(|&&x| x).count();
    println!("{count}");
}
