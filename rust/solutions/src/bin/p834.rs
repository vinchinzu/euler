// Project Euler 834 - Add and Divide
// f(m) via divisors of n(n-1)

const NMAX: usize = 1_234_567;

fn main() {
    // SPF sieve
    let mut spf = vec![0u32; NMAX + 1];
    for i in 2..=NMAX {
        if spf[i] == 0 {
            spf[i] = i as u32;
            if (i as u64) * (i as u64) <= NMAX as u64 {
                let mut j = i * i;
                while j <= NMAX {
                    if spf[j] == 0 { spf[j] = i as u32; }
                    j += i;
                }
            }
        }
    }

    fn factorize(mut n: i64, spf: &[u32]) -> Vec<(i64, u32)> {
        let mut factors = Vec::new();
        let mut p = 2i64;
        while p * p <= n && (p as usize) <= spf.len() - 1 {
            if spf[p as usize] == p as u32 && n % p == 0 {
                let mut e = 0u32;
                while n % p == 0 { n /= p; e += 1; }
                factors.push((p, e));
            }
            p += 1;
        }
        if n > 1 {
            factors.push((n, 1));
        }
        factors
    }

    fn gen_divisors(factors: &[(i64, u32)]) -> Vec<i64> {
        let mut divs = vec![1i64];
        for &(p, e) in factors {
            let cur = divs.len();
            let mut pk = 1i64;
            for _ in 0..e {
                pk *= p;
                for k in 0..cur {
                    divs.push(divs[k] * pk);
                }
            }
        }
        divs
    }

    let mut ans: i64 = 0;

    for n in 3..=NMAX as i64 {
        let nn1 = n * (n - 1);
        let factors = factorize(nn1, &spf);
        let divs = gen_divisors(&factors);

        for &d in &divs {
            if d > n {
                let m = d - n;
                let val = d + 1 - nn1 / d;
                if val % 2 == 0 {
                    ans += m;
                }
            }
        }
    }

    println!("{}", ans);
}
