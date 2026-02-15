// Project Euler 706 - 3-Like Numbers
//
// Matrix exponentiation on states tracking prefix digit sum counts (mod 3)
// and total digit sum (mod 3). 81 states total.

const MOD: i64 = 1_000_000_007;
const K: usize = 3;
const B: usize = 10;
const NSTATES: usize = 81; // 3^3 * 3

fn encode(n0: usize, n1: usize, n2: usize, total: usize) -> usize {
    n0 + n1 * K + n2 * K * K + total * K * K * K
}

fn decode(idx: usize) -> (usize, usize, usize, usize) {
    (idx % K, (idx / K) % K, (idx / (K * K)) % K, idx / (K * K * K))
}

type Mat = [[i64; NSTATES]; NSTATES];

fn mat_mult(a: &Mat, b: &Mat) -> Box<Mat> {
    let mut c = Box::new([[0i64; NSTATES]; NSTATES]);
    for i in 0..NSTATES {
        for k in 0..NSTATES {
            if a[i][k] == 0 {
                continue;
            }
            let aik = a[i][k] as i128;
            for j in 0..NSTATES {
                c[i][j] = (c[i][j] as i128 + aik * b[k][j] as i128) as i64 % MOD;
            }
        }
    }
    c
}

fn mat_pow(base: &Mat, mut exp: i64) -> Box<Mat> {
    let mut result = Box::new([[0i64; NSTATES]; NSTATES]);
    for i in 0..NSTATES {
        result[i][i] = 1;
    }
    let mut b = Box::new(*base);
    while exp > 0 {
        if exp & 1 == 1 {
            result = mat_mult(&result, &b);
        }
        b = mat_mult(&b, &b);
        exp >>= 1;
    }
    result
}

fn ncr2(n: i64) -> i64 {
    if n < 2 { 0 } else { n * (n - 1) / 2 }
}

fn main() {
    let nn: i64 = 100_000;

    // Build transition matrix
    let mut a = Box::new([[0i64; NSTATES]; NSTATES]);

    for n0 in 0..K {
        for n1 in 0..K {
            for n2 in 0..K {
                for total in 0..K {
                    let old_idx = encode(n0, n1, n2, total);
                    for d in 0..B {
                        let new_total = (total + d) % K;
                        let mut nn_arr = [n0, n1, n2];
                        nn_arr[new_total] = (nn_arr[new_total] + 1) % K;
                        let new_idx = encode(nn_arr[0], nn_arr[1], nn_arr[2], new_total);
                        a[new_idx][old_idx] = (a[new_idx][old_idx] + 1) % MOD;
                    }
                }
            }
        }
    }

    let an = mat_pow(&a, nn - 1);

    let mut ans: i64 = 0;
    for n0 in 0..K {
        for n1 in 0..K {
            for n2 in 0..K {
                let f = ncr2(n0 as i64) + ncr2(n1 as i64) + ncr2(n2 as i64);
                if f % K as i64 != 0 {
                    continue;
                }

                for d in 1..B {
                    let mut start_ns = [0usize; 3];
                    start_ns[0] = (start_ns[0] + 1) % K;
                    start_ns[d % K] = (start_ns[d % K] + 1) % K;
                    let start_idx = encode(start_ns[0], start_ns[1], start_ns[2], d % K);

                    for total in 0..K {
                        let final_idx = encode(n0, n1, n2, total);
                        ans = (ans + an[final_idx][start_idx]) % MOD;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
