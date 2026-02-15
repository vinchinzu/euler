// Project Euler 843 - Circle of Absolute Differences
// GF(2) polynomial arithmetic and cyclotomic factorization

const MAX_WORDS: usize = 64;

#[derive(Clone)]
struct GF2Poly {
    w: [u64; MAX_WORDS],
    deg: i32,
}

impl GF2Poly {
    fn zero() -> Self { GF2Poly { w: [0; MAX_WORDS], deg: -1 } }
    fn one() -> Self { let mut p = Self::zero(); p.w[0] = 1; p.deg = 0; p }
    fn is_zero(&self) -> bool { self.deg < 0 }
    fn is_one(&self) -> bool { self.deg == 0 && self.w[0] == 1 }

    fn get_bit(&self, i: usize) -> bool { (self.w[i / 64] >> (i % 64)) & 1 == 1 }
    fn set_bit(&mut self, i: usize) { self.w[i / 64] |= 1u64 << (i % 64); }
    fn flip_bit(&mut self, i: usize) { self.w[i / 64] ^= 1u64 << (i % 64); }

    fn update_deg(&mut self) {
        for i in (0..MAX_WORDS).rev() {
            if self.w[i] != 0 {
                self.deg = i as i32 * 64 + (63 - self.w[i].leading_zeros() as i32);
                return;
            }
        }
        self.deg = -1;
    }

    fn xor(&self, b: &GF2Poly) -> GF2Poly {
        let mut r = GF2Poly { w: [0; MAX_WORDS], deg: 0 };
        for i in 0..MAX_WORDS { r.w[i] = self.w[i] ^ b.w[i]; }
        r.update_deg();
        r
    }

    fn equal(&self, b: &GF2Poly) -> bool {
        if self.deg != b.deg { return false; }
        self.w == b.w
    }

    fn mul(&self, b: &GF2Poly) -> GF2Poly {
        let mut r = GF2Poly::zero();
        if self.deg < 0 || b.deg < 0 { return r; }
        for i in 0..=self.deg as usize {
            if self.get_bit(i) {
                for j in 0..=b.deg as usize {
                    if b.get_bit(j) { r.flip_bit(i + j); }
                }
            }
        }
        r.update_deg();
        r
    }

    fn divmod(&self, b: &GF2Poly) -> (GF2Poly, GF2Poly) {
        let mut q = GF2Poly::zero();
        if self.deg < b.deg { return (q, self.clone()); }
        let mut r = self.clone();
        let db = b.deg;
        while r.deg >= db {
            let shift = (r.deg - db) as usize;
            q.set_bit(shift);
            for i in 0..=db as usize {
                if b.get_bit(i) { r.flip_bit(i + shift); }
            }
            r.update_deg();
        }
        (q, r)
    }

    fn modulo(&self, b: &GF2Poly) -> GF2Poly { self.divmod(b).1 }

    fn powmod(&self, mut exp: u64, m: &GF2Poly) -> GF2Poly {
        let mut result = GF2Poly::one();
        let mut cur = self.modulo(m);
        while exp > 0 {
            if exp & 1 == 1 { result = result.mul(&cur).modulo(m); }
            cur = cur.mul(&cur).modulo(m);
            exp >>= 1;
        }
        result
    }

    fn gcd(a: &GF2Poly, b: &GF2Poly) -> GF2Poly {
        let mut a = a.clone();
        let mut b = b.clone();
        while !b.is_zero() {
            let rem = a.modulo(&b);
            a = b;
            b = rem;
        }
        a
    }
}

const MAX_DEG_Z: usize = 200;

#[derive(Clone)]
struct ZPoly {
    c: [i32; MAX_DEG_Z + 1],
    deg: usize,
}

impl ZPoly {
    fn xn_minus_1(n: usize) -> Self {
        let mut p = ZPoly { c: [0; MAX_DEG_Z + 1], deg: n };
        p.c[n] = 1; p.c[0] = -1;
        p
    }

    fn div(&self, b: &ZPoly) -> ZPoly {
        let mut q = ZPoly { c: [0; MAX_DEG_Z + 1], deg: 0 };
        if self.deg < b.deg { return q; }
        q.deg = self.deg - b.deg;
        let mut ac = self.c;
        for i in (b.deg..=self.deg).rev() {
            if ac[i] == 0 { continue; }
            let coeff = ac[i] / b.c[b.deg];
            q.c[i - b.deg] = coeff;
            for j in 0..=b.deg {
                ac[i - b.deg + j] -= coeff * b.c[j];
            }
        }
        q
    }

    fn to_gf2(&self) -> GF2Poly {
        let mut p = GF2Poly::zero();
        for i in 0..=self.deg {
            if self.c[i] & 1 != 0 { p.set_bit(i); }
        }
        p.update_deg();
        p
    }
}

fn get_divisors(n: usize) -> Vec<usize> {
    let mut divs = Vec::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            divs.push(i);
            if i != n / i { divs.push(n / i); }
        }
        i += 1;
    }
    divs.sort();
    divs
}

