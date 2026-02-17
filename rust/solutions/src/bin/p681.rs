// Project Euler 681 - Maximal Area
// Enumerate (w,x,y,z) with wxyz = K^2, w<=x<=y<=z, z < w+x+y, even perimeter.
// Parallelized Case 2 with rayon.

use rayon::prelude::*;

fn main() {
    let n: i64 = 1_000_000;
    let n2 = n * n;

    // SPF sieve
    let maxn = (n as usize) + 1;
    let mut spf = vec![0u32; maxn];
    for i in 0..maxn { spf[i] = i as u32; }
    let mut i = 2;
    while i * i < maxn {
        if spf[i] == i as u32 {
            let mut j = i * i;
            while j < maxn {
                if spf[j] == j as u32 { spf[j] = i as u32; }
                j += i;
            }
        }
        i += 1;
    }

    // Case 1: z == y, wx must be perfect square
    let mut ans: i64 = 0;
    let mut w = 1i64;
    while w * w * w * w <= n2 {
        let mut x = w;
        while w * x * x * x <= n2 {
            let wx = w * x;
            let swx = (wx as f64).sqrt() as i64;
            let sq = if swx * swx == wx { swx }
                     else if (swx + 1) * (swx + 1) == wx { swx + 1 }
                     else { x += 1; continue };
            if (w + x) & 1 != 0 { x += 1; continue; }
            let y_max = n / sq;
            if y_max < x { x += 1; continue; }
            let count = y_max - x + 1;
            ans += count * (w + x + x + y_max);
            x += 1;
        }
        w += 1;
    }

    // Case 2: z > y, parallelized with rayon
    let ans_dpos: i64 = (4..=n as usize).into_par_iter().map(|k| {
        if spf[k] == k as u32 { return 0i64; } // skip primes

        // Factor K using stack arrays
        let mut prms = [0i64; 20];
        let mut exps = [0usize; 20];
        let mut np = 0;
        let mut tmp = k;
        while tmp > 1 {
            let p = spf[tmp] as usize;
            let mut e = 0;
            while tmp % p == 0 { tmp /= p; e += 1; }
            prms[np] = p as i64;
            exps[np] = e * 2; // exponents of K^2
            np += 1;
        }

        let k2 = (k as i64) * (k as i64);

        // Build divisors in stack array
        let mut divs = [0i64; 4096];
        divs[0] = 1;
        let mut nd = 1usize;
        for i in 0..np {
            let old = nd;
            let mut pp = 1i64;
            for _ in 0..exps[i] {
                pp *= prms[i];
                for j in 0..old {
                    divs[nd] = divs[j] * pp;
                    nd += 1;
                }
            }
        }
        // Sort divisors
        let ds = &mut divs[..nd];
        ds.sort_unstable();

        let mut local_sum = 0i64;
        for i in 0..nd {
            let w = ds[i];
            if w * w * w * w > k2 { break; }
            let r1 = k2 / w;
            for j in i..nd {
                let x = ds[j];
                if x * x * x > r1 { break; }
                if r1 % x != 0 { continue; }
                let r2 = r1 / x;
                for l in j..nd {
                    let y = ds[l];
                    if y * y > r2 { break; }
                    if r2 % y != 0 { continue; }
                    let z = r2 / y;
                    if z <= y { continue; }
                    if z >= w + x + y { continue; }
                    let total = w + x + y + z;
                    if total & 1 != 0 { continue; }
                    local_sum += total;
                }
            }
        }
        local_sum
    }).sum();

    println!("{}", ans + ans_dpos);
}
