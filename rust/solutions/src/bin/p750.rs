// Project Euler 750 - Optimal Card Stacking
//
// Interval DP: dp[s][e] = min cost to merge cards s..e into one stack.
// For a fixed length the starts s are independent (they only read strictly
// shorter intervals). A transposed copy of dp makes both inner-loop loads
// sequential.

use rayon::prelude::*;

const N: usize = 976;
const STRIDE: usize = N + 1;

#[inline(always)]
fn interval_best(dp: &[i32], tdp: &[i32], pos: &[i32; N], s: usize, length: usize) -> i32 {
    let end = s + length;
    let row = s * STRIDE;
    let trow = end * STRIDE;
    // SAFETY: end in 2..=N, so end-1 < N.
    let pos_end = unsafe { *pos.get_unchecked(end - 1) };
    let mut best = i32::MAX;
    // SAFETY: s < N, end <= N, mid in (s+1)..end so 1 <= mid < N.
    // dp is N*STRIDE: s*STRIDE+mid <= (N-1)*STRIDE+(N-1) < N*STRIDE.
    // tdp is (N+1)*STRIDE: end*STRIDE+mid <= N*STRIDE+(N-1) < (N+1)*STRIDE.
    unsafe {
        let dp_ptr = dp.as_ptr();
        let tdp_ptr = tdp.as_ptr();
        let pos_ptr = pos.as_ptr();
        let mut mid = s + 1;
        while mid < end {
            let d = *pos_ptr.add(mid - 1) - pos_end;
            let cost = *dp_ptr.add(row + mid) + *tdp_ptr.add(trow + mid) + d.abs();
            if cost < best {
                best = cost;
            }
            mid += 1;
        }
    }
    best
}

fn main() {
    let mut pos = [0i32; N];
    let mut p: i32 = 1;
    for i in 0..N {
        p = p * 3 % (N as i32 + 1);
        pos[p as usize - 1] = i as i32;
    }

    let mut dp = vec![0i32; N * STRIDE];
    let mut tdp = vec![0i32; (N + 1) * STRIDE];
    let mut buf = Vec::with_capacity(N);

    for length in 2..=N {
        let n_starts = N - length + 1;
        let inner = length - 1;
        if n_starts > 1 && n_starts * inner >= 12_000 {
            let dp_ref = dp.as_slice();
            let tdp_ref = tdp.as_slice();
            let min_len = (8_192 / inner).max(1);
            (0..n_starts)
                .into_par_iter()
                .with_min_len(min_len)
                .map(|s| interval_best(dp_ref, tdp_ref, &pos, s, length))
                .collect_into_vec(&mut buf);
            for s in 0..n_starts {
                let v = buf[s];
                let end = s + length;
                dp[s * STRIDE + end] = v;
                tdp[end * STRIDE + s] = v;
            }
        } else {
            for s in 0..n_starts {
                let v = interval_best(&dp, &tdp, &pos, s, length);
                let end = s + length;
                dp[s * STRIDE + end] = v;
                tdp[end * STRIDE + s] = v;
            }
        }
    }

    println!("{}", dp[N]);
}
