use rayon::prelude::*;

const NN: usize = 20_000_000;
const M: i64 = 10_000_000;

#[inline(always)]
fn l1(v1: i64, v2: i64, v3: i64) -> i64 {
    v1.abs() + v2.abs() + v3.abs()
}

#[inline(always)]
fn gauss(u1: i64, u2: i64, u3: i64, v1: i64, v2: i64, v3: i64) -> i64 {
    let u_l1 = l1(u1, u2, u3);
    let mut candidates = [0i64; 6];
    let mut n_cand = 0usize;
    let comps = [(u1, v1), (u2, v2), (u3, v3)];
    for &(u, v) in &comps {
        if u != 0 {
            let q = v as f64 / u as f64;
            candidates[n_cand] = q.floor() as i64;
            candidates[n_cand + 1] = q.ceil() as i64;
        } else {
            candidates[n_cand] = 0;
            candidates[n_cand + 1] = 0;
        }
        n_cand += 2;
    }

    let mut min_w = (0i64, 0i64, 0i64);
    let mut min_d = i64::MAX;

    for &m in &candidates[..n_cand] {
        if u_l1 > 0 && m.unsigned_abs() < u64::MAX / std::cmp::max(u_l1 as u64, 1) {
            let w1 = v1 - m * u1;
            let w2 = v2 - m * u2;
            let w3 = v3 - m * u3;
            let d = l1(w1, w2, w3);
            if d < min_d {
                min_w = (w1, w2, w3);
                min_d = d;
            }
        }
    }

    if l1(min_w.0, min_w.1, min_w.2) < u_l1 {
        gauss(min_w.0, min_w.1, min_w.2, u1, u2, u3)
    } else {
        u_l1
    }
}

type Mat3 = [[i64; 3]; 3];

fn mat_mul(a: &Mat3, b: &Mat3) -> Mat3 {
    let mut c = [[0i64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let mut s = 0i64;
            for k in 0..3 {
                s += a[i][k] * b[k][j];
            }
            c[i][j] = s % M;
        }
    }
    c
}

fn mat_pow(mut base: Mat3, mut p: usize) -> Mat3 {
    let mut res = [[0i64; 3]; 3];
    for i in 0..3 {
        res[i][i] = 1;
    }
    while p > 0 {
        if p & 1 == 1 {
            res = mat_mul(&res, &base);
        }
        base = mat_mul(&base, &base);
        p >>= 1;
    }
    res
}

// Get (r[k], r[k-1], r[k-2]) for k >= 2
fn get_tribonacci_state(k: usize) -> (i64, i64, i64) {
    if k == 2 {
        return (1, 0, 0);
    }
    let t: Mat3 = [[1, 1, 1], [1, 0, 0], [0, 1, 0]];
    let tk = mat_pow(t, k - 2);
    // State is Tk * [r2, r1, r0]^T = Tk * [1, 0, 0]^T = first column of Tk
    (tk[0][0], tk[1][0], tk[2][0])
}

fn main() {
    // Chunk size
    let num_chunks = 128usize;
    let chunk_size = (NN + num_chunks - 1) / num_chunks;

    let ans: i64 = (0..num_chunks)
        .into_par_iter()
        .map(|chunk_idx| {
            let n_start = chunk_idx * chunk_size + 1;
            let n_end = ((chunk_idx + 1) * chunk_size).min(NN);
            if n_start > n_end {
                return 0;
            }

            let mut chunk_sum = 0i64;

            let (mut r0, mut r1, mut r2) = if n_start == 1 {
                // Handle n = 1 explicitly
                // r[1..12]:
                // r[1]=0, r[2]=1, r[3]=1, r[4]=2, r[5]=4, r[6]=7,
                // r[7]=13, r[8]=24, r[9]=44, r[10]=81, r[11]=149, r[12]=274
                let v1 = 0 - 1;
                let v2 = 1 + 2;
                let v3 = 4 * 7;
                let w1 = 13 - 24;
                let w2 = 44 + 81;
                let w3 = 149 * 274;

                let g = if l1(v1, v2, v3) < l1(w1, w2, w3) {
                    gauss(v1, v2, v3, w1, w2, w3)
                } else {
                    gauss(w1, w2, w3, v1, v2, v3)
                };
                chunk_sum += g;

                // State after r[12]: r[12]=274, r[11]=149, r[10]=81
                (274, 149, 81)
            } else {
                let k = 12 * n_start - 12;
                get_tribonacci_state(k)
            };

            let mut next_r = || -> i64 {
                let mut s = r0 + r1 + r2;
                if s >= 2 * M { s -= 2 * M; }
                if s >= M { s -= M; }
                r2 = r1;
                r1 = r0;
                r0 = s;
                s
            };

            let loop_start = if n_start == 1 { 2 } else { n_start };
            for _ in loop_start..=n_end {
                let r1_val = next_r(); // 12n - 11
                let r2_val = next_r(); // 12n - 10
                let r3_val = next_r(); // 12n - 9
                let r4_val = next_r(); // 12n - 8
                let r5_val = next_r(); // 12n - 7
                let r6_val = next_r(); // 12n - 6
                let r7_val = next_r(); // 12n - 5
                let r8_val = next_r(); // 12n - 4
                let r9_val = next_r(); // 12n - 3
                let r10_val = next_r(); // 12n - 2
                let r11_val = next_r(); // 12n - 1
                let r12_val = next_r(); // 12n

                let v1 = r1_val - r2_val;
                let v2 = r3_val + r4_val;
                let v3 = r5_val * r6_val;
                let w1 = r7_val - r8_val;
                let w2 = r9_val + r10_val;
                let w3 = r11_val * r12_val;

                let g = if l1(v1, v2, v3) < l1(w1, w2, w3) {
                    gauss(v1, v2, v3, w1, w2, w3)
                } else {
                    gauss(w1, w2, w3, v1, v2, v3)
                };
                chunk_sum += g;
            }
            chunk_sum
        })
        .sum();

    println!("{}", ans);
}