fn get_cyclotomic(d: usize, cache: &mut Vec<Option<ZPoly>>) -> ZPoly {
    if let Some(ref p) = cache[d] { return p.clone(); }
    let mut num = ZPoly::xn_minus_1(d);
    let divs = get_divisors(d);
    for &k in &divs {
        if k < d {
            let pk = get_cyclotomic(k, cache);
            num = num.div(&pk);
        }
    }
    cache[d] = Some(num.clone());
    num
}

fn ord2_mod(d: usize) -> usize {
    if d <= 1 { return 1; }
    let mut x = 2u64;
    for i in 1..=d {
        if x % d as u64 == 0 { return d; }
        if x == 1 { return i; }
        x = (x * 2) % d as u64;
    }
    d
}

fn factorize_small(mut n: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 { n /= p; e += 1; }
            result.push((p, e));
        }
        p += 1;
    }
    if n > 1 { result.push((n, 1)); }
    result
}

fn gf2_find_factor(poly: &GF2Poly, fd: usize) -> GF2Poly {
    if poly.deg as usize == fd { return poly.clone(); }

    let mut x_poly = GF2Poly::zero();
    x_poly.set_bit(1);
    x_poly.update_deg();

    let mut xpow = x_poly.clone();
    for _ in 0..fd { xpow = xpow.powmod(2, poly); }
    let diff = xpow.xor(&x_poly);

    let g = GF2Poly::gcd(poly, &diff);
    if g.deg > 0 && g.deg < poly.deg {
        return gf2_find_factor(&g, fd);
    }

    for offset in 2..100 {
        let mut test = GF2Poly::zero();
        test.set_bit(offset);
        test.update_deg();

        let mut xp = test.clone();
        for _ in 0..fd { xp = xp.powmod(2, poly); }
        let d = xp.xor(&test);

        let g = GF2Poly::gcd(poly, &d);
        if g.deg > 0 && g.deg < poly.deg {
            return gf2_find_factor(&g, fd);
        }
    }

    poly.clone()
}

fn get_component_periods(elem: &GF2Poly, mod_poly: &GF2Poly, f: &GF2Poly) -> Vec<u64> {
    let d_f = f.deg as u32;
    let m_val = (1u64 << d_f) - 1;

    let factors = factorize_small(m_val as usize);
    let mut m = m_val;
    for &(p, e) in &factors {
        for _ in 0..e {
            let test = elem.powmod(m / p as u64, f);
            if test.is_one() { m /= p as u64; } else { break; }
        }
    }

    let mut periods = vec![1u64, m];
    let one = GF2Poly::one();
    let mut curr = m;
    loop {
        let test = elem.powmod(curr, mod_poly);
        if test.equal(&one) { break; }
        curr *= 2;
        periods.push(curr);
        if periods.len() > 60 { break; }
    }
    periods
}

fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn lcm_u128(a: u128, b: u128) -> u128 { a / gcd_u128(a, b) * b }

fn main() {
    let mut cyclo_cache: Vec<Option<ZPoly>> = vec![None; MAX_DEG_Z + 1];

    // Initialize Phi_1
    let mut phi1 = ZPoly { c: [0; MAX_DEG_Z + 1], deg: 1 };
    phi1.c[0] = -1; phi1.c[1] = 1;
    cyclo_cache[1] = Some(phi1);

    let mut master: std::collections::HashSet<u128> = std::collections::HashSet::new();

    for n in 3..=100usize {
        let mut n_odd = n;
        let mut s = 0;
        while n_odd % 2 == 0 { n_odd /= 2; s += 1; }

        let divs = get_divisors(n_odd);

        let mut all_sets: Vec<Vec<u64>> = Vec::new();

        for &d in &divs {
            if d == 1 {
                all_sets.push(vec![1]);
                continue;
            }

            let phi_d = get_cyclotomic(d, &mut cyclo_cache);
            let phi_gf2 = phi_d.to_gf2();

            let fd = ord2_mod(d);
            let f = gf2_find_factor(&phi_gf2, fd);

            let exponent = 1usize << s;
            let mut mod_poly = GF2Poly::one();
            for _ in 0..exponent { mod_poly = mod_poly.mul(&f); }

            let mut p_poly = GF2Poly::zero();
            p_poly.set_bit(1);
            p_poly.set_bit(n - 1);
            p_poly.update_deg();

            let base_elem = p_poly.modulo(&mod_poly);

            if base_elem.is_zero() {
                all_sets.push(vec![1]);
            } else {
                let periods = get_component_periods(&base_elem, &mod_poly, &f);
                all_sets.push(periods);
            }
        }

        // Compute all LCMs from Cartesian product
        let mut current: Vec<u128> = vec![1];
        for set in &all_sets {
            let mut next_set: std::collections::HashSet<u128> = std::collections::HashSet::new();
            for &c in &current {
                for &s_val in set {
                    next_set.insert(lcm_u128(c, s_val as u128));
                }
            }
            current = next_set.into_iter().collect();
        }

        for &v in &current { master.insert(v); }
    }

    let total: u128 = master.iter().sum();
    println!("{}", total);
}
