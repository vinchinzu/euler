// Project Euler 919 - Fortunate Triangles
// S(P) = sum of a+b+c over all fortunate triangles with perimeter <= P.
// Two generators based on quadratic forms.

use std::collections::HashSet;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn pack_tri(mut a: i64, mut b: i64, mut c: i64) -> (i64, i64, i64) {
    // Sort a <= b <= c
    if a > b { std::mem::swap(&mut a, &mut b); }
    if b > c { std::mem::swap(&mut b, &mut c); }
    if a > b { std::mem::swap(&mut a, &mut b); }
    (a, b, c)
}

fn main() {
    let limit: i64 = 10_000_000;

    let mut primitives: HashSet<(i64, i64, i64)> = HashSet::new();

    let add_primitive = |a: i64, b: i64, c: i64, limit: i64, set: &mut HashSet<(i64, i64, i64)>| {
        if a <= 0 || b <= 0 || c <= 0 {
            return;
        }
        let (sa, sb, sc) = pack_tri(a, b, c);
        if sa + sb <= sc {
            return; // Invalid triangle
        }
        let g = gcd(sa, gcd(sb, sc));
        let (pa, pb, pc) = (sa / g, sb / g, sc / g);
        if pa + pb + pc > limit {
            return;
        }
        set.insert((pa, pb, pc));
    };

    // Generator 1: u^2 + 15v^2
    let max_v1 = ((2.5 * limit as f64 / 15.0).sqrt() as i64) + 2;
    let max_u1 = ((2.5 * limit as f64).sqrt() as i64) + 2;

    for v in 1..max_v1 {
        for u in 1..max_u1 {
            if gcd(u, v) != 1 {
                continue;
            }

            if u % 2 != 0 && v % 2 != 0 {
                let c_val = u * u + 15 * v * v;
                if c_val % 4 != 0 {
                    continue;
                }
                let c = c_val / 4;
                let b = 2 * u * v;

                let val1 = 15 * v * v - u * u + 2 * u * v;
                if val1 % 4 == 0 {
                    let a1 = (val1 / 4).abs();
                    if a1 > 0 {
                        add_primitive(a1, b, c, limit, &mut primitives);
                    }
                }

                let val2 = 15 * v * v - u * u - 2 * u * v;
                if val2 % 4 == 0 {
                    let a2 = (val2 / 4).abs();
                    if a2 > 0 {
                        add_primitive(a2, b, c, limit, &mut primitives);
                    }
                }
            } else {
                let c = u * u + 15 * v * v;
                let b = 8 * u * v;
                let val1 = 15 * v * v - u * u + 2 * u * v;
                let a1 = val1.abs();
                if a1 > 0 {
                    add_primitive(a1, b, c, limit, &mut primitives);
                }

                let val2 = 15 * v * v - u * u - 2 * u * v;
                let a2 = val2.abs();
                if a2 > 0 {
                    add_primitive(a2, b, c, limit, &mut primitives);
                }
            }
        }
    }

    // Generator 2: 3u^2 + 5v^2
    let max_v2 = ((2.5 * limit as f64 / 5.0).sqrt() as i64) + 2;
    let max_u2 = ((2.5 * limit as f64 / 3.0).sqrt() as i64) + 2;

    for v in 1..max_v2 {
        for u in 1..max_u2 {
            if gcd(u, v) != 1 {
                continue;
            }

            if u % 2 != 0 && v % 2 != 0 {
                let c_val = 3 * u * u + 5 * v * v;
                if c_val % 4 != 0 {
                    continue;
                }
                let c = c_val / 4;
                let b = 2 * u * v;

                let val1 = 5 * v * v - 3 * u * u + 2 * u * v;
                if val1 % 4 == 0 {
                    let a1 = (val1 / 4).abs();
                    if a1 > 0 {
                        add_primitive(a1, b, c, limit, &mut primitives);
                    }
                }

                let val2 = 5 * v * v - 3 * u * u - 2 * u * v;
                if val2 % 4 == 0 {
                    let a2 = (val2 / 4).abs();
                    if a2 > 0 {
                        add_primitive(a2, b, c, limit, &mut primitives);
                    }
                }
            } else {
                let c = 3 * u * u + 5 * v * v;
                let b = 8 * u * v;
                let val1 = 5 * v * v - 3 * u * u + 2 * u * v;
                let a1 = val1.abs();
                if a1 > 0 {
                    add_primitive(a1, b, c, limit, &mut primitives);
                }

                let val2 = 5 * v * v - 3 * u * u - 2 * u * v;
                let a2 = val2.abs();
                if a2 > 0 {
                    add_primitive(a2, b, c, limit, &mut primitives);
                }
            }
        }
    }

    // Sum over all primitives
    let mut ans: i64 = 0;
    for &(a, b, c) in &primitives {
        let p = a + b + c;
        if p > limit {
            continue;
        }
        let count = limit / p;
        ans += p * count * (count + 1) / 2;
    }

    println!("{}", ans);
}
