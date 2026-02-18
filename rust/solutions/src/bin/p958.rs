// Problem 958 - Euclid's Labour
//
// We use the subtraction-only Euclidean algorithm. For coprime inputs (n, m),
// the algorithm ends at (1, 1). Reversing one subtraction step turns (a, b)
// into either (a+b, a) or (a+b, b) (keeping the larger component first).
//
// So d(n, m) is exactly the length of the unique path in this "addition" tree
// from (n, m) down to (1, 1). To minimize d(n, m) for fixed n, we search
// backwards from the penultimate state (2, 1) and look for the first time
// we can reach a state (n, m). If several m appear at that same minimal depth,
// we take the smallest m.

fn f(n: i64) -> i64 {
    assert!(n >= 2);

    // Precompute Fibonacci numbers. We need around 90 for n ~ 1e12.
    let mut fib = [0i64; 202];
    fib[1] = 1;
    for i in 2..202 {
        fib[i] = fib[i - 1].saturating_add(fib[i - 2]);
    }

    // Find starting depth: smallest depth such that fib[depth+3] >= n
    let mut depth: usize = 0;
    while fib[depth + 3] < n {
        depth += 1;
    }

    let n128 = n as i128;

    loop {
        let mut best_m: i64 = i64::MAX;

        // Stack entries: (x, y, p, q, rem)
        // Invariant: x > y >= 1, gcd(x,y)=1, and p*x + q*y = 1.
        let mut stack: Vec<(i64, i64, i64, i64, usize)> = Vec::with_capacity(1 << 16);
        stack.push((2, 1, 0, 1, depth));

        while let Some((x, y, p, q, rem)) = stack.pop() {
            // y only grows in descendants; prune if already >= best.
            if y >= best_m {
                continue;
            }

            if x == n {
                best_m = y;
                continue;
            }

            if rem == 0 || x > n {
                continue;
            }

            // Slow-growth lower bound: x grows by at least y per step.
            if x + (rem as i64) * y > n {
                continue;
            }

            // Fast-growth upper bound: max reachable = fib[rem+1]*x + fib[rem]*y.
            let upper = (fib[rem + 1] as i128) * (x as i128) + (fib[rem] as i128) * (y as i128);
            if upper < n128 {
                continue;
            }

            // Nonneg linear-combination test: can n = a*x + b*y with a,b >= 0?
            let a = ((n128 * p as i128) % y as i128 + y as i128) % y as i128;
            if a * x as i128 > n128 {
                continue;
            }

            let xp = x + y;
            let rem1 = rem - 1;

            // Push children: (x+y,y) explored first via LIFO, so push (x+y,x) first.
            stack.push((xp, x, q, p - q, rem1));
            stack.push((xp, y, p, q - p, rem1));
        }

        if best_m != i64::MAX {
            return best_m;
        }

        depth += 1;
    }
}

fn main() {
    debug_assert_eq!(f(7), 2);
    debug_assert_eq!(f(89), 34);
    debug_assert_eq!(f(8191), 1856);

    let n: i64 = 1_000_000_000_000 + 39;
    println!("{}", f(n));
}
