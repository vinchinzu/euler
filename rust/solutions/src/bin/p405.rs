// Project Euler 405: A rectangular tiling
// Matrix exponentiation with period reduction.

use euler_utils::mod_pow;

const DIM: usize = 4;

type Mat = [[u64; DIM]; DIM];

fn mat_mul(a: &Mat, b: &Mat, m: u64) -> Mat {
    let mut c = [[0u64; DIM]; DIM];
    for i in 0..DIM {
        for k in 0..DIM {
            if a[i][k] == 0 { continue; }
            for j in 0..DIM {
                c[i][j] = (c[i][j] + (a[i][k] as u128 * b[k][j] as u128 % m as u128) as u64) % m;
            }
        }
    }
    c
}

fn mat_pow(mut base: Mat, mut p: u64, m: u64) -> Mat {
    let mut result = [[0u64; DIM]; DIM];
    for i in 0..DIM { result[i][i] = 1; }
    while p > 0 {
        if p & 1 == 1 { result = mat_mul(&result, &base, m); }
        base = mat_mul(&base, &base, m);
        p >>= 1;
    }
    result
}

fn mod_inv(a: u64, m: u64) -> u64 {
    let (mut old_r, mut r) = (a as i64, m as i64);
    let (mut old_s, mut s) = (1i64, 0i64);
    while r != 0 {
        let q = old_r / r;
        let tmp = r; r = old_r - q * r; old_r = tmp;
        let tmp = s; s = old_s - q * s; old_s = tmp;
    }
    ((old_s % m as i64 + m as i64) % m as i64) as u64
}

fn main() {
    let mut modv: u64 = 1;
    for _ in 0..7 { modv *= 17; } // 17^7

    // Period of sequence mod 17 is 8
    // Period mod 17^7 divides 8 * 17^6
    let mut period: u64 = 8;
    for _ in 0..6 { period *= 17; }

    // Compute 10^(10^18) mod period
    // period = 8 * 17^6. CRT: compute mod 8 and mod 17^6 separately.
    let r8: u64 = 0; // 10^k = 0 (mod 8) for k >= 3

    // phi(17^6) = 17^5 * 16
    let mut phi_17_6: u64 = 16;
    for _ in 0..5 { phi_17_6 *= 17; }

    let mut seventeen_6: u64 = 1;
    for _ in 0..6 { seventeen_6 *= 17; }

    // 10^18 mod phi(17^6)
    let exp_mod = mod_pow(10, 18, phi_17_6);
    let r17_6 = mod_pow(10, exp_mod, seventeen_6);

    // CRT: x = 0 (mod 8), x = r17_6 (mod 17^6)
    let inv8 = mod_inv(8, seventeen_6);
    let k = (r17_6 as u128 * inv8 as u128 % seventeen_6 as u128) as u64;
    let mut n_mod_period = (8u128 * k as u128 % period as u128) as u64;

    if n_mod_period < 3 { n_mod_period += period; }
    let steps = n_mod_period - 3;

    let mut m = [[0u64; DIM]; DIM];
    m[0][0] = 5; m[0][1] = modv - 2; m[0][2] = modv - 8; m[0][3] = 6;
    m[1][0] = 1;
    m[2][1] = 1;
    m[3][3] = 1;

    let mn = mat_pow(m, steps, modv);
    let state: [u64; 4] = [16, 2, 0, 1];
    let mut result: u64 = 0;
    for j in 0..4 {
        result = (result + (mn[0][j] as u128 * state[j] as u128 % modv as u128) as u64) % modv;
    }

    println!("{}", result);
}
