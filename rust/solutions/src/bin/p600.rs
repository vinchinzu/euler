// Project Euler 600 - Integer Sided Equi-angular Hexagons
//
// Count distinct (up to D6 symmetry) equi-angular hexagons with perimeter <= N.
// Uses Burnside's lemma for small values + Lagrange interpolation.

fn h_brute(n: i32) -> i64 {
    let mut id_count: i64 = 0;
    let mut rot60_count: i64 = 0;
    let mut rot120_count: i64 = 0;
    let mut rot180_count: i64 = 0;
    let mut refl_vert_count: i64 = 0;
    let mut refl_mid_count: i64 = 0;

    for a in 1..=n {
        for b in 1..=n - a {
            for c in 1..=n - a - b {
                let e_max_1 = a + b - 1; // d >= 1
                let e_max_2 = b + c - 1; // f >= 1
                let e_max = e_max_1.min(e_max_2);
                let e_min = (2 * a + 3 * b + 2 * c - n).max(1);
                if e_min > e_max { continue; }

                id_count += (e_max - e_min + 1) as i64;

                // rot60: a=b=c, e=a
                if a == b && b == c && a >= e_min && a <= e_max {
                    rot60_count += 1;
                }

                // rot120: a=c, e=a
                if a == c && a >= e_min && a <= e_max {
                    rot120_count += 1;
                }

                // rot180: e=b
                if b >= e_min && b <= e_max {
                    rot180_count += 1;
                }

                // reflection through vertex: a=c and e=b
                if a == c && b >= e_min && b <= e_max {
                    refl_vert_count += 1;
                }

                // reflection through midpoint: e=c
                if c >= e_min && c <= e_max {
                    refl_mid_count += 1;
                }
            }
        }
    }

    (id_count + 2 * rot60_count + 2 * rot120_count + rot180_count
        + 3 * refl_vert_count + 3 * refl_mid_count)
        / 12
}

fn lagrange_interp(xs: &[i32], ys: &[i64], x: i64) -> i64 {
    let npts = xs.len();
    let mut result_num: i128 = 0;
    let mut result_den: i128 = 1;

    for i in 0..npts {
        let mut numer = ys[i] as i128;
        let mut denom: i128 = 1;
        for j in 0..npts {
            if j != i {
                numer *= x as i128 - xs[j] as i128;
                denom *= xs[i] as i128 - xs[j] as i128;
            }
        }
        // Add numer/denom to result_num/result_den
        result_num = result_num * denom + numer * result_den;
        result_den *= denom;

        // Simplify
        let g = gcd128(result_num.unsigned_abs(), result_den.unsigned_abs());
        if g > 1 {
            result_num /= g as i128;
            result_den /= g as i128;
        }
    }

    if result_den < 0 {
        result_num = -result_num;
        result_den = -result_den;
    }
    (result_num / result_den) as i64
}

fn gcd128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    let n = 55106;

    // Compute H(n) for n = 0..100 using brute force
    let mut vals = [0i64; 101];
    for nn in 6..=100 {
        vals[nn] = h_brute(nn as i32);
    }

    // Determine residue class
    let r = n % 12; // 55106 % 12 = 2
    let target_k = (n - r) / 12;

    // Collect interpolation points for residue r
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for k in 0..=8 {
        let nn = 12 * k + r;
        if nn >= 1 && nn <= 100 {
            xs.push(k as i32);
            ys.push(vals[nn]);
        }
    }

    let result = lagrange_interp(&xs, &ys, target_k as i64);
    println!("{}", result);
}
