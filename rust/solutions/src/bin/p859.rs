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

    let mut lo = vec![MAX_VAL; N + 1];
    let mut hi = vec![0usize; N + 1];
    lo[0] = OFFSET;
    hi[0] = OFFSET;

    for k in 1..=N {
        let g_val = g[k];
        for n in k..=N {
            if lo[n - k] > hi[n - k] {
                continue;
            }
            let src_base = (n - k) * MAX_VAL;
            let dst_base = n * MAX_VAL;
            if g_val >= 0 {
                let offset = g_val as usize;
                let i_start = lo[n - k];
                let i_end = hi[n - k].min(MAX_VAL - offset - 1);
                for i in i_start..=i_end {
                    unsafe {
                        // SAFETY: i is in [i_start, i_end] where both are < MAX_VAL
                        // and offset ensures i+offset < MAX_VAL
                        // src_base and dst_base are valid row offsets
                        let val = *dp.get_unchecked(src_base + i);
                        if val > 0 {
                            *dp.get_unchecked_mut(dst_base + i + offset) += val;
                        }
                    }
                }
                if i_start <= i_end {
                    lo[n] = lo[n].min(i_start + offset);
                    hi[n] = hi[n].max(i_end + offset);
                }
            } else {
                let neg_offset = (-g_val) as usize;
                let i_start = lo[n - k].max(neg_offset);
                let i_end = hi[n - k];
                for i in i_start..=i_end {
                    unsafe {
                        // SAFETY: i >= neg_offset ensures i-neg_offset >= 0
                        // i <= hi[n-k] < MAX_VAL ensures bounds
                        let val = *dp.get_unchecked(src_base + i);
                        if val > 0 {
                            *dp.get_unchecked_mut(dst_base + i - neg_offset) += val;
                        }
                    }
                }
                if i_start <= i_end {
                    lo[n] = lo[n].min(i_start - neg_offset);
                    hi[n] = hi[n].max(i_end - neg_offset);
                }
            }
        }
    }

    let total: i64 = (0..=OFFSET).map(|i| dp[N * MAX_VAL + i]).sum();
    println!("{}", total);
}
