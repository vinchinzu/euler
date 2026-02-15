// Project Euler 841 - Regular Star Polygons
// Sum of A(F_{n+1}, F_{n-1}) for n=3..34

fn main() {
    let mut f = [0i64; 38];
    f[0] = 0; f[1] = 1;
    for i in 2..=36 { f[i] = f[i - 1] + f[i - 2]; }

    let pi: f64 = std::f64::consts::PI;
    let mut total: f64 = 0.0;

    for n in 3..=34 {
        let p = f[n + 1] as f64;
        let q = f[n - 1];

        let mut s: f64 = 0.0;
        let mut sign: i32 = if (q - 1) % 2 == 0 { 1 } else { -1 };

        for k in 1..q {
            let term = p * (k as f64 * pi / p).tan();
            if sign > 0 { s += term; } else { s -= term; }
            sign = -sign;
        }

        let tq = p * (q as f64 * pi / p).tan();
        let area = tq + 2.0 * s;
        total += area;
    }

    println!("{:.10}", total);
}
