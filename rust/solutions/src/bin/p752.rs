// Project Euler 752 - Powers of 1+sqrt(7)
// Find order of matrix [[1,7],[1,1]] mod p for each prime p, then sum g(x) for x=2..N.

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

    // 2x2 matrix mod m using i128
    type Mat = [i64; 4]; // [a, b, c, d]

    fn mat_mul(a: &Mat, b: &Mat, m: i64) -> Mat {
        [
            ((a[0] as i128 * b[0] as i128 + a[1] as i128 * b[2] as i128) % m as i128) as i64,
            ((a[0] as i128 * b[1] as i128 + a[1] as i128 * b[3] as i128) % m as i128) as i64,
            ((a[2] as i128 * b[0] as i128 + a[3] as i128 * b[2] as i128) % m as i128) as i64,
            ((a[2] as i128 * b[1] as i128 + a[3] as i128 * b[3] as i128) % m as i128) as i64,
        ]
    }

    fn mat_pow(a: &Mat, mut e: i64, m: i64) -> Mat {
        let mut result: Mat = [1, 0, 0, 1];
        let mut base = [a[0] % m, a[1] % m, a[2] % m, a[3] % m];
        while e > 0 {
            if e & 1 == 1 {
                result = mat_mul(&result, &base, m);
            }
            base = mat_mul(&base, &base, m);
            e >>= 1;
        }
        result
    }

    fn is_identity(m: &Mat, modulus: i64) -> bool {
        m[0] % modulus == 1 && m[1] % modulus == 0 && m[2] % modulus == 0 && m[3] % modulus == 1
    }

    fn mat_order(p: i64) -> i64 {
        let a: Mat = [1, 7, 1, 1];
        let n = (p - 1) * (p + 1);

        // Factorize n
        let mut temp = n;
        let mut primes = Vec::new();
        let mut d = 2i64;
        while d * d <= temp {
            if temp % d == 0 {
                primes.push(d);
                while temp % d == 0 {
                    temp /= d;
                }
            }
            d += 1;
        }
        if temp > 1 {
            primes.push(temp);
        }

        let mut order = n;
        for &pr in &primes {
            while order % pr == 0 {
                let trial = order / pr;
                let m = mat_pow(&a, trial, p);
                if is_identity(&m, p) {
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
        if a == 0 || b == 0 { return 0; }
        a / gcd(a, b) * b
    }

    let n = 1_000_000usize;
    let mut g_val = vec![0i64; MAXN];

    for p in 2..=n {
        if spf[p] != p as u32 { continue; }
        if p == 2 || p == 3 {
            g_val[p] = 0;
        } else if p == 7 {
            g_val[p] = 7;
        } else {
            g_val[p] = mat_order(p as i64);
        }
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
