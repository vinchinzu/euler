// Project Euler 373 - Circumscribed circles
// For each radius r, find triangles inscribed in circle with integer sides.
// Uses SPF sieve and exponent signature caching.

const N: usize = 10_000_000;
const MAX_SIG_LEN: usize = 8;

fn main() {
    // SPF sieve
    let mut spf = vec![0u32; N + 1];
    for i in 2..=N {
        if spf[i] == 0 {
            spf[i] = i as u32;
            if (i as u64) * (i as u64) <= N as u64 {
                let mut j = i * i;
                while j <= N {
                    if spf[j] == 0 { spf[j] = i as u32; }
                    j += i;
                }
            }
        }
    }

    // Signature type: sorted exponents of primes ≡ 1 mod 4
    type Sig = [u8; MAX_SIG_LEN];

    fn get_signature(mut n: usize, spf: &[u32]) -> (Sig, u8) {
        let mut sig = [0u8; MAX_SIG_LEN];
        let mut len = 0u8;
        while n > 1 {
            let p = spf[n] as usize;
            let mut e = 0;
            while n > 1 && spf[n] as usize == p { n /= p; e += 1; }
            if p % 4 == 1 {
                sig[len as usize] = e as u8;
                len += 1;
            }
        }
        // insertion sort
        for i in 1..len as usize {
            let key = sig[i];
            let mut j = i;
            while j > 0 && sig[j - 1] > key { sig[j] = sig[j - 1]; j -= 1; }
            sig[j] = key;
        }
        (sig, len)
    }

    fn isqrt128(n: i128) -> i64 {
        if n <= 0 { return 0; }
        let mut x = (n as f64).sqrt() as i64;
        if x < 0 { x = 0; }
        while (x as i128) * (x as i128) > n { x -= 1; }
        while ((x + 1) as i128) * ((x + 1) as i128) <= n { x += 1; }
        x
    }

    use std::collections::HashMap;
    let mut cache: HashMap<(Sig, u8), i64> = HashMap::new();
    let mut total: i64 = 0;

    for r in 1..=N {
        let (sig, len) = get_signature(r, &spf);

        if let Some(&count) = cache.get(&(sig, len)) {
            total += count * r as i64;
            continue;
        }

        // Find all x > 0 with x^2 + y^2 = r^2
        let r2 = (r as i64) * (r as i64);
        let mut sides = Vec::new();
        for x in 1..=r {
            let y2 = r2 - (x as i64) * (x as i64);
            if y2 < 0 { break; }
            let y = (y2 as f64).sqrt() as i64;
            let yadj = if y * y > y2 { y - 1 } else if (y + 1) * (y + 1) <= y2 { y + 1 } else { y };
            if yadj * yadj == y2 {
                sides.push(2 * x as i64);
            }
        }

        let mut num_triangles: i64 = 0;
        let ns = sides.len();
        for i in 0..ns {
            let a = sides[i];
            for j in i..ns {
                let b = sides[j];
                for k in j..ns {
                    let c = sides[k];
                    if a + b <= c { break; }

                    let s2 = a + b + c;
                    let p1 = -a + b + c;
                    let p2 = a - b + c;
                    let p3 = a + b - c;

                    let big_p = s2 as i128 * p1 as i128 * p2 as i128 * p3 as i128;
                    let q = isqrt128(big_p);
                    if q as i128 * q as i128 != big_p { continue; }

                    let abc = a as i128 * b as i128 * c as i128;
                    let rq = r as i128 * q as i128;
                    if abc == rq {
                        num_triangles += 1;
                    }
                }
            }
        }

        cache.insert((sig, len), num_triangles);
        total += num_triangles * r as i64;
    }

    println!("{}", total);
}
