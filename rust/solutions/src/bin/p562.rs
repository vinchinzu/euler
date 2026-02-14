// Project Euler 562 - Maximal Triangle Perimeter
//
// Find triangle with lattice point vertices within circle of radius N=10^7,
// no other lattice points on boundary, maximum perimeter.

use euler_utils::gcd_i64;

fn main() {
    let n: i64 = 10_000_000;
    let l: i64 = 20;
    let mut max_perim: f64 = 0.0;
    let mut best_abc: f64 = 0.0;

    let sq = |x: i64| -> i64 { x * x };
    let isqrt_half = (((n as f64) * (n as f64) / 2.0).sqrt()) as i64;

    let max_possible_perim = |a: f64| -> f64 { a + 2.0 * (a / 2.0).hypot(1.0 / a) };

    fn ext_gcd(a: i64, b: i64) -> (i64, i64) {
        if b == 0 {
            return (1, 0);
        }
        let (x1, y1) = ext_gcd(b, a % b);
        (y1, x1 - (a / b) * y1)
    }

    for x1 in 0..=isqrt_half {
        let mut y1 = ((sq(n) - sq(x1)) as f64).sqrt() as i64;
        while sq(x1) + sq(y1) > sq(n) {
            y1 -= 1;
        }

        let h1 = ((x1 as f64).hypot(y1 as f64));
        if max_possible_perim(n as f64 + h1) < max_perim {
            continue;
        }

        let x2_lo = -x1 - l;
        let mut x2_hi = -x1 + l;
        if x2_hi >= x1 {
            x2_hi = x1 - 1;
        }

        for x2 in x2_lo..=x2_hi {
            let y2_lo = -y1 - l;
            let mut y2_hi = -y1 + l;
            if y2_hi >= y1 {
                y2_hi = y1 - 1;
            }

            for y2 in y2_lo..=y2_hi {
                if sq(x2) + sq(y2) > sq(n) {
                    continue;
                }

                let a = ((x1 - x2) as f64).hypot((y1 - y2) as f64);
                if max_possible_perim(a) < max_perim {
                    continue;
                }
                if gcd_i64((x1 - x2).abs(), (y1 - y2).abs()) != 1 {
                    continue;
                }

                let (ex, ey) = ext_gcd(y1 - y2, x2 - x1);
                let x3 = x2 + ex;
                let y3 = y2 + ey;

                if sq(x3) + sq(y3) <= sq(n) {
                    let b = ((x1 - x3) as f64).hypot((y1 - y3) as f64);
                    let c = ((x2 - x3) as f64).hypot((y2 - y3) as f64);
                    let perim = a + b + c;
                    if perim > max_perim {
                        max_perim = perim;
                        best_abc = a * b * c;
                    }
                }
            }
        }
    }

    let ans = (best_abc / 2.0 / n as f64).round() as i64;
    println!("{}", ans);
}
