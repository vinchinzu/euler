// Project Euler 197: Investigating the behaviour of a recursive sequence.

fn main() {
    let mut u: f64 = -1.0;
    for _ in 0..1000 {
        let power = 30.403243784 - u * u;
        u = (2.0_f64.powf(power)).floor() * 1e-9;
    }
    let u_n = u;
    let power = 30.403243784 - u_n * u_n;
    let u_n1 = (2.0_f64.powf(power)).floor() * 1e-9;
    let sum = u_n + u_n1;
    println!("{:.9}", sum);
}
