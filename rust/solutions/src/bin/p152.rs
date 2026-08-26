// Project Euler 152: 1/2 as a sum of inverse squares.
// L = lcm of candidates; work with integers (L/n)^2 and meet-in-the-middle.

use fxhash::FxHashMap;

static CANDIDATES: &[u64] = &[
    2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 18, 20, 21, 24, 27, 28, 30, 32, 35,
    36, 39, 40, 42, 45, 48, 52, 54, 56, 60, 63, 64, 65, 70, 72, 80,
];

fn main() {
    const L: u64 = 786_240;
    let vals: Vec<u64> = CANDIDATES.iter().map(|&c| (L / c) * (L / c)).collect();
    let target = (L / 2) * L; // L^2 / 2

    let mid = 20;
    let left = &vals[..mid];
    let right = &vals[mid..];

    let mut map: FxHashMap<u64, i32> = FxHashMap::default();
    let nleft = left.len();
    for mask in 0..(1u32 << nleft) {
        let mut sum = 0u64;
        for j in 0..nleft {
            if mask & (1 << j) != 0 {
                sum += left[j];
            }
        }
        *map.entry(sum).or_insert(0) += 1;
    }

    let mut count: i32 = 0;
    let nright = right.len();
    for mask in 0..(1u32 << nright) {
        let mut sum = 0u64;
        for j in 0..nright {
            if mask & (1 << j) != 0 {
                sum += right[j];
            }
        }
        if sum <= target {
            if let Some(&c) = map.get(&(target - sum)) {
                count += c;
            }
        }
    }
    println!("{}", count);
}
