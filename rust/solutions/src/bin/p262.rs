// Project Euler 262: Mountain Range
// Trace contour of H=fmin and find shortest path A->T1->arc->T2->B.

fn h_correct(x: f64, y: f64) -> f64 {
    (5000.0 - 0.005 * (x*x + y*y + x*y) + 12.5 * (x + y))
        * (-((0.000001 * (x*x + y*y) - 0.0015 * (x + y) + 0.7).abs())).exp()
}

fn grad_h(x: f64, y: f64) -> (f64, f64) {
    let eps = 1e-7;
    let gx = (h_correct(x + eps, y) - h_correct(x - eps, y)) / (2.0 * eps);
    let gy = (h_correct(x, y + eps) - h_correct(x, y - eps)) / (2.0 * eps);
    (gx, gy)
}

fn find_fmin() -> f64 {
    let mut a = 0.0_f64;
    let mut b = 1600.0_f64;
    let gr = (5.0_f64.sqrt() + 1.0) / 2.0;
    for _ in 0..200 {
        let c = b - (b - a) / gr;
        let d = a + (b - a) / gr;
        if h_correct(0.0, c) > h_correct(0.0, d) {
            b = d;
        } else {
            a = c;
        }
    }
    h_correct(0.0, (a + b) / 2.0)
}

fn project_to_contour(x: &mut f64, y: &mut f64, fmin: f64) {
    for _ in 0..80 {
        let val = h_correct(*x, *y) - fmin;
        if val.abs() < 1e-14 { break; }
        let (gx, gy) = grad_h(*x, *y);
        let g2 = gx * gx + gy * gy;
        if g2 < 1e-30 { break; }
        let t = val / g2;
        *x -= t * gx;
        *y -= t * gy;
    }
}

fn main() {
    let fmin = find_fmin();

    // Find first diagonal crossing
    let mut t0 = 200.0_f64;
    let mut t1 = 300.0_f64;
    while h_correct(t1, t1) < fmin { t1 += 10.0; }
    while h_correct(t0, t0) > fmin { t0 -= 10.0; }
    for _ in 0..100 {
        let tm = (t0 + t1) / 2.0;
        if h_correct(tm, tm) < fmin { t0 = tm; } else { t1 = tm; }
    }
    let cross1 = (t0 + t1) / 2.0;

    // Find second diagonal crossing
    t0 = 1200.0; t1 = 1400.0;
    while h_correct(t0, t0) < fmin { t0 -= 10.0; }
    while h_correct(t1, t1) > fmin { t1 += 10.0; }
    for _ in 0..100 {
        let tm = (t0 + t1) / 2.0;
        if h_correct(tm, tm) > fmin { t0 = tm; } else { t1 = tm; }
    }
    let cross2 = (t0 + t1) / 2.0;

    // Trace contour
    let mut px = cross1;
    let mut py = cross1;
    project_to_contour(&mut px, &mut py, fmin);

    let ds = 0.05;
    let max_contour = 500000;
    let mut cx = Vec::with_capacity(max_contour);
    let mut cy = Vec::with_capacity(max_contour);
    cx.push(px); cy.push(py);

    let (gx, gy) = grad_h(px, py);
    let gn = (gx * gx + gy * gy).sqrt();
    let (tx1, ty1) = (-gy / gn, gx / gn);
    let (tx2, ty2) = (gy / gn, -gx / gn);

    let (mut tx, mut ty) = if ty1 - tx1 < ty2 - tx2 { (tx1, ty1) } else { (tx2, ty2) };

    let mut nx = px + ds * tx;
    let mut ny = py + ds * ty;
    project_to_contour(&mut nx, &mut ny, fmin);
    px = nx; py = ny;
    cx.push(px); cy.push(py);

    let mut prev_tx = tx;
    let mut prev_ty = ty;

    for _ in 0..max_contour - 2 {
        let (gx, gy) = grad_h(px, py);
        let gn = (gx * gx + gy * gy).sqrt();
        if gn < 1e-15 { break; }

        let (tx1, ty1) = (-gy / gn, gx / gn);
        let (tx2, ty2) = (gy / gn, -gx / gn);

        let dot1 = tx1 * prev_tx + ty1 * prev_ty;
        let dot2 = tx2 * prev_tx + ty2 * prev_ty;
        if dot1 >= dot2 { tx = tx1; ty = ty1; } else { tx = tx2; ty = ty2; }

        nx = px + ds * tx;
        ny = py + ds * ty;
        project_to_contour(&mut nx, &mut ny, fmin);

        if nx > cross2 - 5.0 && ny > cross2 - 5.0 && (nx - ny).abs() < 5.0 {
            let (mut ax, mut ay) = (px, py);
            let (mut bx, mut by) = (nx, ny);
            for _ in 0..80 {
                let mut mx = (ax + bx) / 2.0;
                let mut my = (ay + by) / 2.0;
                project_to_contour(&mut mx, &mut my, fmin);
                if mx > my + 0.001 { ax = mx; ay = my; } else { bx = mx; by = my; }
            }
            cx.push((ax + bx) / 2.0);
            cy.push((ay + by) / 2.0);
            break;
        }

        px = nx; py = ny;
        prev_tx = tx; prev_ty = ty;
        cx.push(px); cy.push(py);
    }

    let contour_len = cx.len();
    let (ax, ay) = (200.0, 200.0);
    let (bx, by) = (1400.0, 1400.0);

    let mut t1_idx: Option<usize> = None;
    let mut t2_idx: Option<usize> = None;
    let mut prev_cross_a = 0.0_f64;
    let mut prev_cross_b = 0.0_f64;

    for i in 1..contour_len - 1 {
        let tanx = cx[i + 1] - cx[i - 1];
        let tany = cy[i + 1] - cy[i - 1];
        let vax = cx[i] - ax;
        let vay = cy[i] - ay;
        let cross_a = vax * tany - vay * tanx;
        let vbx = cx[i] - bx;
        let vby = cy[i] - by;
        let cross_b = vbx * tany - vby * tanx;

        if i > 1 {
            if t1_idx.is_none() && prev_cross_a * cross_a < 0.0 {
                t1_idx = Some(i);
            }
            if t1_idx.is_some() && t2_idx.is_none() && prev_cross_b * cross_b < 0.0 {
                t2_idx = Some(i);
            }
        }
        prev_cross_a = cross_a;
        prev_cross_b = cross_b;
    }

    let t1i = t1_idx.unwrap();
    let t2i = t2_idx.unwrap();

    let d_a_t1 = ((cx[t1i] - ax).powi(2) + (cy[t1i] - ay).powi(2)).sqrt();
    let d_t2_b = ((bx - cx[t2i]).powi(2) + (by - cy[t2i]).powi(2)).sqrt();

    let mut arc = 0.0;
    for i in t1i..t2i {
        let dx = cx[i + 1] - cx[i];
        let dy = cy[i + 1] - cy[i];
        arc += (dx * dx + dy * dy).sqrt();
    }

    let total = d_a_t1 + arc + d_t2_b;
    println!("{:.3}", total);
}
