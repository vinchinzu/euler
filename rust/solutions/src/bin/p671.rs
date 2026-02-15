// Project Euler 671 - Coloured Tiles II
// Matrix exponentiation for colored tiles on a loop. 84 states.

const M_VAL: i64 = 1_000_004_321;
const T_VAL: usize = 3;
const K_VAL: i64 = 10;
const NUM_STATES: usize = 3 + T_VAL * 3 * T_VAL * 3; // 84

fn vert_idx(color: usize) -> usize { color }
fn horiz_idx(top: usize, tc: usize, bot: usize, bc: usize) -> usize {
    3 + (top * 3 + tc) * (T_VAL * 3) + bot * 3 + bc
}
fn min_c(c: usize) -> usize { if c < 2 { c } else { 2 } }

fn mat_mult(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut r = vec![vec![0i64; n]; n];
    for i in 0..n {
        for k in 0..n {
            if a[i][k] == 0 { continue; }
            for j in 0..n {
                r[i][j] = (r[i][j] as i128 + a[i][k] as i128 * b[k][j] as i128 % M_VAL as i128) as i64;
            }
        }
    }
    r
}

fn mat_pow(mat: &Vec<Vec<i64>>, mut exp: i64) -> Vec<Vec<i64>> {
    let n = mat.len();
    let mut result = vec![vec![0i64; n]; n];
    for i in 0..n { result[i][i] = 1; }
    let mut base = mat.clone();
    while exp > 0 {
        if exp & 1 == 1 { result = mat_mult(&result, &base); }
        base = mat_mult(&base, &base);
        exp >>= 1;
    }
    result
}

fn mod_inv_val(a: i64, m: i64) -> i64 {
    let (mut t, mut new_t) = (0i64, 1i64);
    let (mut r, mut new_r) = (m, a % m);
    if new_r < 0 { new_r += m; }
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t; new_t = t - q * new_t; t = tmp;
        let tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    if t < 0 { t += m; }
    t
}

fn main() {
    let n: i64 = 10_004_003_002_001;
    let mut a = vec![vec![0i64; NUM_STATES]; NUM_STATES];

    // Build transition matrix (simplified - matching C logic)
    for c1 in 0..3usize {
        for c2 in 0..K_VAL as usize {
            if c1 != c2 {
                let idx = vert_idx(min_c(c2));
                a[idx][vert_idx(c1)] = (a[idx][vert_idx(c1)] + 1) % M_VAL;
            }
        }
    }
    for c1 in 0..3 {
        for c2 in 0..3 {
            for c3 in 0..K_VAL as usize {
                if (c1 != c2 || c1 == 2) && c1 != c3 && c2 != c3 {
                    if c1 == 2 && c2 == 2 && c3 == 3 { continue; }
                    let mc3 = min_c(c3); let mc2 = min_c(c2);
                    a[vert_idx(mc3)][horiz_idx(0, c1, 0, mc2)] = (a[vert_idx(mc3)][horiz_idx(0, c1, 0, mc2)] + 1) % M_VAL;
                    for i in 1..T_VAL {
                        for j in 0..T_VAL {
                            a[horiz_idx(i-1, c1, j, mc3)][horiz_idx(i, c1, 0, mc2)] = (a[horiz_idx(i-1, c1, j, mc3)][horiz_idx(i, c1, 0, mc2)] + 1) % M_VAL;
                        }
                    }
                    for i in 0..T_VAL {
                        for j in 1..T_VAL {
                            a[horiz_idx(i, mc3, j-1, c1)][horiz_idx(0, mc2, j, c1)] = (a[horiz_idx(i, mc3, j-1, c1)][horiz_idx(0, mc2, j, c1)] + 1) % M_VAL;
                        }
                    }
                }
            }
        }
    }
    for c1 in 0..3 {
        for c2 in 0..K_VAL as usize {
            for c3 in 0..K_VAL as usize {
                if c1 == c2 || c1 == c3 || c2 == c3 { continue; }
                for i in 0..T_VAL {
                    for j in 0..T_VAL {
                        a[horiz_idx(i, min_c(c2), j, min_c(c3))][vert_idx(c1)] = (a[horiz_idx(i, min_c(c2), j, min_c(c3))][vert_idx(c1)] + 1) % M_VAL;
                    }
                }
            }
        }
    }
    for c1 in 0..3 {
        for c2 in 0..3 {
            for i in 1..T_VAL {
                for j in 1..T_VAL {
                    let mc2 = min_c(c2);
                    a[horiz_idx(i-1, c1, j-1, mc2)][horiz_idx(i, c1, j, mc2)] = (a[horiz_idx(i-1, c1, j-1, mc2)][horiz_idx(i, c1, j, mc2)] + 1) % M_VAL;
                }
            }
        }
    }

    let ae = mat_pow(&a, n);
    let mut ans = 0i64;
    ans = (ans + K_VAL * ae[vert_idx(0)][vert_idx(0)]) % M_VAL;
    for i in 0..T_VAL {
        for j in 0..T_VAL {
            ans = (ans + K_VAL * (K_VAL - 1) % M_VAL * ae[horiz_idx(i, 0, j, 1)][horiz_idx(i, 0, j, 1)]) % M_VAL;
        }
    }
    ans = (ans as i128 * mod_inv_val(n % M_VAL, M_VAL) as i128 % M_VAL as i128) as i64;
    println!("{}", ans);
}
