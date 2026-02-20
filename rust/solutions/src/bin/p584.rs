// Project Euler 584 - Birthday Problem Revisited
//
// Expected number of people before K=4 share birthdays within 7 days (D=365 days).
// Uses transfer matrix exponentiation with polynomial coefficients.

use rayon::prelude::*;

const D: usize = 365;
const W: usize = 8;        // "within 7 days" = window of 8 consecutive days
const STATE_LEN: usize = 7; // W-1
const MAX_OCC: usize = 3;  // K-1
const MAX_N: usize = 100;  // truncation for polynomial

fn tuple_to_index(c: &[usize]) -> usize {
    let mut idx = 0;
    for i in 0..STATE_LEN {
        idx = idx * 4 + c[i];
    }
    idx
}

fn gen_states_rec(
    pos: usize, remaining: usize, c: &mut [usize],
    states: &mut Vec<[usize; STATE_LEN]>,
    state_map: &mut [i32],
) {
    if pos == STATE_LEN {
        let idx = tuple_to_index(c);
        state_map[idx] = states.len() as i32;
        let mut s = [0usize; STATE_LEN];
        s.copy_from_slice(&c[..STATE_LEN]);
        states.push(s);
        return;
    }
    for v in 0..=remaining {
        c[pos] = v;
        gen_states_rec(pos + 1, remaining - v, c, states, state_map);
    }
}

fn main() {
    let mut states: Vec<[usize; STATE_LEN]> = Vec::new();
    let mut state_map = vec![-1i32; 16384]; // 4^7
    let mut c = [0usize; STATE_LEN];
    gen_states_rec(0, MAX_OCC, &mut c, &mut states, &mut state_map);

    let ns = states.len();
    let plen = MAX_N + 1;
    let inv_fact = [1.0, 1.0, 0.5, 1.0 / 6.0];

    // Build transfer matrix T
    let mut t_mat = vec![0.0f64; ns * ns * plen];

    for si in 0..ns {
        let s_sum: usize = states[si].iter().sum();
        for c_new in 0..=(MAX_OCC - s_sum) {
            let mut nc = [0usize; STATE_LEN];
            for k in 0..STATE_LEN - 1 {
                nc[k] = states[si][k + 1];
            }
            nc[STATE_LEN - 1] = c_new;
            let mi = tuple_to_index(&nc);
            let sj = state_map[mi];
            if sj >= 0 {
                let idx = (si * ns + sj as usize) * plen + c_new;
                t_mat[idx] += inv_fact[c_new];
            }
        }
    }

    // Matrix exponentiation: compute T^D
    let mut result = vec![0.0f64; ns * ns * plen];
    for i in 0..ns {
        result[(i * ns + i) * plen] = 1.0;
    }

    let mut base = t_mat.clone();
    let mut temp = vec![0.0f64; ns * ns * plen];

    let mut power = D;
    while power > 0 {
        if power & 1 == 1 {
            mat_mul(&result, &base, &mut temp, ns, plen);
            std::mem::swap(&mut result, &mut temp);
        }
        if power > 1 {
            mat_mul(&base.clone(), &base, &mut temp, ns, plen);
            std::mem::swap(&mut base, &mut temp);
        }
        power >>= 1;
    }

    // Trace
    let mut trace = vec![0.0f64; plen];
    for i in 0..ns {
        for k in 0..plen {
            trace[k] += result[(i * ns + i) * plen + k];
        }
    }

    // E = sum n! / D^n * trace[n]
    let mut e = 0.0f64;
    let mut n_fact = 1.0f64;
    let mut d_pow = 1.0f64;

    for n in 0..=MAX_N {
        if n > 0 {
            n_fact *= n as f64;
            d_pow *= D as f64;
        }
        let p = n_fact / d_pow * trace[n];
        e += p;
        if n > 50 && p.abs() < 1e-15 { break; }
    }

    println!("{:.8}", e);
}

fn mat_mul(a: &[f64], b: &[f64], c: &mut [f64], ns: usize, plen: usize) {
    // Parallelize over rows (i)
    let row_size = ns * plen;
    c.par_chunks_mut(row_size).enumerate().for_each(|(i, c_row)| {
        c_row.iter_mut().for_each(|x| *x = 0.0);

        for k in 0..ns {
            let a_start = (i * ns + k) * plen;
            // Check if a row is zero
            let nonzero = a[a_start..a_start + plen].iter().any(|&x| x != 0.0);
            if !nonzero { continue; }

            for j in 0..ns {
                let b_start = (k * ns + j) * plen;
                let c_start = j * plen;
                for p in 0..plen {
                    if a[a_start + p] == 0.0 { continue; }
                    let ap = a[a_start + p];
                    for q in 0..plen - p {
                        c_row[c_start + p + q] += ap * b[b_start + q];
                    }
                }
            }
        }
    });
}
