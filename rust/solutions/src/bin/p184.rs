// Project Euler 184: Triangles containing the origin
use euler_utils::gcd;
use std::collections::HashMap;

fn main() {
    const RADIUS: i32 = 105;
    const RADIUS_SQ: i32 = RADIUS * RADIUS;
    let two_pi = 2.0 * std::f64::consts::PI;

    let mut angles: Vec<f64> = Vec::new();
    let mut dirs: HashMap<(i32, i32), (i64, i64)> = HashMap::new();

    for x in -(RADIUS - 1)..=RADIUS - 1 {
        for y in -(RADIUS - 1)..=RADIUS - 1 {
            if x == 0 && y == 0 { continue; }
            if x * x + y * y >= RADIUS_SQ { continue; }

            angles.push((y as f64).atan2(x as f64));

            let g = gcd(x.unsigned_abs() as u64, y.unsigned_abs() as u64) as i32;
            let dx = x / g;
            let dy = y / g;

            if dy > 0 || (dy == 0 && dx > 0) {
                let entry = dirs.entry((dx, dy)).or_insert((0, 0));
                entry.0 += 1;
            } else {
                let entry = dirs.entry((-dx, -dy)).or_insert((0, 0));
                entry.1 += 1;
            }
        }
    }

    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let count_points = angles.len();

    let mut extended_angles = Vec::with_capacity(count_points * 2);
    for &a in &angles {
        extended_angles.push(a);
    }
    for &a in &angles {
        extended_angles.push(a + two_pi);
    }

    let epsilon = 1e-12;
    let pi = std::f64::consts::PI;
    let mut bad: i64 = 0;
    let mut j = 0usize;
    for i in 0..count_points {
        if j < i + 1 { j = i + 1; }
        while j < i + count_points && extended_angles[j] - angles[i] < pi - epsilon {
            j += 1;
        }
        let m = (j - i - 1) as i64;
        bad += m * (m - 1) / 2;
    }

    let mut opposite: i64 = 0;
    for &(pos, neg) in dirs.values() {
        if pos == 0 || neg == 0 { continue; }
        let total_on_line = pos + neg;
        let other_points = count_points as i64 - total_on_line;

        opposite += pos * neg * other_points;
        opposite += (pos * (pos - 1) / 2) * neg;
        opposite += pos * (neg * (neg - 1) / 2);
    }

    let n = count_points as i64;
    let total = n * (n - 1) * (n - 2) / 6;
    let result = total - bad - opposite;
    println!("{}", result);
}
