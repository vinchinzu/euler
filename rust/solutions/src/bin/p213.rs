const SZ: usize = 30;
const STEPS: usize = 50;
const HALF: usize = SZ / 2;

fn main() {
    let half = HALF;
    
    let neighbor_counts = {
        let mut nc = [[0u8; SZ]; SZ];
        for i in 0..SZ {
            for j in 0..SZ {
                let mut n = 0u8;
                if i > 0 { n += 1; }
                if i < SZ - 1 { n += 1; }
                if j > 0 { n += 1; }
                if j < SZ - 1 { n += 1; }
                nc[i][j] = n;
            }
        }
        nc
    };
    
    let mut table: Box<[[[[f64; SZ]; SZ]; HALF]; HALF]> = 
        Box::new([[[[0.0f64; SZ]; SZ]; HALF]; HALF]);

    for fi in 0..half {
        for fj in 0..half {
            let mut grid_a = [[0.0f64; SZ]; SZ];
            let mut grid_b = [[0.0f64; SZ]; SZ];
            grid_a[fi][fj] = 1.0;

            for _step in 0..STEPS {
                for i in 0..SZ {
                    for j in 0..SZ {
                        let prob = grid_a[i][j];
                        if prob == 0.0 { continue; }
                        let p = prob / neighbor_counts[i][j] as f64;
                        if i > 0 { grid_b[i - 1][j] += p; }
                        if i < SZ - 1 { grid_b[i + 1][j] += p; }
                        if j > 0 { grid_b[i][j - 1] += p; }
                        if j < SZ - 1 { grid_b[i][j + 1] += p; }
                    }
                }
                std::mem::swap(&mut grid_a, &mut grid_b);
                grid_b = [[0.0f64; SZ]; SZ];
            }

            table[fi][fj] = grid_a;
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
