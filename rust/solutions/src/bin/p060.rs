// Project Euler 60: Prime pair sets
// Find the lowest sum set of five primes where any two concatenate (both ways) to a prime.
use euler_utils::sieve;

fn concat_nums(a: u64, b: u64) -> u64 {
    let mut m = 10u64;
    while m <= b {
        m *= 10;
    }
    a * m + b
}

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = ((r as u128 * base as u128) % m as u128) as u64;
        }
        base = ((base as u128 * base as u128) % m as u128) as u64;
        exp >>= 1;
    }
    r
}

// Deterministic Miller-Rabin for n < 4_759_123_141 (covers 8-digit concatenations).
fn miller(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u64, 3, 5, 7] {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }
    let mut d = n - 1;
    let mut s = 0u32;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'witness: for &a in &[2u64, 7, 61] {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = ((x as u128 * x as u128) % n as u128) as u64;
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

fn is_pair(primes: &[u64], cache: &mut [u8], i: usize, j: usize) -> bool {
    let np = primes.len();
    let idx = i * np + j;
    if cache[idx] != 0 {
        return cache[idx] == 1;
    }
    let a = primes[i];
    let b = primes[j];
    // 10^k ≡ 1 (mod 3), so concat ≡ a+b (mod 3). Both > 3 and a+b ≡ 0 (mod 3)
    // means both concatenations are divisible by 3.
    let ok = if a != 3 && b != 3 && (a + b) % 3 == 0 {
        false
    } else {
        miller(concat_nums(a, b)) && miller(concat_nums(b, a))
    };
    let v = if ok { 1 } else { 2 };
    cache[idx] = v;
    cache[j * np + i] = v;
    ok
}

fn main() {
    let limit = 10_000usize;
    let is_p = sieve(limit);
    // 2 and 5 never work: one concatenation is even or ends in 5.
    let primes: Vec<u64> = (3..limit)
        .filter(|&n| is_p[n] && n != 5)
        .map(|n| n as u64)
        .collect();
    let np = primes.len();
    // 0 unknown, 1 pair, 2 not pair
    let mut cache = vec![0u8; np * np];

    for a in 0..np {
        for b in a + 1..np {
            if !is_pair(&primes, &mut cache, a, b) {
                continue;
            }
            for c in b + 1..np {
                if !is_pair(&primes, &mut cache, a, c) || !is_pair(&primes, &mut cache, b, c) {
                    continue;
                }
                for d in c + 1..np {
                    if !is_pair(&primes, &mut cache, a, d)
                        || !is_pair(&primes, &mut cache, b, d)
                        || !is_pair(&primes, &mut cache, c, d)
                    {
                        continue;
                    }
                    for e in d + 1..np {
                        if is_pair(&primes, &mut cache, a, e)
                            && is_pair(&primes, &mut cache, b, e)
                            && is_pair(&primes, &mut cache, c, e)
                            && is_pair(&primes, &mut cache, d, e)
                        {
                            let sum = primes[a] + primes[b] + primes[c] + primes[d] + primes[e];
                            println!("{sum}");
                            return;
                        }
                    }
                }
            }
        }
    }
}
