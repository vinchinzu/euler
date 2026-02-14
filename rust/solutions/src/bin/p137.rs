// Project Euler 137: Fibonacci golden nuggets
// The 15th golden nugget = (L_61 - 1) / 5, where L_n is the nth Lucas number.

fn main() {
    // Compute Lucas number L_61: L_0=2, L_1=1, L_n = L_{n-1} + L_{n-2}
    let mut a: i64 = 2;
    let mut b: i64 = 1;
    for _ in 2..=61 {
        let t = a + b;
        a = b;
        b = t;
    }
    // b = L_61
    println!("{}", (b - 1) / 5);
}
