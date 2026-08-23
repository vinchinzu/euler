// Project Euler 752 - Powers of 1+sqrt(7)
// Find order of matrix [[1,7],[1,1]] mod p for each prime p, then sum g(x) for x=2..N.

use rayon::prelude::*;

fn main() {
    const MAXN: usize = 1_000_001;
    let mut spf = vec![0u32; MAXN];
    for i in 0..MAXN {
        spf[i] = i as u32;
    }
    for i in 2..MAXN {
        if spf[i] == i as u32 {
            let mut j = i * i;
            while j < MAXN {
                if spf[j] == j as u32 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
    }

    // 2x2 matrix mod m. Entries stay in [0, m); products fit in i64 for m <= 1e6.
    type Mat = [i64; 4]; // [a, b, c, d]

    #[inline(always)]
    fn mat_mul(a: &Mat, b: &Mat, m: i64) -> Mat {
        [
            (a[0] * b[0] + a[1] * b[2]) % m,
            (a[0] * b[1] + a[1] * b[3]) % m,
            (a[2] * b[0] + a[3] * b[2]) % m,
            (a[2] * b[1] + a[3] * b[3]) % m,
        ]
    }

    fn mat_pow(mut e: i64, m: i64) -> Mat {
        let mut result: Mat = [1, 0, 0, 1];
        let mut base: Mat = [1, 7, 1, 1];
        while e > 0 {
            if e & 1 == 1 {
                result = mat_mul(&result, &base, m);
            }
            base = mat_mul(&base, &base, m);
            e >>= 1;
        }
        result
    }

    #[inline(always)]
    fn is_identity(m: &Mat) -> bool {
        m[0] == 1 && m[1] == 0 && m[2] == 0 && m[3] == 1
    }

    // Distinct prime factors of (p-1)(p+1) via SPF of p-1 and p+1.
    fn factor_p2m1(p: i64, spf: &[u32], out: &mut [i64; 16]) -> usize {
        let mut np = 0usize;
        let mut x = (p - 1) as usize;
        while x > 1 {
            let pr = spf[x] as i64;
            out[np] = pr;
            np += 1;
            let pu = pr as usize;
            while x % pu == 0 {
                x /= pu;
            }
        }
        // gcd(p-1, p+1) = 2 for odd p; 2 is already recorded.
        let mut x = (p + 1) as usize;
        while x & 1 == 0 {
            x >>= 1;
        }
        while x > 1 {
            let pr = spf[x] as i64;
            out[np] = pr;
            np += 1;
            let pu = pr as usize;
            while x % pu == 0 {
                x /= pu;
            }
        }
        np
    }

    fn mat_order(p: i64, spf: &[u32]) -> i64 {
        let n = (p - 1) * (p + 1);
        let mut primes = [0i64; 16];
        let np = factor_p2m1(p, spf, &mut primes);

        let mut order = n;
        for i in 0..np {
            let pr = primes[i];
            while order % pr == 0 {
                let trial = order / pr;
                if is_identity(&mat_pow(trial, p)) {
                    order = trial;
                } else {
                    break;
                }
            }
        }
        order
    }

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    fn lcm(a: i64, b: i64) -> i64 {
        if a == 0 || b == 0 {
            return 0;
        }
        a / gcd(a, b) * b
    }

    let n = 1_000_000usize;
    let mut prime_list = Vec::with_capacity(80_000);
    for p in 2..=n {
        if spf[p] == p as u32 {
            prime_list.push(p);
        }
    }

    let orders: Vec<i64> = prime_list
        .par_iter()
        .map(|&p| {
            if p == 2 || p == 3 {
                0
            } else if p == 7 {
                7
            } else {
                mat_order(p as i64, &spf)
            }
        })
        .collect();

    let mut g_val = vec![0i64; MAXN];
    for (&p, &g) in prime_list.iter().zip(orders.iter()) {
        g_val[p] = g;
    }

    let mut ans: i64 = 0;
    for x in 2..=n {
        if spf[x] == x as u32 {
            ans += g_val[x];
            continue;
        }

        let mut temp = x;
        let mut gx: i64 = 1;
        let mut is_zero = false;
        while temp > 1 {
            let p = spf[temp] as usize;
            let mut e = 0;
            while temp % p == 0 {
                temp /= p;
                e += 1;
            }
            let mut gpe = g_val[p];
            if gpe == 0 {
                is_zero = true;
                break;
            }
            for _ in 1..e {
                gpe *= p as i64;
            }
            gx = lcm(gx, gpe);
        }
        if is_zero {
            gx = 0;
        }
        g_val[x] = gx;
        ans += gx;
    }

    println!("{}", ans);
}
