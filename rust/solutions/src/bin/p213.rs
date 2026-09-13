const SZ: usize = 30;
const STEPS: usize = 50;

fn main() {
    let half = SZ / 2;
    
    let neighbor_inv = {
        let mut ni = [[0.0f64; SZ]; SZ];
        for i in 0..SZ {
            for j in 0..SZ {
                let mut n = 0;
                if i > 0 { n += 1; }
                if i < SZ - 1 { n += 1; }
                if j > 0 { n += 1; }
                if j < SZ - 1 { n += 1; }
                ni[i][j] = 1.0 / n as f64;
            }
        }
        ni
    };
    
    let mut table = vec![vec![[[0.0f64; SZ]; SZ]; half]; half];

    for fi in 0..half {
        for fj in 0..half {
            let mut grid = [[0.0f64; SZ]; SZ];
            grid[fi][fj] = 1.0;

            for _step in 0..STEPS {
                let mut new_grid = [[0.0f64; SZ]; SZ];
                for i in 0..SZ {
                    for j in 0..SZ {
                        let prob = grid[i][j];
                        if prob == 0.0 { continue; }
                        let p = prob * neighbor_inv[i][j];
                        if i > 0 { new_grid[i - 1][j] += p; }
                        if i < SZ - 1 { new_grid[i + 1][j] += p; }
                        if j > 0 { new_grid[i][j - 1] += p; }
                        if j < SZ - 1 { new_grid[i][j + 1] += p; }
                    }
                }
                grid = new_grid;
            }

            table[fi][fj] = grid;
        }
    }

    let mut ans = 0.0f64;
    for i in 0..SZ {
        for j in 0..SZ {
            let mut prob = 1.0f64;
            for fi in 0..half {
                for fj in 0..half {
                    prob *= (1.0 - table[fi][fj][i][j])
                          * (1.0 - table[fi][fj][i][SZ - 1 - j])
                          * (1.0 - table[fi][fj][SZ - 1 - i][j])
                          * (1.0 - table[fi][fj][SZ - 1 - i][SZ - 1 - j]);
                }
            }
            ans += prob;
        }
    }

    println!("{:.6}", ans);
}
