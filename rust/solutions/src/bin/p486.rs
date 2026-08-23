// Project Euler 486 - Palindromic substrings
//
// Count n in [5, L] with F_5(n) divisible by K.
// Order of 2 mod K via phi factorization; incremental powers instead of pow_mod.

fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut f = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0u32;
            while n % d == 0 {
                n /= d;
                e += 1;
            }
            f.push((d, e));
        }
        d = if d == 2 { 3 } else { d + 2 };
    }
    if n > 1 {
        f.push((n, 1));
    }
    f
}

fn pow_mod(mut base: u64, mut exp: u64, modv: u64) -> u64 {
    let mut result = 1u64;
    base %= modv;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % modv as u128) as u64;
        }
        base = (base as u128 * base as u128 % modv as u128) as u64;
        exp >>= 1;
    }
    result
}

fn order(base: u64, modv: u64) -> u64 {
    if gcd_u64(base, modv) != 1 {
        return 0;
    }
    let facts = factor(modv);
    let mut phi = modv;
    for &(p, _) in &facts {
        phi = phi / p * (p - 1);
    }
    let mut ord = phi;
    for (p, e) in factor(phi) {
        for _ in 0..e {
            if pow_mod(base, ord / p, modv) == 1 {
                ord /= p;
            } else {
                break;
            }
        }
    }
    ord
}

fn mod_inv(a: u64, m: u64) -> u64 {
    let (mut old_r, mut r) = (a as i64, m as i64);
    let (mut old_s, mut s) = (1i64, 0i64);
    while r != 0 {
        let q = old_r / r;
        let tmp = r;
        r = old_r - q * r;
        old_r = tmp;
        let tmp = s;
        s = old_s - q * s;
        old_s = tmp;
    }
    ((old_s % m as i64 + m as i64) % m as i64) as u64
}

fn main() {
    let n: u64 = 1_000_000_000_000_000_000; // 10^18
    let k: u64 = 87_654_321;
    let l = n;

    let ord_val = order(2, k);
    let inv = mod_inv(100 * ord_val / 6, k);
    let c: [i64; 6] = [57, 41, 25, 9, -8, -26];

    let mut ans: u64 = 0;
    let mut pow2k = 1u64;
    for ki in 0..ord_val {
        let term = pow2k as i64 - (100 * (ki / 6)) as i64 + c[(ki % 6) as usize];
        let term = ((term % k as i64) + k as i64) as u64 % k;
        let t = (term as u128 * inv as u128 % k as u128) as u64;
        let n_mod = ord_val as u128 * t as u128 + ki as u128;
        if n_mod >= 5 && n_mod <= l as u128 {
            let count = (l as u128 - n_mod) / (ord_val as u128 * k as u128) + 1;
            ans += count as u64;
        }
        pow2k <<= 1;
        if pow2k >= k {
            pow2k -= k;
        }
    }

    println!("{}", ans);
}
