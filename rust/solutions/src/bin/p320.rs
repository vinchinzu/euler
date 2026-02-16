// Project Euler 320: Factorials divisible by large power
// Sieve smallest prime factors, track exponents, compute N(i) via Legendre's formula.
//
// Key optimization: use the identity legendre(n,p) = (n - s_p(n))/(p-1) where s_p(n)
// is the digit sum of n in base p. Use fixed-point iteration to find the target n,
// then verify with legendre. Combined digit_sum_and_legendre avoids redundant computation.

const MAX_I: usize = 1_000_000;
const MIN_I: usize = 10;
const K: i64 = 1_234_567_890;
const MOD_VAL: u64 = 1_000_000_000_000_000_000;

/// Compute digit sum of n in base p, and legendre = (n - s) / (p-1).
#[inline(always)]
fn digit_sum_and_legendre(n: i64, p: i64) -> (i64, i64) {
    if p == 2 {
        // For p=2, digit sum = popcount, legendre = n - popcount(n)
        let s = (n as u64).count_ones() as i64;
        (s, n - s)
    } else {
        let orig = n;
        let mut nn = n;
        let mut s = 0i64;
        while nn > 0 {
            s += nn % p;
            nn /= p;
        }
        (s, (orig - s) / (p - 1))
    }
}

/// Find the smallest multiple of p such that legendre(n, p) >= needed.
/// Uses fixed-point iteration: n = needed*(p-1) + s_p(n), converges in 2-3 steps.
/// Then fine-tune with linear scan (at most 2-3 steps).
#[inline]
fn advance(p: i64, needed: i64, pn_val: &mut i64, pleg_sum: &mut i64) {
    let cur = *pleg_sum;

    if cur >= needed {
        return;
    }

    let gap = needed - cur;

    // For small gaps, use incremental approach from old position.
    // Each step of p advances legendre by ~1, so gap steps of p needed.
    // The cost of gap steps of digit_sum_and_legendre vs fixed-point (5 iterations):
    // For p=2 with popcount, each call is ~5ns, so threshold ~ 5 steps is break-even.
    // For other primes, each call is ~60ns (38 divisions for p=3), so threshold ~ 5 too.
    if gap <= 5 {
        let mut nv = *pn_val;
        let mut c = cur;
        while c < needed {
            nv += p;
            let mut kk = nv;
            while kk % p == 0 {
                c += 1;
                kk /= p;
            }
        }
        *pn_val = nv;
        *pleg_sum = c;
        return;
    }

    // Fixed-point iteration: n = needed*(p-1) + s_p(n)
    let target = needed * (p - 1);
    let mut n = target;

    // Iterate until convergence (typically 2-3 iterations)
    for _ in 0..5 {
        let (s, _) = digit_sum_and_legendre(n, p);
        let n_new = target + s;
        if n_new == n { break; }
        n = n_new;
    }

    // Round up to multiple of p
    if n % p != 0 {
        n += p - n % p;
    }

    // Check and fine-tune
    let (_, leg) = digit_sum_and_legendre(n, p);
    if leg >= needed {
        // Try going back (usually 0-2 steps)
        loop {
            let prev = n - p;
            if prev <= 0 { break; }
            let (_, legp) = digit_sum_and_legendre(prev, p);
            if legp >= needed {
                n = prev;
            } else {
                break;
            }
        }
        let (_, legf) = digit_sum_and_legendre(n, p);
        *pn_val = n;
        *pleg_sum = legf;
    } else {
        // Go forward (usually 1-3 steps)
        loop {
            n += p;
            let (_, leg) = digit_sum_and_legendre(n, p);
            if leg >= needed {
                *pn_val = n;
                *pleg_sum = leg;
                return;
            }
        }
    }
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
        advance(p, needed, &mut n_for_prime[j], &mut leg_cache[j]);
    }

    let mut max_n: i64 = 0;
    for j in 0..np {
        if n_for_prime[j] > max_n { max_n = n_for_prime[j]; }
    }

    let mut ans: u64 = 0;
    let mut changed_buf: Vec<usize> = Vec::with_capacity(20);

    for i in MIN_I..=MAX_I {
        let mut n = i;
        changed_buf.clear();
        while n > 1 {
            let p = spf[n] as usize;
            let pi = pidx[p];
            if changed_buf.is_empty() || *changed_buf.last().unwrap() != pi {
                // Since SPF factorization processes factors in order,
                // and a prime can only appear once in the changed_buf,
                // we just need to check the last element
                if !changed_buf.contains(&pi) {
                    changed_buf.push(pi);
                }
            }
            while n % p == 0 {
                n /= p;
                exp_f[pi] += 1;
            }
        }

        for &pi in &changed_buf {
            let p = primes_list[pi];
            let needed = K * exp_f[pi];
            advance(p, needed, &mut n_for_prime[pi], &mut leg_cache[pi]);

            if n_for_prime[pi] > max_n { max_n = n_for_prime[pi]; }
        }

        ans = (ans + max_n as u64) % MOD_VAL;
    }

    println!("{}", ans);
}
