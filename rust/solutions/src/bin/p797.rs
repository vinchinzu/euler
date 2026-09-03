// Project Euler 797 - Cyclotomic Polynomials
// Sieve division for F_n(2), then G_n(2) = product of (F_d(2)+1) for d|n.
// Total = sum_{n=1}^N Mertens(N/n) * G_n(2).

const N: usize = 10_000_000;
const MOD: u64 = 1_000_000_007;

fn mod_inv(a: u32) -> u64 {
    let (mut t, mut newt) = (0i32, 1i32);
    let (mut r, mut newr) = (MOD as i32, a as i32);
    while newr != 0 {
        let q = r / newr;
        let tmp = newt;
        newt = t - q * newt;
        t = tmp;
        let tmp = newr;
        newr = r - q * newr;
        r = tmp;
    }
    if t < 0 {
        (t + MOD as i32) as u64
    } else {
        t as u64
    }
}

fn main() {
    // Mobius function using linear sieve
    let mut primes = Vec::with_capacity(664_579);
    let mut min_prime = vec![0u32; N + 1];
    let mut mu = vec![0i8; N + 1];
    mu[1] = 1;

    for i in 2..=N {
        let mp_i = min_prime[i];
        let mp = if mp_i == 0 {
            min_prime[i] = i as u32;
            primes.push(i as u32);
            mu[i] = -1;
            i as u32
        } else {
            mp_i
        };

        for &p in &primes {
            let ip = i * p as usize;
            if ip > N {
                break;
            }
            min_prime[ip] = p;
            if p == mp {
                break;
            }
            mu[ip] = -mu[i];
        }
    }

    // Mertens function
    let mut mertens = vec![0i32; N + 1];
    for i in 1..=N {
        mertens[i] = mertens[i - 1] + mu[i] as i32;
    }
    drop(mu);
    drop(min_prime);
    drop(primes);

    // F[n] = cyclotomic polynomial at 2, computed by sieve division
    let mut f = vec![0u32; N + 1];
    {
        let mut p2 = 1u64;
        for i in 0..=N {
            f[i] = (p2 - 1) as u32;
            let next = p2 * 2;
            p2 = if next >= MOD { next - MOD } else { next };
        }
    }
    for i in 1..=N / 2 {
        if f[i] == 1 {
            continue;
        }
        let inv = mod_inv(f[i]);
        let mut j = 2 * i;
        while j <= N {
            f[j] = ((f[j] as u64 * inv) % MOD) as u32;
            j += i;
        }
    }

    // G[n] = product of (F[d]+1) for all d|n
    let mut g = vec![1u32; N + 1];
    for i in 1..=N {
        let factor = (f[i] as u64 + 1) % MOD;
        if factor == 1 {
            continue;
        }
        let mut j = i;
        while j <= N {
            g[j] = ((g[j] as u64 * factor) % MOD) as u32;
            j += i;
        }
    }
    drop(f);

    // Sum contributions
    let mut ans: u64 = 0;
    for i in 1..=N {
        let m = mertens[N / i];
        let m_mod = if m < 0 {
            (m as i64 + MOD as i64) as u64
        } else {
            m as u64
        };
        ans = (ans + m_mod * g[i] as u64) % MOD;
    }

    println!("{}", ans);
}
