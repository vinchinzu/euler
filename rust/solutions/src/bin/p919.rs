// Project Euler 919 - Fortunate Triangles
// S(P) = sum of a+b+c over all fortunate triangles with perimeter <= P.
// Two generators based on quadratic forms.

use rayon::prelude::*;

const LIMIT: i64 = 10_000_000;

#[inline(always)]
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

#[inline(always)]
fn consider(a: i64, b: i64, c: i64, gbc: i64, out: &mut Vec<(u32, u32, u32)>) {
    if a <= 0 {
        return;
    }
    let s = a + b + c;
    let m = a.max(b).max(c);
    if m * 2 >= s {
        return;
    }
    let g = gcd(a, gbc);
    if s / g > LIMIT {
        return;
    }
    let mut pa = a / g;
    let mut pb = b / g;
    let mut pc = c / g;
    if pa > pb {
        std::mem::swap(&mut pa, &mut pb);
    }
    if pb > pc {
        std::mem::swap(&mut pb, &mut pc);
    }
    if pa > pb {
        std::mem::swap(&mut pa, &mut pb);
    }
    out.push((pa as u32, pb as u32, pc as u32));
}

#[inline(always)]
fn emit_g1_mixed(u: i64, v: i64, v15: i64, out: &mut Vec<(u32, u32, u32)>) {
    let u2 = u * u;
    let uv2 = 2 * u * v;
    let c = u2 + v15;
    let b = 8 * u * v;
    let gbc = gcd(b, c);
    consider((v15 - u2 + uv2).abs(), b, c, gbc, out);
    consider((v15 - u2 - uv2).abs(), b, c, gbc, out);
}

#[inline(always)]
fn emit_g1_odd(u: i64, v: i64, v15: i64, out: &mut Vec<(u32, u32, u32)>) {
    let u2 = u * u;
    let uv2 = 2 * u * v;
    let c = (u2 + v15) / 4;
    let b = uv2;
    let gbc = gcd(b, c);
    consider(((v15 - u2 + uv2) / 4).abs(), b, c, gbc, out);
    consider(((v15 - u2 - uv2) / 4).abs(), b, c, gbc, out);
}

#[inline(always)]
fn emit_g2_mixed(u: i64, v: i64, v5: i64, out: &mut Vec<(u32, u32, u32)>) {
    let u2_3 = 3 * u * u;
    let uv2 = 2 * u * v;
    let c = u2_3 + v5;
    let b = 8 * u * v;
    let gbc = gcd(b, c);
    consider((v5 - u2_3 + uv2).abs(), b, c, gbc, out);
    consider((v5 - u2_3 - uv2).abs(), b, c, gbc, out);
}

#[inline(always)]
fn emit_g2_odd(u: i64, v: i64, v5: i64, out: &mut Vec<(u32, u32, u32)>) {
    let u2_3 = 3 * u * u;
    let uv2 = 2 * u * v;
    let c = (u2_3 + v5) / 4;
    let b = uv2;
    let gbc = gcd(b, c);
    consider(((v5 - u2_3 + uv2) / 4).abs(), b, c, gbc, out);
    consider(((v5 - u2_3 - uv2) / 4).abs(), b, c, gbc, out);
}

fn gen1_v(v: i64, max_u: i64, out: &mut Vec<(u32, u32, u32)>) {
    let v15 = 15 * v * v;
    if v & 1 == 0 {
        for u in (1..max_u).step_by(2) {
            if gcd(u, v) != 1 {
                continue;
            }
            emit_g1_mixed(u, v, v15, out);
        }
    } else {
        for u in (2..max_u).step_by(2) {
            if gcd(u, v) != 1 {
                continue;
            }
            emit_g1_mixed(u, v, v15, out);
        }
        for u in (1..max_u).step_by(2) {
            if gcd(u, v) != 1 {
                continue;
            }
            emit_g1_odd(u, v, v15, out);
        }
    }
}

fn gen2_v(v: i64, max_u: i64, out: &mut Vec<(u32, u32, u32)>) {
    let v5 = 5 * v * v;
    if v & 1 == 0 {
        for u in (1..max_u).step_by(2) {
            if gcd(u, v) != 1 {
                continue;
            }
            emit_g2_mixed(u, v, v5, out);
        }
    } else {
        for u in (2..max_u).step_by(2) {
            if gcd(u, v) != 1 {
                continue;
            }
            emit_g2_mixed(u, v, v5, out);
        }
        for u in (1..max_u).step_by(2) {
            if gcd(u, v) != 1 {
                continue;
            }
            emit_g2_odd(u, v, v5, out);
        }
    }
}

fn collect_gen(
    max_v: i64,
    max_u: i64,
    emit_v: impl Fn(i64, i64, &mut Vec<(u32, u32, u32)>) + Sync,
) -> Vec<(u32, u32, u32)> {
    (1..max_v)
        .into_par_iter()
        .fold(
            || Vec::with_capacity(1 << 18),
            |mut acc, v| {
                emit_v(v, max_u, &mut acc);
                acc
            },
        )
        .reduce(Vec::new, |mut a, mut b| {
            if a.len() < b.len() {
                std::mem::swap(&mut a, &mut b);
            }
            a.append(&mut b);
            a
        })
}

fn main() {
    let max_v1 = ((2.5 * LIMIT as f64 / 15.0).sqrt() as i64) + 2;
    let max_u1 = ((2.5 * LIMIT as f64).sqrt() as i64) + 2;
    let max_v2 = ((2.5 * LIMIT as f64 / 5.0).sqrt() as i64) + 2;
    let max_u2 = ((2.5 * LIMIT as f64 / 3.0).sqrt() as i64) + 2;

    let (mut t1, mut t2) = rayon::join(
        || collect_gen(max_v1, max_u1, gen1_v),
        || collect_gen(max_v2, max_u2, gen2_v),
    );
    if t1.len() < t2.len() {
        std::mem::swap(&mut t1, &mut t2);
    }
    t1.append(&mut t2);
    t1.par_sort_unstable();
    t1.dedup();

    let ans: i64 = t1
        .par_iter()
        .map(|&(a, b, c)| {
            let p = a as i64 + b as i64 + c as i64;
            let count = LIMIT / p;
            p * count * (count + 1) / 2
        })
        .sum();

    println!("{}", ans);
}
