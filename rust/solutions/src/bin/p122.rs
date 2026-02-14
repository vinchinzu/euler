// Project Euler 122 - Efficient exponentiation
// Find sum of m(k) for k=1..200, where m(k) is minimum multiplications
// to compute n^k using addition chains.
// Uses iterative deepening DFS (IDDFS).

const LIMIT: usize = 200;

fn dfs(depth: usize, max_depth: usize, chain: &mut Vec<usize>, m: &mut [usize; LIMIT + 1]) {
    let top = *chain.last().unwrap();

    if top > LIMIT { return; }

    if depth < m[top] {
        m[top] = depth;
    }

    if depth == max_depth { return; }

    // Try all additions using the top element (star chain)
    for i in (0..chain.len()).rev() {
        let new_val = top + chain[i];
        if new_val > LIMIT { continue; }

        chain.push(new_val);
        dfs(depth + 1, max_depth, chain, m);
        chain.pop();
    }
}

fn main() {
    let mut m = [999usize; LIMIT + 1];
    m[1] = 0;

    let mut chain = vec![1usize];

    for max_depth in 1..=11 {
        dfs(0, max_depth, &mut chain, &mut m);
    }

    let total: usize = m[1..=LIMIT].iter().sum();
    println!("{}", total);
}
