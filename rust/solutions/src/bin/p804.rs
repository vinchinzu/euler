// Project Euler 804 - Counting Binary Quadratic Representations
// Count lattice points with x^2+xy+41y^2 <= N

fn isqrt(n: i64) -> i64 {
    if n < 0 { return -1; }
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn main() {
    let n: i64 = 10_000_000_000_000_000; // 10^16
    let coeff: i64 = 163; // 4*41 - 1
    let four_n = 4 * n;

    let mut max_y = isqrt(four_n / coeff);
    while coeff * (max_y + 1) * (max_y + 1) <= four_n { max_y += 1; }
    while coeff * max_y * max_y > four_n { max_y -= 1; }

    let mut ans: i64 = 0;

    for y in -max_y..=max_y {
        let disc = four_n - coeff * y * y;
        if disc < 0 { continue; }
        let sd = isqrt(disc);

        let p_y = ((y % 2) + 2) % 2;

        let mut u_lo = -sd;
        let p_lo = ((u_lo % 2) + 2) % 2;
        if p_lo != p_y { u_lo += 1; }

        let mut u_hi = sd;
        let p_hi = ((u_hi % 2) + 2) % 2;
        if p_hi != p_y { u_hi -= 1; }

        if u_lo > u_hi { continue; }
        ans += (u_hi - u_lo) / 2 + 1;
    }

    ans -= 1; // subtract (0,0)
    println!("{}", ans);
}
