// Project Euler 744 - What? Where? When?
//
// Simple closed-form: probability = 1 - 1/(2*(1-p))

fn main() {
    let p: f64 = 0.4999;
    let ans = 1.0 - 1.0 / 2.0 / (1.0 - p);
    println!("{:.10}", ans);
}
