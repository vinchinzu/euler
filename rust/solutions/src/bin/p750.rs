// Project Euler 750 - Optimal Card Stacking
//
// Interval DP: dp[s][e] = min cost to merge cards s..e into one stack.

const N: usize = 976;

fn main() {
    let mut pos = [0usize; N];
    let mut p: i64 = 1;
    for i in 0..N {
        p = p * 3 % (N as i64 + 1);
        pos[p as usize - 1] = i;
    }

    // Use flat array for dp: dp[s * (N+1) + e]
    let mut dp = vec![0i64; N * (N + 1)];

    for length in 2..=N {
        for s in 0..=N - length {
            let end = s + length;
            let mut best: i64 = -1;
            for mid in (s + 1)..end {
                let cost = dp[s * (N + 1) + mid]
                    + dp[mid * (N + 1) + end]
                    + (pos[mid - 1] as i64 - pos[end - 1] as i64).abs();
                if best < 0 || cost < best {
                    best = cost;
                }
            }
            dp[s * (N + 1) + end] = best;
        }
    }

    println!("{}", dp[0 * (N + 1) + N]);
}
