// Project Euler 705 - Total Inversion Count of Divisibility
//
// Sieve primes up to 10^8, compute digit-based divisor sequence inversions.

const N: usize = 100_000_000;
const M: u64 = 1_000_000_007;

fn mod_inv(a: u64) -> u64 {
    let mut result: u64 = 1;
    let mut exp = M - 2;
    let mut base = a % M;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result as u128 * base as u128 % M as u128 as u64;
        }
        base = base as u128 * base as u128 % M as u128 as u64;
        exp >>= 1;
    }
    result
}

fn digits(mut n: usize) -> Vec<usize> {
    let mut d = Vec::new();
    while n > 0 {
        d.push(n % 10);
        n /= 10;
    }
    d
}

fn main() {
    // Sieve
    let mut is_prime = vec![true; N];
    is_prime[0] = false;
    if N > 1 {
        is_prime[1] = false;
    }
    let mut i = 2;
    while i * i < N {
        if is_prime[i] {
            let mut j = i * i;
            while j < N {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let primes: Vec<usize> = (2..N).filter(|&i| is_prime[i]).collect();

    let divisors_table: [&[usize]; 10] = [
        &[],
        &[1],
        &[1, 2],
        &[1, 3],
        &[1, 2, 4],
        &[1, 5],
        &[1, 2, 3, 6],
        &[1, 7],
        &[1, 2, 4, 8],
        &[1, 3, 9],
    ];
    let num_divisors: [usize; 10] = [0, 1, 2, 2, 3, 2, 4, 2, 4, 3];

    // Compute numSequences
    let mut num_sequences: u64 = 1;
    for &p in primes.iter().rev() {
        let d = digits(p);
        for &k in &d {
            if k > 0 {
                num_sequences = (num_sequences as u128 * num_divisors[k] as u128 % M as u128) as u64;
            }
        }
    }

    // Precompute mod inverses
    let mut mod_invs = [0u64; 10];
    for i in 1..10 {
        mod_invs[i] = mod_inv(i as u64);
    }

    let mut counts = [0u64; 10];
    let mut ans: u64 = 0;

    for &p in primes.iter().rev() {
        let digs = digits(p);
        for &k in &digs {
            if k > 0 {
                let nd = num_divisors[k];
                let inv_nd = mod_invs[nd];

                // Count inversions
                for &div in divisors_table[k] {
                    for i in 1..div {
                        ans = (ans + counts[i] as u128 * inv_nd as u128 % M as u128) as u64 % M;
                    }
                }

                // Update counts
                for &div in divisors_table[k] {
                    counts[div] = (counts[div] as u128 + num_sequences as u128 * inv_nd as u128 % M as u128) as u64 % M;
                }
            }
        }
    }

    println!("{}", ans);
}
