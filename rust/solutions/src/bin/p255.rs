// Project Euler 255: Rounded Square Roots
fn ceil_div(a: i64, b: i64) -> i64 {
    (a + b - 1) / b
}

fn bound(n: i64, low: i64, high: i64) -> i64 {
    n.clamp(low, high)
}

fn sum_iterations(l: i64, h: i64, x: i64, k: i64) -> i64 {
    let x2l = bound(x * (x - 1) + 1, l, h);
    let x2h = bound(x * (x + 1) + 1, l, h);
    sum_remaining_iterations(l, x2l, x, k)
        + (x2h - x2l) * k
        + sum_remaining_iterations(x2h, h, x, k)
}

fn sum_remaining_iterations(mut l: i64, h: i64, x: i64, k: i64) -> i64 {
    let mut total = 0;
    while l < h {
        let next_x = (x + ceil_div(l, x)) / 2;
        let next_l = (((next_x * 2 + 1 - x) * x) + 1).min(h);
        total += sum_iterations(l, next_l, next_x, k + 1);
        l = next_l;
    }
    total
}

fn main() {
    let l = 10_000_000_000_000i64;
    let h = 100_000_000_000_000i64;
    let x0 = 7_000_000i64;

    let total = sum_iterations(l, h, x0, 1);
    let result = total as f64 / (h - l) as f64;
    println!("{:.10}", result);
}
