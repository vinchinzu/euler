// Project Euler 648 - Skipping Squares
// Power series expansion with DP

const NN: usize = 1000;
const MOD: i64 = 1_000_000_000;

fn main() {
    let max_s = (NN / 2) * (NN / 2);

    // Precompute which s are perfect squares and their roots
    let mut is_sq = vec![false; max_s + 1];
    let mut sqrt_of = vec![0i32; max_s + 1];
    {
        let mut r = 0i32;
        while (r * r) as usize <= max_s {
            is_sq[(r * r) as usize] = true;
            sqrt_of[(r * r) as usize] = r;
            r += 1;
        }
    }

    let mut jump1 = vec![0i64; NN + 1];
    let mut jump2 = vec![0i64; NN + 1];
    let mut f = vec![0i64; NN + 1];

    for s in (0..=max_s).rev() {
        if s > 0 && is_sq[s] {
            f.fill(0);
            f[0] = (sqrt_of[s] - 1) as i64;
        } else {
            // SAFETY: k in 0..=NN; all arrays length NN+1
            unsafe {
                *f.get_unchecked_mut(0) = *jump2.get_unchecked(0);
                for k in 1..=NN {
                    let mut v = *jump2.get_unchecked(k) + *jump1.get_unchecked(k - 1)
                        - *jump2.get_unchecked(k - 1);
                    v %= MOD;
                    if v < 0 {
                        v += MOD;
                    }
                    *f.get_unchecked_mut(k) = v;
                }
            }
        }

        // jump2 <- jump1; jump1 <- f  via swaps (no copies)
        std::mem::swap(&mut jump2, &mut jump1);
        std::mem::swap(&mut jump1, &mut f);
    }

    let mut ans = 0i64;
    for &v in &jump1 {
        ans += v;
        if ans >= MOD {
            ans -= MOD;
        }
    }
    println!("{}", ans);
}
