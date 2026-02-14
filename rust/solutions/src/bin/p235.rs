// Project Euler 235: An Arithmetic Geometric sequence

fn s(r: f64) -> f64 {
    let a = 900.0f64;
    let d = 3.0f64;
    let n = 5000.0f64;
    let r_n = r.powf(n);
    if (r - 1.0).abs() < 1e-10 {
        return a * n - d * n * (n + 1.0) / 2.0;
    }
    let rm1 = r - 1.0;
    a * (r_n - 1.0) / rm1 - d * (n * r_n / rm1 - (r_n - 1.0) / (rm1 * rm1))
}

fn main() {
    let target = 600_000_000_000.0f64;
    let mut low = 1.0f64;
    let mut high = 1.1f64;

    for _ in 0..200 {
        let mid = (low + high) / 2.0;
        if s(mid) > -target {
            low = mid;
        } else {
            high = mid;
        }
    }

    println!("{:.12}", low);
}
