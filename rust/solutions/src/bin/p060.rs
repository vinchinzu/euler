// Project Euler 60: Prime pair sets
// Find the lowest sum set of five primes where any two concatenate (both ways) to a prime.
use euler_utils::is_prime;

fn concat_nums(a: u64, b: u64) -> u64 {
    let mut mult = 1u64;
    let mut tmp = b;
    if tmp == 0 {
        mult = 10;
    } else {
        while tmp > 0 {
            mult *= 10;
            tmp /= 10;
        }
    }
    a * mult + b
}

fn is_concat_prime(a: u64, b: u64) -> bool {
    is_prime(concat_nums(a, b)) && is_prime(concat_nums(b, a))
}

fn main() {
    let limit = 10_000u64;
    let primes: Vec<u64> = (2..limit).filter(|&n| is_prime(n)).collect();
    let np = primes.len();

    for a in 0..np {
        for b in a + 1..np {
            if !is_concat_prime(primes[a], primes[b]) {
                continue;
            }
            for c in b + 1..np {
                if !is_concat_prime(primes[a], primes[c])
                    || !is_concat_prime(primes[b], primes[c])
                {
                    continue;
                }
                for d in c + 1..np {
                    if !is_concat_prime(primes[a], primes[d])
                        || !is_concat_prime(primes[b], primes[d])
                        || !is_concat_prime(primes[c], primes[d])
                    {
                        continue;
                    }
                    for e in d + 1..np {
                        if !is_concat_prime(primes[a], primes[e])
                            || !is_concat_prime(primes[b], primes[e])
                            || !is_concat_prime(primes[c], primes[e])
                            || !is_concat_prime(primes[d], primes[e])
                        {
                            continue;
                        }
                        let sum = primes[a] + primes[b] + primes[c] + primes[d] + primes[e];
                        println!("{sum}");
                        return;
                    }
                }
            }
        }
    }
}
