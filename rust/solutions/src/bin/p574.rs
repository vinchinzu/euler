// Project Euler 574 - Verifying Primes
//
// V(p) = smallest A in triplet (A,B,q) such that A>=B>0, gcd(A,B)=1,
// AB divisible by every prime < q, p < q^2, and p=A+B or p=A-B.
// Sum V(p) for all primes p < 3800.
//
// Key optimizations:
// 1. Use u128 for product to avoid overflow (product of 18 primes > i64)
// 2. Handle edge case: p=2,3 have no q_primes
// 3. Replace ext_gcd with precomputed modular inverses using CRT
// 4. Collect all residues, sort, find best

fn sieve_primes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 { is_prime[1] = false; }
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit { is_prime[j] = false; j += i; }
        }
        i += 1;
    }
    (2..=limit).filter(|&i| is_prime[i]).collect()
}

/// Modular inverse of a mod m using extended GCD (small values, u64 sufficient)
fn mod_inv_small(a: u64, m: u64) -> u64 {
    let (mut old_r, mut r) = (a as i64, m as i64);
    let (mut old_s, mut s) = (1i64, 0i64);
    while r != 0 {
        let q = old_r / r;
        let tmp = r; r = old_r - q * r; old_r = tmp;
        let tmp = s; s = old_s - q * s; old_s = tmp;
    }
    ((old_s % m as i64 + m as i64) % m as i64) as u64
}

fn v(p: usize, all_primes: &[usize]) -> u128 {
    let sq = (p as f64).sqrt() as usize;
    let q_primes: Vec<usize> = all_primes.iter()
        .copied()
        .take_while(|&x| x <= sq)
        .collect();
    let nq = q_primes.len();
    if nq == 0 {
        return ((p + 1) / 2) as u128;
    }

    let product: u128 = q_primes.iter().map(|&x| x as u128).product();
    let p128 = p as u128;

    // Case 1: A + B = p
    if product <= p128 * p128 / 4 {
        for a in ((p + 1) / 2)..(p as usize) {
            let a128 = a as u128;
            let b128 = p128 - a128;
            if (a128 * b128) % product == 0 {
                return a128;
            }
        }
    }

    // Case 2: A - B = p
    // For each subset: c0 = product of primes in subset (divides A),
    //                  c1 = product of remaining (divides B).
    // We need A ≡ 0 (mod c0) and A ≡ p (mod c1).
    // CRT gives unique residue mod product.
    //
    // Compute residue using CRT over small primes:
    // For each prime q_j in c1: A ≡ p (mod q_j)
    // For each prime q_j in c0: A ≡ 0 (mod q_j)
    //
    // Using Garner's algorithm / direct CRT:
    // residue = sum over j in c1_primes: p * (product/q_j) * inv(product/q_j, q_j) (mod product)
    // But this is constant for A ≡ p (mod c1) and A ≡ 0 (mod c0).

    // Precompute for CRT: for each prime q_i, compute
    //   M_i = product / q_i
    //   M_i_inv = M_i^{-1} mod q_i
    // Then CRT: x = sum_i (r_i * M_i * M_i_inv) mod product
    // where r_i = p mod q_i for primes in c1, r_i = 0 for primes in c0.

    let mut m_vals: Vec<u128> = Vec::with_capacity(nq);
    let mut m_inv_vals: Vec<u64> = Vec::with_capacity(nq);
    let mut p_mod_qi: Vec<u64> = Vec::with_capacity(nq);

    for i in 0..nq {
        let qi = q_primes[i] as u128;
        let mi = product / qi;
        let mi_mod_qi = (mi % qi) as u64;
        let inv = mod_inv_small(mi_mod_qi, q_primes[i] as u64);
        m_vals.push(mi);
        m_inv_vals.push(inv);
        p_mod_qi.push((p as u64) % (q_primes[i] as u64));
    }

    // For each subset, compute residue:
    // residue = sum over i where bit i is 0 (i.e., q_i in c1):
    //           (p mod q_i) * M_i * (M_i^{-1} mod q_i) mod product
    // Bits set to 1 contribute 0 (since A ≡ 0 mod those primes).

    // Precompute the CRT contribution of each prime (when it's in c1):
    //   contrib_i = (p mod q_i) * M_i * (M_i_inv mod q_i) mod product
    let mut contribs: Vec<u128> = Vec::with_capacity(nq);
    for i in 0..nq {
        let c = (p_mod_qi[i] as u128) * (m_inv_vals[i] as u128) % (q_primes[i] as u128);
        let contrib = c * m_vals[i] % product;
        contribs.push(contrib);
    }

    let num_subsets = 1u32 << nq;
    let p_mod = p128 % product;
    let mut best: u128 = u128::MAX;

    // Helper closure: compute candidate A from residue and update best
    let mut update_best = |r: u128| {
        let a = if r > p_mod {
            p128 - p_mod + r
        } else {
            p128 - p_mod + r + product
        };
        if a < best {
            if a % p128 != 0 {
                best = a;
            } else {
                let a2 = a + product;
                if a2 < best && a2 % p128 != 0 {
                    best = a2;
                }
            }
        }
    };

    // Use Gray code iteration: each step flips one bit, so we add or subtract
    // one contribution instead of recomputing from scratch.
    // Start with subset=0 (all bits 0, all primes in c1): sum of all contribs
    let total_contrib: u128 = contribs.iter().copied().sum::<u128>() % product;
    let mut residue = total_contrib;
    update_best(residue);

    for gray in 1..num_subsets {
        // Find the bit that changed (trailing zero of gray)
        let bit = gray.trailing_zeros() as usize;
        // In Gray code, gray ^ (gray >> 1) gives the subset
        let gray_subset = gray ^ (gray >> 1);
        if (gray_subset >> bit) & 1 == 1 {
            // Bit became 1: prime moved from c1 to c0, subtract its contribution
            if residue >= contribs[bit] {
                residue -= contribs[bit];
            } else {
                residue = residue + product - contribs[bit];
            }
        } else {
            // Bit became 0: prime moved from c0 to c1, add its contribution
            residue += contribs[bit];
            if residue >= product { residue -= product; }
        }
        update_best(residue);
    }

    if best < u128::MAX { best } else { 0 }
}

fn main() {
    let all_primes = sieve_primes(3800);

    let mut ans: u128 = 0;
    for &p in &all_primes {
        if p < 3800 {
            ans += v(p, &all_primes);
        }
    }

    println!("{}", ans);
}
