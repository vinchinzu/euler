// Project Euler 152 - Writing 1/2 as a sum of inverse squares
// Meet-in-the-middle with exact fraction arithmetic.

use std::collections::HashMap;

fn gcd_i128(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Frac { num: i128, den: i128 }

impl Frac {
    fn new(n: i128, d: i128) -> Self {
        let (n, d) = if d < 0 { (-n, -d) } else { (n, d) };
        let g = gcd_i128(n.abs(), d);
        Frac { num: n / g, den: d / g }
    }
    fn add(self, other: Self) -> Self {
        let n = self.num * other.den + other.num * self.den;
        let d = self.den * other.den;
        Frac::new(n, d)
    }
    fn le(self, other: Self) -> bool {
        self.num * other.den <= other.num * self.den
    }
    fn lt(self, other: Self) -> bool {
        self.num * other.den < other.num * self.den
    }
}

static CANDIDATES: &[i64] = &[
    2,3,4,5,6,7,8,9,10,12,13,14,15,16,18,20,21,24,27,28,30,32,35,36,39,40,42,45,48,52,54,56,60,63,64,65,70,72,80
];
const THRESHOLD: i64 = 40;

fn main() {
    let ncand = CANDIDATES.len();
    let target = Frac::new(1, 2);

    // Suffix sums for pruning
    let mut suffix = vec![Frac::new(0, 1); ncand + 1];
    for i in (0..ncand).rev() {
        let c = CANDIDATES[i];
        suffix[i] = suffix[i + 1].add(Frac::new(1, (c * c) as i128));
    }

    // Identify large candidates and build subset sum map
    let large: Vec<i64> = CANDIDATES.iter().copied().filter(|&c| c >= THRESHOLD).collect();
    let mut hash: HashMap<(i128, i128), i32> = HashMap::new();
    let nlarge = large.len();
    for mask in 0..(1u32 << nlarge) {
        let mut sum = Frac::new(0, 1);
        for j in 0..nlarge {
            if mask & (1 << j) != 0 {
                sum = sum.add(Frac::new(1, (large[j] * large[j]) as i128));
            }
        }
        *hash.entry((sum.num, sum.den)).or_insert(0) += 1;
    }

    fn search(
        idx: usize, current: Frac, target: Frac,
        suffix: &[Frac], hash: &HashMap<(i128, i128), i32>,
    ) -> i32 {
        if current == target { return 1; }
        if idx >= CANDIDATES.len() { return 0; }
        if target.lt(current) { return 0; }
        let max_possible = current.add(suffix[idx]);
        if max_possible.lt(target) { return 0; }

        if CANDIDATES[idx] >= THRESHOLD {
            let diff = target.add(Frac::new(-current.num, current.den));
            return *hash.get(&(diff.num, diff.den)).unwrap_or(&0);
        }

        let mut res = search(idx + 1, current, target, suffix, hash);
        let c = CANDIDATES[idx];
        let next = current.add(Frac::new(1, (c * c) as i128));
        if next.le(target) {
            res += search(idx + 1, next, target, suffix, hash);
        }
        res
    }

    let count = search(0, Frac::new(0, 1), target, &suffix, &hash);
    println!("{}", count);
}
