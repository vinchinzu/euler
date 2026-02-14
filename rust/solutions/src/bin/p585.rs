// Project Euler 585 - Nested Square Roots
//
// Count representations of integers as nested square roots.

const N: usize = 5_000_000;

fn main() {
    // Sieve for smallest prime factor
    let mut ff = vec![0u32; N + 1];
    for i in 2..=N {
        if ff[i] == 0 {
            ff[i] = i as u32;
            let mut j = i * i;
            while j <= N {
                if ff[j] == 0 { ff[j] = i as u32; }
                j += i;
            }
        }
    }

    // Compute Euler phi
    let mut phi = vec![0i64; N + 1];
    phi[1] = 1;
    for i in 2..=N {
        let p = ff[i] as i64;
        let mut temp = i as i64;
        let mut pe: i64 = 1;
        while temp % p == 0 {
            temp /= p;
            pe *= p;
        }
        if temp == 1 {
            phi[i] = pe - pe / p;
        } else {
            phi[i] = phi[pe as usize] * phi[temp as usize];
        }
    }

    // Compute square-free flags
    let mut is_square_free = vec![true; N + 1];
    let mut i = 2usize;
    while i * i <= N {
        let sq = i * i;
        let mut j = sq;
        while j <= N {
            is_square_free[j] = false;
            j += sq;
        }
        i += 1;
    }

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 { let t = b; b = a % b; a = t; }
        a
    }

    fn is_sq(n: i64) -> bool {
        if n < 0 { return false; }
        let r = (n as f64).sqrt() as i64;
        (r * r == n) || ((r + 1) * (r + 1) == n)
    }

    let mut f = vec![0i64; N + 1];
    let mut fp = vec![0i64; N + 1];

    for n in 1..=N {
        f[n] = (n as i64 - 1) / 2;
        fp[n] = phi[n] / 2;
    }

    // Subtract cases where a*b is a perfect square
    for k in 1..=N {
        if !is_square_free[k] { continue; }
        let mut s: i64 = 1;
        while (s * s) * (k as i64) <= N as i64 {
            let mut t = s + 1;
            while (s * s + t * t) * (k as i64) <= N as i64 {
                let sum = ((s * s + t * t) * (k as i64)) as usize;
                f[sum] -= 1;
                if k == 1 && gcd(s, t) == 1 {
                    fp[sum] -= 1;
                }
                t += 1;
            }
            s += 1;
        }
    }

    // First case: sum of f[n]
    let mut ans: i64 = 0;
    for n in 1..=N {
        ans += f[n];
    }

    // Second case
    let mut res: i64 = 0;
    for g_plus_h in 1..=N {
        let mut ap_plus_bp = 1;
        while (g_plus_h as i64) * (ap_plus_bp as i64) <= N as i64 {
            res += f[g_plus_h] * fp[ap_plus_bp];
            ap_plus_bp += 1;
        }
    }

    // Compute sizes for each index
    let mut sizes = vec![0i64; N + 1];
    sizes[1] = 1;
    for i in 2..=N {
        let p = ff[i] as i64;
        let mut temp = i as i64;
        let mut e = 0i64;
        while temp % p == 0 {
            temp /= p;
            e += 1;
        }
        sizes[i] = sizes[temp as usize] * (2 * e + 1);
    }

    let mut start_indices = vec![0usize; N + 2];
    for i in 1..=N {
        start_indices[i + 1] = start_indices[i] + ((sizes[i] + 1) / 2) as usize;
    }

    let total_size = start_indices[N + 1];
    let mut curr_indices = start_indices.clone();

    let mut smalls = vec![0i64; total_size];
    let mut bigs = vec![0i64; total_size];

    for k in 1..=N {
        if !is_square_free[k] { continue; }
        let mut s: i64 = 1;
        while s * s * (k as i64) <= N as i64 {
            let mut t = s;
            while (s * s + t * t) * (k as i64) <= N as i64 {
                let idx = (s * t * (k as i64)) as usize;
                if idx <= N && curr_indices[idx] < start_indices[idx + 1] {
                    smalls[curr_indices[idx]] = s * s * (k as i64);
                    bigs[curr_indices[idx]] = t * t * (k as i64);
                    curr_indices[idx] += 1;
                }
                t += 1;
            }
            s += 1;
        }
    }

    for i in 0..=N {
        for ad in start_indices[i]..curr_indices[i] {
            for bc in start_indices[i]..curr_indices[i] {
                let a = smalls[ad];
                let b = smalls[bc];
                let c = bigs[bc];
                let d = bigs[ad];
                if a < b && a + b + c + d <= N as i64 && !is_sq(a * b) {
                    if b == c { res -= 1; } else { res -= 2; }
                }
            }
        }
    }

    ans += res / 2;

    println!("{}", ans);
}
