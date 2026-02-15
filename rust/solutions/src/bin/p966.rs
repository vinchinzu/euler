// Project Euler 966 - Triangle inscribed circle area optimization
// For each valid triangle with integer sides summing to <= 200,
// find the maximal area of a circle of area = triangle area placed inside

use std::f64::consts::PI;

fn arc_area(x1: f64, y1: f64, x2: f64, y2: f64, r2: f64) -> f64 {
    let t1 = y1.atan2(x1);
    let t2 = y2.atan2(x2);
    let mut dt = t2 - t1;
    if dt > PI { dt -= 2.0 * PI; }
    else if dt <= -PI { dt += 2.0 * PI; }
    r2 * dt * 0.5
}

fn seg(x1: f64, y1: f64, x2: f64, y2: f64, r2: f64) -> f64 {
    let d1sq = x1 * x1 + y1 * y1;
    let d2sq = x2 * x2 + y2 * y2;
    let cross = x1 * y2 - x2 * y1;

    if d1sq <= r2 && d2sq <= r2 {
        return cross * 0.5;
    }

    let dx = x2 - x1;
    let dy = y2 - y1;
    let a = dx * dx + dy * dy;
    let b = 2.0 * (x1 * dx + y1 * dy);
    let c = d1sq - r2;
    let disc = b * b - 4.0 * a * c;

    if disc < 0.0 {
        return arc_area(x1, y1, x2, y2, r2);
    }

    let sd = disc.sqrt();
    let mut t1 = (-b - sd) / (2.0 * a);
    let mut t2 = (-b + sd) / (2.0 * a);
    t1 = t1.clamp(0.0, 1.0);
    t2 = t2.clamp(0.0, 1.0);

    if t1 >= t2 - 1e-15 {
        if d1sq > r2 && d2sq > r2 {
            return arc_area(x1, y1, x2, y2, r2);
        }
        return cross * 0.5;
    }

    let ix1 = x1 + t1 * dx;
    let iy1 = y1 + t1 * dy;
    let ix2 = x1 + t2 * dx;
    let iy2 = y1 + t2 * dy;

    let mut area = 0.0;
    if d1sq > r2 { area += arc_area(x1, y1, ix1, iy1, r2); }
    else { area += (x1 * iy1 - ix1 * y1) * 0.5; }
    area += (ix1 * iy2 - ix2 * iy1) * 0.5;
    if d2sq > r2 { area += arc_area(ix2, iy2, x2, y2, r2); }
    else { area += (ix2 * y2 - x2 * iy2) * 0.5; }
    area
}

fn circle_triangle_intersection(cx: f64, cy: f64, r2: f64, v1x: f64, v2x: f64, v2y: f64) -> f64 {
    let t = seg(-cx, -cy, v1x - cx, -cy, r2)
          + seg(v1x - cx, -cy, v2x - cx, v2y - cy, r2)
          + seg(v2x - cx, v2y - cy, -cx, -cy, r2);
    t.abs()
}

fn optimize(r2: f64, v1x: f64, v2x: f64, v2y: f64, sx: f64, sy: f64) -> f64 {
    let mut px = [sx, sx + 0.01, sx];
    let mut py = [sy, sy, sy + 0.01];
    let mut fv = [0.0f64; 3];
    for i in 0..3 {
        fv[i] = -circle_triangle_intersection(px[i], py[i], r2, v1x, v2x, v2y);
    }

    for iter in 0..2000 {
        // Sort vertices by function value
        for i in 0..2 {
            for j in (i + 1)..3 {
                if fv[j] < fv[i] {
                    fv.swap(i, j);
                    px.swap(i, j);
                    py.swap(i, j);
                }
            }
        }

        let cx = (px[0] + px[1]) * 0.5;
        let cy = (py[0] + py[1]) * 0.5;
        let rx = 2.0 * cx - px[2];
        let ry = 2.0 * cy - py[2];
        let fr = -circle_triangle_intersection(rx, ry, r2, v1x, v2x, v2y);

        if fr < fv[0] {
            let ex = 2.0 * rx - cx;
            let ey = 2.0 * ry - cy;
            let fe = -circle_triangle_intersection(ex, ey, r2, v1x, v2x, v2y);
            if fe < fr {
                px[2] = ex; py[2] = ey; fv[2] = fe;
            } else {
                px[2] = rx; py[2] = ry; fv[2] = fr;
            }
        } else if fr < fv[1] {
            px[2] = rx; py[2] = ry; fv[2] = fr;
        } else {
            let (ccx, ccy) = if fr < fv[2] {
                (0.5 * (rx + cx), 0.5 * (ry + cy))
            } else {
                (0.5 * (px[2] + cx), 0.5 * (py[2] + cy))
            };
            let fc = -circle_triangle_intersection(ccx, ccy, r2, v1x, v2x, v2y);
            if fc < fv[2] {
                px[2] = ccx; py[2] = ccy; fv[2] = fc;
            } else {
                for i in 1..3 {
                    px[i] = 0.5 * (px[i] + px[0]);
                    py[i] = 0.5 * (py[i] + py[0]);
                    fv[i] = -circle_triangle_intersection(px[i], py[i], r2, v1x, v2x, v2y);
                }
            }
        }

        if (fv[2] - fv[0]).abs() < 1e-12 && iter > 50 {
            break;
        }
    }
    -fv[0]
}

fn main() {
    let mut total = 0.0f64;

    for a in 1..=200i32 {
        for b in a..=200 {
            if a + b > 200 { break; }
            for c in b..=200 {
                if a + b + c > 200 { break; }
                if c >= a + b { continue; }

                let s = (a + b + c) as f64 * 0.5;
                let area = (s * (s - a as f64) * (s - b as f64) * (s - c as f64)).sqrt();
                if area <= 0.0 { continue; }

                let r = (area / PI).sqrt();
                let r2 = r * r;
                let inr = area / s;

                let cos_a = (b as f64 * b as f64 + c as f64 * c as f64 - a as f64 * a as f64)
                          / (2.0 * b as f64 * c as f64);
                let sin_a = (1.0 - cos_a * cos_a).sqrt();
                let v2x = b as f64 * cos_a;
                let v2y = b as f64 * sin_a;
                let v1x = c as f64;

                if r <= inr {
                    total += area;
                    continue;
                }

                let p = (a + b + c) as f64;
                let ix = (b as f64 * c as f64 + c as f64 * v2x) / p;
                let iy = c as f64 * v2y / p;
                let best1 = optimize(r2, v1x, v2x, v2y, ix, iy);

                let cx2 = (v1x + v2x) / 3.0;
                let cy2 = v2y / 3.0;
                let best2 = optimize(r2, v1x, v2x, v2y, cx2, cy2);

                total += best1.max(best2);
            }
        }
    }

    println!("{:.2}", total);
}
