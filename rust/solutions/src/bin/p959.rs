// Project Euler 959 - Frog on number line
// f(89, 97) = rate of new sites visited via random walk return probability

use std::f64::consts::PI;

fn binom_prob(n: i32, k: i32) -> f64 {
    if k < 0 || k > n { return 0.0; }
    let lp = lgamma(n as f64 + 1.0) - lgamma(k as f64 + 1.0)
           - lgamma((n - k) as f64 + 1.0) - n as f64 * 2.0f64.ln();
    lp.exp()
}

fn lgamma(x: f64) -> f64 {
    // Use the C library lgamma via approximation or Stirling
    // Rust doesn't have lgamma in std, so we implement it
    // using Lanczos approximation
    if x <= 0.0 { return f64::INFINITY; }
    let g = 7.0;
    let c = [
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];
    if x < 0.5 {
        let s = PI / (PI * x).sin();
        return s.ln() - lgamma(1.0 - x);
    }
    let x = x - 1.0;
    let mut a = c[0];
    let t = x + g + 0.5;
    for i in 1..9 {
        a += c[i] / (x + i as f64);
    }
    0.5 * (2.0 * PI).ln() + (t.ln()) * (x + 0.5) - t + a.ln()
}

const RANGE: usize = 1000;
const N: usize = 2 * RANGE;

fn main() {
    let a = 89i32;
    let b = 97i32;
    let p = a + b; // 186

    // Build state list: -RANGE..-1, 1..RANGE
    let mut states = vec![0i32; N];
    let mut idx = 0;
    for x in -(RANGE as i32)..0 {
        states[idx] = x;
        idx += 1;
    }
    for x in 1..=RANGE as i32 {
        states[idx] = x;
        idx += 1;
    }

    let state_idx = |x: i32| -> usize {
        if x < 0 { (x + RANGE as i32) as usize }
        else { RANGE + x as usize - 1 }
    };

    // Compute transition probabilities
    let z_min = -a;
    let z_max = b;
    let z_count = (z_max - z_min + 1) as usize;
    let mut prob = vec![0.0f64; z_count];
    for z in z_min..=z_max {
        let k = z + a;
        prob[(z - z_min) as usize] = binom_prob(p, k);
    }

    // Build banded linear system (I - T)h = e_0
    // Store as dense N x N for the banded solver
    let mut a_mat = vec![0.0f64; N * N];
    let mut b_vec = vec![0.0f64; N];

    // Initialize A = I
    for i in 0..N {
        a_mat[i * N + i] = 1.0;
    }

    // Build system
    for i in 0..N {
        let x = states[i];
        for zi in 0..z_count {
            let z = z_min + zi as i32;
            let pp = prob[zi];
            if pp == 0.0 { continue; }
            let next_x = x + z;
            if next_x == 0 {
                b_vec[i] += pp;
            } else if next_x >= -(RANGE as i32) && next_x <= RANGE as i32 && next_x != 0 {
                let j = state_idx(next_x);
                a_mat[i * N + j] -= pp;
            }
        }
    }

    // Banded Gaussian elimination with partial pivoting
    let bw = p as usize;
    for k in 0..N {
        // Partial pivoting within band
        let mut max_row = k;
        let mut max_val = a_mat[k * N + k].abs();
        let search_end = (k + bw).min(N - 1);
        for r in (k + 1)..=search_end {
            let v = a_mat[r * N + k].abs();
            if v > max_val {
                max_val = v;
                max_row = r;
            }
        }
        if max_row != k {
            // Swap rows
            let col_end = (k + 2 * bw).min(N - 1);
            for j in k..=col_end {
                let tmp = a_mat[k * N + j];
                a_mat[k * N + j] = a_mat[max_row * N + j];
                a_mat[max_row * N + j] = tmp;
            }
            b_vec.swap(k, max_row);
        }
        // Eliminate
        let pivot = a_mat[k * N + k];
        for r in (k + 1)..=search_end {
            let factor = a_mat[r * N + k] / pivot;
            if factor == 0.0 { continue; }
            a_mat[r * N + k] = 0.0;
            let col_end = (k + 2 * bw).min(N - 1);
            for j in (k + 1)..=col_end {
                a_mat[r * N + j] -= factor * a_mat[k * N + j];
            }
            b_vec[r] -= factor * b_vec[k];
        }
    }

    // Back substitution
    let mut h = vec![0.0f64; N];
    for i in (0..N).rev() {
        let mut s = b_vec[i];
        let col_end = (i + 2 * bw).min(N - 1);
        for j in (i + 1)..=col_end {
            s -= a_mat[i * N + j] * h[j];
        }
        h[i] = s / a_mat[i * N + i];
    }

    // Compute f = sum_z prob(z) * (1 - h(z)) for z != 0
    let mut f = 0.0f64;
    for zi in 0..z_count {
        let z = z_min + zi as i32;
        if z == 0 { continue; }
        let pp = prob[zi];
        if pp == 0.0 { continue; }
        if z >= -(RANGE as i32) && z <= RANGE as i32 {
            let j = state_idx(z);
            f += pp * (1.0 - h[j]);
        } else {
            f += pp;
        }
    }

    println!("{:.9}", f);
}
