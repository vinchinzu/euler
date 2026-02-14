// Project Euler 594 - Rhombus Tilings of an Octagon
//
// Count tilings of an octagon with sides A=4, B=2
// using unit squares and unit 45-degree rhombi.
// Uses determinant-based formula over all interior grid point configurations.

const A: i32 = 4;
const B: i32 = 2;
const BSIZ: usize = (B + 2) as usize;

fn n_cr(n: i32, k: i32) -> i64 {
    if k < 0 || n < 0 || k > n { return 0; }
    let k = k.min(n - k) as usize;
    let mut result: i64 = 1;
    for i in 0..k {
        result = result * (n as i64 - i as i64) / (i as i64 + 1);
    }
    result
}

fn det2(mat: &[[i64; 2]; 2]) -> i64 {
    mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0]
}

fn main() {
    let mut ans: i64 = 0;

    let total: usize = ((A + 1) * (A + 1)).pow(B as u32 * B as u32) as usize;

    for idx in 0..total {
        let mut x = [[0i32; BSIZ]; BSIZ];
        let mut y = [[0i32; BSIZ]; BSIZ];

        // Set boundary conditions
        for k in 1..=B {
            x[(B + 1) as usize][k as usize] = A;
            x[k as usize][(B + 1) as usize] = A;
            y[0][k as usize] = A;
            y[k as usize][(B + 1) as usize] = A;
        }

        // Set interior values from idx
        let mut tmp = idx;
        for i in 1..=B {
            for j in 1..=B {
                let val = tmp % ((A + 1) * (A + 1)) as usize;
                tmp /= ((A + 1) * (A + 1)) as usize;
                x[i as usize][j as usize] = (val / (A + 1) as usize) as i32;
                y[i as usize][j as usize] = (val % (A + 1) as usize) as i32;
            }
        }

        // Compute number of tilings
        let mut num_tilings: i64 = 1;
        let mut valid = true;

        for u in 1..=(B + 1) as usize {
            if !valid { break; }

            let mut m_mat = [[0i64; 2]; 2];
            let mut p_mat = [[0i64; 2]; 2];

            for i in 1..=B as usize {
                for j in 1..=B as usize {
                    m_mat[i - 1][j - 1] = n_cr(
                        x[j][u] - x[i][u - 1] + y[j][u] - y[i][u - 1],
                        x[j][u] - x[i][u - 1] + (j as i32 - i as i32),
                    );
                    p_mat[i - 1][j - 1] = n_cr(
                        x[u][j] - x[u - 1][i] + y[u - 1][i] - y[u][j],
                        x[u][j] - x[u - 1][i] + (j as i32 - i as i32),
                    );
                }
            }

            let dm = det2(&m_mat);
            let dp = det2(&p_mat);
            num_tilings *= dm * dp;

            if num_tilings == 0 {
                valid = false;
            }
        }

        if valid {
            ans += num_tilings;
        }
    }

    println!("{}", ans);
}
