// Project Euler 223: Almost right-angled triangles I
// Count a <= b <= c, a^2 + b^2 = c^2 + 1, perimeter <= 25_000_000.
//
// Rewrite c^2 - b^2 = a^2 - 1 as uv = a^2 - 1 with u = b + c, v = c - b.
// For each v, solutions a^2 ≡ 1 (mod v) form APs; count a in the interval
// implied by a <= b and a + u <= N. The a = 1 family (1, k, k) is closed form.

use rayon::prelude::*;

const N: i64 = 25_000_000;

fn main() {
    println!("{}", solve(N));
}

fn solve(n: i64) -> i64 {
    // v <= a (√2 - 1) and a <= n / (2 + √2) ⇒ v ≲ 0.1213 n. Margin for rounding.
    let vmax = n * 13 / 100 + 2048;
    let spf = sieve_spf(vmax as usize);

    // (1, k, k) for 1 + 2k <= n.
    let mut ans = (n - 1) / 2;

    let nchunks = ((vmax as usize) + 4095) / 4096;
    ans += (0..nchunks)
        .into_par_iter()
        .map(|chunk| {
            let mut cur = [0i64; 256];
            let mut nxt = [0i64; 256];
            let start = (chunk * 4096 + 1) as i64;
            let end = ((chunk * 4096 + 4096) as i64).min(vmax);
            let mut local = 0i64;
            let mut v = start;
            while v <= end {
                local += count_v(v, n, &spf, &mut cur, &mut nxt);
                v += 1;
            }
            local
        })
        .sum::<i64>();

    ans
}

