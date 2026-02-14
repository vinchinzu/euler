// Project Euler 224: Almost right-angled triangles II
fn main() {
    let big_n: i64 = 75_000_000;
    let mut stack: Vec<(i64, i64, i64)> = Vec::with_capacity(30000);
    stack.push((2, 2, 3));

    let mut ans: i64 = 0;

    while let Some((a, b, c)) = stack.pop() {
        if a + b + c <= big_n {
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

    println!("{ans}");
}
