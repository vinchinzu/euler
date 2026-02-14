// Project Euler 203: Squarefree Binomial Coefficients
use std::collections::HashSet;

fn is_squarefree(mut n: i64) -> bool {
    if n <= 1 { return true; }
    let mut d = 2i64;
    while d * d <= n {
        if n % d == 0 {
            n /= d;
            if n % d == 0 { return false; }
        }
        d += 1;
    }
    true
}

fn main() {
    let n = 51;
    let mut binom = vec![vec![0i64; n]; n];

    for i in 0..n {
        binom[i][0] = 1;
        binom[i][i] = 1;
        for j in 1..i {
            binom[i][j] = binom[i - 1][j - 1] + binom[i - 1][j];
        }
    }

    let mut vals: HashSet<i64> = HashSet::new();
    for i in 0..n {
        for j in 0..=i / 2 {
            let v = binom[i][j];
            if is_squarefree(v) {
                vals.insert(v);
            }
        }
    }

    let ans: i64 = vals.iter().sum();
    println!("{}", ans);
}
