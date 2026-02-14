// Project Euler 504 - Square on the Inside
// Count quadrilaterals (a,b,c,d) with 1<=a,b,c,d<=100 whose interior lattice point count is a perfect square.

use euler_utils::gcd;

fn main() {
    const N: usize = 100;

    // f[a][b] = ((a+1)*(b+1) - gcd(a,b) - 1) / 2 - a
    let mut f = [[0i32; N + 1]; N + 1];
    for a in 1..=N {
        for b in 1..=N {
            f[a][b] = (((a + 1) * (b + 1) - gcd(a as u64, b as u64) as usize - 1) / 2 - a) as i32;
        }
    }

    let max_val = 2 * N * N;
    let mut is_sq = vec![false; max_val + 1];
    let mut i = 1;
    while i * i <= max_val {
        is_sq[i * i] = true;
        i += 1;
    }

    let mut ans: i64 = 0;
    for a in 1..=N {
        for b in 1..=N {
            let fab = f[a][b];
            for c in 1..=N {
                let fab_fbc = fab + f[b][c];
                for d in 1..=N {
                    let total = (fab_fbc + f[c][d] + f[d][a] + 1) as usize;
                    if total <= max_val && is_sq[total] {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
