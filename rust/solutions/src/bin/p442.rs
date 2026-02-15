use std::collections::HashMap;

const K: i64 = 11;
const B: usize = 10;
const MAX_DIGITS: usize = 20;

struct Solver {
    power_strs: Vec<Vec<u8>>,
    cache: HashMap<(Vec<u8>, usize), i64>,
}

impl Solver {
    fn new() -> Self {
        let mut power_strs = Vec::new();
        let mut pw = 1i64;
        loop {
            let s: Vec<u8> = pw.to_string().bytes().collect();
            power_strs.push(s);
            if pw > 9_000_000_000_000_000_000 / K { break; }
            pw *= K;
        }
        Solver { power_strs, cache: HashMap::new() }
    }

    fn ends_with(hay: &[u8], needle: &[u8]) -> bool {
        if needle.len() > hay.len() { return false; }
        &hay[hay.len() - needle.len()..] == needle
    }

    fn num_eleven_frees(&mut self, prefix: &[u8], num_remaining: usize) -> i64 {
        // Find relevant prefix
        let mut rel: Vec<u8> = Vec::new();

        for pi in 0..self.power_strs.len() {
            if pi > 0 && self.power_strs[pi].len() <= prefix.len() {
                if Self::ends_with(prefix, &self.power_strs[pi]) {
                    return 0;
                }
            }
            for tlen in (rel.len() + 1..self.power_strs[pi].len()).rev() {
                if tlen <= prefix.len() && Self::ends_with(prefix, &self.power_strs[pi][..tlen]) {
                    rel = self.power_strs[pi][..tlen].to_vec();
                    break;
                }
            }
        }

        if num_remaining == 0 { return 1; }

        let key = (rel.clone(), num_remaining);
        if let Some(&v) = self.cache.get(&key) { return v; }

        let mut res = 0i64;
        for d in 0..B as u8 {
            let mut new_prefix = prefix.to_vec();
            new_prefix.push(b'0' + d);
            res += self.num_eleven_frees(&new_prefix, num_remaining - 1);
        }

        self.cache.insert(key, res);
        res
    }
}

fn main() {
    let n_val: i64 = 1_000_000_000_000_000_000;
    let l = MAX_DIGITS;

    let mut solver = Solver::new();
    let mut ans: i64 = 0;
    let mut n = n_val;

    for num_remaining in (0..=l).rev() {
        for d in 0..B as u8 {
            let val = ans * B as i64 + d as i64;
            let prefix: Vec<u8> = val.to_string().bytes().collect();
            let count = solver.num_eleven_frees(&prefix, num_remaining);
            if count > n {
                ans = ans * B as i64 + d as i64;
                break;
            }
            n -= count;
        }
    }

    println!("{}", ans);
}
