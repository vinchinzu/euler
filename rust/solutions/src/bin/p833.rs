// Project Euler 833 - Triangular Square

const MVAL: i64 = 136101521;
const MAX_POLY: usize = 200;
const MAX_YS: usize = 30;

fn mod_inv(a: i64, m: i64) -> i64 {
    let mut g = m;
    let mut x = 0i64;
    let mut y = 1i64;
    let mut a0 = a;
    while a0 != 0 {
        let q = g / a0;
        let t = g - q * a0; g = a0; a0 = t;
        let t2 = x - q * y; x = y; y = t2;
    }
    ((x % m) + m) % m
}

fn sum_powers(n: i64, k: usize) -> i64 {
    if n <= 0 { return 0; }
    if k == 0 { return n % MVAL; }

    let npts = k + 2;
    let mut y = vec![0i64; npts];
    let mut pw = 0i64;
    for j in 1..npts {
        let mut jk = 1i64;
        for _ in 0..k {
            jk = (jk as i128 * j as i128 % MVAL as i128) as i64;
        }
        pw = (pw + jk) % MVAL;
        y[j] = pw;
    }

    let n_mod = n % MVAL;

    let mut num_prod = 1i64;
    for i in 0..npts {
        num_prod = (num_prod as i128 * ((n_mod - i as i64 + MVAL) % MVAL) as i128 % MVAL as i128) as i64;
    }

    let mut fact = vec![1i64; npts];
    for i in 1..npts {
        fact[i] = (fact[i - 1] as i128 * i as i128 % MVAL as i128) as i64;
    }

    let mut result = 0i64;
    for j in 0..npts {
        let nj = (n_mod - j as i64 + MVAL) % MVAL;
        if nj == 0 {
            let mut term = y[j];
            for i in 0..npts {
                if i != j {
                    term = (term as i128 * ((n_mod - i as i64 + MVAL) % MVAL) as i128 % MVAL as i128) as i64;
                    let d = ((j as i64 - i as i64) % MVAL + MVAL) % MVAL;
                    term = (term as i128 * mod_inv(d, MVAL) as i128 % MVAL as i128) as i64;
                }
            }
            result = (result + term) % MVAL;
        } else {
            let mut denom = fact[j] as i128 * fact[npts - 1 - j] as i128 % MVAL as i128;
            if (npts - 1 - j) % 2 == 1 { denom = (MVAL as i128 - denom) % MVAL as i128; }

            let mut term = y[j] as i128 * num_prod as i128 % MVAL as i128;
            term = term * mod_inv(nj, MVAL) as i128 % MVAL as i128;
            term = term * mod_inv(denom as i64, MVAL) as i128 % MVAL as i128;
            result = (result + term as i64) % MVAL;
        }
    }
    result
}

#[derive(Clone)]
struct Poly {
    c_dbl: [f64; MAX_POLY],
    c_mod: [i64; MAX_POLY],
    deg: usize,
}

impl Poly {
    fn zero() -> Self {
        Poly { c_dbl: [0.0; MAX_POLY], c_mod: [0; MAX_POLY], deg: 0 }
    }

    fn constant(v: i64) -> Self {
        let mut p = Self::zero();
        p.c_dbl[0] = v as f64;
        p.c_mod[0] = ((v % MVAL) + MVAL) % MVAL;
        p
    }

    fn add(&self, b: &Poly) -> Poly {
        let mut r = Poly::zero();
        r.deg = self.deg.max(b.deg);
        for i in 0..=r.deg {
            let ad = if i <= self.deg { self.c_dbl[i] } else { 0.0 };
            let bd = if i <= b.deg { b.c_dbl[i] } else { 0.0 };
            r.c_dbl[i] = ad + bd;
            let am = if i <= self.deg { self.c_mod[i] } else { 0 };
            let bm = if i <= b.deg { b.c_mod[i] } else { 0 };
            r.c_mod[i] = (am + bm + MVAL) % MVAL;
        }
        while r.deg > 0 && r.c_dbl[r.deg] == 0.0 && r.c_mod[r.deg] == 0 { r.deg -= 1; }
        r
    }

    fn mul(&self, b: &Poly) -> Poly {
        let mut r = Poly::zero();
        r.deg = (self.deg + b.deg).min(MAX_POLY - 1);
        for i in 0..=self.deg {
            for j in 0..=b.deg {
                if i + j >= MAX_POLY { break; }
                r.c_dbl[i + j] += self.c_dbl[i] * b.c_dbl[j];
                r.c_mod[i + j] = (r.c_mod[i + j] as i128
                    + self.c_mod[i] as i128 * b.c_mod[j] as i128) as i64 % MVAL;
            }
        }
        r
    }

    fn eval_dbl(&self, x: f64) -> f64 {
        let mut r = 0.0;
        let mut xp = 1.0;
        for i in 0..=self.deg {
            r += self.c_dbl[i] * xp;
            xp *= x;
        }
        r
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn main() {
    // D = a(a+1): [0, 1, 1]
    let mut d_poly = Poly::zero();
    d_poly.deg = 2;
    d_poly.c_dbl[1] = 1.0; d_poly.c_dbl[2] = 1.0;
    d_poly.c_mod[1] = 1; d_poly.c_mod[2] = 1;

    // base_x = 2a+1: [1, 2]
    let mut base_x = Poly::zero();
    base_x.deg = 1;
    base_x.c_dbl[0] = 1.0; base_x.c_dbl[1] = 2.0;
    base_x.c_mod[0] = 1; base_x.c_mod[1] = 2;

    let base_y = Poly::constant(2);

    let mut x = base_x.clone();
    let mut y = base_y.clone();

    let mut ys: Vec<Poly> = Vec::new();

    while y.eval_dbl(1.0) < 1e35 && ys.len() < MAX_YS {
        ys.push(y.clone());
        let new_x = x.mul(&base_x).add(&d_poly.mul(&y.mul(&base_y)));
        let new_y = x.mul(&base_y).add(&y.mul(&base_x));
        x = new_x;
        y = new_y;
    }

    let mut ans: i64 = 0;
    for i in 0..ys.len() {
        for j in (i + 1)..ys.len() {
            if gcd((i + 1) as i64, (j + 1) as i64) != 1 { continue; }

            let prod = d_poly.mul(&ys[i].mul(&ys[j]));

            // Binary search for max a
            let mut lo = 0.0f64;
            let mut hi = 1e20f64;
            while hi - lo > 0.5 {
                let mid = (lo + hi) / 2.0;
                if prod.eval_dbl(mid) / 8.0 <= 1e35 { lo = mid; } else { hi = mid; }
            }
            let mut max_a = lo as i64;
            while max_a > 0 && prod.eval_dbl((max_a + 1) as f64) / 8.0 <= 1e35 { max_a += 1; }
            while max_a > 0 && prod.eval_dbl(max_a as f64) / 8.0 > 1e35 { max_a -= 1; }

            if max_a < 1 { continue; }

            for e in 0..=prod.deg {
                if prod.c_mod[e] != 0 {
                    let sp = sum_powers(max_a, e);
                    ans = (ans as i128 + sp as i128 * prod.c_mod[e] as i128) as i64 % MVAL;
                }
            }
        }
    }

    ans = (ans as i128 * mod_inv(8, MVAL) as i128 % MVAL as i128) as i64;
    println!("{}", ans);
}
