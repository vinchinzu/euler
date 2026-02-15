// Project Euler 264: Triangle Centres
use std::collections::HashSet;

fn is_square(n: i64) -> bool {
    if n < 0 { return false; }
    let r = (n as f64).sqrt() as i64;
    for v in (r.saturating_sub(1))..=(r + 1) {
        if v * v == n { return true; }
    }
    false
}

fn isqrt64(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut r = (n as f64).sqrt() as i64;
    while r > 0 && r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

fn factorize(mut n: i64) -> Vec<(i64, i32)> {
    let mut factors = Vec::new();
    let mut p = 2i64;
    while p * p <= n {
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 { n /= p; e += 1; }
            factors.push((p, e));
        }
        p += 1;
    }
    if n > 1 { factors.push((n, 1)); }
    factors
}

fn gen_divisors(factors: &[(i64, i32)]) -> Vec<i64> {
    let mut divs = vec![1i64];
    for &(p, e) in factors {
        let old_len = divs.len();
        let mut pe = 1i64;
        for _ in 0..e {
            pe *= p;
            for k in 0..old_len {
                divs.push(divs[k] * pe);
            }
        }
    }
    divs
}

fn main() {
    let big_n = 100000;
    let max_ax = big_n / 4;

    let mut triangles: HashSet<(i32, i32, i32, i32, i32, i32)> = HashSet::new();

    let sort3 = |mut pts: [(i32, i32); 3]| -> (i32, i32, i32, i32, i32, i32) {
        pts.sort();
        (pts[0].0, pts[0].1, pts[1].0, pts[1].1, pts[2].0, pts[2].1)
    };

    let mut add_triangle = |x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, perim: f64| {
        if perim <= big_n as f64 {
            let key = sort3([(x1, y1), (x2, y2), (x3, y3)]);
            triangles.insert(key);
        }
    };

    for ax in 0..=max_ax {
        let d_val = (5 - ax as i64) * (5 - ax as i64);

        let process_point = |ax: i32, ay: i32, add_tri: &mut dyn FnMut(i32, i32, i32, i32, i32, i32, f64)| {
            let r2 = ax as i64 * ax as i64 + ay as i64 * ay as i64;
            let num = 2 * ay as i64 * ay as i64 * r2;
            let den = (5 - ax as i64) * (5 - ax as i64) + ay as i64 * ay as i64;
            if den == 0 || num % den != 0 { return; }
            let disc = 2 * num / den - ay as i64 * ay as i64;
            if disc < 0 || !is_square(disc) { return; }
            let sqrt_disc = isqrt64(disc);
            let bx = ((5 - ax as i64 + sqrt_disc) / 2) as i32;
            let cx = 5 - ax - bx;
            let by_sq = r2 - bx as i64 * bx as i64;
            if by_sq < 0 || !is_square(by_sq) { return; }
            let mut by = isqrt64(by_sq) as i32;

            if cx as i64 * cx as i64 + (ay + by) as i64 * (ay + by) as i64 != r2 {
                by = -by;
            }

            let (p1x, p1y) = (ax, ay);
            let (p2x, p2y) = (bx, by);
            let (p3x, p3y) = (cx, -(ay + by));

            let area2 = (p1x as i64 * (p2y as i64 - p3y as i64)
                + p2x as i64 * (p3y as i64 - p1y as i64)
                + p3x as i64 * (p1y as i64 - p2y as i64)).abs();
            if area2 == 0 { return; }

            let perim = (((p2x - p1x) as f64).powi(2) + ((p2y - p1y) as f64).powi(2)).sqrt()
                + (((p3x - p2x) as f64).powi(2) + ((p3y - p2y) as f64).powi(2)).sqrt()
                + (((p1x - p3x) as f64).powi(2) + ((p1y - p3y) as f64).powi(2)).sqrt();

            if perim <= big_n as f64 {
                add_tri(p1x, p1y, p2x, p2y, p3x, p3y, perim);
                add_tri(ax, -ay, bx, -by, cx, ay + by, perim);
            }
        };

        if d_val != 0 {
            let u = (5 - ax as i64).abs();
            let v = (2 * ax as i64 - 5).abs();
            if v == 0 { continue; }

            // Build combined factorization: 2 * 5 * u^2 * v
            let mut combined: Vec<(i64, i32)> = vec![(2, 1), (5, 1)];
            let merge = |combined: &mut Vec<(i64, i32)>, factors: &[(i64, i32)], mult: i32| {
                for &(p, e) in factors {
                    if let Some(entry) = combined.iter_mut().find(|x| x.0 == p) {
                        entry.1 += e * mult;
                    } else {
                        combined.push((p, e * mult));
                    }
                }
            };
            if u > 1 { merge(&mut combined, &factorize(u), 2); }
            if v > 1 { merge(&mut combined, &factorize(v), 1); }

            let divs = gen_divisors(&combined);
            let parity_start = ax % 2 + 1;
            let ay_parity = parity_start % 2;

            for d in divs {
                let ay_sq = d - d_val;
                if ay_sq <= 0 { continue; }
                if !is_square(ay_sq) { continue; }
                let ay_val = isqrt64(ay_sq) as i32;
                if ay_val % 2 != ay_parity { continue; }
                if ay_val > max_ax { continue; }

                process_point(ax, ay_val, &mut add_triangle);
            }
        } else {
            // Ax = 5
            let parity_start = ax % 2 + 1;
            let mut ay_val = parity_start;
            while ay_val <= max_ax {
                process_point(ax, ay_val, &mut add_triangle);
                ay_val += 2;
            }
        }

        // Handle Ay = 0 case
        let disc_val = 4 * ax as i64 * ax as i64 - (5 - ax as i64) * (5 - ax as i64);
        if is_square(disc_val) {
            let by = (isqrt64(disc_val) / 2) as i32;
            if by > 0 {
                let (p1x, p1y) = (ax, 0);
                let (p2x, p2y) = ((5 - ax) / 2, by);
                let (p3x, p3y) = ((5 - ax) / 2, -by);
                let perim = (((p2x - p1x) as f64).powi(2) + ((p2y - p1y) as f64).powi(2)).sqrt()
                    + (((p3x - p2x) as f64).powi(2) + ((p3y - p2y) as f64).powi(2)).sqrt()
                    + (((p1x - p3x) as f64).powi(2) + ((p1y - p3y) as f64).powi(2)).sqrt();
                add_triangle(p1x, p1y, p2x, p2y, p3x, p3y, perim);
            }
        }
    }

    // Sum perimeters
    let mut total = 0.0f64;
    for &(x1, y1, x2, y2, x3, y3) in &triangles {
        let perim = (((x2 - x1) as f64).powi(2) + ((y2 - y1) as f64).powi(2)).sqrt()
            + (((x3 - x2) as f64).powi(2) + ((y3 - y2) as f64).powi(2)).sqrt()
            + (((x1 - x3) as f64).powi(2) + ((y1 - y3) as f64).powi(2)).sqrt();
        total += perim;
    }

    println!("{:.4}", total);
}
