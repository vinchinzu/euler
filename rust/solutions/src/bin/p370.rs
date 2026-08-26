// Project Euler 370: Geometric triangles
// Primitive ratios m/n in lowest terms, n <= m < phi*n, k*(m^2+mn+n^2) <= L.
// Equilateral (m = n = 1) contribute L/3; remaining pairs via Möbius + floor sums.

use euler_utils::sieve_smallest_factor;
use rayon::prelude::*;

const L: i64 = 25_000_000_000_000;

#[inline(always)]
fn isqrt(n: i64) -> i64 {
    debug_assert!(n >= 0);
    // n <= 4L = 1e14 < 2^53, so f64 holds n exactly.
    let mut s = (n as f64).sqrt() as i64;
    let sq = s * s;
    if sq > n {
        s -= 1;
    } else if n - sq > s << 1 {
        s += 1;
    }
    s
}

/// Largest m with m^2 + m*n + n^2 <= t.
#[inline(always)]
fn m_from_perim(t: i64, n: i64, nn3: i64) -> i64 {
    let disc = 4 * t - nn3;
    if disc < 0 {
        return 0;
    }
    (isqrt(disc) - n) >> 1
}

/// Hardware 64÷32 unsigned div. Requires (n >> 32) < d so the quotient fits in u32.
#[inline(always)]
fn div64_by_u32(n: u64, d: u32) -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        let quot: u32;
        unsafe {
            core::arch::asm!(
                "div {divisor:e}",
                divisor = in(reg) d,
                inout("eax") (n as u32) => quot,
                inout("edx") ((n >> 32) as u32) => _,
                options(nostack, nomem),
            );
        }
        quot
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        (n / d as u64) as u32
    }
}

/// sum_{rp=1..=r_max} floor(L / (a + b rp + c rp^2)).
/// r_max is already clipped so every denominator is in 1..=L.
#[inline(always)]
fn sum_floor_linear(l: u64, a: i64, b: i64, c: i64, r_max: i64) -> i64 {
    let d1 = a + b + c;
    let d_last = a + b * r_max + c * r_max * r_max;
    // 64÷32 DIV requires high(L) < every denominator, i.e. high(L) < D(1).
    if d_last <= u32::MAX as i64 && (l >> 32) < d1 as u64 {
        let mut d = d1 as u32;
        let mut inc = (b + 3 * c) as u32;
        let mut sum = 0u64;
        let two_c = (c as u32) << 1;
        for _ in 0..r_max {
            sum += div64_by_u32(l, d) as u64;
            d = d.wrapping_add(inc);
            inc = inc.wrapping_add(two_c);
        }
        return sum as i64;
    }
    let mut d = (a + b + c) as u64;
    let mut inc = (b + 3 * c) as u64;
    let mut sum = 0u64;
    let two_c = (c as u64) << 1;
    for _ in 0..r_max {
        sum += l / d;
        d += inc;
        inc += two_c;
    }
    sum as i64
}

#[inline(always)]
fn coprime_count(divs: &[i64], mus: &[i64], nsub: usize, lo: i64, hi: i64) -> i64 {
    if hi < lo {
        return 0;
    }
    // m, q < 5e6 so these all fit in u32; 32-bit DIV is much faster.
    let hi = hi as u32;
    let lo1 = (lo - 1) as u32;
    let mut cnt = 0i64;
    for i in 0..nsub {
        let d = unsafe { *divs.get_unchecked(i) } as u32;
        let mu = unsafe { *mus.get_unchecked(i) };
        cnt += mu * (hi / d - lo1 / d) as i64;
    }
    cnt
}

/// Floor-sum over coprime m in (q, m_max] by jumping constant-quotient runs.
#[inline(always)]
fn sum_blocks(
    l: i64,
    q: i64,
    qq: i64,
    nn3: i64,
    m_max: i64,
    divs: &[i64],
    mus: &[i64],
    nsub: usize,
    walk: bool,
) -> i64 {
    let mut local = 0i64;
    let mut m = q + 1;
    while m <= m_max {
        let s = m * m + m * q + qq;
        let quot = l / s;
        let thigh = l / quot;
        let mend = if walk {
            let mut mm = m;
            loop {
                let nxt = mm + 1;
                if nxt > m_max {
                    break;
                }
                if nxt * nxt + nxt * q + qq > thigh {
                    break;
                }
                mm = nxt;
            }
            mm
        } else {
            let mut e = m_from_perim(thigh, q, nn3);
            if e > m_max {
                e = m_max;
            }
            if e < m {
                e = m;
            }
            e
        };
        if nsub == 1 {
            local += quot * (mend - m + 1);
        } else if mend == m {
            // Single m: Euclidean gcd beats a Möbius sweep.
            let mut a = m - q;
            let mut b = q;
            while a != 0 {
                let t = b % a;
                b = a;
                a = t;
            }
            if b == 1 {
                local += quot;
            }
        } else {
            local += quot * coprime_count(divs, mus, nsub, m, mend);
        }
        m = mend + 1;
    }
    local
}