fn sieve_spf(limit: usize) -> Vec<u32> {
    let mut spf: Vec<u32> = (0..=limit as u32).collect();
    let s = (limit as f64).sqrt() as usize;
    let mut i = 2;
    while i <= s {
        if spf[i] == i as u32 {
            let mut j = i * i;
            while j <= limit {
                if spf[j] == j as u32 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
        i += 1;
    }
    spf
}

/// Smallest a with (a - v)^2 >= 2 v^2 + 1 (i.e. a <= b).
#[inline(always)]
fn min_a(v: i64) -> i64 {
    let need = 2 * v * v + 1;
    let s = (need as u64).isqrt() as i64;
    let k = if s * s < need { s + 1 } else { s };
    v + k
}

/// Largest a with a (a + v) <= v n + 1 (i.e. perimeter <= n).
#[inline(always)]
fn max_a(v: i64, n: i64) -> i64 {
    let rhs = v * n + 1;
    let disc = v * v + 4 * rhs;
    let sd = (disc as u64).isqrt() as i64;
    let mut a = (sd - v) >> 1;
    if a < 0 {
        return -1;
    }
    while a * (a + v) > rhs {
        a -= 1;
    }
    while (a + 1) * (a + 1 + v) <= rhs {
        a += 1;
    }
    a
}

#[inline(always)]
fn count_ap(lo: i64, hi: i64, residue: i64, modulus: i64) -> i64 {
    if lo > hi {
        return 0;
    }
    let first = lo + (residue - lo).rem_euclid(modulus);
    if first > hi {
        0
    } else {
        (hi - first) / modulus + 1
    }
}

#[inline(always)]
fn mod_inv(mut a: i64, m: i64) -> i64 {
    // Extended Euclid; a, m coprime, returns 0..m-1.
    a %= m;
    if a < 0 {
        a += m;
    }
    let (mut t, mut nt) = (0i64, 1i64);
    let (mut r, mut nr) = (m, a);
    while nr != 0 {
        let q = r / nr;
        let tmp = nt;
        nt = t - q * nt;
        t = tmp;
        let tmp = nr;
        nr = r - q * nr;
        r = tmp;
    }
    if t < 0 {
        t += m;
    }
    t
}

#[inline(always)]
fn crt_combine(
    res: &[i64; 256],
    nres: usize,
    m1: i64,
    local: &[i64],
    nloc: usize,
    pe: i64,
    out: &mut [i64; 256],
) -> usize {
    let inv = mod_inv(m1 % pe, pe);
    let mut kout = 0usize;
    let mut i = 0;
    while i < nres {
        let r = res[i];
        let mut j = 0;
        while j < nloc {
            let s = local[j];
            let k = ((s - r).rem_euclid(pe) * inv) % pe;
            out[kout] = r + m1 * k;
            kout += 1;
            j += 1;
        }
        i += 1;
    }
    kout
}

fn count_v(v: i64, n: i64, spf: &[u32], buf0: &mut [i64; 256], buf1: &mut [i64; 256]) -> i64 {
    let a_lo = min_a(v);
    let a_hi = max_a(v, n);
    if a_lo > a_hi {
        return 0;
    }

    // Residues of x^2 ≡ 1 (mod v), built from prime-power solutions via CRT.
    // `side == 0` ⇒ live residues in buf0.
    buf0[0] = 0;
    let mut clen = 1usize;
    let mut modulus = 1i64;
    let mut side = 0u8;
    let mut x = v as u32;

    if x & 1 == 0 {
        let mut e = 0u32;
        while x & 1 == 0 {
            x >>= 1;
            e += 1;
        }
        let pe = 1i64 << e;
        let h = 1i64 << e.saturating_sub(1);
        let (local, nloc): ([i64; 4], usize) = match e {
            1 => ([1, 0, 0, 0], 1),
            2 => ([1, 3, 0, 0], 2),
            _ => ([1, pe - 1, h - 1, h + 1], 4),
        };
        clen = if side == 0 {
            crt_combine(buf0, clen, modulus, &local[..nloc], nloc, pe, buf1)
        } else {
            crt_combine(buf1, clen, modulus, &local[..nloc], nloc, pe, buf0)
        };
        side ^= 1;
        modulus *= pe;
    }

    while x > 1 {
        let p = spf[x as usize];
        let mut e = 0u32;
        while x % p == 0 {
            x /= p;
            e += 1;
        }
        let mut pe = p as i64;
        let mut k = 1u32;
        while k < e {
            pe *= p as i64;
            k += 1;
        }
        let local = [1i64, pe - 1];
        clen = if side == 0 {
            crt_combine(buf0, clen, modulus, &local, 2, pe, buf1)
        } else {
            crt_combine(buf1, clen, modulus, &local, 2, pe, buf0)
        };
        side ^= 1;
        modulus *= pe;
    }

    let src = if side == 0 { &*buf0 } else { &*buf1 };
    let mut total = 0i64;
    let mut i = 0;
    if v & 1 == 0 {
        while i < clen {
            let r = src[i];
            // Need (r^2 - 1)/v ≡ v (mod 2). For even v this is independent of the AP index.
            if r > 0 {
                let u0 = (r * r - 1) / v;
                if (u0 & 1) == 0 {
                    total += count_ap(a_lo, a_hi, r, v);
                }
            }
            i += 1;
        }
    } else {
        // v odd ⇒ a must be even (so u is odd too).
        let m = v << 1;
        while i < clen {
            let r = src[i];
            let even_r = if r & 1 == 0 { r } else { r + v };
            total += count_ap(a_lo, a_hi, even_r, m);
            i += 1;
        }
    }
    total
}

#[cfg(test)]
fn tree_count(n: i64) -> i64 {
    let mut stack = vec![(1i64, 1i64, 1i64), (1, 2, 2)];
    let mut ans = 0i64;
    while let Some((a, b, c)) = stack.pop() {
        if a + b + c <= n {
            ans += 1;
            stack.push((a - 2 * b + 2 * c, 2 * a - b + 2 * c, 2 * a - 2 * b + 3 * c));
            if a != b {
                stack.push((-a + 2 * b + 2 * c, -2 * a + b + 2 * c, -2 * a + 2 * b + 3 * c));
            }
            stack.push((a + 2 * b + 2 * c, 2 * a + b + 2 * c, 2 * a + 2 * b + 3 * c));
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_tree_small() {
        for n in [10, 20, 30, 50, 100, 200, 300, 500, 1000, 2000, 5000, 10000] {
            assert_eq!(solve(n), tree_count(n), "n={n}");
        }
    }
}
