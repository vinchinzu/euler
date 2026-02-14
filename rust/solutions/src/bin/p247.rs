use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(PartialEq)]
struct Entry {
    s: f64,
    ix: i32,
    iy: i32,
    x0: f64,
    y0: f64,
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.s.partial_cmp(&other.s)
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

fn make_entry(ix: i32, iy: i32, x0: f64, y0: f64) -> Entry {
    let s = ((x0 * x0 + y0 * y0 - 2.0 * x0 * y0 + 4.0).sqrt() - (x0 + y0)) / 2.0;
    Entry { s, ix, iy, x0, y0 }
}

fn comb(n: i32, k: i32) -> i32 {
    if k > n { return 0; }
    if k == 0 || k == n { return 1; }
    let mut r = 1;
    for i in 0..k {
        r = r * (n - i) / (i + 1);
    }
    r
}

fn main() {
    let ix_target = 3;
    let iy_target = 3;
    let mut num_at_index = comb(ix_target + iy_target, ix_target);

    let mut heap = BinaryHeap::new();
    heap.push(make_entry(0, 0, 1.0, 0.0));

    let mut ans = 0;

    while num_at_index > 0 {
        let e = heap.pop().unwrap();
        let s = e.s;

        if e.ix == ix_target && e.iy == iy_target {
            num_at_index -= 1;
        }

        heap.push(make_entry(e.ix + 1, e.iy, e.x0 + s, e.y0));
        heap.push(make_entry(e.ix, e.iy + 1, e.x0, e.y0 + s));
        ans += 1;
    }

    println!("{}", ans);
}
