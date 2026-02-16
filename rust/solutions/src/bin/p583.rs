// Project Euler 583 - Heron Envelopes
//
// A Heron envelope is a pentagon ABCDE = rectangle ABDE + isosceles triangle BCD,
// with all sides and diagonals integral, and flap height < rectangle height.
//
// Setting up coordinates with half-width b, rectangle height h, flap height f:
//   Condition 1: b^2 + f^2 = s^2  (triangle side BC = CD = s)
//   Condition 2: (2b)^2 + h^2 = d^2  (rectangle diagonal)
//   Condition 3: b^2 + (h+f)^2 = t^2  (pentagon diagonal AC = CE)
//   f < h, perimeter = 2b + 2h + 2s <= N
//
// Scaling conditions 1,3 by 2 gives common leg w=2b:
//   w^2 + (2f)^2 = (2s)^2
//   w^2 + h^2 = d^2
//   w^2 + (2h+2f)^2 = (2t)^2
//
// For each even w, collect Pythagorean other_legs. Then find (A_leg=2f, C_leg=h)
// such that A_leg + 2*C_leg is also an other_leg, A's hypotenuse is even (so 2f is
// even and 2s is even), f < h (i.e., A_leg/2 < C_leg).
// Perimeter = A_hyp + w + 2*C_leg.

use euler_utils::gcd_i64;
use rayon::prelude::*;
use std::collections::HashSet;

const LIMIT: i64 = 10_000_000;

fn main() {
    let n = LIMIT as usize;

    // G[w] stores (other_leg, hypotenuse) for Pythagorean triples with even leg w.
    // We only need hypotenuse <= N since peri > hyp.
    // Use a flat storage with index arrays for cache-friendliness.

    // First pass: count entries per even w
    let mut count: Vec<u32> = vec![0; n + 1];

    let m_limit = {
        let mut m = 1i64;
        while m * m < n as i64 {
            m += 1;
        }
        m
    };

    for p in 1..=m_limit {
        for q in 1..p {
            if (p + q) % 2 == 0 || gcd_i64(p, q) != 1 {
                continue;
            }
            let a0 = p * p - q * q;
            let b0 = 2 * p * q;
            let c0 = p * p + q * q;
            if c0 > LIMIT {
                break;
            }

            let mut t = 1i64;
            while t * c0 <= LIMIT {
                let a = (a0 * t) as usize;
                let b = (b0 * t) as usize;

                // b = 2*p*q*t is always even
                if b <= n {
                    count[b] += 1;
                }
                // a = (p^2 - q^2)*t; with gcd=1 and p+q odd, exactly one of p,q is even,
                // so p^2 - q^2 is odd, meaning a*t is even only when t is even
                if a % 2 == 0 && a <= n {
                    count[a] += 1;
                }

                t += 1;
            }
        }
    }

    // Build offset array for flat storage
    let mut offsets: Vec<u32> = vec![0; n + 2];
    for i in 0..=n {
        offsets[i + 1] = offsets[i] + count[i];
    }
    let total = offsets[n + 1] as usize;
    let mut legs = vec![0i32; total];
    let mut hyps = vec![0i32; total];
    let mut pos = offsets.clone(); // current write position for each w

    // Second pass: fill entries
    for p in 1..=m_limit {
        for q in 1..p {
            if (p + q) % 2 == 0 || gcd_i64(p, q) != 1 {
                continue;
            }
            let a0 = p * p - q * q;
            let b0 = 2 * p * q;
            let c0 = p * p + q * q;
            if c0 > LIMIT {
                break;
            }

            let mut t = 1i64;
            while t * c0 <= LIMIT {
                let a = (a0 * t) as i32;
                let b = (b0 * t) as i32;
                let c = (c0 * t) as i32;

                if (b as usize) <= n {
                    let idx = pos[b as usize] as usize;
                    legs[idx] = a;
                    hyps[idx] = c;
                    pos[b as usize] += 1;
                }
                if a % 2 == 0 && (a as usize) <= n {
                    let idx = pos[a as usize] as usize;
                    legs[idx] = b;
                    hyps[idx] = c;
                    pos[a as usize] += 1;
                }

                t += 1;
            }
        }
    }

    // Collect even w values that have triples
    let even_ws: Vec<usize> = (2..=n)
        .step_by(2)
        .filter(|&w| offsets[w] < offsets[w + 1])
        .collect();

    // Process each even w in parallel using HashSet for membership testing.
    // HashSet is better than a 40MB mark array because:
    // - Most w values have few triples (<10), so the HashSet fits in L1 cache
    // - The mark array has random access patterns causing cache misses
    let ans: i64 = even_ws
        .par_iter()
        .map(|&w| {
            let start = offsets[w] as usize;
            let end = offsets[w + 1] as usize;

            // Build a HashSet of legs for O(1) membership testing
            let mut leg_set = HashSet::with_capacity(end - start);
            for i in start..end {
                leg_set.insert(legs[i]);
            }

            let mut local_ans: i64 = 0;

            // For each A with even hypotenuse:
            for i in start..end {
                let a_leg = legs[i];
                let a_hyp = hyps[i];
                if a_hyp % 2 != 0 {
                    continue;
                }
                // a_leg must be even (since w even and a_hyp even -> a_leg even)
                let f = a_leg / 2;

                // For each C = (h, d) where h > f
                for j in start..end {
                    let h = legs[j];
                    if h <= f {
                        continue;
                    }

                    let peri = a_hyp as i64 + w as i64 + 2 * h as i64;
                    if peri > LIMIT {
                        continue;
                    }

                    // Check if a_leg + 2*h is also an other_leg
                    let target = a_leg + 2 * h;
                    if target > 0 && leg_set.contains(&target) {
                        local_ans += peri;
                    }
                }
            }
            local_ans
        })
        .sum();

    println!("{}", ans);
}
