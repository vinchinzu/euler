// Project Euler 693 - Finite Sequence Generator
// Divide-and-conquer with caching to find max g(x) for x <= N.
// Optimized: reuse used[] / ys buffers; monotonic generation stamps (no zeroing).

const CACHE_SIZE: usize = 4_000_000;

struct State {
    used: Vec<i32>,
    ys: Vec<usize>,
    new_ys: Vec<usize>,
    stamp_gen: i32,
    cache: Vec<i32>,
}

impl State {
    fn new() -> Self {
        Self {
            used: Vec::new(),
            ys: Vec::new(),
            new_ys: Vec::new(),
            stamp_gen: 0,
            cache: vec![-1i32; CACHE_SIZE],
        }
    }

    fn compute_g(&mut self, x: usize) -> i32 {
        if x <= 2 {
            return 0;
        }
        if x < CACHE_SIZE && self.cache[x] != -1 {
            return self.cache[x];
        }

        let need = 2 * x + 100;
        if self.used.len() < need {
            self.used.resize(need, 0);
        }

        self.ys.clear();
        self.ys.extend(2..x);

        let mut z = x;
        loop {
            if self.ys.is_empty() {
                let result = (z - x + 1) as i32;
                if x < CACHE_SIZE {
                    self.cache[x] = result;
                }
                return result;
            }

            self.stamp_gen += 1;
            let stamp = self.stamp_gen;
            self.new_ys.clear();
            for &y in &self.ys {
                let val = ((y as u64 * y as u64) % z as u64) as usize;
                if val > 1 && self.used[val] != stamp {
                    self.new_ys.push(val);
                }
                self.used[val] = stamp;
            }
            std::mem::swap(&mut self.ys, &mut self.new_ys);
            z += 1;
        }
    }

    fn helper(&mut self, low: usize, high: usize, depth: i32, global_best: &mut i32) {
        if low >= high {
            return;
        }

        let g_high = self.compute_g(high);
        if g_high > *global_best {
            *global_best = g_high;
        }

        if low + 1 == high || depth == 0 {
            return;
        }
        if *global_best >= g_high + (high - low) as i32 {
            return;
        }

        let mid = (low + high) / 2;
        self.helper(low, mid, depth - 1, global_best);
        self.helper(mid, high, depth - 1, global_best);
    }
}

fn main() {
    let big_n = 3_000_000;
    let mut global_best = 0i32;
    let mut state = State::new();

    let mut depth = 1;
    while (1u64 << depth) < big_n as u64 {
        state.helper(0, big_n, depth, &mut global_best);
        depth += 1;
    }

    println!("{}", global_best);
}
