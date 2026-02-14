// Project Euler 428: Necklace of Circles
// T(N) = S3 + S4 + S6 where each Si counts necklace triplets for k=3,4,6.
// Uses: Mertens function, quotient grouping, Lucy DP for pi_1,
// DFS over 1mod3-smooth numbers.

use std::collections::HashMap;

const NVAL: i64 = 1_000_000_000;

fn isqrt_ll(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

struct Context {
    sieve_limit: usize,
    mu_arr: Vec<i8>,
    mu_prefix: Vec<i64>,
    primes_all: Vec<i32>,
    mertens_cache: HashMap<i64, i64>,
    f_cache: HashMap<i64, i64>,
    t_cache: HashMap<i64, i64>,
    to_cache: HashMap<i64, i64>,
    ton3_cache: HashMap<i64, i64>,
    l_cache: HashMap<i64, i64>,
}

impl Context {
    fn sieve_mu(limit: usize) -> Self {
        let mut mu_arr = vec![0i8; limit + 1];
        mu_arr[1] = 1;
        let mut is_comp = vec![false; limit + 1];
        let mut primes_all = Vec::new();

        for i in 2..=limit {
            if !is_comp[i] {
                primes_all.push(i as i32);
                mu_arr[i] = -1;
            }
            for j in 0..primes_all.len() {
                let v = primes_all[j] as usize * i;
                if v > limit { break; }
                is_comp[v] = true;
                if i % primes_all[j] as usize == 0 {
                    mu_arr[v] = 0;
                    break;
                }
                mu_arr[v] = -mu_arr[i];
            }
        }

        let mut mu_prefix = vec![0i64; limit + 1];
        for i in 1..=limit {
            mu_prefix[i] = mu_prefix[i - 1] + mu_arr[i] as i64;
        }

        Context {
            sieve_limit: limit,
            mu_arr,
            mu_prefix,
            primes_all,
            mertens_cache: HashMap::new(),
            f_cache: HashMap::new(),
            t_cache: HashMap::new(),
            to_cache: HashMap::new(),
            ton3_cache: HashMap::new(),
            l_cache: HashMap::new(),
        }
    }

    fn mertens(&mut self, n: i64) -> i64 {
        if n <= self.sieve_limit as i64 { return self.mu_prefix[n as usize]; }
        if let Some(&v) = self.mertens_cache.get(&n) { return v; }
        let mut s = 0i64;
        let mut d = 2i64;
        while d <= n {
            let q = n / d;
            let d_max = n / q;
            s += (d_max - d + 1) * self.mertens(q);
            d = d_max + 1;
        }
        let result = 1 - s;
        self.mertens_cache.insert(n, result);
        result
    }

    fn q_val(&self, x: i64) -> i64 {
        if x <= 0 { return 0; }
        let sq = isqrt_ll(x);
        let mut s = 0i64;
        for k in 1..=sq {
            if self.mu_arr[k as usize] != 0 {
                s += self.mu_arr[k as usize] as i64 * (x / (k * k));
            }
        }
        s
    }

    fn f_val(&mut self, x: i64) -> i64 {
        if x <= 0 { return 0; }
        if let Some(&v) = self.f_cache.get(&x) { return v; }
        let sx = isqrt_ll(x);
        let mut result = 0i64;
        for d in 1..=sx {
            if self.mu_arr[d as usize] != 0 {
                result += x / d;
            }
        }
        let max_q = x / (sx + 1);
        for q in 1..=max_q {
            result += q * (self.q_val(x / q) - self.q_val(x / (q + 1)));
        }
        self.f_cache.insert(x, result);
        result
    }

    fn t_c(&mut self, x: i64) -> i64 {
        if x <= 0 { return 0; }
        if let Some(&v) = self.t_cache.get(&x) { return v; }
        let mut result = 0i64;
        let mut d = 1i64;
        while d <= x {
            let q = x / d;
            let d_max = x / q;
            let f1 = self.f_val(d_max);
            let f2 = self.f_val(d - 1);
            result += q * (f1 - f2);
            d = d_max + 1;
        }
        self.t_cache.insert(x, result);
        result
    }

    fn t_odd(&mut self, x: i64) -> i64 {
        if x <= 0 { return 0; }
        if let Some(&v) = self.to_cache.get(&x) { return v; }
        let mut result = self.t_c(x);
        let mut a = 1;
        let mut pw = 2i64;
        while pw <= x {
            result -= (2 * a + 1) * self.t_odd(x / pw);
            a += 1;
            pw *= 2;
        }
        self.to_cache.insert(x, result);
        result
    }

    fn t_on3(&mut self, x: i64) -> i64 {
        if x <= 0 { return 0; }
        if let Some(&v) = self.ton3_cache.get(&x) { return v; }
        let mut result = self.t_odd(x);
        let mut c = 1;
        let mut pw = 3i64;
        while pw <= x {
            result -= (2 * c + 1) * self.t_on3(x / pw);
            c += 1;
            pw *= 3;
        }
        self.ton3_cache.insert(x, result);
        result
    }

    fn l_val(&mut self, x: i64) -> i64 {
        if x <= 0 { return 0; }
        if let Some(&v) = self.l_cache.get(&x) { return v; }
        let sq = isqrt_ll(x);
        let mut total = 0i64;
        for j in 1..=sq {
            total += self.mertens(x / (j * j));
        }
        self.l_cache.insert(x, total);
        total
    }

    fn l3(&mut self, x: i64) -> i64 {
        if x <= 0 { return 0; }
        self.l_val(x) + self.l_val(x / 3)
    }
}

fn main() {
    let n = NVAL;
    let sqrt_n = isqrt_ll(n) as usize;

    let cbrt = {
        let mut c = (n as f64).cbrt().round() as i64;
        while (c + 1) * (c + 1) * (c + 1) <= n { c += 1; }
        while c * c * c > n { c -= 1; }
        c as usize
    };

    let mut sl = cbrt * cbrt;
    if sl < sqrt_n + 1 { sl = sqrt_n + 1; }

    let mut ctx = Context::sieve_mu(sl);

    // Pre-fill mertens cache
    {
        let mut d = 1i64;
        while d <= n {
            ctx.mertens(n / d);
            d = n / (n / d) + 1;
        }
    }

    // Precompute F for needed T arguments
    let mut needed_t = Vec::new();
    for a in 0..61 {
        let pw2: i64 = 1i64.checked_shl(a).unwrap_or(i64::MAX);
        if pw2 > n { break; }
        let mut pw3 = 1i64;
        for _ in 0..40 {
            let pw = pw2.checked_mul(pw3).unwrap_or(i64::MAX);
            if pw > n { break; }
            needed_t.push(n / pw);
            pw3 = pw3.checked_mul(3).unwrap_or(i64::MAX);
        }
    }
    needed_t.sort_unstable();
    needed_t.dedup();

    for &x in &needed_t {
        let mut dd = 1i64;
        while dd <= x {
            let qq = x / dd;
            ctx.f_val(qq);
            dd = x / qq + 1;
        }
    }

    // S4
    let mut s4 = 0i64;
    {
        let mut a = 0i64;
        let mut pw = 1i64;
        while pw <= n {
            s4 += (2 * a + 2) * ctx.t_odd(n / pw);
            a += 1;
            pw *= 2;
        }
    }

    // S3
    let mut s3 = 0i64;
    {
        let mut a = 0;
        let mut pw2 = 1i64;
        while pw2 <= n {
            let mut c = 0;
            let mut pw3 = 1i64;
            while pw2 * pw3 <= n {
                s3 += (2 * a + 3) as i64 * (2 * c + 2) as i64 * ctx.t_on3(n / (pw2 * pw3));
                c += 1;
                pw3 *= 3;
            }
            a += 1;
            pw2 *= 2;
        }
    }

    // S6_div3
    let mut s6_div3 = 0i64;
    {
        let mut v = 1i64;
        let mut pw3 = 3i64;
        while pw3 <= n {
            let mut a = 0;
            let mut pw2 = 1i64;
            while pw2 * pw3 <= n {
                s6_div3 += (2 * v - 1) * (2 * a + 3) as i64 * ctx.t_on3(n / (pw2 * pw3));
                a += 1;
                pw2 *= 2;
            }
            v += 1;
            pw3 *= 3;
        }
    }

    // S6_tau
    let mut s6_tau = 0i64;
    {
        let mut a = 0i64;
        let mut pw = 1i64;
        while pw <= n {
            s6_tau += (2 * a + 3) * ctx.t_on3(n / pw);
            a += 1;
            pw *= 2;
        }
    }

    // Lucy DP for pi_1
    let mut small_pi1 = vec![0i64; sqrt_n + 1];
    let mut big_pi1 = vec![0i64; sqrt_n + 2];
    let mut small_pi2 = vec![0i64; sqrt_n + 1];
    let mut big_pi2 = vec![0i64; sqrt_n + 2];

    for v in 1..=sqrt_n {
        small_pi1[v] = (v as i64 + 2) / 3 - 1;
        small_pi2[v] = (v as i64 + 1) / 3;
    }
    for k in 1..=sqrt_n {
        let v = n / k as i64;
        big_pi1[k] = (v + 2) / 3 - 1;
        big_pi2[k] = (v + 1) / 3;
    }

    // Collect quotients in descending order
    let mut quotients_desc = Vec::new();
    {
        let mut d = 1i64;
        while d <= n {
            quotients_desc.push(n / d);
            d = n / (n / d) + 1;
        }
    }

    // Primes for Lucy DP
    let primes_small: Vec<i32> = ctx.primes_all.iter()
        .filter(|&&p| p as usize <= sqrt_n)
        .copied()
        .collect();

    for &p in &primes_small {
        if p == 3 { continue; }
        let pp = p as i64 * p as i64;
        let p1 = small_pi1[p as usize - 1];
        let p2 = small_pi2[p as usize - 1];

        for &v in &quotients_desc {
            if v < pp { break; }
            let vp = v / p as i64;
            let c1 = if vp <= sqrt_n as i64 { small_pi1[vp as usize] } else { big_pi1[(n / vp) as usize] };
            let c2 = if vp <= sqrt_n as i64 { small_pi2[vp as usize] } else { big_pi2[(n / vp) as usize] };

            if p % 3 == 1 {
                if v <= sqrt_n as i64 {
                    small_pi1[v as usize] -= c1 - p1;
                    small_pi2[v as usize] -= c2 - p2;
                } else {
                    let k = (n / v) as usize;
                    big_pi1[k] -= c1 - p1;
                    big_pi2[k] -= c2 - p2;
                }
            } else {
                if v <= sqrt_n as i64 {
                    let old1 = small_pi1[v as usize];
                    let old2 = small_pi2[v as usize];
                    small_pi1[v as usize] = old1 - (c2 - p2);
                    small_pi2[v as usize] = old2 - (c1 - p1);
                } else {
                    let k = (n / v) as usize;
                    let old1 = big_pi1[k];
                    let old2 = big_pi2[k];
                    big_pi1[k] = old1 - (c2 - p2);
                    big_pi2[k] = old2 - (c1 - p1);
                }
            }
        }
    }

    let pi1 = |v: i64| -> i64 {
        if v < 2 { return 0; }
        if v <= sqrt_n as i64 { small_pi1[v as usize] } else { big_pi1[(n / v) as usize] }
    };

    // Collect primes 1 mod 3 up to sqrtN
    let primes_1mod3: Vec<i32> = ctx.primes_all.iter()
        .filter(|&&p| p % 3 == 1)
        .copied()
        .collect();

    // DFS for S6_chi
    let mut sum_g = 0i64;

    struct DfsState {
        idx: usize,
        d_val: i64,
        b_val: i64,
        last_prime: i64,
    }

    let mut dfs_stack = vec![DfsState { idx: 0, d_val: 1, b_val: 1, last_prime: 0 }];

    while let Some(state) = dfs_stack.pop() {
        let l3_val = ctx.l3(n / state.d_val);
        sum_g += state.b_val * l3_val;

        let upper_p = n / state.d_val;
        let lower_p = std::cmp::max(state.last_prime, sqrt_n as i64);

        if upper_p > lower_p {
            let mut p = lower_p + 1;
            let mut large_sum = 0i64;
            while p <= upper_p {
                let q = n / (state.d_val * p);
                let p_range_hi = std::cmp::min(upper_p, if q > 0 { n / (state.d_val * q) } else { upper_p });
                let p_range_lo = std::cmp::max(lower_p + 1, if q < upper_p { n / (state.d_val * (q + 1)) + 1 } else { lower_p + 1 });

                let cnt = pi1(p_range_hi) - pi1(p_range_lo - 1);
                if cnt > 0 {
                    large_sum += cnt * ctx.l3(q);
                }
                p = p_range_hi + 1;
            }
            sum_g += 4 * state.b_val * large_sum;
        }

        // Push children in reverse order for correct processing
        let mut children = Vec::new();
        for i in state.idx..primes_1mod3.len() {
            let pr = primes_1mod3[i];
            if pr as usize > sqrt_n { break; }
            if state.d_val * pr as i64 > n { break; }
            let mut pk = pr as i64;
            let mut k = 1;
            while state.d_val * pk <= n {
                children.push(DfsState {
                    idx: i + 1,
                    d_val: state.d_val * pk,
                    b_val: state.b_val * (4 * k),
                    last_prime: pr as i64,
                });
                k += 1;
                pk *= pr as i64;
            }
        }
        for child in children.into_iter().rev() {
            dfs_stack.push(child);
        }
    }

    let s6_chi = -sum_g;
    let s6 = s6_div3 + (s6_tau + s6_chi) / 2;

    println!("{}", s3 + s4 + s6);
}
