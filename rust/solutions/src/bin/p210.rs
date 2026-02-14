// Project Euler 210: Obtuse Angled Triangles

fn main() {
    let n: i64 = 1_000_000_000;
    let q = n / 4;

    let mut ans: i64 = 3 * n / 2 * n + (q + 1) * (q + 1) - (q + 1) - 2;

    let r = n / 8;
    let min_x = (-(r as f64) * (std::f64::consts::SQRT_2 - 1.0)) as i32;

    for x in min_x..0 {
        let xf = x as f64;
        let rf = r as f64;
        let dy = (2.0 * rf * rf - (rf - xf) * (rf - xf)).sqrt();
        let idy = dy.ceil() as i64;
        ans += 4 * (2 * idy - 1);
    }

    println!("{}", ans);
}
