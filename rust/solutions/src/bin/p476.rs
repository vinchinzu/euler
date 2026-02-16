use rayon::prelude::*;
use std::f64::consts::PI;

#[inline(always)]
fn corner_radius(angle: f64, r: f64) -> f64 {
    let sh = (angle * 0.5).sin();
    r * (1.0 - sh) / (1.0 + sh)
}

fn main() {
    let n: i32 = 1803;

    // Parallelize over 'a' values; each a has enough inner work to amortize overhead
    let (total_area, count) = (1..=n)
        .into_par_iter()
        .map(|a| {
            let mut local_area = 0.0f64;
            let mut local_count = 0u64;
            let af = a as f64;
            let ab = af * af;

            let b_max = n.min(n - a); // a + b <= n => b <= n - a
            for b in a..=b_max {
                let bf = b as f64;
                let bb = bf * bf;
                let c_max = n.min(a + b - 1);
                for c in b..=c_max {
                    let cf = c as f64;
                    let cb = cf * cf;

                    // Heron's formula
                    let s = (af + bf + cf) * 0.5;
                    let area2 = s * (s - af) * (s - bf) * (s - cf);
                    if area2 < 1e-24 { continue; }
                    let area = area2.sqrt();
                    let r = area / s;

                    let cos_a = ((bb + cb - ab) / (2.0 * bf * cf)).clamp(-1.0, 1.0);
                    let cos_b = ((ab + cb - bb) / (2.0 * af * cf)).clamp(-1.0, 1.0);
                    let cos_c = ((ab + bb - cb) / (2.0 * af * bf)).clamp(-1.0, 1.0);
                    let angle_a = cos_a.acos();
                    let angle_b = cos_b.acos();
                    let angle_c = cos_c.acos();

                    let r_a = corner_radius(angle_a, r);
                    let r_b = corner_radius(angle_b, r);
                    let r_c = corner_radius(angle_c, r);

                    // Find largest corner circle
                    let (first_r, first_angle, second_r) = if r_a >= r_b && r_a >= r_c {
                        (r_a, angle_a, if r_b >= r_c { r_b } else { r_c })
                    } else if r_b >= r_c {
                        (r_b, angle_b, if r_a >= r_c { r_a } else { r_c })
                    } else {
                        (r_c, angle_c, if r_a >= r_b { r_a } else { r_b })
                    };

                    let mut best_third = second_r;

                    // Corner-of-corner
                    let coc = corner_radius(first_angle, first_r);
                    if coc > best_third { best_third = coc; }

                    // Soddy circle
                    let k1 = 1.0 / r;
                    let k2 = 1.0 / first_r;
                    let k3 = k1 + k2 + 2.0 * (k1 * k2).sqrt();
                    let soddy = 1.0 / k3;
                    if soddy > best_third { best_third = soddy; }

                    local_area += r * r + first_r * first_r + best_third * best_third;
                    local_count += 1;
                }
            }
            (local_area, local_count)
        })
        .reduce(|| (0.0f64, 0u64), |(a1, c1), (a2, c2)| (a1 + a2, c1 + c2));

    println!("{:.5}", PI * total_area / count as f64);
}
