// Project Euler 842 - Irregular Star Polygons
// T(n) = sum of I(S) over all n-star polygons S.
// Odd n: every interior crossing has multiplicity 2.
// Even n: group diagonal crossings geometrically, then weight by multiplicity.
// Sum T(n) for n=3..60 mod 10^9+7.

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;

fn contrib(m: usize, n: usize, fact: &[u64; 62]) -> u64 {
    // sum_{k=2}^m (-1)^k (k-1) C(m,k) 2^{k-1} (n-k-1)!
    let mut c = 0u64;
    let mut binom = (m * (m - 1) / 2) as u64; // C(m, 2)
    let mut pow2 = 2u64; // 2^{k-1}
    for k in 2..=m {
        let term = (k as u64 - 1) * binom * pow2 % MOD * fact[n - k - 1] % MOD;
        if k & 1 == 0 {
            c += term;
        } else {
            c += MOD - term;
        }
        if k < m {
            binom = binom * (m - k) as u64 / (k as u64 + 1);
            pow2 <<= 1;
        }
    }
    c % MOD
}

fn t_even(n: usize, fact: &[u64; 62]) -> u64 {
    let mut xs = [0f64; 60];
    let mut ys = [0f64; 60];
    let theta = std::f64::consts::TAU / n as f64;
    for k in 0..n {
        let a = k as f64 * theta;
        xs[k] = a.cos();
        ys[k] = a.sin();
    }

    let cap = n * (n - 1) / 2 * (n - 2) / 3 * (n - 3) / 4;
    let mut pts = Vec::with_capacity(cap);
    let xs = &xs[..n];
    let ys = &ys[..n];

    for a in 0..n - 3 {
        let ax = unsafe { *xs.get_unchecked(a) };
        let ay = unsafe { *ys.get_unchecked(a) };
        for b in a + 1..n - 2 {
            let bx = unsafe { *xs.get_unchecked(b) };
            let by = unsafe { *ys.get_unchecked(b) };
            let axbx = ax - bx;
            let ayby = ay - by;
            for c in b + 1..n - 1 {
                let cx = unsafe { *xs.get_unchecked(c) };
                let cy = unsafe { *ys.get_unchecked(c) };
                let acx = ax - cx;
                let acy = ay - cy;
                for d in c + 1..n {
                    let dx = unsafe { *xs.get_unchecked(d) };
                    let dy = unsafe { *ys.get_unchecked(d) };
                    let bdx = bx - dx;
                    let bdy = by - dy;
                    let denom = acx * bdy - acy * bdx;
                    let t = (axbx * bdy - ayby * bdx) / denom;
                    let px = ax - t * acx;
                    let py = ay - t * acy;
                    let kx = (px * 1e9).round() as i32 as u32 as u64;
                    let ky = (py * 1e9).round() as i32 as u32 as u64;
                    pts.push((kx << 32) | ky);
                }
            }
        }
    }

    pts.sort_unstable();

    let mut mcount = [0u32; 32];
    let mut i = 0;
    let len = pts.len();
    while i < len {
        let key = unsafe { *pts.get_unchecked(i) };
        let mut j = i + 1;
        while j < len && unsafe { *pts.get_unchecked(j) } == key {
            j += 1;
        }
        let pairs = j - i;
        let m = (1 + (1 + 8 * pairs).isqrt()) / 2;
        mcount[m] += 1;
        i = j;
    }

    let mut total = 0u64;
    for m in 2..32 {
        let cnt = mcount[m] as u64;
        if cnt != 0 {
            total += contrib(m, n, fact) * cnt;
        }
    }
    total % MOD
}

fn main() {
    let mut fact = [0u64; 62];
    fact[0] = 1;
    for i in 1..=61 {
        fact[i] = fact[i - 1] * i as u64 % MOD;
    }

    let odd_sum = (5..61).step_by(2).map(|n| {
        let bn4 = n * (n - 1) / 2 * (n - 2) / 3 * (n - 3) / 4;
        bn4 as u64 * 2 % MOD * fact[n - 3] % MOD
    }).sum::<u64>() % MOD;

    let even_sum = (2..31)
        .into_par_iter()
        .map(|k| t_even(2 * k, &fact))
        .reduce(|| 0, |a, b| a + b)
        % MOD;

    println!("{}", (odd_sum + even_sum) % MOD);
}
