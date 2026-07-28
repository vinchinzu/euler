// Project Euler 606 - Gozinta Chains
// Lucy DP for sum of p^3 for primes p <= n, then pair-product sums.
// Optimization: pure u64 modular mul (MOD=1e9, products fit in u64).

const M: u64 = 1_000_000_000;
const L: i64 = 1_000_000_000_000;

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

#[inline(always)]
fn mul_mod(a: u64, b: u64) -> u64 {
    // a,b < M => a*b < 1e18 < 2^64
    (a * b) % M
}

fn sum_cubes_mod(n: i64) -> u64 {
    let n = ((n % (2 * M as i64)) + 2 * M as i64) % (2 * M as i64);
    let t = if n % 2 == 0 {
        let h = ((n / 2) as u64) % M;
        mul_mod(h, ((n + 1) as u64) % M)
    } else {
        let nm = (n as u64) % M;
        mul_mod(nm, (((n + 1) / 2) as u64) % M)
    };
    mul_mod(t, t)
}

fn pow_mod_local(mut base: u64, mut exp: i32) -> u64 {
    let mut result = 1u64;
    base %= M;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base);
        }
        base = mul_mod(base, base);
        exp >>= 1;
    }
    result
}

fn main() {
    let sqrt_l = isqrt(L);

    let mut small_s = vec![0u64; (sqrt_l + 2) as usize];
    let mut large_s = vec![0u64; (sqrt_l + 2) as usize];

    for v in 1..=sqrt_l {
        small_s[v as usize] = (sum_cubes_mod(v) + M - 1) % M;
    }
    for k in 1..=sqrt_l {
        let v = L / k;
        large_s[k as usize] = (sum_cubes_mod(v) + M - 1) % M;
    }

    #[inline(always)]
    fn get(v: i64, ss: &[u64], ls: &[u64], sqrt_l: i64) -> u64 {
        if v <= sqrt_l {
            ss[v as usize]
        } else {
            ls[(L / v) as usize]
        }
    }

    for p in 2..=sqrt_l {
        if get(p, &small_s, &large_s, sqrt_l) == get(p - 1, &small_s, &large_s, sqrt_l) {
            continue;
        }
        let p3 = pow_mod_local(p as u64, 3);
        let s_pm1 = get(p - 1, &small_s, &large_s, sqrt_l);

        for k in 1..=sqrt_l {
            let v = L / k;
            if v < p * p {
                break;
            }
            let vp = v / p;
            let old = large_s[k as usize];
            let sub = mul_mod(p3, (get(vp, &small_s, &large_s, sqrt_l) + M - s_pm1) % M);
            large_s[k as usize] = (old + M - sub) % M;
        }
        for v in (p * p..=sqrt_l).rev() {
            let vp = v / p;
            let old = small_s[v as usize];
            let sub = mul_mod(p3, (get(vp, &small_s, &large_s, sqrt_l) + M - s_pm1) % M);
            small_s[v as usize] = (old + M - sub) % M;
        }
    }

    // Sieve for primes up to sqrt_l
    let mut is_prime = vec![true; (sqrt_l + 1) as usize];
    is_prime[0] = false;
    if sqrt_l >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2i64;
    while i * i <= sqrt_l {
        if is_prime[i as usize] {
            let mut j = i * i;
            while j <= sqrt_l {
                is_prime[j as usize] = false;
                j += i;
            }
        }
        i += 1;
    }

    let get2 = |v: i64| -> u64 {
        if v <= sqrt_l {
            small_s[v as usize]
        } else {
            large_s[(L / v) as usize]
        }
    };

    let mut ans: u64 = 0;
    for p in 2..=sqrt_l {
        if !is_prime[p as usize] {
            continue;
        }
        let p3 = pow_mod_local(p as u64, 3);
        let q_max = L / p;
        if q_max > p {
            let sum_qmax = get2(q_max);
            let sum_p = get2(p);
            let sum_q = (sum_qmax + M - sum_p) % M;
            ans = (ans + mul_mod(p3, sum_q)) % M;
        }
    }

    println!("{}", ans);
}
