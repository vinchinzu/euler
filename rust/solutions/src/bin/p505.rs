// Project Euler 505 - Bidirectional Recurrence
// Recursive divide-and-conquer on a binary tree structure.

const N: u64 = 1_000_000_000_000;
const K: u64 = (1u64 << 60) - 1;

fn helper(k: u64, prev_x: u64, x: u64, alpha: u64, beta: u64) -> u64 {
    if k >= N {
        return x;
    }
    let y = helper(2 * k, x, (2 * prev_x + 3 * x) & K, K - beta, K - alpha);
    if K - y <= alpha {
        return alpha;
    }
    let y2 = helper(2 * k + 1, x, (3 * prev_x + 2 * x) & K, y, K - alpha);
    K - std::cmp::max(y, y2)
}

fn main() {
    let result = helper(1, 0, 1, 0, K);
    println!("{}", result);
}
