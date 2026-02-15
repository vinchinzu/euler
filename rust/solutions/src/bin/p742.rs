// Project Euler 742 - Minimum Area of a Convex Grid Polygon
//
// Branch-and-bound over grid polygon edge vectors.

const MAX_SIDES: usize = 500;
const MAX_BRANCHES: usize = 2000;
const MAX_NUM_SIDES: usize = 250;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Branch {
    x: i32,
    twice_area: i32,
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    let n = 1000;
    let l = 40;

    let mut sides: Vec<Point> = Vec::new();
    for x in 1..=l {
        for y in 1..x {
            if gcd(x, y) == 1 {
                sides.push(Point { x, y });
            }
        }
    }
    sides.sort_by(|a, b| {
        let aa = (a.y as f64).atan2(a.x as f64);
        let ab = (b.y as f64).atan2(b.x as f64);
        aa.partial_cmp(&ab).unwrap()
    });

    let mut branches_a: Vec<Vec<Branch>> = vec![Vec::new(); MAX_NUM_SIDES];
    branches_a[0] = vec![Branch { x: 0, twice_area: 0 }];

    for si in 0..sides.len() {
        let sx = sides[si].x;
        let sy = sides[si].y;

        branches_a[0] = vec![Branch { x: 0, twice_area: 0 }];

        let mut branches_b: Vec<Vec<Branch>> = vec![Vec::new(); MAX_NUM_SIDES];

        for ns in 1..n / 4 {
            let prev = &branches_a[ns];
            let prev_m1 = &branches_a[ns - 1];

            let curr: Vec<Branch> = prev_m1
                .iter()
                .map(|b| Branch {
                    x: b.x + sx,
                    twice_area: b.twice_area + sy * (2 * b.x + sx + 1),
                })
                .collect();

            // Merge prev and curr by x, keeping Pareto-optimal (minimum twice_area)
            let mut merged: Vec<Branch> = Vec::new();
            let (mut i, mut j) = (0, 0);
            while i < prev.len() || j < curr.len() {
                let b = if i < prev.len() && j < curr.len() {
                    if prev[i].x < curr[j].x
                        || (prev[i].x == curr[j].x && prev[i].twice_area <= curr[j].twice_area)
                    {
                        i += 1;
                        prev[i - 1]
                    } else {
                        j += 1;
                        curr[j - 1]
                    }
                } else if i < prev.len() {
                    i += 1;
                    prev[i - 1]
                } else {
                    j += 1;
                    curr[j - 1]
                };
                if merged.is_empty() || b.twice_area < merged.last().unwrap().twice_area {
                    merged.push(b);
                }
            }

            branches_b[ns] = merged;
        }

        for ns in 1..n / 4 {
            branches_a[ns] = branches_b[ns].clone();
        }
    }

    let mut ans: i64 = i64::MAX;
    for ns1 in 1..n / 8 {
        let ns2 = n / 4 - 2 - ns1;
        if ns2 >= MAX_NUM_SIDES {
            continue;
        }
        for bi in &branches_a[ns1] {
            for bj in &branches_a[ns2] {
                let area = 2i64 * bi.twice_area as i64
                    + 2 * bj.twice_area as i64
                    + 4 * bi.x as i64 * bj.x as i64
                    + 6 * bi.x as i64
                    + 6 * bj.x as i64
                    + 7;
                if area < ans {
                    ans = area;
                }
            }
        }
    }

    println!("{}", ans);
}
