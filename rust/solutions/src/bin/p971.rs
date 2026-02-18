// Project Euler 971 - Periodic points in mu5 for primes
// For primes p ≡ 1 (mod 5), compute periodic points of a map on the 5th roots of unity mod p,
// then sum C_p = 1 + n * t where n = (p-1)/5 and t = count of periodic points in mu5.

fn sieve_primes_upto(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let half = n / 2 + 1;
    let mut sieve = vec![false; half];
    let limit = ((n as f64).sqrt() as usize) + 1;
    for i in 1..=(limit / 2) {
        if !sieve[i] {
            let p = 2 * i + 1;
            let start = (p * p - 1) / 2;
            let mut j = start;
            while j < half {
                sieve[j] = true;
                j += p;
            }
        }
    }
    let mut primes = vec![2usize];
    for i in 1..half {
        if !sieve[i] {
            let val = 2 * i + 1;
            if val <= n {
                primes.push(val);
            }
        }
    }
    primes
}

#[inline]
fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

fn count_periodic_in_mu5(p: u64) -> u64 {
    let n = (p - 1) / 5;

    // Find a primitive 5th root of unity mod p
    let mut a = 2u64;
    let mut zeta = pow_mod(a, n, p);
    while zeta == 1 {
        a += 1;
        zeta = pow_mod(a, n, p);
    }

    // Build mu5 = {1, zeta, zeta^2, zeta^3, zeta^4}
    let mut mu5 = [0u64; 5];
    mu5[0] = 1;
    let mut cur = 1u64;
    for i in 1..5 {
        cur = (cur as u128 * zeta as u128 % p as u128) as u64;
        mu5[i] = cur;
    }

    // phi(s) = s * (1+s)^n mod p
    let mut phi = [0u64; 5];
    for i in 0..5 {
        let s = mu5[i];
        let one_plus = (1 + s) % p;
        let pow_val = pow_mod(one_plus, n, p);
        phi[i] = (s as u128 * pow_val as u128 % p as u128) as u64;
    }

    // Map phi values back to indices in mu5
    // phi_idx[i] = index j such that mu5[j] == phi[i], or 255 if not in mu5
    let mut phi_idx = [255u8; 5];
    for i in 0..5 {
        for j in 0..5 {
            if mu5[j] == phi[i] {
                phi_idx[i] = j as u8;
                break;
            }
        }
    }

    // Find periodic points (nodes in cycles)
    let mut in_cycle = 0u64;
    let mut visited = [0u8; 5]; // 0=unvisited, 1=in-progress, 2=done

    for start in 0..5u8 {
        if visited[start as usize] != 0 {
            continue;
        }
        let mut path = Vec::new();
        let mut cur_idx = start;
        loop {
            if visited[cur_idx as usize] != 0 {
                if visited[cur_idx as usize] == 1 {
                    // Found a cycle - mark nodes from cur_idx onwards in path
                    let pos = path.iter().position(|&x| x == cur_idx).unwrap();
                    in_cycle += (path.len() - pos) as u64;
                }
                for &node in &path {
                    visited[node as usize] = 2;
                }
                break;
            }
            let next = phi_idx[cur_idx as usize];
            if next == 255 {
                // phi maps outside mu5
                for &node in &path {
                    visited[node as usize] = 2;
                }
                break;
            }
            visited[cur_idx as usize] = 1;
            path.push(cur_idx);
            cur_idx = next;
        }
    }

    in_cycle
}

fn compute_s(limit: usize) -> u64 {
    let primes = sieve_primes_upto(limit);
    let mut total = 0u64;
    for &p in &primes {
        if p < 5 {
            continue;
        }
        if p % 5 != 1 {
            continue;
        }
        let p = p as u64;
        let n = (p - 1) / 5;
        let t = count_periodic_in_mu5(p);
        let c_p = 1 + n * t;
        total += c_p;
    }
    total
}

fn main() {
    println!("{}", compute_s(100_000_000));
}
