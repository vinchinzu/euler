// Project Euler 878 - XOR-Equation B
//
// The XOR-product x*y is carryless (polynomial) multiplication in GF(2).
// Define (a*a) XOR (2*a*b) XOR (b*b) = k. This equals the "S-norm" of (a,b)
// in the extension ring Z_2[x] adjoin omega where omega^2 + x*omega + 1 = 0.
//
// We factor k over GF(2)[x], lift to S-elements via prime generators,
// enumerate all factorizations, and count valid (a,b) pairs with 0 <= a <= b <= N.
//
// G(N, m) = sum_{k=0..m} F(N, k), where F(N,k) counts pairs (a,b) with
// 0 <= a <= b <= N and S-norm(a,b) = k.

use rayon::prelude::*;

type Poly = u64;
type SElement = (Poly, Poly);

const X: Poly = 2;

// Packed generator table: index by polynomial bitmask (all keys are <= m).
// 0 = unknown, 1 = inert (no prime generator), 2 = Some(gen_val[p]).
struct GenTable {
    flag: Vec<u8>,
    val: Vec<SElement>,
}

impl GenTable {
    #[inline(always)]
    fn get(&self, p: Poly) -> Option<SElement> {
        let i = p as usize;
        match self.flag[i] {
            1 => None,
            2 => Some(self.val[i]),
            _ => find_prime_generator(p),
        }
    }
}

#[inline(always)]
fn poly_deg(a: Poly) -> i32 {
    if a == 0 {
        -1
    } else {
        63 - a.leading_zeros() as i32
    }
}

