// Project Euler 246: Tangents to an ellipse
fn main() {
    let r: f64 = 15000.0;
    let gx: f64 = 8000.0;
    let mx: f64 = -2000.0;

    let a2 = (r / 2.0) * (r / 2.0);
    let b2 = (r / 2.0) * (r / 2.0) - ((gx - mx) / 2.0) * ((gx - mx) / 2.0);

    let mut ans: i64 = 0;
    let mut y = 0i32;

    loop {
        let y2 = (y as f64) * (y as f64);
        let x2 = a2 + 3.0 * b2 - y2 + 2.0 * (2.0 * b2 * b2 + (a2 - b2) * y2).sqrt();

        if x2 < 0.0 { break; }

        let x = x2.sqrt().ceil() as i32;
        let num_points = if y2 > b2 {
            2 * x - 1
        } else {
            2 * (x - (a2 * (1.0 - y2 / b2)).sqrt().floor() as i32 - 1)
        };

        ans += if y == 0 { 1 } else { 2 } * num_points as i64;
        y += 1;
    }

    println!("{ans}");
}
