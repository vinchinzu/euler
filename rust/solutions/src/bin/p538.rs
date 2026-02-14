// Project Euler 538 - Maximum Quadrilateral
//
// Maintain sorted set of u values, track best 4 consecutive for
// Brahmagupta's formula. Uses sqrt-decomposition sorted list.

const BLOCK_SIZE: usize = 2000;
const MAX_BLOCKS: usize = 3000;

struct SortedBlocks {
    blocks: Vec<Vec<i64>>,
    total: usize,
}

impl SortedBlocks {
    fn new() -> Self {
        SortedBlocks { blocks: vec![Vec::new()], total: 0 }
    }

    fn insert(&mut self, val: i64) -> (usize, usize) {
        // Find which block
        let mut bi = 0;
        for b in 0..self.blocks.len() {
            if !self.blocks[b].is_empty() && *self.blocks[b].last().unwrap() < val {
                bi = b + 1;
            } else {
                bi = b;
                break;
            }
        }
        if bi >= self.blocks.len() { bi = self.blocks.len() - 1; }

        // Binary search in block
        let pos = self.blocks[bi].partition_point(|&x| x < val);
        self.blocks[bi].insert(pos, val);
        self.total += 1;

        let ret = (bi, pos);

        // Split if too large
        if self.blocks[bi].len() > BLOCK_SIZE * 3 / 2 {
            let half = self.blocks[bi].len() / 2;
            let new_block = self.blocks[bi].split_off(half);
            self.blocks.insert(bi + 1, new_block);
        }

        ret
    }

    fn global_index(&self, block: usize, pos: usize) -> usize {
        let mut idx = pos;
        for b in 0..block { idx += self.blocks[b].len(); }
        idx
    }

    fn get_at(&self, mut idx: usize) -> i64 {
        for b in &self.blocks {
            if idx < b.len() { return b[idx]; }
            idx -= b.len();
        }
        -1
    }
}

fn ipow(base: i64, exp: u32) -> i64 {
    let mut result = 1i64;
    for _ in 0..exp { result *= base; }
    result
}

fn main() {
    let n = 3_000_000usize;
    let k = 4usize;

    let mut sb = SortedBlocks::new();

    let mut max_area2 = 0.0f64;
    let mut best_perim = 0i64;
    let mut best_min_side = 0i64;
    let mut ans = 0i64;

    for nn in 1..=n {
        let b3n = ((3 * nn) as u64).count_ones();
        let b2n = ((2 * nn) as u64).count_ones();
        let bn1 = ((nn + 1) as u64).count_ones();
        let u = ipow(2, b3n) + ipow(3, b2n) + bn1 as i64;

        let (ins_block, ins_pos) = sb.insert(u);

        if sb.total < k {
            ans += best_perim;
            continue;
        }

        if u >= best_min_side {
            let gidx = sb.global_index(ins_block, ins_pos);

            let start = if gidx >= k - 1 { gidx - (k - 1) } else { 0 };
            let mut end = gidx;
            if end + k - 1 >= sb.total { end = sb.total - k; }
            if end < start { continue; }

            for i in start..=end {
                if i + k - 1 >= sb.total { break; }
                let mut sides = [0i64; 4];
                for j in 0..k { sides[j] = sb.get_at(i + j); }
                let perim = sides[0] + sides[1] + sides[2] + sides[3];

                let mut area2 = 1.0f64;
                for j in 0..k {
                    area2 *= (perim - 2 * sides[j]) as f64;
                }

                if area2 >= max_area2 {
                    max_area2 = area2;
                    best_perim = perim;
                    best_min_side = sides[0];
                }
            }
        }

        ans += best_perim;
    }

    println!("{ans}");
}
