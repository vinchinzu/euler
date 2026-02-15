// Project Euler 606 - Gozinta Chains
// Lucy DP for sum of p^3 for primes p <= n, then pair-product sums

const M: i64 = 1_000_000_000;
const L: i64 = 1_000_000_000_000;

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn sum_cubes_mod(n: i64) -> i64 {
    let n = ((n % (2 * M)) + 2 * M) % (2 * M);
    let t = if n % 2 == 0 {
        let h = (n / 2) % M;
        (h as i128 * ((n + 1) % M) as i128 % M as i128) as i64
    } else {
        let nm = n % M;
        (nm as i128 * (((n + 1) / 2) % M) as i128 % M as i128) as i64
    };
    (t as i128 * t as i128 % M as i128) as i64
}

fn pow_mod_local(mut base: i64, mut exp: i32, m: i64) -> i64 {
    let mut result = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn main() {
    let sqrt_l = isqrt(L);

    let mut small_s = vec![0i64; (sqrt_l + 2) as usize];
    let mut large_s = vec![0i64; (sqrt_l + 2) as usize];

    for v in 1..=sqrt_l {
        small_s[v as usize] = (sum_cubes_mod(v) - 1 + M) % M;
    }
    for k in 1..=sqrt_l {
        let v = L / k;
        large_s[k as usize] = (sum_cubes_mod(v) - 1 + M) % M;
    }

    let get = |v: i64, ss: &[i64], ls: &[i64]| -> i64 {
        if v <= sqrt_l { ss[v as usize] } else { ls[(L / v) as usize] }
    };

    for p in 2..=sqrt_l {
        if get(p, &small_s, &large_s) == get(p - 1, &small_s, &large_s) { continue; }
        let p3 = pow_mod_local(p, 3, M);
        let s_pm1 = get(p - 1, &small_s, &large_s);

        for k in 1..=sqrt_l {
            let v = L / k;
            if v < p * p { break; }
            let vp = v / p;
            let old = large_s[k as usize];
            let sub = (p3 as i128 * ((get(vp, &small_s, &large_s) - s_pm1 + M) % M) as i128 % M as i128) as i64;
            large_s[k as usize] = (old - sub + M) % M;
        }
        for v in (p * p..=sqrt_l).rev() {
            let vp = v / p;
            let old = small_s[v as usize];
            let sub = (p3 as i128 * ((get(vp, &small_s, &large_s) - s_pm1 + M) % M) as i128 % M as i128) as i64;
            small_s[v as usize] = (old - sub + M) % M;
        }
    }

    // Sieve for primes up to sqrt_l
    let mut is_prime = vec![true; (sqrt_l + 1) as usize];
    is_prime[0] = false;
    if sqrt_l >= 1 { is_prime[1] = false; }
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

    let get2 = |v: i64| -> i64 {
        if v <= sqrt_l { small_s[v as usize] } else { large_s[(L / v) as usize] }
    };

    let mut ans: i64 = 0;
    for p in 2..=sqrt_l {
        if !is_prime[p as usize] { continue; }
        let p3 = pow_mod_local(p, 3, M);
        let q_max = L / p;
        if q_max > p {
            let sum_qmax = get2(q_max);
            let sum_p = get2(p);
            let sum_q = (sum_qmax - sum_p + M) % M;
            ans = (ans + (p3 as i128 * sum_q as i128 % M as i128) as i64) % M;
        }
    }

    println!("{}", ans);
}
