// Project Euler 236: Luxury Hampers
use euler_utils::gcd;

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x1, y1) = ext_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

fn main() {
    let a1: i64 = 5248;
    let a2: i64 = 7872;
    let a3: i64 = 5760;
    let b1: i64 = 640;
    let b2: i64 = 11328;
    let b3: i64 = 3776;

    let mut best_num: i64 = 1;
    let mut best_den: i64 = 1;

    for a1v in 1..a1 {
        for b1v in 1..b1 {
            let u = b1v * a1;
            let v = a1v * b1;

            if u * best_den <= v * best_num { continue; }

            // r2 = Fraction(A2*v, B2*u)
            let r2n_raw = a2 * v;
            let r2d_raw = b2 * u;
            let g2 = gcd(r2n_raw.unsigned_abs(), r2d_raw.unsigned_abs()) as i64;
            let r2n = r2n_raw / g2;
            let r2d = r2d_raw / g2;

            let r3n_raw = a3 * v;
            let r3d_raw = b3 * u;
            let g3 = gcd(r3n_raw.unsigned_abs(), r3d_raw.unsigned_abs()) as i64;
            let r3n = r3n_raw / g3;
            let r3d = r3d_raw / g3;

            if r2n > a2 || r2d > b2 || r3n > a3 || r3d > b3 { continue; }
            if r2n <= 0 || r3n <= 0 { continue; }

            // s = Fraction((A1+A2+A3)*u, (B1+B2+B3)*v)
            let sn_raw = (a1 + a2 + a3) * u;
            let sd_raw = (b1 + b2 + b3) * v;
            let gs = gcd(sn_raw.unsigned_abs(), sd_raw.unsigned_abs()) as i64;
            let sn = sn_raw / gs;
            let sd = sd_raw / gs;

            let ca = sd * r2n - sn * r2d;
            let cb = sd * r3n - sn * r3d;
            let cc = sn * b1v - sd * a1v;

            let g_ab = gcd(ca.unsigned_abs(), cb.unsigned_abs()) as i64;
            if g_ab == 0 {
                if cc == 0 { best_num = u; best_den = v; }
                continue;
            }
            if cc % g_ab != 0 { continue; }

            let (mut g, mut ex, mut ey) = ext_gcd(ca, cb);
            if g < 0 { g = -g; ex = -ex; ey = -ey; }
            if cc % g != 0 { continue; }

            let px = ex * (cc / g);
            let py = ey * (cc / g);

            let step_t2 = cb / g;
            let step_t3 = -(ca / g);

            let max_t2 = (a2 / r2n).min(b2 / r2d);
            let max_t3 = (a3 / r3n).min(b3 / r3d);

            let mut t_lo = -1e18_f64;
            let mut t_hi = 1e18_f64;

            if step_t2 > 0 {
                let lo = (1.0 - px as f64) / step_t2 as f64;
                let hi = (max_t2 as f64 - px as f64) / step_t2 as f64;
                if lo > t_lo { t_lo = lo; }
                if hi < t_hi { t_hi = hi; }
            } else if step_t2 < 0 {
                let lo = (max_t2 as f64 - px as f64) / step_t2 as f64;
                let hi = (1.0 - px as f64) / step_t2 as f64;
                if lo > t_lo { t_lo = lo; }
                if hi < t_hi { t_hi = hi; }
            } else if px < 1 || px > max_t2 {
                continue;
            }

            if step_t3 > 0 {
                let lo = (1.0 - py as f64) / step_t3 as f64;
                let hi = (max_t3 as f64 - py as f64) / step_t3 as f64;
                if lo > t_lo { t_lo = lo; }
                if hi < t_hi { t_hi = hi; }
            } else if step_t3 < 0 {
                let lo = (max_t3 as f64 - py as f64) / step_t3 as f64;
                let hi = (1.0 - py as f64) / step_t3 as f64;
                if lo > t_lo { t_lo = lo; }
                if hi < t_hi { t_hi = hi; }
            } else if py < 1 || py > max_t3 {
                continue;
            }

            let t_min = t_lo.ceil() as i64;
            let t_max = t_hi.floor() as i64;

            if t_min <= t_max {
                best_num = u;
                best_den = v;
            }
        }
    }

    let g = gcd(best_num as u64, best_den as u64) as i64;
    println!("{}/{}", best_num / g, best_den / g);
}
