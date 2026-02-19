// Project Euler 382 — Generating Polygons
// f(n) counts subsets of U_n that generate at least one polygon.
// Find last 9 digits of f(10^18).

use euler_utils::{mod_pow, ModMatrix};

const MOD: u64 = 1_000_000_000;

fn build_transition_matrix() -> ModMatrix<12> {
    let mut m = ModMatrix::<12>::zero(MOD);

    // Row 0: b_{i+1} = 2*b_{i-2} + b_{i-3} - b_{i-5} + 5*p_{i-3} + 1
    m.data[0][2] = 2;
    m.data[0][3] = 1;
    m.data[0][5] = MOD - 1; // -1 mod MOD
    m.data[0][9] = 5;
    m.data[0][11] = 1;

    // Shift b's
    m.data[1][0] = 1;
    m.data[2][1] = 1;
    m.data[3][2] = 1;
    m.data[4][3] = 1;
    m.data[5][4] = 1;

    // Powers of two
    m.data[6][6] = 2;
    m.data[7][6] = 1;
    m.data[8][7] = 1;
    m.data[9][8] = 1;

    // Prefix sum S_{i+1} = S_i + b_{i+1}
    m.data[10][10] = 1;
    for j in 0..12 {
        m.data[10][j] = (m.data[10][j] + m.data[0][j]) % MOD;
    }

    // Constant 1
    m.data[11][11] = 1;

    m
}

fn prefix_sum_b(n: u64) -> u64 {
    if n == 0 { return 0; }
    let idx = n - 1;
    let b_init: [u64; 6] = [1, 2, 4, 6, 11, 20];

    if idx <= 5 {
        let mut s = 0u64;
        for i in 0..=idx as usize {
            s = (s + b_init[i]) % MOD;
        }
        return s;
    }

    let s5: u64 = b_init.iter().sum::<u64>() % MOD;
    let v: [u64; 12] = [
        b_init[5],
        b_init[4],
        b_init[3],
        b_init[2],
        b_init[1],
        b_init[0],
        mod_pow(2, 5, MOD),
        mod_pow(2, 4, MOD),
        mod_pow(2, 3, MOD),
        mod_pow(2, 2, MOD),
        s5,
        1,
    ];

    let m = build_transition_matrix();
    let p = m.pow(idx - 5);
    let v2 = p.mul_vec(&v);
    v2[10] % MOD
}

fn f(n: u64) -> u64 {
    let bad = prefix_sum_b(n);
    (mod_pow(2, n, MOD) + MOD - 1 + MOD - bad) % MOD
}

fn main() {
    debug_assert_eq!(f(5), 7);
    debug_assert_eq!(f(10), 501);
    debug_assert_eq!(f(25), 18635853);

    let n = 1_000_000_000_000_000_000u64;
    let ans = f(n);
    println!("{:09}", ans);
}
