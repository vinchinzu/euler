// Project Euler 519 - Tricolored Coin Fountains
// Count 3-colorings over all fountains of N coins. DP with suffix sums.

const N_VAL: usize = 20_000;
const M_VAL: i64 = 1_000_000_000;

fn main() {
    let n = N_VAL;
    let l = (2.0 * n as f64).sqrt() as usize + 1;
    let m = M_VAL;

    // dp[n][k] and suf[n][k] - full arrays needed
    let stride_dp = l + 1;
    let stride_suf = l + 2;
    let mut dp = vec![0i64; (n + 1) * stride_dp];
    let mut suf = vec![0i64; (n + 1) * stride_suf];

    macro_rules! dp_idx { ($nn:expr, $kk:expr) => { ($nn) * stride_dp + ($kk) } }
    macro_rules! suf_idx { ($nn:expr, $kk:expr) => { ($nn) * stride_suf + ($kk) } }

    dp[dp_idx!(1, 1)] = 3;
    for prev in (0..=l).rev() {
        suf[suf_idx!(1, prev)] = (suf[suf_idx!(1, prev + 1)] + dp[dp_idx!(1, prev)]) % m;
    }

    for nn in 2..=n {
        let kmax = l.min(nn);
        for k in 1..=kmax {
            if nn < k { continue; }
            if k == 1 {
                dp[dp_idx!(nn, k)] = (2 * suf[suf_idx!(nn - 1, 1)]) % m;
            } else {
                let mut total = suf[suf_idx!(nn - k, k - 1)];
                if k == 2 {
                    total = (total + dp[dp_idx!(nn - k, 1)]) % m;
                }
                dp[dp_idx!(nn, k)] = total % m;
            }
        }
        for prev in (0..=l).rev() {
            suf[suf_idx!(nn, prev)] = (suf[suf_idx!(nn, prev + 1)] + dp[dp_idx!(nn, prev)]) % m;
        }
    }

    let mut ans = 0i64;
    for k in 1..=l {
        ans = (ans + dp[dp_idx!(n, k)]) % m;
    }

    println!("{}", ans);
}
