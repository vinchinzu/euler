// Project Euler 671 - Coloured Tiles II
// Matrix exponentiation for colored tiles on a loop. 84 states.

const M_VAL: i64 = 1_000_004_321;
const T_VAL: usize = 3;
const K_VAL: i64 = 10;
const NUM_STATES: usize = 3 + T_VAL * 3 * T_VAL * 3; // 84
const NS: usize = NUM_STATES;

fn vert_idx(color: usize) -> usize { color }
fn horiz_idx(top: usize, tc: usize, bot: usize, bc: usize) -> usize {
    3 + (top * 3 + tc) * (T_VAL * 3) + bot * 3 + bc
}
fn min_c(c: usize) -> usize { if c < 2 { c } else { 2 } }

fn mat_mult(a: &[i64], b: &[i64], r: &mut [i64]) {
    r.fill(0);
    for i in 0..NS {
        for k in 0..NS {
            let aik = a[i * NS + k];
            if aik == 0 { continue; }
            let aik128 = aik as i128;
            for j in 0..NS {
                r[i * NS + j] = (r[i * NS + j] as i128 + aik128 * b[k * NS + j] as i128 % M_VAL as i128) as i64;
            }
        }
    }
}

fn mat_pow(mat: &[i64], mut exp: i64) -> Vec<i64> {
    let mut result = vec![0i64; NS * NS];
    for i in 0..NS { result[i * NS + i] = 1; }
    let mut base = mat.to_vec();
    let mut temp = vec![0i64; NS * NS];
    while exp > 0 {
        if exp & 1 == 1 {
            mat_mult(&result, &base, &mut temp);
            std::mem::swap(&mut result, &mut temp);
        }
        mat_mult(&base, &base, &mut temp);
        std::mem::swap(&mut base, &mut temp);
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
    let mut a = vec![0i64; NS * NS];

    // Build transition matrix (simplified - matching C logic)
    for c1 in 0..3usize {
        for c2 in 0..K_VAL as usize {
            if c1 != c2 {
                let row = vert_idx(min_c(c2));
                let col = vert_idx(c1);
                a[row * NS + col] = (a[row * NS + col] + 1) % M_VAL;
            }
        }
    }
    for c1 in 0..3 {
        for c2 in 0..3 {
            for c3 in 0..K_VAL as usize {
                if (c1 != c2 || c1 == 2) && c1 != c3 && c2 != c3 {
                    if c1 == 2 && c2 == 2 && c3 == 3 { continue; }
                    let mc3 = min_c(c3); let mc2 = min_c(c2);
                    let r = vert_idx(mc3);
                    let c = horiz_idx(0, c1, 0, mc2);
                    a[r * NS + c] = (a[r * NS + c] + 1) % M_VAL;
                    for i in 1..T_VAL {
                        for j in 0..T_VAL {
                            let r = horiz_idx(i-1, c1, j, mc3);
                            let c = horiz_idx(i, c1, 0, mc2);
                            a[r * NS + c] = (a[r * NS + c] + 1) % M_VAL;
                        }
                    }
                    for i in 0..T_VAL {
                        for j in 1..T_VAL {
                            let r = horiz_idx(i, mc3, j-1, c1);
                            let c = horiz_idx(0, mc2, j, c1);
                            a[r * NS + c] = (a[r * NS + c] + 1) % M_VAL;
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
                        let r = horiz_idx(i, min_c(c2), j, min_c(c3));
                        let c = vert_idx(c1);
                        a[r * NS + c] = (a[r * NS + c] + 1) % M_VAL;
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
                    let r = horiz_idx(i-1, c1, j-1, mc2);
                    let c = horiz_idx(i, c1, j, mc2);
                    a[r * NS + c] = (a[r * NS + c] + 1) % M_VAL;
                }
            }
        }
    }

    let ae = mat_pow(&a, n);
    let mut ans = 0i64;
    let vi0 = vert_idx(0);
    ans = (ans + K_VAL * ae[vi0 * NS + vi0]) % M_VAL;
    for i in 0..T_VAL {
        for j in 0..T_VAL {
            let h = horiz_idx(i, 0, j, 1);
            ans = (ans + K_VAL * (K_VAL - 1) % M_VAL * ae[h * NS + h]) % M_VAL;
        }
    }
    ans = (ans as i128 * mod_inv_val(n % M_VAL, M_VAL) as i128 % M_VAL as i128) as i64;
    println!("{}", ans);
}
