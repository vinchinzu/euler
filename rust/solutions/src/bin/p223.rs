// Project Euler 223: Almost right-angled triangles I
// Count barely acute triangles (a <= b <= c, a^2 + b^2 = c^2 + 1)
// with perimeter <= 25,000,000 using ternary tree generation.

fn main() {
    let n: i64 = 25_000_000;
    let mut stack: Vec<(i64, i64, i64)> = Vec::new();

    stack.push((1, 1, 1));
    stack.push((1, 2, 2));

    let mut ans: i64 = 0;

    while let Some((a, b, c)) = stack.pop() {
        if a + b + c <= n {
            ans += 1;
            // Child 1
            stack.push((a - 2*b + 2*c, 2*a - b + 2*c, 2*a - 2*b + 3*c));
            // Child 2 (only if a != b)
            if a != b {
                stack.push((-a + 2*b + 2*c, -2*a + b + 2*c, -2*a + 2*b + 3*c));
            }
            // Child 3
            stack.push((a + 2*b + 2*c, 2*a + b + 2*c, 2*a + 2*b + 3*c));
        }
    }

    println!("{}", ans);
}
