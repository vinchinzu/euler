use std::f64::consts::PI;

fn corner_radius(angle: f64, r: f64) -> f64 {
    let sh = (angle / 2.0).sin();
    r * (1.0 - sh) / (1.0 + sh)
}

fn main() {
    let n = 1803;
    let mut total_area = 0.0f64;
    let mut count = 0u64;

    for a in 1..=n {
        for b in a..=n {
            if a + b > n { break; }
            for c in b..a + b {
                if c > n { break; }
                let s = (a + b + c) as f64 / 2.0;
                let area = (s * (s - a as f64) * (s - b as f64) * (s - c as f64)).sqrt();
                if area < 1e-12 { continue; }
                let r = area / s;

                let (af, bf, cf) = (a as f64, b as f64, c as f64);
                let ab = af * af;
                let bb = bf * bf;
                let cb = cf * cf;
                let cos_a = ((bb + cb - ab) / (2.0 * bf * cf)).clamp(-1.0, 1.0);
                let cos_b = ((ab + cb - bb) / (2.0 * af * cf)).clamp(-1.0, 1.0);
                let cos_c = ((ab + bb - cb) / (2.0 * af * bf)).clamp(-1.0, 1.0);
                let angle_a = cos_a.acos();
                let angle_b = cos_b.acos();
                let angle_c = cos_c.acos();

                let r_a = corner_radius(angle_a, r);
                let r_b = corner_radius(angle_b, r);
                let r_c = corner_radius(angle_c, r);

                let corners = [r_a, r_b, r_c];
                let angles = [angle_a, angle_b, angle_c];
                let mut first_idx = 0;
                if corners[1] > corners[0] { first_idx = 1; }
                if corners[2] > corners[first_idx] { first_idx = 2; }
                let first_r = corners[first_idx];
                let first_angle = angles[first_idx];

                let mut best_third = 0.0f64;
                for i in 0..3 {
                    if i != first_idx && corners[i] > best_third { best_third = corners[i]; }
                }

                let coc = corner_radius(first_angle, first_r);
                if coc > best_third { best_third = coc; }

                let k1 = 1.0 / r;
                let k2 = 1.0 / first_r;
                let k3 = k1 + k2 + 2.0 * (k1 * k2).sqrt();
                let soddy = 1.0 / k3;
                if soddy > best_third { best_third = soddy; }

                total_area += PI * (r * r + first_r * first_r + best_third * best_third);
                count += 1;
            }
        }
    }

    println!("{:.5}", total_area / count as f64);
}
