// Project Euler 266 - Pseudo Square Root
// Meet-in-the-middle: split primes < 190 into two halves,
// enumerate subset products (as logs), binary search for best match.

fn main() {
    const M: u128 = 10_000_000_000_000_000; // 10^16

    // Sieve primes below 190
    let mut is_prime = vec![true; 190];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..190 {
        if is_prime[i] {
            let mut j = i * i;
            while j < 190 {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    let primes: Vec<usize> = (2..190).filter(|&i| is_prime[i]).collect();
    let num_primes = primes.len();

    let plogs: Vec<f64> = primes.iter().map(|&p| (p as f64).ln()).collect();

    let mid = num_primes / 2;
    let na = mid;
    let nb = num_primes - mid;

    // Build A subsets
    let size_a = 1usize << na;
    let mut pa: Vec<(f64, u32)> = Vec::with_capacity(size_a);
    for s in 0..size_a {
        let mut lv = 0.0f64;
        for i in 0..na {
            if s & (1 << i) != 0 {
                lv += plogs[i];
            }
        }
        pa.push((lv, s as u32));
    }
    pa.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // log(sqrt(n))
    let log_sqrt: f64 = plogs.iter().sum::<f64>() / 2.0;

    let mut best_log = -1.0f64;
    let mut best_a_mask = 0u32;
    let mut best_b_mask = 0u32;

    // For each B subset, binary search in A
    for sb in 0..(1u32 << nb) {
        let mut b_log = 0.0f64;
        for i in 0..nb {
            if sb & (1 << i) != 0 {
                b_log += plogs[mid + i];
            }
        }
        let target = log_sqrt - b_log;

        // Binary search for largest pa[idx].0 <= target
        let idx = pa.partition_point(|e| e.0 <= target);
        if idx > 0 {
            let cand = b_log + pa[idx - 1].0;
            if cand > best_log {
                best_log = cand;
                best_a_mask = pa[idx - 1].1;
                best_b_mask = sb;
            }
        }
    }

    // Compute answer mod M
    let mut ans: u128 = 1;
    for i in 0..na {
        if best_a_mask & (1 << i) != 0 {
            ans = ans * primes[i] as u128 % M;
        }
    }
    for i in 0..nb {
        if best_b_mask & (1 << i) != 0 {
            ans = ans * primes[mid + i] as u128 % M;
        }
    }

    println!("{}", ans);
}
