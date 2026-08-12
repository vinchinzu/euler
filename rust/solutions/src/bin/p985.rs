// Problem 985: Telescoping Triangles
// Ported from the Python reference: near-equilateral families (n,n,n+1) and (n,n+1,n+1).

const EPS: f64 = 1e-12;
const TARGET_STEPS: i32 = 20;

fn triangle_angles(a: i64, b: i64, c: i64) -> (f64, f64, f64) {
    let af = a as f64;
    let bf = b as f64;
    let cf = c as f64;

    let clamp = |x: f64| -> f64 {
        if x < -1.0 {
            -1.0
        } else if x > 1.0 {
            1.0
        } else {
            x
        }
    };

    let cos_a = clamp((bf * bf + cf * cf - af * af) / (2.0 * bf * cf));
    let cos_b = clamp((af * af + cf * cf - bf * bf) / (2.0 * af * cf));
    let cos_c = clamp((af * af + bf * bf - cf * cf) / (2.0 * af * bf));

    (cos_a.acos(), cos_b.acos(), cos_c.acos())
}

fn next_angles(a: f64, b: f64, c: f64) -> (f64, f64, f64) {
    (
        std::f64::consts::PI - 2.0 * b,
        std::f64::consts::PI - 2.0 * c,
        std::f64::consts::PI - 2.0 * a,
    )
}

fn num_existing_steps(a: i64, b: i64, c: i64, max_steps: i32) -> i32 {
    let (mut a_ang, mut b_ang, mut c_ang) = triangle_angles(a, b, c);
    let mut steps = 0;
    for _ in 0..max_steps {
        let next = next_angles(a_ang, b_ang, c_ang);
        a_ang = next.0;
        b_ang = next.1;
        c_ang = next.2;
        if a_ang <= EPS || b_ang <= EPS || c_ang <= EPS {
            break;
        }
        steps += 1;
    }
    steps
}

fn solve_main(target_steps: i32) -> i64 {
    let mut best_perimeter: Option<i64> = None;
    let mut n: i64 = 2;

    loop {
        let candidates = [(n, n, n + 1), (n, n + 1, n + 1)];
        for (a, b, c) in candidates {
            let steps = num_existing_steps(a, b, c, target_steps + 2);
            if steps == target_steps {
                let p = a + b + c;
                if best_perimeter.is_none() || p < best_perimeter.unwrap() {
                    best_perimeter = Some(p);
                }
            }
        }

        n += 1;

        if let Some(best) = best_perimeter {
            if 3 * n + 1 > best {
                break;
            }
        }

        if n > 5_000_000 {
            panic!("Search did not converge");
        }
    }

    best_perimeter.expect("no solution found")
}

fn main() {
    debug_assert_eq!(num_existing_steps(8, 9, 10, 10), 2);
    println!("{}", solve_main(TARGET_STEPS));
}
