// Project Euler 454 - Diophantine reciprocals III
// F(L) = #{ 1/x + 1/y = 1/n : x < y <= L }.
// F(L) = sum_g μ(g) sum_{y^2 <= L/g^2} sum_{s=y+1}^{2y-1} floor((L/g^2)/(y s)).

use rayon::prelude::*;

const N: u64 = 1_000_000_000_000;

/// Linear sieve of μ on odds; evens via μ(2k)=0 (k even) or −μ(k) (k odd).
fn sieve_mu(limit: usize) -> Vec<i8> {
    let mut mu = vec![0i8; limit + 1];
    let mut vis = vec![false; limit + 1];
    let mut primes: Vec<usize> = Vec::with_capacity(limit / 10);
    mu[1] = 1;
    for i in (3..=limit).step_by(2) {
        if !vis[i] {
            primes.push(i);
            mu[i] = -1;
        }
        let mu_i = mu[i];
        for &p in &primes {
            if p > limit / i {
                break;
            }
            vis[i * p] = true;
            if i % p == 0 {
                break;
            }
            mu[i * p] = -mu_i;
        }
    }
    if limit >= 2 {
        mu[2] = -1;
    }
    for g in (6..=limit).step_by(2) {
        if g & 3 == 0 {
            mu[g] = 0;
        } else {
            mu[g] = -mu[g >> 1];
        }
    }
    mu
}

/// sum_{s=y+1}^{2y-1} floor((n/y)/s). Brute when y^3 <= n, else Dirichlet q-blocks.
#[inline(always)]
fn inner_y(n: u64, y: u64) -> u64 {
    let m = n / y;
    let y2 = y * y;
    if y2 <= m {
        let mut ans = 0u64;
        if m <= u32::MAX as u64 {
            let m32 = m as u32;
            let mut s = (y + 1) as u32;
            let hi = ((y << 1) - 1) as u32;
            while s + 7 <= hi {
                ans += (m32 / s) as u64;
                ans += (m32 / (s + 1)) as u64;
                ans += (m32 / (s + 2)) as u64;
                ans += (m32 / (s + 3)) as u64;
                ans += (m32 / (s + 4)) as u64;
                ans += (m32 / (s + 5)) as u64;
                ans += (m32 / (s + 6)) as u64;
                ans += (m32 / (s + 7)) as u64;
                s += 8;
            }
            while s <= hi {
                ans += (m32 / s) as u64;
                s += 1;
            }
        } else {
            let mut s = y + 1;
            let hi = (y << 1) - 1;
            while s <= hi {
                ans += m / s;
                s += 1;
            }
        }
        ans
    } else if m <= u32::MAX as u64 {
        let m32 = m as u32;
        let y32 = y as u32;
        let two_y_1 = (y32 << 1) - 1;
        let mut q = m32 / two_y_1;
        if q < 1 {
            q = 1;
        }
        let mut ans = 0u64;
        loop {
            let mut upper = m32 / q;
            if upper > two_y_1 {
                upper = two_y_1;
            }
            let mut lower = m32 / (q + 1);
            if lower < y32 {
                lower = y32;
            }
            ans += (upper - lower) as u64 * q as u64;
            if lower == y32 {
                break;
            }
            q += 1;
        }
        ans
    } else {
        let two_y_1 = (y << 1) - 1;
        let mut q = m / two_y_1;
        if q < 1 {
            q = 1;
        }
        let mut ans = 0u64;
        loop {
            let mut upper = m / q;
            if upper > two_y_1 {
                upper = two_y_1;
            }
            let mut lower = m / (q + 1);
            if lower < y {
                lower = y;
            }
            ans += (upper - lower) * q;
            if lower == y {
                break;
            }
            q += 1;
        }
        ans
    }
}

/// y-loop for one n, only y ≡ 2+off (mod stride).
#[inline(always)]
fn contrib_strided(n: u64, stride: u64, mut y: u64) -> u64 {
    let mut ans = 0u64;
    while y * y <= n {
        ans += inner_y(n, y);
        y += stride;
    }
    ans
}

#[inline(always)]
fn contrib(n: u64) -> u64 {
    contrib_strided(n, 1, 2)
}

fn main() {
    let limit = N.isqrt() as usize + 1;
    let mu = sieve_mu(limit);

    let mut heavy: Vec<(u64, i8)> = Vec::new();
    let mut light: Vec<(u64, i8)> = Vec::with_capacity((limit * 3) / 5);

    for g in 1..=limit {
        let mu_g = unsafe { *mu.get_unchecked(g) };
        if mu_g == 0 {
            continue;
        }
        let n = N / (g as u64 * g as u64);
        if n < 4 {
            continue;
        }
        if n >= 1_000_000 {
            heavy.push((n, mu_g));
        } else {
            light.push((n, mu_g));
        }
    }

    let nt = rayon::current_num_threads().max(1) as u32;
    let stride = nt as u64;

    let ans: i64 = (0..nt)
        .into_par_iter()
        .map(|off| {
            let mut acc = 0i64;
            let y0 = 2 + off as u64;
            for &(n, mu_g) in &heavy {
                acc += mu_g as i64 * contrib_strided(n, stride, y0) as i64;
            }
            let mut i = off as usize;
            while i < light.len() {
                let (n, mu_g) = unsafe { *light.get_unchecked(i) };
                acc += mu_g as i64 * contrib(n) as i64;
                i += nt as usize;
            }
            acc
        })
        .sum();

    println!("{}", ans);
}
