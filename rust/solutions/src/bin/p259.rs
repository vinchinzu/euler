// Project Euler 259: Reachable Numbers
use std::collections::HashSet;

fn gcd_func(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Frac { num: i64, den: i64 }

fn make_frac(mut n: i64, mut d: i64) -> Frac {
    if d < 0 { n = -n; d = -d; }
    if n == 0 { d = 1; }
    else { let g = gcd_func(n, d); n /= g; d /= g; }
    Frac { num: n, den: d }
}

fn try_add(set: &mut HashSet<Frac>, n: i128, d: i128) {
    if d == 0 { return; }
    let (mut n, mut d) = (n, d);
    let an = n.abs();
    let ad = d.abs();
    let mut ga = an;
    let mut gb = ad;
    while gb != 0 { let t = gb; gb = ga % gb; ga = t; }
    if ga > 0 { n /= ga; d /= ga; }
    if n.abs() > 4_000_000_000_000_000_000 || d.abs() > 4_000_000_000_000_000_000 { return; }
    set.insert(make_frac(n as i64, d as i64));
}

fn add_ops(set: &mut HashSet<Frac>, a: Frac, b: Frac) {
    // addition
    try_add(set,
        a.num as i128 * b.den as i128 + b.num as i128 * a.den as i128,
        a.den as i128 * b.den as i128);
    // subtraction
    try_add(set,
        a.num as i128 * b.den as i128 - b.num as i128 * a.den as i128,
        a.den as i128 * b.den as i128);
    // multiplication
    {
        let g1 = gcd_func(a.num, b.den);
        let g2 = gcd_func(b.num, a.den);
        try_add(set,
            (a.num / g1) as i128 * (b.num / g2) as i128,
            (a.den / g2) as i128 * (b.den / g1) as i128);
    }
    // division
    if b.num != 0 {
        let g1 = gcd_func(a.num, b.num);
        let g2 = gcd_func(a.den, b.den);
        try_add(set,
            (a.num / g1) as i128 * (b.den / g2) as i128,
            (a.den / g2) as i128 * (b.num / g1) as i128);
    }
}

fn main() {
    let base = 10usize;
    // sets[start][end]
    let mut sets: Vec<Vec<HashSet<Frac>>> = vec![vec![HashSet::new(); base]; base - 1];

    // Single digits
    for i in 0..base - 1 {
        sets[i][i + 1].insert(make_frac(i as i64 + 1, 1));
    }

    // Build up longer sequences
    for length in 2..base {
        for start in 0..base - length {
            let end = start + length;

            // Concatenated number
            let mut concat = 0i64;
            for d in start..end {
                concat = concat * 10 + (d as i64 + 1);
            }

            let mut cur = HashSet::new();
            cur.insert(make_frac(concat, 1));

            // Try all splits
            for left in 1..length {
                let mid = start + left;
                let lset: Vec<Frac> = sets[start][mid].iter().cloned().collect();
                let rset: Vec<Frac> = sets[mid][end].iter().cloned().collect();
                for &fa in &lset {
                    for &fb in &rset {
                        add_ops(&mut cur, fa, fb);
                    }
                }
            }

            sets[start][end] = cur;
        }
    }

    let ans: i64 = sets[0][base - 1].iter()
        .filter(|f| f.den == 1 && f.num > 0)
        .map(|f| f.num)
        .sum();

    println!("{}", ans);
}
