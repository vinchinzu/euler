// Project Euler 73: Counting fractions in a range
// Count reduced fractions n/d strictly between 1/3 and 1/2 with d <= 12000.
//
// #{gcd(n,d)=1} = sum_t μ(t) * #{n,d : t|n, t|d, 1/3 < n/d < 1/2, d <= N}
//               = sum_t μ(t) * f(⌊N/t⌋)
// where f(m) counts (not necessarily reduced) fractions with denominator <= m.

fn main() {
    const N: usize = 12_000;

    let mut mu = vec![0i8; N + 1];
    let mut lp = vec![0u16; N + 1];
    let mut primes = Vec::new();
    mu[1] = 1;
    for i in 2..=N {
        if lp[i] == 0 {
            lp[i] = i as u16;
            primes.push(i);
            mu[i] = -1;
        }
        for &p in &primes {
            let ip = i * p;
            if ip > N {
                break;
            }
            lp[ip] = p as u16;
            if i % p == 0 {
                mu[ip] = 0;
                break;
            }
            mu[ip] = -mu[i];
        }
    }

    // prefix[m] = number of n/d with d <= m and 1/3 < n/d < 1/2 (gcd unrestricted)
    let mut prefix = vec![0i32; N + 1];
    for d in 1..=N {
        let n_min = d / 3 + 1;
        let n_max = (d - 1) / 2;
        let add = if n_min <= n_max {
            (n_max - n_min + 1) as i32
        } else {
            0
        };
        prefix[d] = prefix[d - 1] + add;
    }

    let mut count = 0i64;
    for t in 1..=N {
        let m = mu[t];
        if m != 0 {
            count += m as i64 * prefix[N / t] as i64;
        }
    }

    println!("{count}");
}
