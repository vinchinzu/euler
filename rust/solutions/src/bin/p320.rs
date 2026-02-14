// Project Euler 320: Factorials divisible by large power
// Sieve smallest prime factors, track exponents, compute N(i) via Legendre's formula.

const MAX_I: usize = 1_000_000;
const MIN_I: usize = 10;
const K: i64 = 1_234_567_890;
const MOD_VAL: u64 = 1_000_000_000_000_000_000;

fn legendre(n: i64, p: i64) -> i64 {
    let mut cur = 0i64;
    let mut pk = p;
    while pk <= n {
        cur += n / pk;
        if pk > n / p { break; }
        pk *= p;
    }
    cur
}

fn advance(p: i64, needed: i64, pn_val: &mut i64, pleg_sum: &mut i64) {
    let mut n_val = *pn_val;
    let mut cur = *pleg_sum;

    if cur >= needed {
        return;
    }

    let gap = needed - cur;
    if gap > 1000 {
        let est = n_val + gap * (p - 1);
        let est = est - est % p;
        let est = if est < n_val { n_val } else { est };

        let mut lo = n_val;
        let mut hi = est + gap * (p - 1);
        hi -= hi % p;
        if hi < lo { hi = lo; }

        let mut leg_hi = legendre(hi, p);
        while leg_hi < needed {
            hi *= 2;
            hi -= hi % p;
            leg_hi = legendre(hi, p);
        }

        while lo < hi {
            let mut mid = lo + (hi - lo) / 2;
            mid -= mid % p;
            if mid < lo { break; }
            if mid == lo { break; }
            let leg_mid = legendre(mid, p);
            if leg_mid >= needed {
                hi = mid;
            } else {
                lo = mid + p;
                lo -= lo % p;
            }
        }
        n_val = lo;
        cur = legendre(n_val, p);
    }

    while cur < needed {
        n_val += p;
        let mut kk = n_val;
        while kk % p == 0 {
            cur += 1;
            kk /= p;
        }
    }

    *pn_val = n_val;
    *pleg_sum = cur;
}

fn main() {
    let mut spf = vec![0u32; MAX_I + 1];
    for i in 0..=MAX_I { spf[i] = i as u32; }
    {
        let mut i = 2;
        while i * i <= MAX_I {
            if spf[i] == i as u32 {
                let mut j = i * i;
                while j <= MAX_I {
                    if spf[j] == j as u32 { spf[j] = i as u32; }
                    j += i;
                }
            }
            i += 1;
        }
    }

    let mut primes_list = Vec::new();
    let mut pidx = vec![0usize; MAX_I + 1];
    for i in 2..=MAX_I {
        if spf[i] == i as u32 {
            pidx[i] = primes_list.len();
            primes_list.push(i as i64);
        }
    }
    let np = primes_list.len();

    let mut exp_f = vec![0i64; np];
    let mut n_for_prime = vec![0i64; np];
    let mut leg_cache = vec![0i64; np];

    for j in 2..MIN_I {
        let mut n = j;
        while n > 1 {
            let p = spf[n] as usize;
            let pi = pidx[p];
            while n % p == 0 {
                n /= p;
                exp_f[pi] += 1;
            }
        }
    }

    for j in 0..np {
        if exp_f[j] == 0 { continue; }
        let needed = K * exp_f[j];
        let p = primes_list[j];
        let mut np_val = (p - 1) * needed;
        np_val -= np_val % p;
        let mut cur = legendre(np_val, p);
        advance(p, needed, &mut np_val, &mut cur);
        n_for_prime[j] = np_val;
        leg_cache[j] = cur;
    }

    let mut max_n: i64 = 0;
    for j in 0..np {
        if n_for_prime[j] > max_n { max_n = n_for_prime[j]; }
    }

    let mut ans: u64 = 0;
    let mut changed_buf = Vec::with_capacity(20);

    for i in MIN_I..=MAX_I {
        let mut n = i;
        changed_buf.clear();
        while n > 1 {
            let p = spf[n] as usize;
            let pi = pidx[p];
            if !changed_buf.contains(&pi) {
                changed_buf.push(pi);
            }
            while n % p == 0 {
                n /= p;
                exp_f[pi] += 1;
            }
        }

        for &pi in &changed_buf {
            let p = primes_list[pi];
            let needed = K * exp_f[pi];
            let mut np_val = n_for_prime[pi];
            let mut cur = leg_cache[pi];

            if np_val == 0 {
                np_val = (p - 1) * needed;
                np_val -= np_val % p;
                cur = legendre(np_val, p);
            }
            advance(p, needed, &mut np_val, &mut cur);
            n_for_prime[pi] = np_val;
            leg_cache[pi] = cur;

            if np_val > max_n { max_n = np_val; }
        }

        ans = (ans + max_n as u64) % MOD_VAL;
    }

    println!("{}", ans);
}
