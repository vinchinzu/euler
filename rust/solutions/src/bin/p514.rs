// Project Euler 514 - Geoboard Shapes
// Expected area of convex hull of random pins on a lattice.

fn main() {
    let n: usize = 100;
    let p: f64 = 1.0 / (n + 1) as f64;
    let q: f64 = 1.0 - p;

    let max_power = (n + 1) * (n + 1);
    let mut pow_q = vec![0.0f64; max_power + 1];
    pow_q[0] = 1.0;
    for i in 1..=max_power {
        pow_q[i] = pow_q[i - 1] * q;
    }

    // Compute E[w][h]
    let mut e = vec![vec![0.0f64; n + 1]; n + 1];

    for w in 1..=n {
        for h in 1..=n {
            let min_dim = w.min(h - 1);
            for s in 0..=min_dim {
                for x in 0..s {
                    let tri_idx = s * (s - 1) / 2 + x;
                    let prob = p * pow_q[tri_idx];
                    let area = s as f64 * s as f64 / 2.0 + e[x][h - s] + e[w - s][s - x];
                    e[w][h] += prob * area;
                }
            }

            {
                let tri_idx = min_dim * (min_dim + 1) / 2;
                let prob = pow_q[tri_idx];
                let area = if w < h {
                    w as f64 * w as f64 / 2.0 + e[w][h - w]
                } else {
                    h as f64 * h as f64 / 2.0 + e[w - h][h]
                };
                e[w][h] += prob * area;
            }
        }
    }

    let f_val = |w: i32, h: i32| -> f64 {
        if w < -1 || h < -1 { return 0.0; }
        let idx = (n as i32 + 1) * (n as i32 + 1) - (w + 1) * (h + 1);
        if idx < 0 || idx as usize > max_power { return 0.0; }
        pow_q[idx as usize]
    };

    let mut ans = 0.0f64;
    for w in 1..=n as i32 {
        for h in 1..=n as i32 {
            let num_regions = (n as i32 - w + 1) * (n as i32 - h + 1);
            let prob = f_val(w, h) - 2.0 * f_val(w, h - 1) - 2.0 * f_val(w - 1, h)
                     + f_val(w, h - 2) + 4.0 * f_val(w - 1, h - 1) + f_val(w - 2, h)
                     - 2.0 * f_val(w - 1, h - 2) - 2.0 * f_val(w - 2, h - 1) + f_val(w - 2, h - 2);
            ans += num_regions as f64 * prob * w as f64 * h as f64;

            for x in 1..=w as usize {
                for y in 1..=h as usize {
                    let mult = if x == w as usize && y == h as usize {
                        1.0
                    } else if x == w as usize {
                        1.0 - pow_q[w as usize]
                    } else if y == h as usize {
                        1.0 - pow_q[h as usize + 1]
                    } else {
                        p + q * (1.0 - pow_q[w as usize - 1]) * (1.0 - pow_q[h as usize])
                    };
                    ans -= 4.0 * num_regions as f64 * mult * p * p
                           * pow_q[x + h as usize - y]
                           * f_val(w, h) * e[x][y];
                }
            }
        }
    }

    println!("{:.5}", ans);
}
