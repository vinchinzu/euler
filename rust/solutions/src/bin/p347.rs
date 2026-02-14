// Project Euler 347: Largest integer divisible by two primes
//
// S(N): sum of all distinct M(p,q,N) for prime pairs p<q with p*q<=N.
// M(p,q,N) = largest m<=N with exactly prime factors {p,q}.

use euler_utils::primes_up_to;

fn main() {
    const N: u64 = 10_000_000;

    let primes = primes_up_to(N as usize);

    let mut total: u64 = 0;

    for (i, &p) in primes.iter().enumerate() {
        let p = p as u64;
        if p * p > N {
            break;
        }

        for &q in &primes[i + 1..] {
            let q = q as u64;
            if p * q > N {
                break;
            }

            // Find largest p^a * q^b <= N with a>=1, b>=1
            let mut best: u64 = 0;
            let mut pa = p;
            while pa * q <= N {
                // For this pa, find largest q^b such that pa * q^b <= N
                let mut val = pa;
                while val * q <= N {
                    val *= q;
                }
                if val > best {
                    best = val;
                }
                pa *= p;
            }

            total += best;
        }
    }

    println!("{}", total);
}
