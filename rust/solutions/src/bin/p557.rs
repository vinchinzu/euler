// Project Euler 557 - Cutting a Triangle
//
// A triangle has integer area S. A cevian and a line parallel to one side
// divide it into four regions with integer areas a, b, c, d.
// Find sum of S for all valid (a,b,c,d) with S <= 10000.

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    let n: i64 = 10_000;
    let mut ans: i64 = 0;

    for a in 1..n {
        let a2 = a * a;
        for s in (a + 3)..=n {
            let aps = a + s;
            let g = gcd(a2, aps);
            let mult = aps / g;

            let sa = s - a;
            let mut d = mult;
            while d <= sa - 2 {
                let k = d / mult;
                let bc = (a2 / g) * k;

                let bpc = sa - d;
                if bpc < 2 || bc < 1 {
                    d += mult;
                    continue;
                }

                let disc = bpc * bpc - 4 * bc;
                if disc < 0 {
                    d += mult;
                    continue;
                }

                let mut sq = (disc as f64).sqrt() as i64;
                while sq * sq > disc { sq -= 1; }
                while (sq + 1) * (sq + 1) <= disc { sq += 1; }

                if sq * sq == disc && (bpc + sq) % 2 == 0 {
                    let b_val = (bpc + sq) / 2;
                    let c_val = (bpc - sq) / 2;
                    if b_val >= 1 && c_val >= 1 {
                        ans += s;
                    }
                }
                d += mult;
            }
        }
    }

    println!("{ans}");
}
