// Project Euler 448: Average least common multiple
// S(N) = (N + sum_{k=1}^N floor(N/k) * k * phi(k)) / 2  (mod MOD)
//
// Prefix of k*phi(k) is sieved to ~N^{2/3}. Remaining S(floor(N/i))
// values are filled bottom-up (Du Jiao linearization): each is O(sqrt)
// with array hits, no recursive HashMap.

const N: u64 = 99_999_999_019;
const MOD: u64 = 999_999_017;
const INV2: u64 = (MOD + 1) / 2;

fn mod_inv(mut a: i64, m: i64) -> i64 {
    let (mut g, mut x, mut y) = (m, 0i64, 1i64);
    while a != 0 {
        let q = g / a;
        let t = g - q * a;
        g = a;
        a = t;
        let t = x - q * y;
        x = y;
        y = t;
    }
    ((x % m) + m) % m
}

#[inline(always)]
fn mul_mod(a: u64, b: u64) -> u64 {
    a * b % MOD
}

#[inline(always)]
fn add_mod(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= MOD { s - MOD } else { s }
}

#[inline(always)]
fn sub_mod(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + MOD - b }
}


/// n(n+1)(2n+1)/6 mod MOD.
#[inline(always)]
fn p2(n: u64, inv6: u64) -> u64 {
    let n = n % MOD;
    let np1 = n + 1;
    let np1 = if np1 >= MOD { 0 } else { np1 };
    mul_mod(mul_mod(mul_mod(n, np1), (2 * n + 1) % MOD), inv6)
}

/// S(x) = sum_{k<=x} k phi(k) from the identity
/// sum_{d<=x} d S(floor(x/d)) = P2(x).
fn compute_s(x: u64, limit: u64, small: &[u32], large: &[u64], parent: u64, inv6: u64) -> u64 {
    let s = x.isqrt();
    let mut sum_terms = 0u128;

    // Part 1: l <= s where r == l
    let mut l = 2u64;
    while l <= s {
        let q = x / l;
        let sq = if q <= limit {
            unsafe { *small.get_unchecked(q as usize) as u64 }
        } else {
            unsafe { *large.get_unchecked((parent / q) as usize) }
        };
        sum_terms += (l as u128) * (sq as u128);
        l += 1;
    }

    // Part 2: q decreases from x / l down to 1
    // In this range, q <= s <= limit, so sq ALWAYS comes from small!
    let mut prev_r = s;
    let q_start = x / l;
    for q in (1..=q_start).rev() {
        let r = x / q;
        let cur_l = prev_r + 1;
        if cur_l <= r {
            let count = r - cur_l + 1;
            let sum_ends = cur_l + r;
            let (a, b) = if count % 2 == 0 {
                (count / 2, sum_ends)
            } else {
                (count, sum_ends / 2)
            };
            let sum_d = ((a % MOD) * (b % MOD)) % MOD;
            let sq = unsafe { *small.get_unchecked(q as usize) as u64 };
            sum_terms += (sum_d as u128) * (sq as u128);
            prev_r = r;
        }
    }

    let total_sub = (sum_terms % (MOD as u128)) as u64;
    sub_mod(p2(x, inv6), total_sub)
}

fn main() {
    let inv6 = mod_inv(6, MOD as i64) as u64;

    let limit: usize = 35_000_000;
    let limit_u = limit as u64;

    // Direct linear sieve of k * phi(k) mod MOD into a single u32 array
    let mut f = vec![0u32; limit + 1];
    let mut primes: Vec<u32> = Vec::with_capacity(limit / 14);
    f[1] = 1;
    let f_ptr = f.as_mut_ptr();
    for i in 2..=limit {
        let fi = unsafe { *f_ptr.add(i) };
        if fi == 0 {
            let i64_val = i as u64;
            let val = (i64_val * (i64_val - 1) % MOD) as u32;
            unsafe { *f_ptr.add(i) = val; }
            primes.push(i as u32);
        }
        let fi = unsafe { *f_ptr.add(i) } as u64;
        for &p in &primes {
            let j = i * p as usize;
            if j > limit {
                break;
            }
            let p64 = p as u64;
            if i % p as usize == 0 {
                let p2 = (p64 * p64) % MOD;
                unsafe { *f_ptr.add(j) = (fi * p2 % MOD) as u32; }
                break;
            } else {
                let fp = unsafe { *f_ptr.add(p as usize) } as u64;
                unsafe { *f_ptr.add(j) = (fi * fp % MOD) as u32; }
            }
        }
    }
    drop(primes);

    let mut acc = 0u64;
    for k in 1..=limit {
        acc += unsafe { *f_ptr.add(k) } as u64;
        if acc >= MOD {
            acc -= MOD;
        }
        unsafe { *f_ptr.add(k) = acc as u32; }
    }
    let small = f;

    let max_i = (N / (limit_u + 1)) as usize;
    let mut large = vec![0u64; max_i + 1];
    let large_ptr = large.as_mut_ptr();
    for i in (1..=max_i).rev() {
        let x = N / i as u64;
        let val = compute_s(x, limit_u, &small, &large, N, inv6);
        unsafe {
            *large_ptr.add(i) = val;
        }
    }

    // sum_k floor(N/k) * k phi(k) via floor blocks; S(l-1) is the previous S(r).
    let mut ans = 0u64;
    let mut l = 1u64;
    let mut sprev = 0u64;
    let small_ptr = small.as_ptr();
    while l <= N {
        let q = N / l;
        let r = N / q;
        let sr = if r <= limit_u {
            unsafe { *small_ptr.add(r as usize) as u64 }
        } else {
            unsafe { *large_ptr.add(q as usize) }
        };
        ans = add_mod(ans, mul_mod(q % MOD, sub_mod(sr, sprev)));
        sprev = sr;
        l = r + 1;
    }

    ans = mul_mod(add_mod(ans, N % MOD), INV2);
    println!("{ans}");
}
