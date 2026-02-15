// Project Euler 201: Subsets with a Unique Sum

fn main() {
    let n = 100usize;
    let k = 50usize;
    let l: i64 = (1..=n as i64).map(|i| i * i).sum();
    let half = (l / 2) as usize;

    // dp[i][j] = min(2, number of i-element subsets summing to j)
    let mut dp = vec![0u8; (k + 1) * (half + 1)];
    let idx = |i: usize, j: usize| -> usize { i * (half + 1) + j };
    dp[idx(0, 0)] = 1;

    for nn in 1..=n {
        let sq = nn * nn;
        let max_j_ll = nn * (nn + 1) * (2 * nn + 1) / 6;
        let max_j = max_j_ll.min(half);
        let max_i = k.min(nn);

        for i in (1..=max_i).rev() {
            let end = max_j + 1;
            if end <= sq { continue; }
            for j in (sq..end).rev() {
                unsafe {
                    let idx_ij = i * (half + 1) + j;
                    let idx_prev = (i - 1) * (half + 1) + (j - sq);
                    let val = *dp.get_unchecked(idx_ij) as u16 + *dp.get_unchecked(idx_prev) as u16;
                    *dp.get_unchecked_mut(idx_ij) = val.min(2) as u8;
                }
            }
        }
    }

    let mut ans: i64 = 0;
    for j in 0..=half {
        if unsafe { *dp.get_unchecked(k * (half + 1) + j) } == 1 {
            ans += l;
        }
    }

    println!("{}", ans);
}