fn contrib_q(q: i64, l: i64, spf: &[u32]) -> i64 {
    let qq = q * q;
    let nn3 = 3 * qq;
    let m_phi = (q + isqrt(5 * qq)) >> 1;
    // S(floor(phi q)) <= 5.236 q^2; if 6 q^2 <= L the perimeter bound is slack.
    let m_max = if 6 * qq <= l {
        m_phi
    } else {
        let m_perim = m_from_perim(l, q, nn3);
        if m_phi < m_perim { m_phi } else { m_perim }
    };
    if m_max <= q {
        return 0;
    }
    let r_max = m_max - q;

    let mut primes = [0i64; 8];
    let mut nf = 0usize;
    let mut x = q as usize;
    while x > 1 {
        let p = unsafe { *spf.get_unchecked(x) } as usize;
        unsafe {
            *primes.get_unchecked_mut(nf) = p as i64;
        }
        nf += 1;
        while x % p == 0 {
            x /= p;
        }
    }

    let s1 = nn3 + 3 * q + 1;
    let t_span = l / s1 - l / (m_max * m_max + m_max * q + qq) + 1;
    // Linear is one cheap division per r; blocks cost ~2 divs + isqrt + Möbius
    // per run. Prefer linear until T is several times smaller than R.
    let use_linear = t_span.saturating_mul(6) >= r_max;
    let walk = !use_linear && r_max < (t_span << 3);

    // Smallest prime factor > r_max ⇒ every r in 1..=r_max is coprime to q.
    if nf == 0 || unsafe { *primes.get_unchecked(0) } > r_max {
        if use_linear {
            return sum_floor_linear(l as u64, nn3, 3 * q, 1, r_max);
        }
        return sum_blocks(l, q, qq, nn3, m_max, &[1], &[1], 1, walk);
    }

    let nsub = 1usize << nf;
    if nsub <= 16 {
        let mut divs = [0i64; 16];
        let mut mus = [0i64; 16];
        expand_mu(&primes, nf, &mut divs, &mut mus);
        finish_q(
            l, q, qq, nn3, m_max, r_max, use_linear, walk, &divs, &mus, nsub,
        )
    } else {
        let mut divs = [0i64; 128];
        let mut mus = [0i64; 128];
        expand_mu(&primes, nf, &mut divs, &mut mus);
        finish_q(
            l, q, qq, nn3, m_max, r_max, use_linear, walk, &divs, &mus, nsub,
        )
    }
}

#[inline(always)]
fn expand_mu(primes: &[i64; 8], nf: usize, divs: &mut [i64], mus: &mut [i64]) {
    divs[0] = 1;
    mus[0] = 1;
    for i in 0..nf {
        let p = unsafe { *primes.get_unchecked(i) };
        let old = 1usize << i;
        for j in 0..old {
            unsafe {
                *divs.get_unchecked_mut(old + j) = *divs.get_unchecked(j) * p;
                *mus.get_unchecked_mut(old + j) = -*mus.get_unchecked(j);
            }
        }
    }
}

#[inline(always)]
fn finish_q(
    l: i64,
    q: i64,
    qq: i64,
    nn3: i64,
    m_max: i64,
    r_max: i64,
    use_linear: bool,
    walk: bool,
    divs: &[i64],
    mus: &[i64],
    nsub: usize,
) -> i64 {
    if use_linear {
        let mut local = 0i64;
        for i in 0..nsub {
            let d = unsafe { *divs.get_unchecked(i) };
            let r = r_max / d;
            if r < 1 {
                continue;
            }
            let sign = unsafe { *mus.get_unchecked(i) };
            local += sign * sum_floor_linear(l as u64, nn3, 3 * q * d, d * d, r);
        }
        local
    } else {
        sum_blocks(l, q, qq, nn3, m_max, divs, mus, nsub, walk)
    }
}

fn main() {
    let maxq = 2_900_000usize;
    let spf = sieve_smallest_factor(maxq);

    let mut q_max = isqrt(L / 3);
    while 3 * q_max * q_max + 3 * q_max + 1 > L {
        q_max -= 1;
    }
    if q_max > maxq as i64 {
        q_max = maxq as i64;
    }

    // Smaller chunks where work/q is largest (around the linear/block switch).
    let mut ranges = Vec::with_capacity(12_000);
    let mut q = 1i64;
    while q <= q_max {
        let sz = if q < 8_000 {
            16
        } else if q < 80_000 {
            8
        } else if q < 400_000 {
            64
        } else {
            512
        };
        let hi = (q + sz - 1).min(q_max);
        ranges.push((q, hi));
        q = hi + 1;
    }

    let total_parallel: i64 = ranges
        .into_par_iter()
        .map(|(lo, hi)| {
            let mut acc = 0i64;
            for qq in lo..=hi {
                acc += contrib_q(qq, L, &spf);
            }
            acc
        })
        .sum();

    println!("{}", L / 3 + total_parallel);
}
