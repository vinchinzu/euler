// Project Euler 222: Sphere Packing
fn main() {
    let n = 50;
    let k = 21;
    let mut order = Vec::new();

    for i in (n - k + 1)..=n {
        if i % 2 == 0 {
            order.insert(0, i);
        } else {
            order.push(i);
        }
    }

    let mut length = (order[0] + order[k - 1]) as f64;
    for i in 1..k {
        let sum_radii = (order[i - 1] + order[i]) as f64;
        let diff = 2.0 * n as f64 - sum_radii;
        length += (sum_radii * sum_radii - diff * diff).sqrt();
    }

    println!("{}", (1000.0 * length).round() as i64);
}
