// Project Euler 297: Zeckendorf Representation
use std::collections::HashMap;

fn main() {
    let n_val: i64 = 100_000_000_000_000_000; // 10^17

    // Generate Fibonacci sequence
    let mut fibs = vec![1i64, 2];
    while *fibs.last().unwrap() < n_val {
        let len = fibs.len();
        fibs.push(fibs[len - 1] + fibs[len - 2]);
    }

    let mut cache: HashMap<i64, i64> = HashMap::new();

    fn z(n: i64, fibs: &[i64], cache: &mut HashMap<i64, i64>) -> i64 {
        if n <= 1 { return 0; }
        if let Some(&val) = cache.get(&n) { return val; }

        // Find largest Fibonacci number < n
        let mut a = 1i64;
        for &f in fibs {
            if f < n { a = f; }
            else { break; }
        }

        let result = z(a, fibs, cache) + (n - a) + z(n - a, fibs, cache);
        cache.insert(n, result);
        result
    }

    println!("{}", z(n_val, &fibs, &mut cache));
}
