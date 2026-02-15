// Project Euler 408: Admissible paths through a grid
// Count paths from (0,0) to (N,N) that avoid inadmissible points.
// Inadmissible points are (a^2, b^2) where a^2+b^2 is a perfect square.

use euler_utils::mod_pow;

const N: usize = 10_000_000;
const MOD: u64 = 1_000_000_007;
const MAX_FACT: usize = 2 * N + 1;

fn power(base: u64, exp: u64) -> u64 {
    mod_pow(base % MOD, exp, MOD)
}

fn main() {
    // Precompute factorials
    let mut fact = vec![1u64; MAX_FACT];
    for i in 1..MAX_FACT {
        fact[i] = fact[i - 1] * i as u64 % MOD;
    }
    let mut inv_fact = vec![1u64; MAX_FACT];
    inv_fact[MAX_FACT - 1] = power(fact[MAX_FACT - 1], MOD - 2);
    for i in (0..MAX_FACT - 1).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i + 1) as u64 % MOD;
    }

    let ncr = |n: usize, r: usize| -> u64 {
        if r > n { return 0; }
        fact[n] % MOD * inv_fact[r] % MOD * inv_fact[n - r] % MOD
    };

    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 { let t = b; b = a % b; a = t; }
        a
    }

    // Generate inadmissible points via Pythagorean triples
    let sq_limit = (N as f64).sqrt() as usize;
    let m_limit = (4.0 * sq_limit as f64).sqrt() as usize + 1;

    let mut pts: Vec<(usize, usize)> = Vec::new();

    for m in 2..=m_limit {
        for n in 1..m {
            if (m + n) % 2 == 1 && gcd(m, n) == 1 {
                let a = m * m - n * n;
                let b = 2 * m * n;
                let c = m * m + n * n;
                let mut k = 1;
                while k * c <= 4 * sq_limit {
                    let ax = (k * a) * (k * a);
                    let bx = (k * b) * (k * b);
                    if ax <= N && bx <= N {
                        pts.push((ax, bx));
                    }
                    if bx <= N && ax <= N && ax != bx {
                        pts.push((bx, ax));
                    }
                    k += 1;
                }
            }
        }
    }

    pts.sort();
    pts.dedup();

    // Add destination
    pts.push((N, N));
    pts.sort();

    let npts = pts.len();
    let mut adm = vec![0u64; npts];

    for pi in 0..npts {
        let mut total = ncr(pts[pi].0 + pts[pi].1, pts[pi].0);
        for qi in 0..pi {
            if pts[qi].0 <= pts[pi].0 && pts[qi].1 <= pts[pi].1 {
                let dx = pts[pi].0 - pts[qi].0;
                let dy = pts[pi].1 - pts[qi].1;
                total = (total + MOD - adm[qi] % MOD * ncr(dx + dy, dx) % MOD) % MOD;
            }
        }
        adm[pi] = total % MOD;
    }

    println!("{}", adm[npts - 1]);
}
