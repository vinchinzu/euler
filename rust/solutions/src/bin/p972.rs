// Project Euler 972 - Hyperbolic geodesics T(12) = 3575508
// Count ordered triples of V(12) points on a common geodesic
// (diameter or circle orthogonal to unit disc)

use std::collections::HashMap;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(Clone, Copy)]
struct Point {
    xn: i64, xd: i64,
    yn: i64, yd: i64,
}

#[derive(Hash, Eq, PartialEq, Clone)]
enum Geodesic {
    Diameter(i64, i64),                   // (da, db) normalized direction
    Circle(i64, i64, i64, i64),           // (h_num, h_den, k_num, k_den) reduced
}

fn main() {
    let n = 12i64;

    // Generate all distinct rationals with denominator <= N in (-1,1)
    let mut rats: Vec<(i64, i64)> = vec![(0, 1)];
    for q in 1..=n {
        for p in (-(q - 1))..=q - 1 {
            if p == 0 { continue; }
            if gcd(p.abs(), q) == 1 {
                rats.push((p, q));
            }
        }
    }

    // Generate all points in V(N)
    let mut pts: Vec<Point> = Vec::new();
    for &(xn, xd) in &rats {
        for &(yn, yd) in &rats {
            let x2 = xn * xn * yd * yd;
            let y2 = yn * yn * xd * xd;
            let r2 = xd * xd * yd * yd;
            if x2 + y2 < r2 {
                pts.push(Point { xn, xd, yn, yd });
            }
        }
    }

    let npts = pts.len();

    // For each pair of distinct points, determine their geodesic
    let mut geodesic_points: HashMap<Geodesic, Vec<usize>> = HashMap::new();

    for i in 0..npts {
        let xn1 = pts[i].xn;
        let xd1 = pts[i].xd;
        let yn1 = pts[i].yn;
        let yd1 = pts[i].yd;

        for j in (i + 1)..npts {
            let xn2 = pts[j].xn;
            let xd2 = pts[j].xd;
            let yn2 = pts[j].yn;
            let yd2 = pts[j].yd;

            // Check collinearity through origin
            let lhs = xn1 * yn2 * xd2 * yd1;
            let rhs = xn2 * yn1 * xd1 * yd2;

            let geo = if lhs == rhs {
                // On a diameter
                let (mut da, mut db) = if xn1 != 0 || yn1 != 0 {
                    (xn1 * yd1, yn1 * xd1)
                } else {
                    (xn2 * yd2, yn2 * xd2)
                };
                if da == 0 && db == 0 { continue; }
                let g = gcd(da.abs(), db.abs());
                da /= g;
                db /= g;
                if da < 0 || (da == 0 && db < 0) { da = -da; db = -db; }
                Geodesic::Diameter(da, db)
            } else {
                // Orthogonal circle
                let d1 = xd1 * yd1;
                let d2 = xd2 * yd2;
                let a1 = xn1 * yd1;
                let b1 = yn1 * xd1;
                let a2 = xn2 * yd2;
                let b2 = yn2 * xd2;
                let s1_num = a1 * a1 + b1 * b1 + d1 * d1;
                let s2_num = a2 * a2 + b2 * b2 + d2 * d2;

                let a11 = a1 * d1;
                let a12 = b1 * d1;
                let a21 = a2 * d2;
                let a22 = b2 * d2;
                let det = a11 * a22 - a12 * a21;
                if det == 0 { continue; }

                let mut h_num = s1_num * a22 - s2_num * a12;
                let mut h_den = 2 * det;
                let mut k_num = a11 * s2_num - a21 * s1_num;
                let mut k_den = 2 * det;

                let g1 = gcd(h_num.abs(), h_den.abs()).max(1);
                h_num /= g1;
                h_den /= g1;
                if h_den < 0 { h_num = -h_num; h_den = -h_den; }

                let g2 = gcd(k_num.abs(), k_den.abs()).max(1);
                k_num /= g2;
                k_den /= g2;
                if k_den < 0 { k_num = -k_num; k_den = -k_den; }

                Geodesic::Circle(h_num, h_den, k_num, k_den)
            };

            let entry = geodesic_points.entry(geo).or_insert_with(Vec::new);
            if !entry.contains(&i) { entry.push(i); }
            if !entry.contains(&j) { entry.push(j); }
        }
    }

    // Sum s*(s-1)*(s-2) over all geodesics
    let mut total: i64 = 0;
    for (_geo, points) in &geodesic_points {
        let s = points.len() as i64;
        if s >= 3 {
            total += s * (s - 1) * (s - 2);
        }
    }

    println!("{}", total);
}
