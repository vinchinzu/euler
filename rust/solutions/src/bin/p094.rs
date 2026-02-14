// Project Euler 94: Almost equilateral triangles
// Sum of perimeters of almost equilateral triangles with integer area,
// perimeter <= 1,000,000,000. Uses Pell equation recurrence.

fn main() {
    let limit: i64 = 1_000_000_000;
    let mut sum_of_perimeters: i64 = 0;

    // Pell equation: X_{n+1} = 4*X_n - X_{n-1}
    let mut xk_minus_1: i64 = 1; // X_0
    let mut xk: i64 = 2;          // X_1
    let mut current_k = 1;

    loop {
        let xk_plus_1 = 4 * xk - xk_minus_1;
        let index = current_k + 1;

        let perimeter = if index % 2 == 0 {
            2 * xk_plus_1 + 2
        } else {
            2 * xk_plus_1 - 2
        };

        if perimeter > limit {
            break;
        }

        sum_of_perimeters += perimeter;

        xk_minus_1 = xk;
        xk = xk_plus_1;
        current_k = index;
    }

    println!("{sum_of_perimeters}");
}
