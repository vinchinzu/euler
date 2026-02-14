// Project Euler 547 - Distance of Random Points on Hollow Square Laminae
//
// Compute the sum of expected distances for all hollow square laminae of size N=40.
// Uses closed-form integrals for expected distance between unit squares.

const N: usize = 40;

fn hypot_(a: f64, b: f64) -> f64 {
    (a * a + b * b).sqrt()
}

fn i_hypot(x: f64, y: f64) -> f64 {
    if x == 0.0 && y == 0.0 { return 0.0; }
    let h = hypot_(x, y);
    let mut res = 4.0 * x * y * y * y / h
                + 4.0 * x * x * x * y / h
                - 2.0 * y * y * y / 3.0;
    if x != 0.0 {
        let yx = y / x;
        res += x * x * x * x * hypot_(yx, 1.0) * yx.asinh() / h
             + 3.0 * x * x * x * (h + y).ln()
             - 2.0 * x * x * x * (y / h).atanh();
    }
    if y != 0.0 {
        res += 2.0 * y * y * y * (h + x).ln();
    }
    res / 12.0
}

fn i_x_hypot(x: f64, y: f64) -> f64 {
    let h = hypot_(x, y);
    let mut res = 5.0 * x * x * y + 2.0 * y * y * y;
    if x != 0.0 {
        let yx = y / x;
        res += 3.0 * x * x * x * yx.asinh() / hypot_(yx, 1.0);
    }
    res * h / 24.0
}

fn i_xy_hypot(x: f64, y: f64) -> f64 {
    let h = hypot_(x, y);
    h * h * h * h * h / 15.0
}

fn definite_integral(xl: f64, xh: f64, yl: f64, yh: f64, f: fn(f64, f64) -> f64) -> f64 {
    f(xh, yh) - f(xh, yl) - f(xl, yh) + f(xl, yl)
}

fn e_val(dx: i32, dy: i32, w: i32, h: i32) -> f64 {
    let mut res = 0.0;
    let signs_w = [-w, w];
    let signs_h = [-h, h];

    for &sw in &signs_w {
        for &sh in &signs_h {
            let dxw = dx as f64 + sw as f64;
            let dyh = dy as f64 + sh as f64;

            res += definite_integral(dx as f64, dxw, dy as f64, dyh, i_xy_hypot)
                 - dyh * definite_integral(dx as f64, dxw, dy as f64, dyh, i_x_hypot)
                 - dxw * definite_integral(dy as f64, dyh, dx as f64, dxw, i_x_hypot)
                 + dxw * dyh * definite_integral(dx as f64, dxw, dy as f64, dyh, i_hypot);
        }
    }
    res
}

fn sq(n: i64) -> i64 { n * n }

fn main() {
    // unit_to_unit[dx][dy]
    let mut unit_to_unit = vec![vec![0.0f64; N]; N];
    for dx in 0..N {
        for dy in 0..N {
            unit_to_unit[dx][dy] = e_val(dx as i32, dy as i32, 1, 1);
        }
    }

    // full_to_unit[x1][y1] = sum over all (x2,y2) in NxN grid
    let mut full_to_unit = vec![vec![0.0f64; N]; N];
    for x1 in 0..N {
        for y1 in 0..N {
            let mut s = 0.0;
            for x2 in 0..N {
                for y2 in 0..N {
                    s += unit_to_unit[(x1 as i32 - x2 as i32).unsigned_abs() as usize]
                                    [(y1 as i32 - y2 as i32).unsigned_abs() as usize];
                }
            }
            full_to_unit[x1][y1] = s;
        }
    }

    // region_to_itself[w][h]
    let mut region_to_itself = vec![vec![0.0f64; N + 1]; N + 1];
    for w in 1..=N {
        for h in 1..=N {
            region_to_itself[w][h] = e_val(0, 0, w as i32, h as i32);
        }
    }

    let mut ans = 0.0;

    for xl in 1..N {
        for yl in 1..N {
            let mut full_to_region = vec![vec![0.0f64; N]; N];

            for xh in (xl + 1)..N {
                for yh in (yl + 1)..N {
                    full_to_region[xh][yh] = full_to_unit[xh - 1][yh - 1]
                        + full_to_region[xh][yh - 1]
                        + full_to_region[xh - 1][yh]
                        - full_to_region[xh - 1][yh - 1];

                    let area = sq(N as i64) - (xh - xl) as i64 * (yh - yl) as i64;
                    ans += (region_to_itself[N][N]
                          - 2.0 * full_to_region[xh][yh]
                          + region_to_itself[xh - xl][yh - yl])
                         / sq(area) as f64;
                }
            }
        }
    }

    println!("{:.4}", ans);
}
