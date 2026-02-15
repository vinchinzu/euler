// Project Euler 965 - Stern-Brocot / Farey-like sum
// Enumerate Farey sequence mediant pairs and sum contributions

fn main() {
    let n = 10_000i64;
    let mut total = 0.0f64;
    let (mut a, mut b, mut c, mut d) = (0i64, 1i64, 1i64, n);

    loop {
        total += 1.0 / (2.0 * b as f64 * (d as f64) * (d as f64));
        let k = (n + b) / d;
        let (na, nb, nc, nd) = (c, d, k * c - a, k * d - b);
        a = na;
        b = nb;
        c = nc;
        d = nd;
        if a == 1 && b == 1 {
            break;
        }
    }

    println!("{:.13}", total);
}
