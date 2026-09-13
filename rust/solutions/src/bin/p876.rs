// Project Euler 876 - Triplet Tricks
// For k=1..18, a=6^k, b=10^k.
// For each divisor pair (x,y) with x|a, y|b:
// - Compute s = euclid_subtraction_steps(x,y) once
// - Generate c1 = (x+y)*(a/x + b/y) with s steps
// - Generate c2 = (x-y)*(a/x - b/y) with s-1 steps (if positive and s>1)
// Track min steps per c using FxHashMap, sum results.

use fxhash::FxHashMap;

#[inline(always)]
fn euclid_subtraction_steps(mut x: i64, mut y: i64) -> i64 {
    let mut steps = 0i64;
    while y != 0 {
        steps += x / y;
        let r = x % y;
        x = y;
        y = r;
    }
    steps
}

fn main() {
    let mut ans: i64 = 0;

    for k in 1..=18 {
        let a = 6i64.pow(k);
        let b = 10i64.pow(k);

        let mut a_divs = Vec::with_capacity(((k + 1) * (k + 1)) as usize);
        let mut pw2 = 1i64;
        for _ in 0..=k {
            let mut pw3 = 1i64;
            for _ in 0..=k {
                a_divs.push(pw2 * pw3);
                pw3 *= 3;
            }
            pw2 *= 2;
        }

        let mut b_divs = Vec::with_capacity(((k + 1) * (k + 1)) as usize);
        pw2 = 1;
        for _ in 0..=k {
            let mut pw5 = 1i64;
            for _ in 0..=k {
                b_divs.push(pw2 * pw5);
                pw5 *= 5;
            }
            pw2 *= 2;
        }

        let capacity = (a_divs.len() * b_divs.len()) / 2;
        let mut best: FxHashMap<i128, i64> = FxHashMap::with_capacity_and_hasher(capacity, Default::default());

        for &x in &a_divs {
            let u = a / x;
            for &y in &b_divs {
                let v = b / y;
                let s = euclid_subtraction_steps(x, y);

                let c1 = (x as i128 + y as i128) * (u as i128 + v as i128);
                best.entry(c1).and_modify(|e| *e = (*e).min(s)).or_insert(s);

                let c2 = ((x - y) as i128) * ((u - v) as i128);
                if c2 > 0 && s > 1 {
                    best.entry(c2).and_modify(|e| *e = (*e).min(s - 1)).or_insert(s - 1);
                }
            }
        }

        ans += best.values().sum::<i64>();
    }

    println!("{}", ans);
}
