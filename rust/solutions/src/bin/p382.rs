// Project Euler 382 — Generating Polygons
// f(n) counts subsets of U_n that generate at least one polygon.
// Find last 9 digits of f(10^18).
// Expected: 697003956

const MOD: u64 = 1_000_000_000;

fn mat_mul(a: &[[u64; 12]; 12], b: &[[u64; 12]; 12]) -> Box<[[u64; 12]; 12]> {
    let mut res = Box::new([[0u64; 12]; 12]);
    for i in 0..12 {
        for k in 0..12 {
            let aik = a[i][k];
            if aik == 0 { continue; }
            for j in 0..12 {
                res[i][j] = ((res[i][j] as u128 + aik as u128 * b[k][j] as u128) % MOD as u128) as u64;
            }
        }
    }
    res
}

fn mat_pow(m: &[[u64; 12]; 12], mut e: u64) -> Box<[[u64; 12]; 12]> {
    let mut result = Box::new([[0u64; 12]; 12]);
    for i in 0..12 { result[i][i] = 1; }
    let mut base = Box::new(*m);
    while e > 0 {
        if e & 1 == 1 {
            result = mat_mul(&base, &result);
        }
        base = mat_mul(&base, &base);
        e >>= 1;
    }
    result
}

fn mat_vec_mul(m: &[[u64; 12]; 12], v: &[u64; 12]) -> [u64; 12] {
    let mut out = [0u64; 12];
    for i in 0..12 {
        let mut s = 0u128;
        for j in 0..12 {
            s += m[i][j] as u128 * v[j] as u128;
        }
        out[i] = (s % MOD as u128) as u64;
    }
    out
}

fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % MOD as u128) as u64;
        }
        base = (base as u128 * base as u128 % MOD as u128) as u64;
        exp >>= 1;
    }
    result
}

fn build_transition_matrix() -> Box<[[u64; 12]; 12]> {
    let mut m = Box::new([[0u64; 12]; 12]);

    // Row 0: b_{i+1} = 2*b_{i-2} + b_{i-3} - b_{i-5} + 5*p_{i-3} + 1
    m[0][2] = 2;
    m[0][3] = 1;
    m[0][5] = MOD - 1; // -1 mod MOD
    m[0][9] = 5;
    m[0][11] = 1;

    // Shift b's
    m[1][0] = 1;
    m[2][1] = 1;
    m[3][2] = 1;
    m[4][3] = 1;
    m[5][4] = 1;

    // Powers of two
    m[6][6] = 2;
    m[7][6] = 1;
    m[8][7] = 1;
    m[9][8] = 1;

    // Prefix sum S_{i+1} = S_i + b_{i+1}
    m[10][10] = 1;
    for j in 0..12 {
        m[10][j] = (m[10][j] + m[0][j]) % MOD;
    }

    // Constant 1
    m[11][11] = 1;

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
        mod_pow(2, 5),
        mod_pow(2, 4),
        mod_pow(2, 3),
        mod_pow(2, 2),
        s5,
        1,
    ];

    let m = build_transition_matrix();
    let p = mat_pow(&m, idx - 5);
    let v2 = mat_vec_mul(&p, &v);
    v2[10] % MOD
}

fn f(n: u64) -> u64 {
    let bad = prefix_sum_b(n);
    (mod_pow(2, n) + MOD - 1 + MOD - bad) % MOD
}

fn main() {
    debug_assert_eq!(f(5), 7);
    debug_assert_eq!(f(10), 501);
    debug_assert_eq!(f(25), 18635853);

    let n = 1_000_000_000_000_000_000u64;
    let ans = f(n);
    println!("{:09}", ans);
}