#[inline(always)]
fn poly_mul64(a: Poly, b: Poly) -> Poly {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        return poly_mul64_pclmul(a, b);
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        poly_mul64_soft(a, b)
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
#[inline]
unsafe fn poly_mul64_pclmul(a: Poly, b: Poly) -> Poly {
    use std::arch::x86_64::*;
    let va = _mm_cvtsi64_si128(a as i64);
    let vb = _mm_cvtsi64_si128(b as i64);
    let r = _mm_clmulepi64_si128::<0>(va, vb);
    _mm_cvtsi128_si64(r) as u64
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
fn poly_mul64_soft(mut a: Poly, mut b: Poly) -> Poly {
    let mut res: Poly = 0;
    while b > 0 {
        if b & 1 != 0 {
            res ^= a;
        }
        a <<= 1;
        b >>= 1;
    }
    res
}

#[inline(always)]
fn poly_mod(mut a: Poly, b: Poly) -> Poly {
    if a == 0 {
        return 0;
    }
    let lb = b.leading_zeros();
    while a != 0 {
        let la = a.leading_zeros();
        if la > lb {
            break;
        }
        a ^= b << (lb - la);
    }
    a
}

#[inline(always)]
fn poly_divmod(mut a: Poly, b: Poly) -> (Poly, Poly) {
    if a == 0 {
        return (0, 0);
    }
    let lb = b.leading_zeros();
    let mut q: Poly = 0;
    while a != 0 {
        let la = a.leading_zeros();
        if la > lb {
            break;
        }
        let shift = lb - la;
        q |= 1u64 << shift;
        a ^= b << shift;
    }
    (q, a)
}

#[inline(always)]
fn poly_try_div(mut a: Poly, b: Poly) -> Option<Poly> {
    if a == 0 {
        return Some(0);
    }
    let lb = b.leading_zeros();
    let mut q: Poly = 0;
    while a != 0 {
        let la = a.leading_zeros();
        if la > lb {
            return None;
        }
        let shift = lb - la;
        q |= 1u64 << shift;
        a ^= b << shift;
    }
    Some(q)
}

// a(x)^2 = a(x^2): scatter bits into even positions (fits in u64 for deg < 32).
#[inline(always)]
fn poly_sq64(a: Poly) -> Poly {
    let mut x = a;
    x = (x | (x << 16)) & 0x0000_FFFF_0000_FFFF;
    x = (x | (x << 8)) & 0x00FF_00FF_00FF_00FF;
    x = (x | (x << 4)) & 0x0F0F_0F0F_0F0F_0F0F;
    x = (x | (x << 2)) & 0x3333_3333_3333_3333;
    x = (x | (x << 1)) & 0x5555_5555_5555_5555;
    x
}

#[inline(always)]
fn s_mul(alpha: SElement, beta: SElement) -> SElement {
    let (a1, b1) = alpha;
    let (a2, b2) = beta;
    let a1a2 = poly_mul64(a1, a2);
    let b1b2 = poly_mul64(b1, b2);
    let outer = poly_mul64(a1, b2) ^ poly_mul64(b1, a2);
    (a1a2 ^ b1b2, outer ^ (b1b2 << 1))
}

fn s_norm(alpha: SElement) -> Poly {
    let (a, b) = alpha;
    poly_sq64(a) ^ poly_mul64(a << 1, b) ^ poly_sq64(b)
}

#[inline(always)]
fn s_conjugate(alpha: SElement) -> SElement {
    let (a, b) = alpha;
    (a ^ (b << 1), b)
}

fn s_divmod(alpha: SElement, beta: SElement) -> (SElement, SElement) {
    let beta_conj = s_conjugate(beta);
    let num = s_mul(alpha, beta_conj);
    let den = s_norm(beta);
    if den == 0 {
        panic!("division by zero s-element");
    }
    let (u, v) = num;
    let (qu, _) = poly_divmod(u, den);
    let (qv, _) = poly_divmod(v, den);
    let q = (qu, qv);
    let prod = s_mul(q, beta);
    (q, (alpha.0 ^ prod.0, alpha.1 ^ prod.1))
}

fn s_gcd(mut alpha: SElement, mut beta: SElement) -> SElement {
    if alpha == (0, 0) {
        return beta;
    }
    if beta == (0, 0) {
        return alpha;
    }
    while beta != (0, 0) {
        let (_, r) = s_divmod(alpha, beta);
        alpha = beta;
        beta = r;
    }
    alpha
}

fn get_irreducibles(max_deg: i32) -> Vec<Poly> {
    let mut irreds: Vec<Poly> = Vec::new();
    let upper = 1u64 << (max_deg + 1);
    for i in 2..upper {
        if i > 2 && (i & 1) == 0 {
            continue;
        }
        let deg = poly_deg(i);
        let mut is_irred = true;
        for &p in &irreds {
            if poly_deg(p) * 2 > deg {
                break;
            }
            if poly_mod(i, p) == 0 {
                is_irred = false;
                break;
            }
        }
        if is_irred {
            irreds.push(i);
        }
    }
    irreds
}

// Smallest irreducible polynomial factor of every k in 1..=m.
fn build_spf(m: u64, irreds: &[Poly]) -> Vec<u32> {
    let mu = m as usize;
    let mut spf = vec![0u32; mu + 1];
    let max_deg = poly_deg(m);
    for &p in irreds {
        let dp = poly_deg(p);
        let max_q = 1u64 << ((max_deg - dp + 1) as u32);
        for q in 1..max_q {
            let prod = poly_mul64(p, q);
            if prod <= m {
                let idx = prod as usize;
                if spf[idx] == 0 {
                    spf[idx] = p as u32;
                }
            }
        }
    }
    spf
}

const MAX_FACTORS: usize = 12;

fn factor_poly_spf(mut k: Poly, spf: &[u32]) -> ([(Poly, u32); MAX_FACTORS], usize) {
    let mut items = [(0u64, 0u32); MAX_FACTORS];
    let mut n = 0usize;
    if k <= 1 {
        return (items, 0);
    }
    let tz = k.trailing_zeros();
    if tz > 0 {
        items[n] = (X, tz);
        n += 1;
        k >>= tz;
    }
    while k > 1 {
        let p = spf[k as usize] as Poly;
        if p == 0 {
            items[n] = (k, 1);
            n += 1;
            break;
        }
        let mut e = 0u32;
        loop {
            match poly_try_div(k, p) {
                Some(q) => {
                    k = q;
                    e += 1;
                }
                None => break,
            }
        }
        items[n] = (p, e);
        n += 1;
    }
    (items, n)
}

fn inverse_poly_mod(a: Poly, m: Poly) -> Poly {
    let mut t: Poly = 0;
    let mut newt: Poly = 1;
    let mut r = m;
    let mut newr = a;
    while newr != 0 {
        let (q, _) = poly_divmod(r, newr);
        let tmp_t = newt;
        newt = t ^ poly_mul64(q, newt);
        t = tmp_t;
        let tmp_r = newr;
        newr = r ^ poly_mul64(q, newr);
        r = tmp_r;
    }
    if poly_deg(r) > 0 {
        panic!("not invertible");
    }
    t
}

fn solve_root_quadratic(c_val: Poly, p: Poly) -> Option<Poly> {
    let d = poly_deg(p);
    if d % 2 == 1 {
        let mut z: Poly = 0;
        let mut term = c_val;
        for _ in 0..((d + 1) / 2) {
            z ^= term;
            term = poly_mod(poly_sq64(term), p);
            term = poly_mod(poly_sq64(term), p);
        }
        return Some(z);
    }
    let du = d as usize;
    let mut rows = [0u32; 24];
    for i in 0..du {
        let xi = 1u64 << i;
        let x2i_mod_p = poly_mod(poly_sq64(xi), p);
        let col = x2i_mod_p ^ xi;
        for j in 0..du {
            if (col >> j) & 1 != 0 {
                rows[j] |= 1u32 << i;
            }
        }
    }
    for j in 0..du {
        if (c_val >> j) & 1 != 0 {
            rows[j] |= 1u32 << du;
        }
    }
    let mut cur_row = 0;
    let mut pivots = [usize::MAX; 24];
    for col in 0..du {
        for row in cur_row..du {
            if (rows[row] >> col) & 1 != 0 {
                rows.swap(cur_row, row);
                pivots[col] = cur_row;
                let pivot = rows[cur_row];
                for r in 0..du {
                    if r != cur_row && (rows[r] >> col) & 1 != 0 {
                        rows[r] ^= pivot;
                    }
                }
                cur_row += 1;
                break;
            }
        }
    }
    for row in cur_row..du {
        if (rows[row] >> du) & 1 != 0 {
            return None;
        }
    }
    let mut z: Poly = 0;
    for col in 0..du {
        if pivots[col] != usize::MAX && (rows[pivots[col]] >> du) & 1 != 0 {
            z |= 1u64 << col;
        }
    }
    Some(z)
}

fn find_prime_generator(p: Poly) -> Option<SElement> {
    if p == X {
        return Some((1, 1));
    }
    let inv_x2 = inverse_poly_mod(poly_sq64(X), p);
    let z = solve_root_quadratic(inv_x2, p)?;
    let val = poly_sq64(z) ^ z;
    if poly_mod(val, p) != inv_x2 {
        return None;
    }
    let t0 = poly_mod(z << 1, p);
    let pg = s_gcd((p, 0), (t0, 1));
    Some(pg)
}

// Multiplication by omega: (A, B) -> (B, A + x*B)
fn iterate_orbit_omega(start: SElement, n_limit: Poly, n_deg: i32) -> u64 {
    let mut count = 0u64;
    let mut a = start.0;
    let mut b = start.1;
    let n_deg_p2 = n_deg + 2;
    for _ in 0..100_000 {
        let da = poly_deg(a);
        let db = poly_deg(b);
        if da > n_deg_p2 && db > n_deg_p2 {
            break;
        }
        if a <= n_limit && b <= n_limit && a <= b {
            count += 1;
        } else if a > n_limit && b > n_limit && da > n_deg && db > n_deg {
            break;
        }
        let nb = a ^ b.wrapping_shl(1);
        a = b;
        b = nb;
    }
    count
}

// Multiplication by omega^{-1}: (A, B) -> (x*A + B, A)
fn iterate_orbit_omega_inv(start: SElement, n_limit: Poly, n_deg: i32) -> u64 {
    let mut count = 0u64;
    let mut a = start.0;
    let mut b = start.1;
    let n_deg_p2 = n_deg + 2;
    for _ in 0..100_000 {
        let da = poly_deg(a);
        let db = poly_deg(b);
        if da > n_deg_p2 && db > n_deg_p2 {
            break;
        }
        if a <= n_limit && b <= n_limit && a <= b {
            count += 1;
        } else if a > n_limit && b > n_limit && da > n_deg && db > n_deg {
            break;
        }
        let na = a.wrapping_shl(1) ^ b;
        b = a;
        a = na;
    }
    count
}

fn count_for_k(
    k: Poly,
    n_limit: Poly,
    n_deg: i32,
    spf: &[u32],
    gens: &GenTable,
    current: &mut Vec<SElement>,
    next: &mut Vec<SElement>,
) -> u64 {
    if k == 0 {
        return 1;
    }

    let (factors, nfact) = factor_poly_spf(k, spf);

    current.clear();
    current.push((1, 0));

    for i in 0..nfact {
        let (p, e) = factors[i];
        if p == X {
            let pg = gens.get(p);
            if let Some(g) = pg {
                let mut term: SElement = (1, 0);
                let mut base = g;
                let mut exp = e;
                while exp > 0 {
                    if exp & 1 != 0 {
                        term = s_mul(term, base);
                    }
                    base = s_mul(base, base);
                    exp >>= 1;
                }
                for cg in current.iter_mut() {
                    *cg = s_mul(*cg, term);
                }
            } else {
                for cg in current.iter_mut() {
                    *cg = (0, 0);
                }
            }
            continue;
        }

        let pg = gens.get(p);
        if pg.is_none() {
            if e % 2 != 0 {
                return 0;
            }
            let half = e / 2;
            let mut scale: Poly = 1;
            for _ in 0..half {
                scale = poly_mul64(scale, p);
            }
            for (ga, gb) in current.iter_mut() {
                *ga = poly_mul64(*ga, scale);
                *gb = poly_mul64(*gb, scale);
            }
            continue;
        }

        let pg = pg.unwrap();
        let pg_conj = s_conjugate(pg);

        let eu = e as usize;
        let mut pow_pi = [(1u64, 0u64); 20];
        let mut pow_bar = [(1u64, 0u64); 20];
        let mut curr: SElement = (1, 0);
        for i in 1..=eu {
            curr = s_mul(curr, pg);
            pow_pi[i] = curr;
        }
        curr = (1, 0);
        for i in 1..=eu {
            curr = s_mul(curr, pg_conj);
            pow_bar[i] = curr;
        }

        next.clear();
        for &g in current.iter() {
            for a in 0..=eu {
                let b = eu - a;
                let factor = s_mul(pow_pi[a], pow_bar[b]);
                next.push(s_mul(g, factor));
            }
        }
        std::mem::swap(current, next);
    }

    let mut total_count = 0u64;
    for &g in current.iter() {
        total_count += iterate_orbit_omega(g, n_limit, n_deg);
        // g * omega^{-1} = (x*A + B, A)
        let g_inv = (g.0.wrapping_shl(1) ^ g.1, g.0);
        total_count += iterate_orbit_omega_inv(g_inv, n_limit, n_deg);
    }
    total_count
}

fn solve(n: u64, m: u64) -> u64 {
    let irreds = get_irreducibles(10);
    let n_deg = 64 - n.leading_zeros() as i32;
    let mu = m as usize;
    let spf = build_spf(m, &irreds);

    let mut keys: Vec<Poly> = Vec::with_capacity(irreds.len() + 65536);
    keys.extend_from_slice(&irreds);
    for k in 2..=m {
        if spf[k as usize] == 0 {
            keys.push(k);
        }
    }

    let computed: Vec<(Poly, Option<SElement>)> = keys
        .into_par_iter()
        .map(|p| (p, find_prime_generator(p)))
        .collect();

    let mut flag = vec![0u8; mu + 1];
    let mut val = vec![(0u64, 0u64); mu + 1];
    for (p, g) in computed {
        let i = p as usize;
        match g {
            None => flag[i] = 1,
            Some(s) => {
                flag[i] = 2;
                val[i] = s;
            }
        }
    }
    let gens = GenTable { flag, val };

    let total: u64 = (1..mu + 1)
        .into_par_iter()
        .with_min_len(64)
        .map_init(
            || (Vec::with_capacity(16), Vec::with_capacity(16)),
            |(current, next), k| count_for_k(k as Poly, n, n_deg, &spf, &gens, current, next),
        )
        .sum();
    total + 1
}

fn main() {
    println!("{}", solve(100_000_000_000_000_000, 1_000_000));
}
