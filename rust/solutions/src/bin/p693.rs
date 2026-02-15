// Project Euler 693 - Finite Sequence Generator
// Divide-and-conquer with caching to find max g(x) for x <= N.

const CACHE_SIZE: usize = 4_000_000;

fn compute_g(x: usize, cache: &mut [i32]) -> i32 {
    if x <= 2 { return 0; }
    if x < CACHE_SIZE && cache[x] != -1 { return cache[x]; }

    let mut used = vec![0i32; 2 * x + 100];
    let mut ys: Vec<usize> = (2..x).collect();

    let mut z = x;
    loop {
        if ys.is_empty() {
            let result = (z - x + 1) as i32;
            if x < CACHE_SIZE { cache[x] = result; }
            return result;
        }

        let mut new_ys = Vec::new();
        for &y in &ys {
            let val = ((y as u64 * y as u64) % z as u64) as usize;
            if val > 1 && used[val] != z as i32 {
                new_ys.push(val);
            }
            used[val] = z as i32;
        }

        ys = new_ys;
        z += 1;
    }
}

fn helper(low: usize, high: usize, depth: i32, global_best: &mut i32, cache: &mut [i32]) {
    if low >= high { return; }

    let g_high = compute_g(high, cache);
    if g_high > *global_best {
        *global_best = g_high;
    }

    if low + 1 == high || depth == 0 { return; }
    if *global_best >= g_high + (high - low) as i32 { return; }

    let mid = (low + high) / 2;
    helper(low, mid, depth - 1, global_best, cache);
    helper(mid, high, depth - 1, global_best, cache);
}

fn main() {
    let big_n = 3_000_000;
    let mut global_best = 0i32;
    let mut cache = vec![-1i32; CACHE_SIZE];

    let mut depth = 1;
    while (1u64 << depth) < big_n as u64 {
        helper(0, big_n, depth, &mut global_best, &mut cache);
        depth += 1;
    }

    println!("{}", global_best);
}
