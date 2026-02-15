// Project Euler 825 - Chasing Game
// Sum of S(n) for n=2..N using linear system + harmonic approximation

fn main() {
    let nn: f64 = 1e14;
    let l = 100;

    fn solve_linear_system(n: usize, a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
        let mut aug: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                let mut row = a[i].clone();
                row.push(b[i]);
                row
            })
            .collect();

        for i in 0..n {
            let max_row = (i..n).max_by(|&a, &b| {
                aug[a][i].abs().partial_cmp(&aug[b][i].abs()).unwrap()
            }).unwrap();
            aug.swap(i, max_row);

            for k in (i + 1)..n {
                let factor = aug[k][i] / aug[i][i];
                for j in i..=n {
                    aug[k][j] -= factor * aug[i][j];
                }
            }
        }

        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            x[i] = aug[i][n];
            for j in (i + 1)..n {
                x[i] -= aug[i][j] * x[j];
            }
            x[i] /= aug[i][i];
        }
        x
    }

    fn s_val(n: usize) -> f64 {
        let dim = 2 * n;
        let mut a = vec![vec![0.0; dim]; dim];
        let mut b = vec![1.0; dim];

        for i in 0..dim {
            a[i][i] = 1.0;
            for j in 1..=3usize {
                if j < i {
                    a[i][dim - i + j] += 1.0 / 3.0;
                }
            }
        }

        let x = solve_linear_system(dim, &a, &b);
        2.0 * x[n] - 1.0
    }

    fn harmonic_approx(n: f64) -> f64 {
        let gamma = 0.5772156649015329;
        gamma + n.ln() + 1.0 / (2.0 * n) - 1.0 / (12.0 * n * n)
    }

    let mut ans = 0.0;
    for n in 2..=l {
        ans += s_val(n);
    }

    let sqrt3: f64 = 3.0_f64.sqrt();
    let offset = 1.0 / (3.0 - sqrt3);
    ans += harmonic_approx(nn) - harmonic_approx(l as f64 - offset);

    println!("{:.8}", ans);
}
