use rayon::prelude::*;

const N: i64 = 10_000_000;
const M: i64 = 100;
const R: i64 = M - 1;
const RU: usize = R as usize;

fn main() {
    let n_total = N - 1;
    let mut inv_den = [0.0f64; RU];
    for i in 0..RU {
        inv_den[i] = 1.0 / (n_total - i as i64) as f64;
    }

    // E[Y] = sum_{k>=1} P(Y >= k), Y = second-shortest segment length.
    // P(Y >= k) = [M * C(N2, R) - (M-1) * C(N1, R)] / C(N-1, R)
    // N2 = N - R*k + (M-2), N1 = N - M*k + (M-1)
    let kmax = n_total / R;
    let nthreads = rayon::current_num_threads().max(1) as i64;

    let ans: f64 = (0..nthreads)
        .into_par_iter()
        .map(|t| {
            let start = 1 + kmax * t / nthreads;
            let end = 1 + kmax * (t + 1) / nthreads;
            let mut local = 0.0;
            for k in start..end {
                let n2 = N - R * k + (M - 2);
                let n1 = N - M * k + (M - 1);
                local += term(n1, n2, &inv_den);
            }
            local
        })
        .sum();

    println!("{:.5}", ans);
}

#[inline]
fn term(n1: i64, n2: i64, inv_den: &[f64; RU]) -> f64 {
    let mut c2a = 1.0;
    let mut c2b = 1.0;
    if n1 >= R {
        let mut c1a = 1.0;
        let mut c1b = 1.0;
        let mut i = 0;
        while i + 1 < RU {
            // SAFETY: i, i+1 < RU
            let inv0 = unsafe { *inv_den.get_unchecked(i) };
            let inv1 = unsafe { *inv_den.get_unchecked(i + 1) };
            let ii = i as i64;
            c2a *= (n2 - ii) as f64 * inv0;
            c2b *= (n2 - ii - 1) as f64 * inv1;
            c1a *= (n1 - ii) as f64 * inv0;
            c1b *= (n1 - ii - 1) as f64 * inv1;
            i += 2;
        }
        let invl = unsafe { *inv_den.get_unchecked(RU - 1) };
        c2a *= (n2 - (RU as i64 - 1)) as f64 * invl;
        c1a *= (n1 - (RU as i64 - 1)) as f64 * invl;
        (M as f64) * (c2a * c2b) - (R as f64) * (c1a * c1b)
    } else {
        let mut i = 0;
        while i + 1 < RU {
            let inv0 = unsafe { *inv_den.get_unchecked(i) };
            let inv1 = unsafe { *inv_den.get_unchecked(i + 1) };
            let ii = i as i64;
            c2a *= (n2 - ii) as f64 * inv0;
            c2b *= (n2 - ii - 1) as f64 * inv1;
            i += 2;
        }
        let invl = unsafe { *inv_den.get_unchecked(RU - 1) };
        c2a *= (n2 - (RU as i64 - 1)) as f64 * invl;
        (M as f64) * (c2a * c2b)
    }
}
