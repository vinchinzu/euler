// Project Euler 527 - Randomized Binary Search
//
// R(n) = 2*(n+1)/n * H(n) - 3, B(n) via recursive binary search.
// H(n) uses Euler-Maclaurin asymptotic expansion.
// Answer: R(n) - B(n) for n = 10^10.

use std::collections::HashMap;

const BERNOULLI_2K: [f64; 20] = [
    1.0/6.0, -1.0/30.0, 1.0/42.0, -1.0/30.0, 5.0/66.0,
    -691.0/2730.0, 7.0/6.0, -3617.0/510.0, 43867.0/798.0, -174611.0/330.0,
    854513.0/138.0, -236364091.0/2730.0, 8553103.0/6.0, -23749461029.0/870.0,
    8615841276005.0/14322.0, -7709321041217.0/510.0, 2577687858367.0/6.0,
    -26315271553053477373.0/1919190.0, 2929993913841559.0/6.0,
    -261082718496449122051.0/13530.0,
];

fn harmonic_large(n: i64) -> f64 {
    let nd = n as f64;
    let gamma: f64 = 0.5772156649015328606065120900824024310421;
    let mut hn = nd.ln() + gamma + 1.0 / (2.0 * nd);

    let mut n_pow = nd * nd;
    for k in 1..=15 {
        hn -= BERNOULLI_2K[k - 1] / (2.0 * k as f64 * n_pow);
        n_pow *= nd * nd;
    }
    hn
}

fn b_func(n: i64, cache: &mut HashMap<i64, f64>) -> f64 {
    if n <= 1 { return 1.0; }
    if let Some(&v) = cache.get(&n) { return v; }

    let mid = (n + 1) / 2;
    let left = mid - 1;
    let right = n - mid;
    let res = 1.0 + (left as f64 * b_func(left, cache) + right as f64 * b_func(right, cache)) / n as f64;
    cache.insert(n, res);
    res
}

fn main() {
    let n: i64 = 10_000_000_000;
    let hn = harmonic_large(n);
    let r_val = 2.0 * (n + 1) as f64 / n as f64 * hn - 3.0;

    let mut cache = HashMap::new();
    let b_val = b_func(n, &mut cache);
    let ans = r_val - b_val;

    println!("{:.8}", ans);
}
