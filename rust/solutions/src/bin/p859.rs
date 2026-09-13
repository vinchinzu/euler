// Project Euler 859 - Cookies game (partition DP)

const N: usize = 300;
const OFFSET: usize = 2000;
const MAX_VAL: usize = 4001;

fn main() {
    let mut g = vec![0i32; N + 1];
    for k in 1..=N {
        if k % 2 == 1 {
            let m = (k - 1) / 2;
            let val = 2 * g[m];
            g[k] = if val < 0 { 0 } else { val + 1 };
        } else {
            let m = (k - 2) / 2;
            let val = 2 * g[m];
            g[k] = if val > 0 { 0 } else { val - 1 };
        }
    }

    let mut dp = vec![0i64; (N + 1) * MAX_VAL];
    dp[OFFSET] = 1;

    for k in 1..=N {
        let g_val = g[k];
        for n in k..=N {
            let src_base = (n - k) * MAX_VAL;
            let dst_base = n * MAX_VAL;
            if g_val >= 0 {
                let g_usize = g_val as usize;
                let i_end = MAX_VAL - g_usize;
                for i in 0..i_end {
                    let src_idx = src_base + i;
                    let dst_idx = dst_base + i + g_usize;
                    unsafe {
                        // SAFETY: src_idx = (n-k)*MAX_VAL + i where n-k < N+1, i < MAX_VAL
                        // and dst_idx = n*MAX_VAL + i + g_usize where n <= N, i+g_usize < MAX_VAL
                        // Both indices are within dp size of (N+1)*MAX_VAL
                        let val = *dp.get_unchecked(src_idx);
                        if val > 0 {
                            *dp.get_unchecked_mut(dst_idx) += val;
                        }
                    }
                }
            } else {
                let i_start = (-g_val) as usize;
                for i in i_start..MAX_VAL {
                    let src_idx = src_base + i;
                    let dst_idx = dst_base + (i as i32 + g_val) as usize;
                    unsafe {
                        // SAFETY: src_idx = (n-k)*MAX_VAL + i where n-k < N+1, i < MAX_VAL
                        // and dst_idx = n*MAX_VAL + (i+g_val) where n <= N, i >= -g_val ensures i+g_val >= 0,
                        // and i < MAX_VAL with g_val negative ensures (i+g_val) < MAX_VAL
                        // Both indices are within dp size of (N+1)*MAX_VAL
                        let val = *dp.get_unchecked(src_idx);
                        if val > 0 {
                            *dp.get_unchecked_mut(dst_idx) += val;
                        }
                    }
                }
            }
        }
    }

    let total: i64 = (0..=OFFSET).map(|i| dp[N * MAX_VAL + i]).sum();
    println!("{}", total);
}
