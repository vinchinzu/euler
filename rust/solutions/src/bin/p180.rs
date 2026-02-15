// Project Euler 180: Rational zeros of a function of three variables
// For reduced fractions 0 < x < 1 with denominator <= 35, find all (x,y,z)
// satisfying power-sum equations. Sum distinct s = x+y+z values.

use std::collections::HashSet;

const MAX_DEN: i64 = 35;

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

#[derive(Clone, Copy)]
struct Frac { num: i128, den: i128 }

impl Frac {
    fn new(n: i128, d: i128) -> Self {
        if d == 0 { return Frac { num: 0, den: 0 }; }
        let (mut n, mut d) = if d < 0 { (-n, -d) } else { (n, d) };
        let g = gcd(n, d);
        if g > 0 { n /= g; d /= g; }
        Frac { num: n, den: d }
    }
    fn add(self, b: Frac) -> Frac {
        let g = gcd(self.den, b.den);
        let da = self.den / g;
        let db = b.den / g;
        Frac::new(self.num * db + b.num * da, da * b.den)
    }
    fn sub(self, b: Frac) -> Frac {
        let g = gcd(self.den, b.den);
        let da = self.den / g;
        let db = b.den / g;
        Frac::new(self.num * db - b.num * da, da * b.den)
    }
    fn mul(self, b: Frac) -> Frac {
        let g1 = gcd(self.num.abs(), b.den);
        let g2 = gcd(b.num.abs(), self.den);
        Frac::new((self.num / g1) * (b.num / g2), (self.den / g2) * (b.den / g1))
    }
    fn div(self, b: Frac) -> Frac {
        self.mul(Frac { num: b.den, den: b.num })
    }
    fn eq(self, b: Frac) -> bool { self.num == b.num && self.den == b.den }
    fn gt(self, b: Frac) -> bool { self.num * b.den > b.num * self.den }
    fn key(self) -> (i64, i64) { (self.num as i64, self.den as i64) }
}

fn isqrt128(n: i128) -> i128 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).sqrt() as i128;
    if x < 0 { x = 0; }
    while x * x < n { x += 1; }
    while x * x > n { x -= 1; }
    x
}

fn frac_sqrt(r: Frac) -> Frac {
    if r.num < 0 || r.den == 0 { return Frac { num: 0, den: 0 }; }
    if r.num == 0 { return Frac::new(0, 1); }
    let sn = isqrt128(r.num);
    if sn * sn != r.num { return Frac { num: 0, den: 0 }; }
    let sd = isqrt128(r.den);
    if sd * sd != r.den { return Frac { num: 0, den: 0 }; }
    Frac::new(sn, sd)
}

fn main() {
    let fzero = Frac::new(0, 1);
    let fone = Frac::new(1, 1);

    let mut fractions = Vec::new();
    for den in 1..=MAX_DEN {
        for num in 1..den {
            let mut a = num as i128;
            let mut b = den as i128;
            while b != 0 { let t = b; b = a % b; a = t; }
            if a == 1 {
                fractions.push(Frac::new(num as i128, den as i128));
            }
        }
    }
    fractions.sort_by(|a, b| (a.num * b.den).cmp(&(b.num * a.den)));

    let frac_set: HashSet<(i64, i64)> = fractions.iter().map(|f| f.key()).collect();

    let valid = |r: Frac| -> bool {
        r.den > 0 && r.gt(fzero) && fone.gt(r) && r.den <= MAX_DEN as i128 && frac_set.contains(&r.key())
    };

    let sq: Vec<Frac> = fractions.iter().map(|f| f.mul(*f)).collect();

    let mut sums = HashSet::new();

    // Case 1: x + y = z
    for i in 0..fractions.len() {
        for j in i..fractions.len() {
            let z = fractions[i].add(fractions[j]);
            if !valid(z) { continue; }
            let s = fractions[i].add(fractions[j]).add(z);
            sums.insert(s.key());
        }
    }

    // Case 2: x^2 + y^2 = z^2
    for k in 0..fractions.len() {
        let target = sq[k];
        for i in 0..fractions.len() {
            let diff = target.sub(sq[i]);
            if diff.num <= 0 { continue; }
            for j in i..fractions.len() {
                if sq[j].eq(diff) {
                    let s = fractions[i].add(fractions[j]).add(fractions[k]);
                    sums.insert(s.key());
                }
                if sq[j].gt(diff) { break; }
            }
        }
    }

    // Case 3: 1/x + 1/y = 1/z => z = xy/(x+y)
    for i in 0..fractions.len() {
        for j in i..fractions.len() {
            let denom = fractions[i].add(fractions[j]);
            if denom.num == 0 { continue; }
            let z = fractions[i].mul(fractions[j]).div(denom);
            if !valid(z) { continue; }
            let s = fractions[i].add(fractions[j]).add(z);
            sums.insert(s.key());
        }
    }

    // Case 4: 1/x^2 + 1/y^2 = 1/z^2
    for i in 0..fractions.len() {
        for j in i..fractions.len() {
            let denom = sq[i].add(sq[j]);
            if denom.num == 0 { continue; }
            let z_sq = sq[i].mul(sq[j]).div(denom);
            let z = frac_sqrt(z_sq);
            if z.den == 0 { continue; }
            if !valid(z) { continue; }
            let s = fractions[i].add(fractions[j]).add(z);
            sums.insert(s.key());
        }
    }

    let mut total = Frac::new(0, 1);
    for &(n, d) in &sums {
        total = total.add(Frac::new(n as i128, d as i128));
    }

    let result = total.num + total.den;
    println!("{}", result);
}
