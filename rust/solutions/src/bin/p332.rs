// Project Euler 332: Spherical triangles

fn main() {
    let mut total = 0.0f64;

    for r in 1..=50i32 {
        let r2 = r as i64 * r as i64;

        // Find lattice points on sphere
        let mut pts: Vec<(i32, i32, i32)> = Vec::new();
        for x in 0..=r {
            for y in -r..=r {
                for z in -r..=r {
                    if x as i64 * x as i64 + y as i64 * y as i64 + z as i64 * z as i64 == r2 {
                        pts.push((x, y, z));
                    }
                }
            }
        }

        let n = pts.len();
        if n < 3 { continue; }

        // Precompute pairwise angles
        let mut angles = vec![0.0f64; n * n];
        for i in 0..n {
            for j in 0..n {
                let dot = pts[i].0 as f64 * pts[j].0 as f64
                        + pts[i].1 as f64 * pts[j].1 as f64
                        + pts[i].2 as f64 * pts[j].2 as f64;
                let cos_val = (dot / r2 as f64).clamp(-1.0, 1.0);
                angles[i * n + j] = cos_val.acos();
            }
        }

        let mut min_area = 1e30f64;

        for i in 0..n {
            let (ax, ay, az) = pts[i];
            for j in (i + 1)..n {
                let (bx, by, bz) = pts[j];
                for k in (j + 1)..n {
                    let (cx, cy, cz) = pts[k];

                    let cross_x = ay as i64 * bz as i64 - az as i64 * by as i64;
                    let cross_y = az as i64 * bx as i64 - ax as i64 * bz as i64;
                    let cross_z = ax as i64 * by as i64 - ay as i64 * bx as i64;
                    let det = cross_x * cx as i64 + cross_y * cy as i64 + cross_z * cz as i64;
                    if det == 0 { continue; }

                    let a = angles[i * n + j];
                    let b = angles[i * n + k];
                    let c = angles[j * n + k];
                    let s = (a + b + c) / 2.0;

                    let val = (s / 2.0).tan() * ((s - a) / 2.0).tan()
                            * ((s - b) / 2.0).tan() * ((s - c) / 2.0).tan();
                    let val = val.max(0.0);
                    let e = 4.0 * val.sqrt().atan();
                    let area = r2 as f64 * e;

                    if area < min_area { min_area = area; }
                }
            }
        }

        if min_area < 1e29 { total += min_area; }
    }

    println!("{:.6}", total);
}
