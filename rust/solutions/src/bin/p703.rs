// Project Euler 703 - Circular Logic III
//
// Build a functional graph from f on N-bit boolean tuples, count independent
// set colorings on each connected component using tree DP.

const NN: usize = 20;
const SZ: usize = 1 << NN;
const MOD: i64 = 1_001_001_011;

struct FT {
    f: i64,
    t: i64,
}

fn main() {
    // Build next array
    let mut next_arr = vec![0usize; SZ];
    for i in 0..SZ {
        let shifted = i >> 1;
        let b1 = i & 1;
        let b2 = (i >> 1) & 1;
        let b3 = (i >> 2) & 1;
        let highest_bit = b1 & (b2 ^ b3);
        next_arr[i] = shifted + (highest_bit << (NN - 1));
    }

    // Build reverse graph
    let mut prevs: Vec<Vec<usize>> = vec![Vec::new(); SZ];
    for i in 0..SZ {
        prevs[next_arr[i]].push(i);
    }

    let mut used = vec![false; SZ];
    let mut ans: i64 = 1;

    // Stack-based DFS helper
    struct Frame {
        ptr: usize,
        avoid: usize,
        child_idx: i32,
        f_val: i64,
        t_val: i64,
    }

    let mut stack: Vec<Frame> = Vec::with_capacity(SZ);

    let mut helper = |start_ptr: usize, start_avoid: usize, used: &mut Vec<bool>, prevs: &Vec<Vec<usize>>| -> FT {
        stack.clear();
        stack.push(Frame {
            ptr: start_ptr,
            avoid: start_avoid,
            child_idx: -1,
            f_val: 1,
            t_val: 1,
        });

        loop {
            let sp = stack.len() - 1;
            let ptr = stack[sp].ptr;

            if stack[sp].child_idx == -1 {
                used[ptr] = true;
                if ptr == stack[sp].avoid {
                    let res = FT { f: 1, t: 0 };
                    if sp == 0 {
                        return res;
                    }
                    stack.pop();
                    let parent = stack.last_mut().unwrap();
                    parent.f_val = parent.f_val * ((res.f + res.t) % MOD) % MOD;
                    parent.t_val = parent.t_val * res.f % MOD;
                    parent.child_idx += 1;
                    continue;
                }
                stack[sp].child_idx = 0;
            }

            let ci = stack[sp].child_idx as usize;
            if ci < prevs[ptr].len() {
                let child = prevs[ptr][ci];
                let avoid = stack[sp].avoid;
                stack.push(Frame {
                    ptr: child,
                    avoid,
                    child_idx: -1,
                    f_val: 1,
                    t_val: 1,
                });
                continue;
            }

            // All children processed
            let res = FT {
                f: stack[sp].f_val % MOD,
                t: stack[sp].t_val % MOD,
            };
            if sp == 0 {
                return res;
            }
            stack.pop();
            let parent = stack.last_mut().unwrap();
            parent.f_val = parent.f_val * ((res.f + res.t) % MOD) % MOD;
            parent.t_val = parent.t_val * res.f % MOD;
            parent.child_idx += 1;
        }
    };

    for i in 0..SZ {
        if !used[i] {
            // Find root of cycle
            let mut root = i;
            while !used[root] {
                used[root] = true;
                root = next_arr[root];
            }

            // Reset used for DFS
            // Root is false: all subtrees independent
            let mut f_val: i64 = 1;
            let prev_root: Vec<usize> = prevs[root].clone();
            for &child in &prev_root {
                let res = helper(child, root, &mut used, &prevs);
                f_val = f_val * ((res.f + res.t) % MOD) % MOD;
            }

            // Root is true: next[root] must be false
            let t_val;
            if next_arr[root] == root {
                t_val = 0;
            } else {
                let nr = next_arr[root];
                let r1 = helper(nr, root, &mut used, &prevs);
                let r2 = helper(root, nr, &mut used, &prevs);
                t_val = r1.f * r2.t % MOD;
            }

            ans = ans * ((f_val + t_val) % MOD) % MOD;
        }
    }

    println!("{}", ans);
}
