// Project Euler 416: A frog's trip
use euler_utils::mod_inv;

const K: usize = 10;
const FK: usize = 2 * K;
const MAX_STATES: usize = 1000;

fn multinomial(n: i32, a: i32, b: i32, c: i32) -> i64 {
    if a < 0 || b < 0 || c < 0 || a + b + c != n { return 0; }
    let mut r = 1i64;
    for i in 0..a {
        r = r * (n - i) as i64 / (i + 1) as i64;
    }
    for i in 0..b {
        r = r * (n - a - i) as i64 / (i + 1) as i64;
    }
    r
}

fn mat_mul(a: &[i64], b: &[i64], n: usize, modulus: i64) -> Vec<i64> {
    let mut res = vec![0i64; n * n];
    for i in 0..n {
        for k in 0..n {
            let aik = a[i * n + k];
            if aik == 0 { continue; }
            for j in 0..n {
                res[i * n + j] += aik * b[k * n + j];
            }
        }
        for j in 0..n {
            res[i * n + j] %= modulus;
        }
    }
    res
}

fn mat_pow(base: &[i64], n: usize, mut exp: i64, modulus: i64) -> Vec<i64> {
    let mut result = vec![0i64; n * n];
    for i in 0..n { result[i * n + i] = 1; }
    let mut b = base.to_vec();

    while exp > 0 {
        if exp & 1 == 1 {
            result = mat_mul(&result, &b, n, modulus);
        }
        b = mat_mul(&b, &b, n, modulus);
        exp >>= 1;
    }
    result
}

fn main() {
    let n_val: i64 = 1_000_000_000_000;
    let m1: i64 = 512;      // 2^9
    let m2: i64 = 1_953_125; // 5^9

    // Build states
    let mut states: Vec<(i32, i32, i32)> = Vec::new(); // (a, b, u)
    let mut state_idx = vec![vec![vec![-1i32; 2]; FK + 1]; FK + 1];

    for a in 0..=FK as i32 {
        for b in 0..=(FK as i32 - a) {
            for u in 0..2i32 {
                state_idx[a as usize][b as usize][u as usize] = states.len() as i32;
                states.push((a, b, u));
            }
        }
    }
    let n_states = states.len();

    // Build transition matrix
    let mut a_mat = vec![0i64; n_states * n_states];
    for i in 0..n_states {
        let (a, b, u) = states[i];
        let c = FK as i32 - a - b;
        let new_u = u + if a == 0 { 1 } else { 0 };
        if new_u > 1 { continue; }

        for j1 in 0..=a {
            for j2 in 0..=(a - j1) {
                let j3 = a - j1 - j2;
                let new_a = b + j1;
                let new_b = c + j2;
                if new_a + new_b + j3 != FK as i32 { continue; }
                if new_a > FK as i32 || new_b > FK as i32 { continue; }

                let j = state_idx[new_a as usize][new_b as usize][new_u as usize];
                if j < 0 { continue; }

                let coeff = multinomial(a, j1, j2, j3);
                a_mat[j as usize * n_states + i] += coeff;
            }
        }
    }

    let start = state_idx[FK][0][0] as usize;
    let end0 = state_idx[FK][0][0] as usize;
    let end1 = state_idx[FK][0][1] as usize;

    // Compute mod M1
    let mut a1 = a_mat.clone();
    for v in a1.iter_mut() { *v %= m1; }
    let r1_mat = mat_pow(&a1, n_states, n_val - 1, m1);
    let r1 = (r1_mat[end0 * n_states + start] + r1_mat[end1 * n_states + start]) % m1;

    // Compute mod M2
    let mut a2 = a_mat;
    for v in a2.iter_mut() { *v %= m2; }
    let r2_mat = mat_pow(&a2, n_states, n_val - 1, m2);
    let r2 = (r2_mat[end0 * n_states + start] + r2_mat[end1 * n_states + start]) % m2;

    // CRT
    let big_m = m1 * m2;
    let inv1 = mod_inv(m2 as u64, m1 as u64).unwrap() as i64;
    let inv2 = mod_inv(m1 as u64, m2 as u64).unwrap() as i64;
    let result = (r1 * m2 % big_m * inv1 % big_m + r2 * m1 % big_m * inv2 % big_m) % big_m;

    println!("{}", result);
}
