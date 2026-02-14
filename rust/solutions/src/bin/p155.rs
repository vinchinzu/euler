// Project Euler 155 - Counting Capacitor Circuits
// Count distinct capacitance values with N=18 unit capacitors.
use std::collections::HashSet;

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

const MAXN: usize = 18;

fn main() {
    let mut exact: Vec<HashSet<(i32, i32)>> = vec![HashSet::new(); MAXN + 1];

    exact[1].insert((1, 1));

    for k in 2..=MAXN {
        let mut found = HashSet::new();

        for i in 1..=k / 2 {
            let j = k - i;
            // Collect items from exact[i] and exact[j]
            let l1: Vec<(i32, i32)> = exact[i].iter().copied().collect();
            let l2: Vec<(i32, i32)> = exact[j].iter().copied().collect();

            if i == j {
                for a in 0..l1.len() {
                    let (n1, d1) = l1[a];
                    for b in a..l1.len() {
                        let (n2, d2) = l1[b];
                        let num = n1 as i64 * d2 as i64 + n2 as i64 * d1 as i64;
                        let den = d1 as i64 * d2 as i64;
                        let g = gcd(num.abs(), den.abs());
                        let sn = (num / g) as i32;
                        let sd = (den / g) as i32;
                        found.insert((sn, sd));
                        found.insert((sd, sn));
                    }
                }
            } else {
                for &(n1, d1) in &l1 {
                    for &(n2, d2) in &l2 {
                        let num = n1 as i64 * d2 as i64 + n2 as i64 * d1 as i64;
                        let den = d1 as i64 * d2 as i64;
                        let g = gcd(num.abs(), den.abs());
                        let sn = (num / g) as i32;
                        let sd = (den / g) as i32;
                        found.insert((sn, sd));
                        found.insert((sd, sn));
                    }
                }
            }
        }

        exact[k] = found;
    }

    let mut all = HashSet::new();
    for k in 1..=MAXN {
        for &item in &exact[k] {
            all.insert(item);
        }
    }

    println!("{}", all.len());
}
