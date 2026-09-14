// Project Euler 180: Rational zeros of a function of three variables
// For reduced fractions 0 < x < 1 with denominator <= 35, find all (x,y,z)
// satisfying power-sum equations. Sum distinct s = x+y+z values.

use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

const MAX_DEN: usize = 35;

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

    let mut valid_table = [[false; MAX_DEN + 1]; MAX_DEN + 1];
    for f in &fractions {
        let n = f.num as usize;
        let d = f.den as usize;
        if n <= MAX_DEN && d <= MAX_DEN {
            valid_table[n][d] = true;
        }
    }

    let valid = |r: Frac| -> bool {
        if r.den <= 0 || r.num <= 0 || r.num >= r.den || r.den > MAX_DEN as i128 {
            return false;
        }
        let n = r.num as usize;
        let d = r.den as usize;
        n <= MAX_DEN && d <= MAX_DEN && valid_table[n][d]
    };

    let sq: Vec<Frac> = fractions.iter().map(|f| f.mul(*f)).collect();

    let mut sq_map: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
    for (idx, &s) in sq.iter().enumerate() {
        sq_map.entry(s.key()).or_insert_with(Vec::new).push(idx);
    }

    let mut sums: Vec<(i64, i64)> = Vec::with_capacity(8000);

    // Case 1: x + y = z
    for i in 0..fractions.len() {
        let fi = fractions[i];
        for j in i..fractions.len() {
            let z = fi.add(fractions[j]);
            if valid(z) {
                sums.push(z.add(z).key());
            }
        }
    }

    // Case 2: x^2 + y^2 = z^2 with HashMap lookup (parallelized)
    let sums2 = Mutex::new(Vec::with_capacity(4000));
    (0..fractions.len()).into_par_iter().for_each(|k| {
        let target = sq[k];
        let fk = fractions[k];
        let mut local = Vec::new();
        for i in 0..fractions.len() {
            let diff = target.sub(sq[i]);
            if diff.num <= 0 { continue; }
            
            if let Some(indices) = sq_map.get(&diff.key()) {
                let fi = fractions[i];
                for &j in indices {
                    if j >= i {
                        local.push(fi.add(fractions[j]).add(fk).key());
                    }
                }
            }
        }
        sums2.lock().unwrap().extend(local);
    });
    sums.extend(sums2.into_inner().unwrap());

    // Case 3: 1/x + 1/y = 1/z
    for i in 0..fractions.len() {
        let fi = fractions[i];
        for j in i..fractions.len() {
            let fj = fractions[j];
            let denom = fi.add(fj);
            if denom.num == 0 { continue; }
            let z = fi.mul(fj).div(denom);
            if valid(z) {
                sums.push(fi.add(fj).add(z).key());
            }
        }
    }

    // Case 4: 1/x^2 + 1/y^2 = 1/z^2
    for i in 0..fractions.len() {
        let fi = fractions[i];
        let sqi = sq[i];
        for j in i..fractions.len() {
            let denom = sqi.add(sq[j]);
            if denom.num == 0 { continue; }
            let z_sq = sqi.mul(sq[j]).div(denom);
            let z = frac_sqrt(z_sq);
            if z.den > 0 && valid(z) {
                sums.push(fi.add(fractions[j]).add(z).key());
            }
        }
    }

    sums.sort_unstable();
    sums.dedup();

    let mut total = Frac::new(0, 1);
    for &(n, d) in &sums {
        total = total.add(Frac::new(n as i128, d as i128));
    }

    let result = total.num + total.den;
    println!("{}", result);
}
