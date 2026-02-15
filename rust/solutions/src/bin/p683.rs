// Project Euler 683 - The Chase II
// Band matrix linear system for expected money per round.

const MAX_L: usize = 260;
const MAX_DIFF: usize = 3;
const BAND_W: usize = 2 * MAX_DIFF + 1;

fn ncr(n: i32, r: i32) -> f64 {
    if r < 0 || r > n { return 0.0; }
    if r == 0 || r == n { return 1.0; }
    let r = r.min(n - r) as usize;
    let mut result = 1.0;
    for i in 0..r {
        result = result * (n as f64 - i as f64) / (i as f64 + 1.0);
    }
    result
}

fn solve_band(band: &mut [[f64; BAND_W]; MAX_L], b_vec: &mut [f64; MAX_L], res: &mut [f64; MAX_L], l: usize) {
    // Forward elimination
    for i in 0..l {
        for jj in 1..=MAX_DIFF {
            if i + jj >= l { break; }
            let pivot = band[i][MAX_DIFF];
            if pivot.abs() < 1e-15 { continue; }
            let ratio = band[i + jj][MAX_DIFF - jj] / pivot;
            for k in 0..=MAX_DIFF {
                let col_offset = k as i32 - jj as i32;
                let idx = MAX_DIFF as i32 + col_offset;
                if idx >= 0 && (idx as usize) < BAND_W {
                    band[i + jj][idx as usize] -= ratio * band[i][MAX_DIFF + k];
                }
            }
            b_vec[i + jj] -= ratio * b_vec[i];
        }
    }
    // Back substitution
    for i in (0..l).rev() {
        res[i] = b_vec[i];
        for jj in 1..=MAX_DIFF {
            if i + jj < l {
                res[i] -= band[i][MAX_DIFF + jj] * res[i + jj];
            }
        }
        let pivot = band[i][MAX_DIFF];
        if pivot.abs() > 1e-15 {
            res[i] /= pivot;
        }
    }
}

fn expected_round_money(n: i32, big_e: i32) -> f64 {
    let l = (n / 2 + 1) as usize;
    let mut x_arr = vec![[0.0f64; MAX_L]; 3];

    for d in 0..l {
        x_arr[0][d] = 1.0;
    }

    let mut band = [[0.0f64; BAND_W]; MAX_L];
    let mut b_vec = [0.0f64; MAX_L];
    let mut res = [0.0f64; MAX_L];

    for e in 1..=big_e as usize {
        band = [[0.0; BAND_W]; MAX_L];
        b_vec = [0.0; MAX_L];

        for d in 0..l {
            band[d][MAX_DIFF] = 1.0;
        }

        for d in 1..l {
            for da in -1i32..=1 {
                for db in -1i32..=1 {
                    let nd_raw = ((d as i32 + da + db) % n + n) % n;
                    let nd = nd_raw.min(n - nd_raw) as usize;
                    let diff = nd as i32 - d as i32;
                    if diff.unsigned_abs() as usize <= MAX_DIFF {
                        band[d][(MAX_DIFF as i32 + diff) as usize] -= 1.0 / 9.0;
                    }
                    for ep in 0..e {
                        b_vec[d] += ncr(e as i32, ep as i32) * x_arr[ep][nd] / 9.0;
                    }
                }
            }
        }

        solve_band(&mut band, &mut b_vec, &mut res, l);
        for d in 0..l {
            x_arr[e][d] = res[d];
        }
    }

    let mut expected = 0.0;
    for d in 0..n as usize {
        let md = d.min(n as usize - d);
        expected += x_arr[big_e as usize][md];
    }
    expected / n as f64
}

fn main() {
    let big_n = 500;
    let big_e = 2;

    let mut ans = 0.0;
    for n in 2..=big_n {
        ans += expected_round_money(n, big_e);
    }

    // Format: scientific notation with 8 decimal digits, no '+' in exponent
    let formatted = format!("{:.8e}", ans);
    let out: String = formatted.chars().filter(|&c| c != '+').collect();
    println!("{}", out);
}
