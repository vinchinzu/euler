// Project Euler 414: Kaprekar cycles
use rayon::prelude::*;

const MOD: u128 = 1_000_000_000_000_000_000;
const N: i32 = 300;

/// Next gap pair (u', v') after one 5-digit Kaprekar step in base `b`.
///
/// The unsorted base-b digits of the Kaprekar difference are
///   v = 0: (u-1, b-1, b-1, b-1, b-u)
///   v > 0: (u, v-1, b-1, b-v-1, b-u)
/// and the next state is (max-min, 2ndmax-2ndmin). For v > 0 this reduces to
/// comparisons among x = min(u, b-u) and y = min(v-1, b-v-1).
#[inline(always)]
fn next_uv(b: i32, u: i32, v: i32) -> (i32, i32) {
    if v == 0 {
        let a = u - 1;
        let e = b - u;
        if a <= e {
            (b - 1 - a, b - 1 - e)
        } else {
            (b - 1 - e, b - 1 - a)
        }
    } else {
        let x = if u <= b - u { u } else { b - u };
        let y = if v - 1 <= b - v - 1 { v - 1 } else { b - v - 1 };
        if x <= y {
            (b - 1 - x, b - x - y)
        } else {
            let nd1 = b - 1 - y;
            let nd2 = if x <= y + 2 { b - 2 * x } else { b - 2 - y - x };
            (nd1, nd2)
        }
    }
}

fn s_base(b: i32) -> u128 {
    let bu = b as usize;
    let mut dist = vec![0u32; bu * bu];
    // Kaprekar constant: (u, v) = (2b/3, b/3). sb = 1 here; we subtract 1 later
    // so the constant itself contributes 0.
    dist[(2 * bu / 3) * bu + bu / 3] = 1;

    let mut path = Vec::with_capacity(128);
    let mut total = 0u128;

    for u in 1..bu {
        let bp = (bu - u) as u128;
        let uu = u as u128;
        let row = u * bu;
        for v in 0..=u {
            let start = row + v;
            // SAFETY: u < b, v <= u, so start = u*b+v < b*b.
            if unsafe { *dist.get_unchecked(start) } == 0 {
                path.clear();
                let mut cur = start;
                let mut cu = u as i32;
                let mut cv = v as i32;
                loop {
                    // SAFETY: next_uv stays in 0 <= nv <= nu < b, so cur < b*b.
                    if unsafe { *dist.get_unchecked(cur) } != 0 {
                        break;
                    }
                    path.push(cur);
                    let (nu, nv) = next_uv(b, cu, cv);
                    debug_assert!(nu >= 0 && nv >= 0 && nv <= nu && (nu as usize) < bu);
                    cu = nu;
                    cv = nv;
                    cur = (nu as usize) * bu + nv as usize;
                }
                // SAFETY: cur is a previously visited / base state with dist != 0.
                let mut d = unsafe { *dist.get_unchecked(cur) };
                for &node in path.iter().rev() {
                    d += 1;
                    unsafe {
                        *dist.get_unchecked_mut(node) = d;
                    }
                }
            }
            // SAFETY: start in range; dist[start] now filled.
            let s = unsafe { *dist.get_unchecked(start) } as u128;
            let inner = if v == 0 {
                20 * uu - 10
            } else if v == u {
                30 * uu - 10
            } else {
                120 * (v as u128) * (uu - v as u128) - 20
            };
            total += bp * inner * s;
        }
    }
    (total + MOD - 1) % MOD
}

fn main() {
    // Independent bases, largest first so rayon grabs the heavy k's immediately.
    let ans: u128 = (0..(N - 1))
        .into_par_iter()
        .map(|i| s_base(6 * (N - i) + 3))
        .sum();
    println!("{}", ans % MOD);
}
