// Project Euler 140: Modified Fibonacci golden nuggets
// Pell equation X^2 - 5Y^2 = 44, need X ≡ 2 (mod 5) for positive integer N.
// N = (X - 7) / 5.

fn main() {
    let base_x: [i128; 7] = [7, 8, 13, 17, 32, 43, 83];

    let mut nuggets: Vec<i128> = Vec::new();

    for &x0 in &base_x {
        let x1: i128 = 18 * x0 - x0; // placeholder
        // Generate using recurrence: X_{n+1} = 18*X_n - X_{n-1}
        // First compute x_{-1} from x0 and the forward step
        let x1_fwd: i128 = 18 * x0 - 0; // we need actual x_{-1}

        // Check x0
        if x0 > 7 && (x0 - 7) % 5 == 0 {
            nuggets.push((x0 - 7) / 5);
        }

        // Use recurrence X_{n+1} = 18*X_n - X_{n-1}
        // We need x1 from the Pell solution: x1 = 9*x0 + 20*y0
        // But we don't have y0. Instead, compute from base solutions:
        // Use the trick: start from x0, compute forward via 18*x - x_prev
        // The "previous" x_{-1} can be obtained from: x1 = 18*x0 - x_{-1}
        // So x_{-1} = 18*x0 - x1.
        // We need x1: from the recurrence with fundamental (9,4):
        // x1 = 9*x0 + 20*y0 where y0^2 = (x0^2 - 44)/5
    }

    // Simpler approach: just enumerate all base solutions with their y values
    let bases: [(i128, i128); 7] = [
        (7, 1), (8, 2), (13, 5), (17, 7), (32, 14), (43, 19), (83, 37),
    ];

    nuggets.clear();

    for &(x0, y0) in &bases {
        let x1 = 9 * x0 + 20 * y0;
        // x_{-1} = 18*x0 - x1
        let xm1 = 18 * x0 - x1;

        if x0 > 7 && (x0 - 7) % 5 == 0 {
            nuggets.push((x0 - 7) / 5);
        }

        let mut xprev = xm1;
        let mut xcurr = x0;

        for _ in 0..15 {
            let xnext = 18 * xcurr - xprev;
            if xnext > 7 && (xnext - 7) % 5 == 0 {
                nuggets.push((xnext - 7) / 5);
            }
            xprev = xcurr;
            xcurr = xnext;
        }
    }

    nuggets.sort();
    nuggets.dedup();
    nuggets.retain(|&x| x > 0);

    let total: i128 = nuggets.iter().take(30).sum();
    println!("{}", total);
}
