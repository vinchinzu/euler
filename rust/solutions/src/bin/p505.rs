// Project Euler 505 - Bidirectional Recurrence
// Alpha-beta on y_n with child ordering by remaining-depth parity.

const N: u64 = 1_000_000_000_000;
const K: u64 = (1u64 << 60) - 1;

#[inline(always)]
fn leaf_parent(prev_x: u64, x: u64, alpha: u64) -> u64 {
    let xl = (2 * prev_x + 3 * x) & K;
    let xr = (3 * prev_x + 2 * x) & K;
    let mx = xl.max(xr);
    if K - mx <= alpha {
        alpha
    } else {
        K - mx
    }
}

#[inline(always)]
fn depth2(prev_x: u64, x: u64, alpha: u64, beta: u64) -> u64 {
    let xl = (2 * prev_x + 3 * x) & K;
    let xr = (3 * prev_x + 2 * x) & K;
    // Remaining depth 2 is even: try the smaller-x child first.
    let (y, y2) = if xl <= xr {
        let y = leaf_parent(x, xl, K - beta);
        if K - y <= alpha {
            return alpha;
        }
        (y, leaf_parent(x, xr, y))
    } else {
        let y = leaf_parent(x, xr, K - beta);
        if K - y <= alpha {
            return alpha;
        }
        (y, leaf_parent(x, xl, y))
    };
    K - y.max(y2)
}

#[inline(always)]
fn depth3(prev_x: u64, x: u64, alpha: u64, beta: u64) -> u64 {
    let xl = (2 * prev_x + 3 * x) & K;
    let xr = (3 * prev_x + 2 * x) & K;
    // Remaining depth 3 is odd: try the larger-x child first.
    let (y, y2) = if xl >= xr {
        let y = depth2(x, xl, K - beta, K - alpha);
        if K - y <= alpha {
            return alpha;
        }
        (y, depth2(x, xr, y, K - alpha))
    } else {
        let y = depth2(x, xr, K - beta, K - alpha);
        if K - y <= alpha {
            return alpha;
        }
        (y, depth2(x, xl, y, K - alpha))
    };
    K - y.max(y2)
}

#[inline(always)]
fn depth4(prev_x: u64, x: u64, alpha: u64, beta: u64) -> u64 {
    let xl = (2 * prev_x + 3 * x) & K;
    let xr = (3 * prev_x + 2 * x) & K;
    let (y, y2) = if xl <= xr {
        let y = depth3(x, xl, K - beta, K - alpha);
        if K - y <= alpha {
            return alpha;
        }
        (y, depth3(x, xr, y, K - alpha))
    } else {
        let y = depth3(x, xr, K - beta, K - alpha);
        if K - y <= alpha {
            return alpha;
        }
        (y, depth3(x, xl, y, K - alpha))
    };
    K - y.max(y2)
}

#[inline(always)]
fn depth5(prev_x: u64, x: u64, alpha: u64, beta: u64) -> u64 {
    let xl = (2 * prev_x + 3 * x) & K;
    let xr = (3 * prev_x + 2 * x) & K;
    let (y, y2) = if xl >= xr {
        let y = depth4(x, xl, K - beta, K - alpha);
        if K - y <= alpha {
            return alpha;
        }
        (y, depth4(x, xr, y, K - alpha))
    } else {
        let y = depth4(x, xr, K - beta, K - alpha);
        if K - y <= alpha {
            return alpha;
        }
        (y, depth4(x, xl, y, K - alpha))
    };
    K - y.max(y2)
}

fn helper(k: u64, prev_x: u64, x: u64, alpha: u64, beta: u64) -> u64 {
    if k >= N {
        return x;
    }
    if 2 * k >= N {
        return leaf_parent(prev_x, x, alpha);
    }
    if 4 * k >= N {
        return depth2(prev_x, x, alpha, beta);
    }
    if 8 * k >= N {
        return depth3(prev_x, x, alpha, beta);
    }
    if 16 * k >= N {
        return depth4(prev_x, x, alpha, beta);
    }
    if 32 * k >= N {
        return depth5(prev_x, x, alpha, beta);
    }
    let xl = (2 * prev_x + 3 * x) & K;
    let xr = (3 * prev_x + 2 * x) & K;
    // floor(log2(k)) even <=> remaining depth even (N in (2^39, 2^40]).
    // Odd remaining depth: larger x first; even: smaller x first.
    let odd = (k.ilog2() & 1) == 1;
    let left_first = if odd { xl >= xr } else { xl <= xr };
    let (y, y2) = if left_first {
        let y = helper(2 * k, x, xl, K - beta, K - alpha);
        if K - y <= alpha {
            return alpha;
        }
        (y, helper(2 * k + 1, x, xr, y, K - alpha))
    } else {
        let y = helper(2 * k + 1, x, xr, K - beta, K - alpha);
        if K - y <= alpha {
            return alpha;
        }
        (y, helper(2 * k, x, xl, y, K - alpha))
    };
    K - y.max(y2)
}

fn main() {
    let result = helper(1, 0, 1, 0, K);
    println!("{}", result);
}
