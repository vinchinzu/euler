// Project Euler 727 - Triangle of Circles
//
// Compute distance DE between incenter and equal detour point using
// barycentric coordinates, for all coprime triples (ra, rb, rc).

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn de(ra: i32, rb: i32, rc: i32) -> f64 {
    let a = (rb + rc) as f64;
    let b = (ra + rc) as f64;
    let c = (ra + rb) as f64;
    let s = (a + b + c) / 2.0;
    let k = (s * (s - a) * (s - b) * (s - c)).sqrt();

    let d_norm = a + b + c;

    let ea = a + k / (s - a);
    let eb = b + k / (s - b);
    let ec = c + k / (s - c);
    let e_norm = ea + eb + ec;

    let x = a / d_norm - ea / e_norm;
    let y = b / d_norm - eb / e_norm;
    let z = c / d_norm - ec / e_norm;

    (-(a * a * y * z + b * b * x * z + c * c * x * y)).sqrt()
}

fn main() {
    let n = 100;
    let mut count = 0;
    let mut total = 0.0;

    for ra in 1..=n {
        for rb in (ra + 1)..=n {
            let g = gcd(ra, rb);
            for rc in (rb + 1)..=n {
                if gcd(g, rc) == 1 {
                    count += 1;
                    total += de(ra, rb, rc);
                }
            }
        }
    }

    println!("{:.8}", total / count as f64);
}
