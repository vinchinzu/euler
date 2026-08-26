// Project Euler 852 - Coins in a Box
// V(f,u) = G(p) + p V(f-1,u) + (1-p) V(f,u-1); G is one-coin optimal stopping.
// Posterior odds depend only on the reduced prior (f:u) and 3^h / 2^n.

use rayon::prelude::*;

const N: usize = 50;
const MAX_TOSS: usize = 300;
const REWARD: f64 = 20.0;
const COST: f64 = 1.0;

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

// 2^{-n} exactly (n <= 1022).
#[inline(always)]
fn pow_half(n: usize) -> f64 {
    f64::from_bits(((1023 - n) as u64) << 52)
}

fn expected_gain(fair: usize, unfair: usize, prev: &mut Vec<f64>, curr: &mut Vec<f64>) -> f64 {
    if fair == 0 || unfair == 0 {
        return REWARD;
    }

    let fair_f = fair as f64;
    let unfair_f = unfair as f64;

    let mut l = pow_half(MAX_TOSS);
    for h in 0..=MAX_TOSS {
        let pf = fair_f / (fair_f + unfair_f * l);
        let m = if pf > 0.5 { pf } else { 1.0 - pf };
        // SAFETY: h <= MAX_TOSS, prev.len() == MAX_TOSS + 2
        unsafe {
            *prev.get_unchecked_mut(h) = 70.0 * m - 50.0;
        }
        l *= 3.0;
    }

    for total in (0..MAX_TOSS).rev() {
        let mut l = pow_half(total);
        for h in 0..=total {
            let pf = fair_f / (fair_f + unfair_f * l);
            let m = if pf > 0.5 { pf } else { 1.0 - pf };
            let best_guess = 70.0 * m - 50.0;
            let p_heads = 0.75 - 0.25 * pf;
            // SAFETY: h <= total < MAX_TOSS => h + 1 <= MAX_TOSS; buffers length MAX_TOSS + 2
            unsafe {
                let val_h = *prev.get_unchecked(h + 1);
                let val_t = *prev.get_unchecked(h);
                let ev_toss = p_heads.mul_add(val_h - val_t, val_t) - COST;
                *curr.get_unchecked_mut(h) = if best_guess > ev_toss {
                    best_guess
                } else {
                    ev_toss
                };
            }
            l *= 3.0;
        }
        std::mem::swap(prev, curr);
    }
    prev[0]
}

fn main() {
    let stride = N + 1;

    let mut keys = Vec::with_capacity(stride * stride);
    let mut seen = vec![false; stride * stride];
    for f in 0..=N {
        for u in 0..=N {
            if f == 0 && u == 0 {
                continue;
            }
            let g = gcd(f, u);
            let rf = f / g;
            let ru = u / g;
            let idx = rf * stride + ru;
            if !seen[idx] {
                seen[idx] = true;
                keys.push((rf, ru));
            }
        }
    }

    let gains: Vec<((usize, usize), f64)> = keys
        .into_par_iter()
        .map_with(
            (vec![0.0f64; MAX_TOSS + 2], vec![0.0f64; MAX_TOSS + 2]),
            |bufs, (fair, unfair)| {
                let g = expected_gain(fair, unfair, &mut bufs.0, &mut bufs.1);
                ((fair, unfair), g)
            },
        )
        .collect();

    let mut g_table = vec![0.0f64; stride * stride];
    for ((fair, unfair), g) in gains {
        g_table[fair * stride + unfair] = g;
    }

    let mut v = vec![0.0f64; stride * stride];
    for total in 1..=2 * N {
        let start_fair = total.saturating_sub(N);
        let end_fair = total.min(N);
        for fair in start_fair..=end_fair {
            let unfair = total - fair;
            let d = gcd(fair, unfair);
            let g = g_table[(fair / d) * stride + unfair / d];
            let pf = fair as f64 / total as f64;
            let fut_f = if fair > 0 {
                v[(fair - 1) * stride + unfair]
            } else {
                0.0
            };
            let fut_u = if unfair > 0 {
                v[fair * stride + unfair - 1]
            } else {
                0.0
            };
            v[fair * stride + unfair] = g + pf * fut_f + (1.0 - pf) * fut_u;
        }
    }

    println!("{:.6}", v[N * stride + N]);
}
