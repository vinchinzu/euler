// Project Euler 177: Integer Angled Quadrilaterals
use std::collections::HashSet;

fn main() {
    let c = 180i32;
    let rad = std::f64::consts::PI / 180.0;

    let mut sin_table = vec![0.0f64; c as usize];
    let mut cos_table = vec![0.0f64; c as usize];
    for i in 0..c as usize {
        sin_table[i] = (i as f64 * rad).sin();
        cos_table[i] = (i as f64 * rad).cos();
    }

    let mut seen = HashSet::new();

    for a in 1..=c / 4 {
        for b in a..=c - 3 * a {
            for ci in a..=c - 2 * a - b {
                for d in a..=c - a - b - ci {
                    let ad = sin_table[ci as usize] / sin_table[(a + b + ci) as usize];
                    let denom = sin_table[(b + ci + d) as usize];
                    if denom == 0.0 { continue; }
                    let ac = sin_table[(ci + d) as usize] / denom;
                    let diff = ac - ad * cos_table[a as usize];
                    if diff == 0.0 { continue; }
                    let f = (ad * sin_table[a as usize] / diff).atan() * 180.0 / std::f64::consts::PI;
                    let fi_round = if f >= 0.0 { (f + 0.5) as i32 } else { (f - 0.5) as i32 };
                    if (f - fi_round as f64).abs() > 1e-9 { continue; }
                    let fi = if fi_round < 0 { fi_round + c } else { fi_round };

                    let angles = [
                        a, b, ci, d,
                        c - b - ci - d,
                        fi,
                        b + ci - fi,
                        c - a - b - ci,
                    ];

                    if angles.iter().any(|&x| x <= 0) { continue; }

                    // Canonicalize
                    let mut min_hash = i64::MAX;
                    for start in 0..8 {
                        let mut h: i64 = 0;
                        if start % 2 == 0 {
                            for i in 0..8 {
                                h = h * c as i64 + angles[(start + i) % 8] as i64;
                            }
                        } else {
                            for i in 0..8 {
                                h = h * c as i64 + angles[((start as i32 - i as i32).rem_euclid(8)) as usize] as i64;
                            }
                        }
                        if h < min_hash { min_hash = h; }
                    }
                    seen.insert(min_hash);
                }
            }
        }
    }

    println!("{}", seen.len());
}
