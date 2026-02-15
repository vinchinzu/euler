// Project Euler 880 - Fermat equation with cubes

fn is_perfect_cube(n: i64) -> bool {
    if n <= 0 { return n == 0; }
    let c = (n as f64).cbrt().round() as i64;
    for x in [c - 1, c, c + 1] {
        if x >= 0 && x * x * x == n { return true; }
    }
    false
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn isqrt(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).sqrt() as i64;
    while (x + 1) * (x + 1) <= n { x += 1; }
    while x * x > n { x -= 1; }
    x
}

fn main() {
    let n: i64 = 1_000_000_000_000_000; // 10^15
    let m: i64 = 1_095_912_793; // 1031^3 + 2
    let mut ans: i64 = 0;

    // sign_a = 1
    let mut r: i64 = 1;
    while r * r * r <= n {
        for s in 1.. {
            let v = s + 2 * r;
            let check = 4i128 * s as i128 * v as i128 * v as i128 * v as i128;
            if check > n as i128 { break; }
            if gcd(r, s) != 1 { continue; }

            let val1 = r - 4 * s;
            let val2 = s + 2 * r;
            let av1 = val1.abs();
            let av1_3 = av1 as i128 * av1 as i128 * av1 as i128;
            let v2_3 = val2 as i128 * val2 as i128 * val2 as i128;
            let maybe_cube = 2 * r * s * s;

            if !is_perfect_cube(maybe_cube) {
                let denom_y = 4i128 * s as i128 * v2_3;
                let mut max_g2: i64 = if denom_y == 0 || denom_y > n as i128 { 0 }
                    else { (n as i128 / denom_y) as i64 };

                if av1_3 > 0 {
                    let denom_x = r as i128 * av1_3;
                    if denom_x > 0 && denom_x <= n as i128 {
                        let max_g2_x = (n as i128 / denom_x) as i64;
                        if max_g2_x < max_g2 { max_g2 = max_g2_x; }
                    } else if denom_x > n as i128 {
                        max_g2 = 0;
                    }
                }

                let max_g = isqrt(max_g2);
                if max_g >= 1 {
                    let coeff = r as i128 * av1_3 + 4i128 * s as i128 * v2_3;
                    let sg = max_g as i128 * (max_g as i128 + 1) * (2 * max_g as i128 + 1) / 6;
                    let coeff_mod = (coeff % m as i128) as i64;
                    let sg_mod = (sg % m as i128) as i64;
                    ans = (ans + (coeff_mod as i128 * sg_mod as i128 % m as i128) as i64) % m;
                }
            }
        }
        r += 2;
    }

    // sign_a = -1
    r = 1;
    while r * r * r <= n {
        for s in 1.. {
            let v = r + 4 * s;
            let check = r as i128 * v as i128 * v as i128 * v as i128;
            if check > n as i128 { break; }
            if gcd(r, s) != 1 { continue; }

            let val1 = r + 4 * s;
            let val2 = s - 2 * r;
            let v1_3 = val1 as i128 * val1 as i128 * val1 as i128;
            let av2 = val2.abs();
            let av2_3 = av2 as i128 * av2 as i128 * av2 as i128;
            let maybe_cube = 2 * r * s * s;

            if !is_perfect_cube(maybe_cube) {
                let denom_x = r as i128 * v1_3;
                let mut max_g2: i64 = if denom_x == 0 || denom_x > n as i128 { 0 }
                    else { (n as i128 / denom_x) as i64 };

                if av2_3 > 0 {
                    let denom_y = 4i128 * s as i128 * av2_3;
                    if denom_y > 0 && denom_y <= n as i128 {
                        let max_g2_y = (n as i128 / denom_y) as i64;
                        if max_g2_y < max_g2 { max_g2 = max_g2_y; }
                    } else if denom_y > n as i128 {
                        max_g2 = 0;
                    }
                }

                let max_g = isqrt(max_g2);
                if max_g >= 1 {
                    let coeff = r as i128 * v1_3 + 4i128 * s as i128 * av2_3;
                    let sg = max_g as i128 * (max_g as i128 + 1) * (2 * max_g as i128 + 1) / 6;
                    let coeff_mod = (coeff % m as i128) as i64;
                    let sg_mod = (sg % m as i128) as i64;
                    ans = (ans + (coeff_mod as i128 * sg_mod as i128 % m as i128) as i64) % m;
                }
            }
        }
        r += 2;
    }

    println!("{}", ans);
}
