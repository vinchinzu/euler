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
            // SAFETY: a, b in range 1..=N; f has size [N+1][N+1]
            let fab = unsafe { *f.get_unchecked(a).get_unchecked(b) };
            for c in 1..=N {
                // SAFETY: b, c in range 1..=N; f has size [N+1][N+1]
                let fbc = unsafe { *f.get_unchecked(b).get_unchecked(c) };
                let fab_fbc = fab + fbc;
                for d in 1..=N {
                    // SAFETY: c, d, a in range 1..=N; f has size [N+1][N+1]
                    let fcd = unsafe { *f.get_unchecked(c).get_unchecked(d) };
                    let fda = unsafe { *f.get_unchecked(d).get_unchecked(a) };
                    let total = (fab_fbc + fcd + fda + 1) as usize;
                    if total <= max_val {
                        // SAFETY: total <= max_val checked above; is_sq has size max_val+1
                        if unsafe { *is_sq.get_unchecked(total) } {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
