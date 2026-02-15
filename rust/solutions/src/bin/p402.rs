// Project Euler 402: Integer-valued Polynomials
// M(a,b,c) = max m dividing n^4+a*n^3+b*n^2+c*n for all integers n.
// S(N) = sum_{0<a,b,c<=N} M(a,b,c).
// Find last 9 digits of sum_{k=2}^{1234567890123} S(F_k).
// Uses Pisano period mod 24, cubic polynomial fitting, 11x11 matrix exponentiation.

const MOD_BASE: i64 = 1_000_000_000;
const WMOD: i64 = 288 * MOD_BASE; // 288_000_000_000
const DIM: usize = 11;

fn mm(a: i64, b: i64, p: i64) -> i64 {
    ((a as i128 * b as i128) % p as i128) as i64
}

fn gcd_ll(mut a: i64, mut b: i64) -> i64 {
    a = a.abs(); b = b.abs();
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

type M2 = [[i64; 2]; 2];

fn m2_mul(a: &M2, b: &M2, p: i64) -> M2 {
    let mut c = [[0i64; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            let mut s: i128 = 0;
            for k in 0..2 { s += a[i][k] as i128 * b[k][j] as i128; }
            c[i][j] = (s % p as i128) as i64;
        }
    }
    c
}

fn m2_pow(m: &M2, mut e: i64, p: i64) -> M2 {
    let mut r: M2 = [[1, 0], [0, 1]];
    let mut b = *m;
    while e > 0 {
        if e & 1 == 1 { r = m2_mul(&r, &b, p); }
        b = m2_mul(&b, &b, p);
        e >>= 1;
    }
    r
}

fn fib_mod(n: i64, p: i64) -> i64 {
    if n <= 0 { return 0; }
    if n == 1 { return 1 % p; }
    let f: M2 = [[1, 1], [1, 0]];
    let r = m2_pow(&f, n - 1, p);
    r[0][0]
}

type M11 = [[i64; DIM]; DIM];

fn m11_mul(a: &M11, b: &M11, p: i64) -> M11 {
    let mut c = [[0i64; DIM]; DIM];
    for i in 0..DIM {
        for k in 0..DIM {
            if a[i][k] == 0 { continue; }
            for j in 0..DIM {
                c[i][j] = ((c[i][j] as i128 + a[i][k] as i128 * b[k][j] as i128) % p as i128) as i64;
            }
        }
    }
    c
}

fn m11_pow(m: &M11, mut e: i64, p: i64) -> M11 {
    let mut r = [[0i64; DIM]; DIM];
    for i in 0..DIM { r[i][i] = 1; }
    let mut b = *m;
    while e > 0 {
        if e & 1 == 1 { r = m11_mul(&r, &b, p); }
        b = m11_mul(&b, &b, p);
        e >>= 1;
    }
    r
}

fn count_res(n: i64, j: usize) -> i64 {
    let q = n / 24;
    let r = (n % 24) as usize;
    if j == 0 { q } else { q + if j <= r { 1 } else { 0 } }
}

fn s_exact(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut m_tab = [[[0i32; 24]; 24]; 24];
    for a in 0..24usize {
        for b in 0..24usize {
            for c in 0..24usize {
                let p1 = (1 + a + b + c) as i64;
                let p2 = (16 + 8*a + 4*b + 2*c) as i64;
                let p3 = (81 + 27*a + 9*b + 3*c) as i64;
                let p4 = (256 + 64*a + 16*b + 4*c) as i64;
                let g1 = gcd_ll(p1, p2);
                let g2 = gcd_ll(p3, p4);
                m_tab[a][b][c] = gcd_ll(g1, g2) as i32;
            }
        }
    }
    let mut cnt = [0i64; 24];
    for j in 0..24 { cnt[j] = count_res(n, j); }
    let mut t: i64 = 0;
    for a in 0..24 {
        if cnt[a] == 0 { continue; }
        for b in 0..24 {
            if cnt[b] == 0 { continue; }
            for c in 0..24 {
                if cnt[c] == 0 { continue; }
                t += cnt[a] * cnt[b] * cnt[c] * m_tab[a][b][c] as i64;
            }
        }
    }
    t
}

#[derive(Clone, Copy)]
struct Frac { num: i64, den: i64 }

impl Frac {
    fn new(n: i64, d: i64) -> Frac {
        let (mut n, mut d) = if d < 0 { (-n, -d) } else { (n, d) };
        let g = gcd_ll(n.abs(), d.abs());
        if g > 0 { n /= g; d /= g; }
        Frac { num: n, den: d }
    }
    fn from(v: i64) -> Frac { Frac { num: v, den: 1 } }
    fn sub(self, b: Frac) -> Frac { Frac::new(self.num * b.den - b.num * self.den, self.den * b.den) }
    fn div(self, b: Frac) -> Frac { Frac::new(self.num * b.den, self.den * b.num) }
    fn mul(self, b: Frac) -> Frac { Frac::new(self.num * b.num, self.den * b.den) }
}

fn build_t(aa: i64, bb: i64, cc: i64, dd: i64, e0: i64, e1: i64, e2: i64, e3: i64, p: i64) -> M11 {
    let mut t = [[0i64; DIM]; DIM];
    let d = dd % p; let c = cc % p; let b = bb % p; let a = aa % p;

    t[0][0] = mm(mm(d,d,p),d,p);
    t[0][1] = 3*mm(mm(d,d,p),c,p) % p;
    t[0][2] = 3*mm(mm(d,c,p),c,p) % p;
    t[0][3] = mm(mm(c,c,p),c,p);
    t[1][0] = mm(mm(b,d,p),d,p);
    t[1][1] = (mm(mm(a,d,p),d,p) + 2*mm(mm(b,d,p),c,p)) % p;
    t[1][2] = (2*mm(mm(a,d,p),c,p) + mm(mm(b,c,p),c,p)) % p;
    t[1][3] = mm(mm(a,c,p),c,p);
    t[2][0] = mm(mm(d,b,p),b,p);
    t[2][1] = (2*mm(mm(d,a,p),b,p) + mm(mm(c,b,p),b,p)) % p;
    t[2][2] = (mm(mm(d,a,p),a,p) + 2*mm(mm(c,a,p),b,p)) % p;
    t[2][3] = mm(mm(c,a,p),a,p);
    t[3][0] = mm(mm(b,b,p),b,p);
    t[3][1] = 3*mm(mm(b,b,p),a,p) % p;
    t[3][2] = 3*mm(mm(b,a,p),a,p) % p;
    t[3][3] = mm(mm(a,a,p),a,p);
    t[4][4] = mm(d,d,p); t[4][5] = 2*mm(d,c,p) % p; t[4][6] = mm(c,c,p);
    t[5][4] = mm(b,d,p); t[5][5] = (mm(a,d,p) + mm(b,c,p)) % p; t[5][6] = mm(a,c,p);
    t[6][4] = mm(b,b,p); t[6][5] = 2*mm(a,b,p) % p; t[6][6] = mm(a,a,p);
    t[7][7] = d; t[7][8] = c;
    t[8][7] = b; t[8][8] = a;
    t[9][9] = 1;

    for j in 0..4 { t[10][j] = mm(e3, t[0][j], p); }
    for j in 4..7 { t[10][j] = mm(e2, t[4][j], p); }
    for j in 7..9 { t[10][j] = mm(e1, t[7][j], p); }
    t[10][9] = e0 % p;
    t[10][10] = 1;

    for i in 0..DIM {
        for j in 0..DIM {
            t[i][j] = ((t[i][j] % p) + p) % p;
        }
    }
    t
}

fn main() {
    let n_target: i64 = 1234567890123;

    // Precompute M_tab
    let mut m_tab = [[[0i32; 24]; 24]; 24];
    for a in 0..24usize {
        for b in 0..24usize {
            for c in 0..24usize {
                let p1 = (1 + a + b + c) as i64;
                let p2 = (16 + 8*a + 4*b + 2*c) as i64;
                let p3 = (81 + 27*a + 9*b + 3*c) as i64;
                let p4 = (256 + 64*a + 16*b + 4*c) as i64;
                let g1 = gcd_ll(p1, p2);
                let g2 = gcd_ll(p3, p4);
                m_tab[a][b][c] = gcd_ll(g1, g2) as i32;
            }
        }
    }

    // Fibonacci mod 24
    let mut fmod24 = [0usize; 26];
    fmod24[0] = 0; fmod24[1] = 1;
    for i in 2..26 { fmod24[i] = (fmod24[i-1] + fmod24[i-2]) % 24; }

    // Fit 288*S(N) as cubic polynomial for each residue class mod 24
    let mut i288 = [[0i64; 4]; 24];
    for s in 0..24usize {
        let xs: [i64; 4] = if s == 0 { [24,48,72,96] } else { [s as i64, s as i64+24, s as i64+48, s as i64+72] };
        let mut ys = [0i64; 4];
        for i in 0..4 { ys[i] = s_exact(xs[i]); }

        let mut a_mat = [[Frac::from(0); 4]; 4];
        let mut bv = [Frac::from(0); 4];
        for i in 0..4 {
            bv[i] = Frac::from(ys[i]);
            for j in 0..4 {
                let mut xp: i64 = 1;
                for _ in 0..j { xp *= xs[i]; }
                a_mat[i][j] = Frac::from(xp);
            }
        }
        for col in 0..4 {
            let mut piv = col;
            while a_mat[piv][col].num == 0 { piv += 1; }
            if piv != col {
                a_mat.swap(col, piv);
                bv.swap(col, piv);
            }
            for row in 0..4 {
                if row == col { continue; }
                let f = a_mat[row][col].div(a_mat[col][col]);
                for j in 0..4 {
                    a_mat[row][j] = a_mat[row][j].sub(f.mul(a_mat[col][j]));
                }
                bv[row] = bv[row].sub(f.mul(bv[col]));
            }
        }
        for i in 0..4 {
            let coeff = bv[i].div(a_mat[i][i]);
            let c288 = coeff.mul(Frac::from(288));
            i288[s][i] = c288.num / c288.den;
        }
    }

    let f24: M2 = [[1, 1], [1, 0]];
    let a24 = m2_pow(&f24, 24, WMOD);
    let aa = a24[0][0]; let bb = a24[0][1];
    let cc = a24[1][0]; let dd = a24[1][1];

    let mut total_288: i64 = 0;

    for i in 0..24usize {
        let s = fmod24[i];
        let m_start: i64 = if i >= 2 { 0 } else { 1 };
        let m_end = (n_target - i as i64) / 24;
        if m_end < m_start { continue; }
        let num_terms = m_end - m_start + 1;

        let e0 = ((i288[s][0] % WMOD) + WMOD) % WMOD;
        let e1 = ((i288[s][1] % WMOD) + WMOD) % WMOD;
        let e2 = ((i288[s][2] % WMOD) + WMOD) % WMOD;
        let e3 = ((i288[s][3] % WMOD) + WMOD) % WMOD;

        let t = build_t(aa, bb, cc, dd, e0, e1, e2, e3, WMOD);

        let k0 = i as i64 + 24 * m_start;
        let g0 = fib_mod(k0, WMOD);
        let h0 = fib_mod(k0 + 1, WMOD);

        let s0_288 = ((e0 as i128
            + (e1 as i128 * g0 as i128) % WMOD as i128
            + (e2 as i128 * ((g0 as i128 * g0 as i128) % WMOD as i128)) % WMOD as i128
            + (e3 as i128 * ((g0 as i128 * g0 as i128 % WMOD as i128) * g0 as i128 % WMOD as i128)) % WMOD as i128
        ) % WMOD as i128) as i64;

        let mut state = [0i64; DIM];
        state[0] = mm(mm(g0, g0, WMOD), g0, WMOD);
        state[1] = mm(mm(g0, g0, WMOD), h0, WMOD);
        state[2] = mm(mm(g0, h0, WMOD), h0, WMOD);
        state[3] = mm(mm(h0, h0, WMOD), h0, WMOD);
        state[4] = mm(g0, g0, WMOD);
        state[5] = mm(g0, h0, WMOD);
        state[6] = mm(h0, h0, WMOD);
        state[7] = g0;
        state[8] = h0;
        state[9] = 1;
        state[10] = s0_288;

        if num_terms == 1 {
            total_288 = (total_288 + s0_288) % WMOD;
            continue;
        }

        let tp = m11_pow(&t, num_terms - 1, WMOD);
        let mut val: i64 = 0;
        for j in 0..DIM {
            val = ((val as i128 + tp[10][j] as i128 * state[j] as i128) % WMOD as i128) as i64;
        }
        total_288 = (total_288 + val) % WMOD;
    }

    println!("{}", (total_288 / 288) % MOD_BASE);
}
