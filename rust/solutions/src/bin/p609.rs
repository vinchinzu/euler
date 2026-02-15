// Project Euler 609 - Pi sequences
// Iterated prime-counting sequences, product of bin counts

const NLIMIT: usize = 100_000_000;
const MOD: u64 = 1_000_000_007;

fn main() {
    let mut is_prime = vec![true; NLIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= NLIMIT {
        if is_prime[i] {
            let mut j = i * i;
            while j <= NLIMIT { is_prime[j] = false; j += i; }
        }
        i += 1;
    }

    let mut pi = vec![0u32; NLIMIT + 1];
    let mut count = 0u32;
    for i in 0..=NLIMIT {
        if is_prime[i] { count += 1; }
        pi[i] = count;
    }

    // Find max chain length
    let mut max_len = 0;
    let mut n = NLIMIT;
    while n > 0 { max_len += 1; n = pi[n] as usize; }

    let mut ps = vec![0u64; max_len + 2];

    // Collect primes
    let primes: Vec<usize> = (2..=NLIMIT).filter(|&i| is_prime[i]).collect();

    for (idx, &p) in primes.iter().enumerate() {
        let cnt = if idx == primes.len() - 1 { NLIMIT } else { primes[idx + 1] - 1 } - p;

        let mut n = pi[p] as usize;
        let mut c = 0usize;
        while n > 0 {
            if !is_prime[n] { c += 1; }
            ps[c] += 1;
            ps[c + 1] += cnt as u64;
            n = pi[n] as usize;
        }
    }

    let mut ans = 1u64;
    for i in 0..max_len + 2 {
        if ps[i] != 0 {
            ans = (ans as u128 * (ps[i] % MOD) as u128 % MOD as u128) as u64;
        }
    }

    println!("{}", ans);
}
