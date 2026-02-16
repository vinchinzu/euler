use euler_utils::{gcd, lcm};
use rayon::prelude::*;

const N: i64 = 10_000_000;

#[inline(always)]
fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn isqrt_func(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

#[inline(always)]
fn tr(n: i64) -> i64 {
    n * (n + 1) / 2
}

// Compute gcd(r2 * (x+y), den) where r2 and den fit in i64 but r2*(x+y) may not
// First reduce: gcd(a, b) = gcd(a % b, b) and compute (r2*(x+y)) % den using i128 for one mul
#[inline(always)]
fn gcd_special(r2: i64, xy_sum: i64, den: i64) -> i64 {
    // num = r2 * xy_sum, compute num % den
    let num_mod = ((r2 as i128 * xy_sum as i128) % den as i128) as i64;
    gcd_i64(num_mod, den)
}

fn main() {
    let l_val = (N as f64 / 108.0f64.sqrt()) as i64;
    let map_size = (l_val + 10) as usize;

    let mut tri_map: Vec<Vec<i64>> = vec![Vec::new(); map_size];

    let mut m = 2i64;
    while 2 * m * m <= N {
        let mut n = 1i64;
        while n < m && 2 * m * (m + n) <= N {
            if (m + n) % 2 == 1 && gcd(m as u64, n as u64) == 1 {
                let a = m * m - n * n;
                let b = 2 * m * n;
                if (a as usize) < map_size { tri_map[a as usize].push(b); }
                if (b as usize) < map_size { tri_map[b as usize].push(a); }
            }
            n += 1;
        }
        m += 1;
    }

    let mut keys: Vec<usize> = Vec::new();
    for i in 1..map_size {
        if !tri_map[i].is_empty() { keys.push(i); }
    }

    // Pre-compute divisors for each key
    let key_divisors: Vec<Vec<i64>> = keys.iter().map(|&a1| {
        let mut divs = Vec::new();
        let mut i = 1i64;
        let a = a1 as i64;
        while i * i <= a {
            if a % i == 0 {
                divs.push(i);
                if i != a / i { divs.push(a / i); }
            }
            i += 1;
        }
        divs
    }).collect();

    // Parallel: each key produces a list of candidate (x, y, z) triples
    let all_candidates: Vec<Vec<(i64, i64, i64)>> = keys.par_iter().zip(key_divisors.par_iter())
        .map(|(&a1, divs)| {
            let mut local_candidates = Vec::new();
            let a1_i64 = a1 as i64;

            for &d in divs {
                let mut mult = 1i64;
                while mult * a1_i64 < map_size as i64 && mult * d <= a1_i64 {
                    let a2 = (mult * d) as usize;
                    if a2 >= map_size || tri_map[a2].is_empty() { mult += 1; continue; }

                    let r = lcm(a1 as u64, a2 as u64) as i64;
                    if r > N { mult += 1; continue; }

                    let r2 = r * r; // fits in i64: r <= 10^7, r^2 <= 10^14

                    for bi in 0..tri_map[a1].len() {
                        let b1 = tri_map[a1][bi];
                        for bj in 0..tri_map[a2].len() {
                            let b2 = tri_map[a2][bj];

                            let x = b1 * r / a1_i64;
                            let y = b2 * r / a2 as i64;
                            let den = x * y - r2; // fits in i64

                            if den <= 0 { continue; }

                            // Quick check: num/den = r2*(x+y)/den, need 2*(x+y+num/den) <= N
                            // num/den computed via i128 for one division
                            let xy_sum = x + y;
                            let z_approx = (r2 as i128 * xy_sum as i128 / den as i128) as i64;
                            if 2 * (xy_sum + z_approx) > N { continue; }

                            // Compute gcd using optimized approach (one i128 mul then i64 gcd)
                            let g = gcd_special(r2, xy_sum, den);
                            let num_r = (r2 as i128 * xy_sum as i128 / g as i128) as i64;
                            let den_r = den / g;
                            if 2 * (x as i128 * den_r as i128 + y as i128 * den_r as i128 + num_r as i128) <= N as i128 {
                                let mut sx = (x as i128 * den_r as i128) as i64;
                                let mut sy = (y as i128 * den_r as i128) as i64;
                                let mut sz = num_r;
                                // Sort
                                if sx > sy { std::mem::swap(&mut sx, &mut sy); }
                                if sy > sz { std::mem::swap(&mut sy, &mut sz); }
                                if sx > sy { std::mem::swap(&mut sx, &mut sy); }
                                local_candidates.push((sx, sy, sz));
                            }
                        }
                    }
                    mult += 1;
                }
            }
            local_candidates
        })
        .collect();

    // Merge and deduplicate using hash set
    let mut sol_keys: Vec<(i64, i64, i64)> = vec![(0, 0, 0); 2_000_003];
    let mut sol_used: Vec<bool> = vec![false; 2_000_003];
    let mut solutions: Vec<(i64, i64, i64)> = Vec::new();
    const HASH_SIZE: usize = 2_000_003;

    for candidates in &all_candidates {
        for &(x, y, z) in candidates {
            let h = ((x as u64).wrapping_mul(1_000_000_007).wrapping_add(y as u64)).wrapping_mul(1_000_000_007).wrapping_add(z as u64);
            let mut idx = (h % HASH_SIZE as u64) as usize;
            loop {
                if !sol_used[idx] {
                    sol_keys[idx] = (x, y, z);
                    sol_used[idx] = true;
                    solutions.push((x, y, z));
                    break;
                }
                if sol_keys[idx] == (x, y, z) {
                    break;
                }
                idx = (idx + 1) % HASH_SIZE;
            }
        }
    }

    let mut ans: i64 = 0;
    for &(x, y, z) in &solutions {
        let r2 = (x * y * z) / (x + y + z);
        let perim = 2 * (x + y + z);
        let ia = isqrt_func(r2 + x * x);
        let ib = isqrt_func(r2 + y * y);
        let ic = isqrt_func(r2 + z * z);
        ans += tr(N / perim) * (perim + ia + ib + ic);
    }

    println!("{}", ans);
}
