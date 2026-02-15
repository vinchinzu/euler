// Project Euler 707 - Lights Out
//
// Compute sum_{k=1}^N F(N, f_k) using GCD of polynomials over GF(2),
// Fibonacci sequences, and modular exponentiation.

const MAX_DEG: usize = 256;

#[derive(Clone)]
struct Poly {
    deg: i32,
    c: Vec<u8>,
}

impl Poly {
    fn zero() -> Self {
        Poly { deg: -1, c: vec![0; MAX_DEG + 1] }
    }
    fn one() -> Self {
        let mut p = Self::zero();
        p.c[0] = 1;
        p.deg = 0;
        p
    }
    fn x() -> Self {
        let mut p = Self::zero();
        p.c[1] = 1;
        p.deg = 1;
        p
    }
    fn is_zero(&self) -> bool {
        self.deg < 0
    }
    fn fix_deg(&mut self) {
        let mut d = self.c.len() as i32 - 1;
        while d >= 0 && self.c[d as usize] == 0 {
            d -= 1;
        }
        self.deg = d;
    }
    fn add_inplace(&mut self, b: &Poly) {
        let md = self.deg.max(b.deg);
        for i in 0..=md as usize {
            self.c[i] ^= b.c[i];
        }
        self.fix_deg();
    }
    fn shift_up(&self, n: usize) -> Poly {
        let mut result = Poly::zero();
        if self.deg < 0 {
            return result;
        }
        for i in (0..=self.deg as usize).rev() {
            result.c[i + n] = self.c[i];
        }
        result.deg = self.deg + n as i32;
        result
    }
    fn mod_by(&mut self, divisor: &Poly) {
        if self.deg < divisor.deg {
            return;
        }
        while self.deg >= divisor.deg {
            if self.c[self.deg as usize] == 0 {
                self.deg -= 1;
                continue;
            }
            let shift = (self.deg - divisor.deg) as usize;
            for i in 0..=divisor.deg as usize {
                self.c[shift + i] ^= divisor.c[i];
            }
            self.fix_deg();
        }
    }
    fn gcd(a: &Poly, b: &Poly) -> Poly {
        let mut aa = a.clone();
        let mut bb = b.clone();
        while !bb.is_zero() {
            let tmp = bb.clone();
            aa.mod_by(&bb);
            bb = aa;
            aa = tmp;
        }
        aa
    }
}

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    if m <= 1 {
        return 0;
    }
    base = ((base % m) + m) % m;
    let mut result: i64 = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % m as i128) as i64;
        }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn fibonacci(n: i64, m: i64) -> i64 {
    if m == 1 {
        return 0;
    }
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1 % m;
    }
    // Matrix exponentiation
    let mut result = [[1i64, 0], [0, 1]];
    let mut base = [[1i64, 1], [1, 0]];
    let mut exp = n - 1;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mat2_mult(&result, &base, m);
        }
        base = mat2_mult(&base, &base, m);
        exp >>= 1;
    }
    result[0][0] % m
}

fn mat2_mult(a: &[[i64; 2]; 2], b: &[[i64; 2]; 2], m: i64) -> [[i64; 2]; 2] {
    let mut c = [[0i64; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                c[i][j] = (c[i][j] + (a[i][k] as i128 * b[k][j] as i128 % m as i128) as i64) % m;
            }
        }
    }
    c
}

fn main() {
    let n = 199;
    let m = 1_000_000_007i64;

    // Build p_n(x) for n=0..N
    let mut px: Vec<Poly> = Vec::with_capacity(n + 1);
    px.push(Poly::one());
    px.push(Poly::x());
    for i in 2..=n {
        let shifted = px[i - 1].shift_up(1);
        let mut p = shifted;
        p.add_inplace(&px[i - 2]);
        px.push(p);
    }

    // Build p_h(x+1) mod p_N(x) over GF(2), find period
    let max_period = 100_000;
    let mut px1: Vec<Poly> = Vec::with_capacity(max_period + 2);
    px1.push(Poly::one());
    let mut p1 = Poly::zero();
    p1.c[0] = 1;
    p1.c[1] = 1;
    p1.deg = 1;
    px1.push(p1);

    let mut period = 2usize;
    while !px1[period - 1].is_zero() {
        let shifted = px1[period - 1].shift_up(1); // x * last
        let mut tmp = shifted;
        tmp.add_inplace(&px1[period - 1]); // (x+1) * last
        tmp.add_inplace(&px1[period - 2]); // + penult
        tmp.mod_by(&px[n]);
        px1.push(tmp);
        period += 1;
        if period > max_period {
            break;
        }
    }

    let mut ans: i64 = 0;
    for k in 1..=n {
        let fib_k_mod_period = fibonacci(k as i64, period as i64) as usize;
        let fib_k_mod_m_minus_1 = fibonacci(k as i64, m - 1);

        let gcd_poly = Poly::gcd(&px[n], &px1[fib_k_mod_period]);
        let corank = if gcd_poly.deg < 0 { 0 } else { gcd_poly.deg as i64 };

        let exponent = ((n as i64 * fib_k_mod_m_minus_1 - corank) % (m - 1) + (m - 1)) % (m - 1);
        ans = (ans + pow_mod(2, exponent, m)) % m;
    }

    println!("{}", ans);
}
