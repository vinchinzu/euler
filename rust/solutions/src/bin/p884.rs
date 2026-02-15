// Project Euler 884
// Recursive S(n) using cube root intervals and prefix sums.

fn integer_cbrt(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).cbrt() as i64;
    while (x + 1) * (x + 1) * (x + 1) <= n { x += 1; }
    while x * x * x > n { x -= 1; }
    x
}

const MAX_K: usize = 470_000;

fn main() {
    let n: i64 = 100_000_000_000_000_000; // 10^17

    if n <= 1 {
        println!("0");
        return;
    }

    let k_max = integer_cbrt(n - 1) as usize;

    let mut prefix_t = vec![0i64; MAX_K + 1];

    fn recursive_s(n: i64, prefix_t: &[i64]) -> i64 {
        if n <= 1 { return 0; }
        let k = integer_cbrt(n - 1);

        // Contribution from full intervals 1..k-1
        let full_intervals_sum = prefix_t[k as usize - 1];

        // Contribution from partial interval [k^3, n-1]
        let l = n - k * k * k;
        let partial_sum = l + recursive_s(l, prefix_t);

        full_intervals_sum + partial_sum
    }

    for k in 1..=k_max as i64 {
        let l_k = 3 * k * k + 3 * k + 1;
        let val = recursive_s(l_k, &prefix_t);
        let term = l_k + val;
        prefix_t[k as usize] = prefix_t[k as usize - 1] + term;
    }

    println!("{}", recursive_s(n, &prefix_t));
}
