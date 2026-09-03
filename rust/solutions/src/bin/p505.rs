// Project Euler 505 - Bidirectional Recurrence
// Optimized with branchless bottom-up leaf evaluation, unrolled lower levels,
// branchless state transitions, and top-level scoped parallelization.

const N: u64 = 1_000_000_000_000;
const K: u64 = (1u64 << 60) - 1;

#[inline(always)]
fn step(prev_x: u64, x: u64) -> (u64, u64) {
    let base = (prev_x + x) << 1;
    ((base + x) & K, (base + prev_x) & K)
}

#[inline(always)]
fn leaf_val(parent_x: u64, x: u64) -> u64 {
    let (xl, xr) = step(parent_x, x);
    K - xl.max(xr)
}

#[inline(always)]
fn depth2(prev_x: u64, x: u64, alpha: u64) -> u64 {
    let (xl, xr) = step(prev_x, x);
    let yl = leaf_val(x, xl);
    let yr = leaf_val(x, xr);
    (K - yl.max(yr)).max(alpha)
}

#[inline(always)]
fn depth3(prev_x: u64, x: u64, alpha: u64, beta: u64) -> u64 {
    let (xl, xr) = step(prev_x, x);
    let (c1, c2) = if xl >= xr { (xl, xr) } else { (xr, xl) };
    let y = depth2(x, c1, K - beta);
    if K - y <= alpha {
        return alpha;
    }
    let y2 = depth2(x, c2, y);
    K - y.max(y2)
}

#[inline(always)]
fn depth4(prev_x: u64, x: u64, alpha: u64, beta: u64) -> u64 {
    let (xl, xr) = step(prev_x, x);
    let (c1, c2) = if xl <= xr { (xl, xr) } else { (xr, xl) };
    let y = depth3(x, c1, K - beta, K - alpha);
    if K - y <= alpha {
        return alpha;
    }
    let y2 = depth3(x, c2, y, K - alpha);
    K - y.max(y2)
}

#[inline(always)]
fn depth5(prev_x: u64, x: u64, alpha: u64, beta: u64) -> u64 {
    let (xl, xr) = step(prev_x, x);
    let (c1, c2) = if xl >= xr { (xl, xr) } else { (xr, xl) };
    let y = depth4(x, c1, K - beta, K - alpha);
    if K - y <= alpha {
        return alpha;
    }
    let y2 = depth4(x, c2, y, K - alpha);
    K - y.max(y2)
}

#[inline(always)]
fn depth6(prev_x: u64, x: u64, alpha: u64, beta: u64) -> u64 {
    let (xl, xr) = step(prev_x, x);
    let (c1, c2) = if xl <= xr { (xl, xr) } else { (xr, xl) };
    let y = depth5(x, c1, K - beta, K - alpha);
    if K - y <= alpha {
        return alpha;
    }
    let y2 = depth5(x, c2, y, K - alpha);
    K - y.max(y2)
}

#[inline(always)]
fn depth7(prev_x: u64, x: u64, alpha: u64, beta: u64) -> u64 {
    let (xl, xr) = step(prev_x, x);
    let (c1, c2) = if xl >= xr { (xl, xr) } else { (xr, xl) };
    let y = depth6(x, c1, K - beta, K - alpha);
    if K - y <= alpha {
        return alpha;
    }
    let y2 = depth6(x, c2, y, K - alpha);
    K - y.max(y2)
}

fn helper(k: u64, prev_x: u64, x: u64, alpha: u64, beta: u64) -> u64 {
    if 128 * k >= N {
        return depth7(prev_x, x, alpha, beta);
    }
    let (xl, xr) = step(prev_x, x);
    let odd = (k.leading_zeros() & 1) == 0;
    let first_left = (xl <= xr) ^ odd;
    let (c1, c1_x, c2, c2_x) = if first_left {
        (2 * k, xl, 2 * k + 1, xr)
    } else {
        (2 * k + 1, xr, 2 * k, xl)
    };
    let y = helper(c1, x, c1_x, K - beta, K - alpha);
    if K - y <= alpha {
        return alpha;
    }
    let y2 = helper(c2, x, c2_x, y, K - alpha);
    K - y.max(y2)
}

fn main() {
    // Generate states for top levels:
    let (x2, x3) = step(0, 1);
    let (x6, x7) = step(1, x3);
    let (x12, x13) = step(x3, x6);
    let (x4, x5) = step(1, x2);
    let (x8, x9) = step(x2, x4);

    // Step 1: k=13 and k=12 in parallel
    let (y13, y12) = std::thread::scope(|s| {
        let h1 = s.spawn(|| helper(13, x6, x13, 0, K));
        let h2 = s.spawn(|| helper(12, x6, x12, 0, K));
        (h1.join().unwrap(), h2.join().unwrap())
    });
    let y6 = K - y13.max(y12);
    let y7 = helper(7, x3, x7, y6, K);
    let y3 = K - y6.max(y7);

    // Step 2: k=9 and k=8 in parallel with alpha = y3, beta = K
    let (y9, y8) = std::thread::scope(|s| {
        let h1 = s.spawn(|| helper(9, x4, x9, y3, K));
        let h2 = s.spawn(|| helper(8, x4, x8, y3, K));
        (h1.join().unwrap(), h2.join().unwrap())
    });
    let y4 = K - y9.max(y8);
    let y2 = if K - y4 <= y3 {
        y3
    } else {
        let y5 = helper(5, x2, x5, y4, K);
        K - y4.max(y5)
    };

    let result = K - y3.max(y2);
    println!("{}", result);
}
