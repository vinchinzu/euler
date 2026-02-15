// Project Euler 662 - Fibonacci Paths
// 2D DP on lattice with Fibonacci-length jumps. N=10000.

fn main() {
    let n = 10000usize;
    let m: u32 = 1_000_000_007;
    let max_fib_limit = ((2.0 * (n as f64) * (n as f64)).sqrt() as usize) + 2;
    let mut fibs = Vec::new();
    let (mut fa, mut fb) = (1usize, 1usize);
    while fa <= max_fib_limit {
        fibs.push(fa);
        let tmp = fa + fb; fa = fb; fb = tmp;
    }
    let mut h_jumps = Vec::new();
    let mut v_jumps = Vec::new();
    let mut diag_jumps: Vec<(usize, usize)> = Vec::new();
    let mut h_seen = vec![false; max_fib_limit + 2];
    let mut v_seen = vec![false; max_fib_limit + 2];
    for &f in &fibs {
        for dx in 0..=f.min(n) {
            let dy2 = f * f - dx * dx;
            let dy = (dy2 as f64).sqrt() as usize;
            for ddy_try in dy.saturating_sub(1)..=dy+1 {
                if ddy_try * ddy_try == dy2 && ddy_try <= n {
                    if dx > 0 && ddy_try == 0 {
                        if !h_seen[dx] { h_seen[dx] = true; h_jumps.push(dx); }
                    } else if dx == 0 && ddy_try > 0 {
                        if !v_seen[ddy_try] { v_seen[ddy_try] = true; v_jumps.push(ddy_try); }
                    } else if dx > 0 && ddy_try > 0 {
                        diag_jumps.push((dx, ddy_try));
                    }
                }
            }
        }
    }
    let w = n + 1;
    let mut dp = vec![0u32; w * w]; // flat 2D: dp[y * w + x]
    dp[0] = 1;
    for y in 0..w {
        for vi in &v_jumps {
            if y < *vi { continue; }
            let src_row = (y - vi) * w;
            let dst_row = y * w;
            for x in 0..w {
                let v = dp[dst_row + x].wrapping_add(dp[src_row + x]);
                dp[dst_row + x] = if v >= m { v - m } else { v };
            }
        }
        for &(dx, dyv) in &diag_jumps {
            if y < dyv { continue; }
            let src_row = (y - dyv) * w;
            let dst_row = y * w;
            for x in dx..w {
                let v = dp[dst_row + x].wrapping_add(dp[src_row + x - dx]);
                dp[dst_row + x] = if v >= m { v - m } else { v };
            }
        }
        let row = y * w;
        for x in 1..w {
            let mut sum = 0u32;
            for &dx in &h_jumps {
                if dx > x { continue; }
                sum = sum.wrapping_add(dp[row + x - dx]);
                if sum >= m { sum -= m; }
            }
            dp[row + x] = dp[row + x].wrapping_add(sum);
            if dp[row + x] >= m { dp[row + x] -= m; }
        }
    }
    println!("{}", dp[n * w + n]);
}
