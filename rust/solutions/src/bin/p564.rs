// Project Euler 564 - Expected Maximal Polygon Area
//
// Enumerate all compositions of n sides summing to 2n-3, compute the maximal
// cyclic polygon area for each, weight by multinomial count, and average over
// C(2n-4, n-1) total compositions. Sum E(n) for n=3..50.

use std::f64::consts::PI;

#[derive(Clone, Copy)]
struct Side {
    val: i32,
    count: i32,
}

fn ffactorial(n: i32) -> f64 {
    let mut r = 1.0;
    for i in 2..=n {
        r *= i as f64;
    }
    r
}

fn fn_cr(n: i32, r: i32) -> f64 {
    if r < 0 || r > n {
        return 0.0;
    }
    let r = r.min(n - r);
    let mut result = 1.0;
    for i in 0..r {
        result = result * (n - i) as f64 / (i + 1) as f64;
    }
    result
}

fn feq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-10
}

fn center_outside(sides: &[Side]) -> bool {
    let nsides = sides.len();
    if sides[nsides - 1].count > 1 {
        return false;
    }
    let max_side = sides[nsides - 1].val as f64;
    let mut angle = 0.0;
    for s in sides {
        angle += s.count as f64 * (s.val as f64 / max_side).asin();
    }
    angle < PI
}

fn helper(n: i32, rem_sides: i32, rem_perim: i32, sides: &mut Vec<Side>, ans: &mut f64) {
    if rem_sides == 0 {
        if rem_perim != 0 {
            return;
        }

        let nsides = sides.len();
        let co = center_outside(sides);
        let mut low = sides[nsides - 1].val as f64;
        let mut high = 2.0 * n as f64;
        let mut prev;

        while !feq(low, high) {
            let mid = (low + high) / 2.0;
            let mut angle = 0.0;
            prev = 0.0;
            for s in sides.iter() {
                prev = s.count as f64 * (s.val as f64 / mid).asin();
                angle += prev;
            }
            if co {
                angle = PI + 2.0 * prev - angle;
            }
            if angle > PI {
                low = mid;
            } else {
                high = mid;
            }
        }

        let mut area = 0.0;
        prev = 0.0;
        for s in sides.iter() {
            prev = s.count as f64 * s.val as f64 * (low * low - (s.val as f64) * (s.val as f64)).sqrt() / 4.0;
            area += prev;
        }
        if co {
            area -= 2.0 * prev;
        }

        area *= ffactorial(n);
        for s in sides.iter() {
            area /= ffactorial(s.count);
        }
        *ans += area / fn_cr(2 * n - 4, n - 1);
        return;
    }

    let start_val = if sides.is_empty() { 1 } else { sides.last().unwrap().val + 1 };
    for val in start_val..=(rem_perim / rem_sides) {
        for count in 1..=rem_sides {
            if val * count <= rem_perim {
                sides.push(Side { val, count });
                helper(n, rem_sides - count, rem_perim - val * count, sides, ans);
                sides.pop();
            }
        }
    }
}

fn main() {
    let mut ans = 0.0;
    let mut sides = Vec::new();

    for n in 3..=50 {
        helper(n, n, 2 * n - 3, &mut sides, &mut ans);
    }

    println!("{:.6}", ans);
}
