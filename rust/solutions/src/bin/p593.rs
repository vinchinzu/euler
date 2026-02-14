// Project Euler 593 - Fleeting Medians
//
// Uses a sieve for primes, discrete log in Z/10007Z,
// and a Fenwick tree for sliding-window median computation.

const N: usize = 10_000_000;
const K: usize = 100_000;
const M: usize = 10_007;
const DD: usize = 10_000;

fn main() {
    // Sieve primes
    let limit: usize = 200_000_000;
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 { is_prime[1] = false; }
    {
        let mut i = 2;
        while i * i <= limit {
            if is_prime[i] {
                let mut j = i * i;
                while j <= limit {
                    is_prime[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }
    let mut primes = Vec::with_capacity(N + 2);
    for i in 2..=limit {
        if is_prime[i] {
            primes.push(i);
            if primes.len() >= N + 1 { break; }
        }
    }
    drop(is_prime);

    // Find generator of Z/M
    let g = {
        let phi = M - 1;
        let mut factors = Vec::new();
        let mut temp = phi;
        let mut p = 2;
        while p * p <= temp {
            if temp % p == 0 {
                factors.push(p);
                while temp % p == 0 { temp /= p; }
            }
            p += 1;
        }
        if temp > 1 { factors.push(temp); }

        let mut candidate = 2;
        loop {
            let mut is_gen = true;
            for &f in &factors {
                let mut x: u64 = 1;
                for _ in 0..phi / f {
                    x = x * candidate as u64 % M as u64;
                }
                if x == 1 { is_gen = false; break; }
            }
            if is_gen { break candidate; }
            candidate += 1;
        }
    };

    // Precompute powers
    let mut pows = vec![0u64; M];
    pows[0] = 1;
    for i in 1..M {
        pows[i] = pows[i - 1] * g as u64 % M as u64;
    }

    // Discrete log table
    let mut logs = vec![0usize; M];
    let mut gp: u64 = 1;
    for i in 0..M {
        logs[gp as usize] = i;
        gp = gp * g as u64 % M as u64;
    }

    // Compute S
    let mut s = vec![0u64; N + 2];
    for k in 1..=N + 1 {
        if k - 1 >= primes.len() { break; }
        let p = primes[k - 1];
        if p == M {
            s[k] = 0;
        } else {
            let idx = (k as u64 % (M as u64 - 1)) * (logs[p % M] as u64) % (M as u64 - 1);
            s[k] = pows[idx as usize];
        }
    }

    // Compute S2
    let max_val = 2 * M + 1;
    let mut s2 = vec![0i32; N + 2];
    for k in 1..=N + 1 {
        s2[k] = (s[k] + s[k / DD + 1]) as i32;
    }

    // Fenwick tree
    let mut bit = vec![0i32; max_val];

    let bit_update = |bit: &mut Vec<i32>, mut i: usize, delta: i32| {
        while i < max_val {
            bit[i] += delta;
            i += i & i.wrapping_neg();
        }
    };

    let bit_find_kth = |bit: &[i32], k: i32| -> i32 {
        let mut pos = 0usize;
        let mut sum = 0i32;
        let mut i = 14i32; // log2(max_val) ~ 14
        while i >= 0 {
            let next = pos + (1 << i);
            if next < max_val && sum + bit[next] < k {
                sum += bit[next];
                pos = next;
            }
            i -= 1;
        }
        (pos + 1) as i32
    };

    // Add first K elements
    for i in 1..=K {
        bit_update(&mut bit, s2[i] as usize + 1, 1);
    }

    let mut f = 0.0f64;
    for next in K + 1..=N + 1 {
        let v1 = bit_find_kth(&bit, (K / 2) as i32) - 1;
        let v2 = bit_find_kth(&bit, (K / 2 + 1) as i32) - 1;
        f += (v1 + v2) as f64 / 2.0;

        // Remove old element
        let old_val = s2[next - K];
        bit_update(&mut bit, old_val as usize + 1, -1);

        // Add new element
        let new_val = s2[next];
        bit_update(&mut bit, new_val as usize + 1, 1);
    }

    println!("{:.1}", f);
}
