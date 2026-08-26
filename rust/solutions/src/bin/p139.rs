// Project Euler 139: Pythagorean tiles.
// Primitive triples with c % |a-b| == 0 are consecutive Pell pairs
// (m, n) = (2, 1), (5, 2), (12, 5), ... with perimeter p0 = 2m(m+n).
// Count multiples d*p0 < 100_000_000.

fn main() {
    const LIMIT: i64 = 100_000_000;
    let mut total = 0i64;
    let mut n = 1i64;
    let mut m = 2i64;
    loop {
        let p0 = 2 * m * (m + n);
        if p0 >= LIMIT {
            break;
        }
        total += (LIMIT - 1) / p0;
        let next_m = 2 * m + n;
        n = m;
        m = next_m;
    }
    println!("{}", total);
}
