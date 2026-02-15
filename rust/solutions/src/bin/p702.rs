// Project Euler 702 - Jumping Flea
//
// Sum of J(T) for all upward facing triangles in the top half of a hexagon
// with side length N. Uses recursive inversion counting on sequences modulo
// powers of 2.

fn tr(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn iceil_div(a: i64, b: i64) -> i64 {
    (a + b - 1) / b
}

fn num_inversions(n: i64, m: i64) -> i64 {
    if n == 0 {
        return 0;
    }
    let big = m % n;
    let small = n - big;
    let mut count = (n * (n - 1) / 2) * tr(m / n);
    if big > 0 {
        count += num_inversions(n % big, big) * iceil_div(m, n);
    }
    if small > 0 {
        count -= num_inversions(n % small, small) * (m / n);
    }
    count
}

fn num_points_in_shaded(n: i64, point_dist: i64) -> i64 {
    (point_dist - 2) * (point_dist - 1) - num_inversions(n % point_dist, point_dist)
}

fn main() {
    let n: i64 = 123456789;

    // Find smallest power of 2 >= n
    let mut l: i64 = 1;
    while l < n {
        l *= 2;
    }

    let mut nsj: Vec<i64> = Vec::new();

    let mut k: i64 = 1;
    while k < l {
        nsj.push(num_points_in_shaded(n, k));
        k *= 2;
    }

    // For k == l
    let lmod = ((-l % n) + n) % n;
    nsj.push(num_points_in_shaded(n, l) - 2 * num_points_in_shaded(lmod, l - n));

    // Final value
    nsj.push(2 * tr(n) + tr(n - 1));

    let mut ans: i64 = 0;
    for j in 1..nsj.len() {
        ans += j as i64 * (nsj[j] - nsj[j - 1]);
    }

    println!("{}", ans);
}
