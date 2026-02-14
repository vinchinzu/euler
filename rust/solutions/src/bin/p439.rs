// Project Euler 439: Sum of sum of divisors
// S(N) = sum_{i=1}^N sum_{j=1}^N d(i*j) mod 10^9.
// Uses Mobius function, hyperbola method, and memoized recursive sums.

const N: i64 = 100_000_000_000;
const MOD: i64 = 1_000_000_000;
const CACHE_SIZE: usize = 400_000;

fn modd(x: i64) -> i64 {
    ((x % MOD) + MOD) % MOD
}

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn tr(n: i64) -> i64 {
    let n = ((n % (2 * MOD)) + 2 * MOD) % (2 * MOD);
    n * (n + 1) / 2 % MOD
}

fn sum_range(a: i64, b: i64) -> i64 {
    modd(tr(b) - tr(a - 1))
}

struct Context {
    l: usize,
    mobius: Vec<i8>,
    sigma_prefix: Vec<i64>,
    n_mu_prefix: Vec<i64>,
    sigma_cache: Vec<i64>,
    n_mu_cache: Vec<i64>,
    sigma_cached: Vec<bool>,
    n_mu_cached: Vec<bool>,
}

impl Context {
    fn sigma_sum(&mut self, n: i64) -> i64 {
        if n <= self.l as i64 {
            return self.sigma_prefix[n as usize];
        }
        let idx = (N / n) as usize;
        if idx < CACHE_SIZE && self.sigma_cached[idx] {
            return self.sigma_cache[idx];
        }

        let mut result: i64 = 0;
        let sqrt_n = isqrt(n);

        for d in 1..=sqrt_n {
            result = modd(result + (d % MOD) * ((n / d) % MOD));
        }
        for k in 1..=sqrt_n {
            let d_hi = n / k;
            if d_hi > sqrt_n {
                let d_lo = n / (k + 1) + 1;
                result = modd(result + sum_range(d_lo, d_hi) * (k % MOD));
            }
        }

        if idx < CACHE_SIZE {
            self.sigma_cache[idx] = result;
            self.sigma_cached[idx] = true;
        }
        result
    }

    fn n_mu_sum(&mut self, n: i64) -> i64 {
        if n <= self.l as i64 {
            return self.n_mu_prefix[n as usize];
        }
        let idx = (N / n) as usize;
        if idx < CACHE_SIZE && self.n_mu_cached[idx] {
            return self.n_mu_cache[idx];
        }

        let mut result: i64 = 1;
        let sqrt_n = isqrt(n);

        for d in 2..=sqrt_n {
            let sub = self.n_mu_sum(n / d);
            result = modd(result - sub * (d % MOD));
        }
        for k in 1..=sqrt_n {
            let d_hi = n / k;
            let d_lo = n / (k + 1);
            if d_hi > sqrt_n && d_lo >= sqrt_n {
                let sub = self.n_mu_sum(k);
                result = modd(result - sub * sum_range(d_lo + 1, d_hi));
            }
        }

        if idx < CACHE_SIZE {
            self.n_mu_cache[idx] = result;
            self.n_mu_cached[idx] = true;
        }
        result
    }
}

fn main() {
    let l = (N as f64).sqrt() as usize + 10;

    // Sieve for Mobius function
    let mut mobius = vec![1i8; l + 1];
    let mut spf = vec![0u32; l + 1];
    for i in 2..=l {
        if spf[i] == 0 {
            spf[i] = i as u32;
            let mut j = (i as u64 * i as u64) as usize;
            while j <= l {
                if spf[j] == 0 { spf[j] = i as u32; }
                j += i;
            }
        }
    }
    for i in 2..=l {
        if spf[i] == i as u32 {
            mobius[i] = -1;
        } else {
            let p = spf[i] as usize;
            let q = i / p;
            if q % p == 0 {
                mobius[i] = 0;
            } else {
                mobius[i] = -mobius[q];
            }
        }
    }

    // Compute prefix sums
    let mut sigma = vec![0i64; l + 1];
    for d in 1..=l {
        let mut m = d;
        while m <= l {
            sigma[m] += d as i64;
            m += d;
        }
    }
    let mut sigma_prefix = vec![0i64; l + 1];
    for i in 1..=l {
        sigma_prefix[i] = modd(sigma_prefix[i - 1] + sigma[i]);
    }

    let mut n_mu_prefix = vec![0i64; l + 1];
    for i in 1..=l {
        n_mu_prefix[i] = modd(n_mu_prefix[i - 1] + i as i64 * mobius[i] as i64);
    }

    let mut ctx = Context {
        l,
        mobius: mobius.iter().map(|&x| x).collect(),
        sigma_prefix,
        n_mu_prefix,
        sigma_cache: vec![0i64; CACHE_SIZE],
        n_mu_cache: vec![0i64; CACHE_SIZE],
        sigma_cached: vec![false; CACHE_SIZE],
        n_mu_cached: vec![false; CACHE_SIZE],
    };

    let mut ans: i64 = 0;

    // First part: g = 1..L
    for g in 1..=ctx.l {
        if ctx.mobius[g] != 0 {
            let ss = ctx.sigma_sum(N / g as i64);
            let term = modd(ctx.mobius[g] as i64 * (g as i64 % MOD) % MOD * ss % MOD * ss % MOD);
            ans = modd(ans + term);
        }
    }

    // Second part: quotient values
    let mut q = 1i64;
    while q < N / ctx.l as i64 {
        let g_hi = N / q;
        let mut g_lo = N / (q + 1);
        if g_lo < ctx.l as i64 { g_lo = ctx.l as i64; }
        if g_hi > g_lo {
            let ss = ctx.sigma_sum(q);
            let mu_diff = modd(ctx.n_mu_sum(g_hi) - ctx.n_mu_sum(g_lo));
            let term = modd(mu_diff * ss % MOD * ss % MOD);
            ans = modd(ans + term);
        }
        q += 1;
    }

    println!("{}", modd(ans));
}
