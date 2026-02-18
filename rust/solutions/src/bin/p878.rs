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

use std::collections::HashMap;

// ---- GF(2) polynomial arithmetic ----
// Polynomials over GF(2) represented as u64 bitmasks.

type Poly = u64;

const X: Poly = 2;

fn poly_deg(a: Poly) -> i32 {
    if a == 0 { -1 } else { 63 - a.leading_zeros() as i32 }
}

// Poly mul that stays in u64 (for cases where we know result fits)
fn poly_mul64(a: Poly, b: Poly) -> Poly {
    let mut res: Poly = 0;
    let mut aa = a;
    let mut bb = b;
    while bb > 0 {
        if bb & 1 != 0 {
            res ^= aa;
        }
        aa <<= 1;
        bb >>= 1;
    }
    res
}

fn poly_mod(a: Poly, b: Poly) -> Poly {
    if b == 0 { panic!("division by zero"); }
    let db = poly_deg(b);
    let mut aa = a;
    let mut da = poly_deg(aa);
    while da >= db {
        aa ^= b << (da - db);
        da = poly_deg(aa);
    }
    aa
}

fn poly_divmod(a: Poly, b: Poly) -> (Poly, Poly) {
    if b == 0 { panic!("division by zero"); }
    let db = poly_deg(b);
    let mut aa = a;
    let mut da = poly_deg(aa);
    if da < db { return (0, a); }
    let mut q: Poly = 0;
    while da >= db {
        let shift = da - db;
        q ^= 1u64 << shift;
        aa ^= b << shift;
        da = poly_deg(aa);
    }
    (q, aa)
}

// poly_sq that stays in u64
fn poly_sq64(a: Poly) -> Poly {
    let mut res: Poly = 0;
    let mut shift = 0;
    let mut aa = a;
    while aa > 0 {
        if aa & 1 != 0 {
            res |= 1u64 << shift;
        }
        aa >>= 1;
        shift += 2;
    }
    res
}

// ---- S-element: elements of Z_2[x][omega] / (omega^2 + x*omega + 1) ----
// Represented as (A, B) meaning A + B*omega, where A, B are GF(2) polynomials.

type SElement = (Poly, Poly);

const OMEGA: SElement = (0, 1);
const OMEGA_INV: SElement = (X, 1);

fn s_mul(alpha: SElement, beta: SElement) -> SElement {
    let (a1, b1) = alpha;
    let (a2, b2) = beta;
    // (A1 + B1*w)(A2 + B2*w) = A1*A2 + (A1*B2 + B1*A2)*w + B1*B2*w^2
    // w^2 = x*w + 1, so B1*B2*w^2 = B1*B2 + x*B1*B2*w
    // Result: (A1*A2 + B1*B2, A1*B2 + B1*A2 + x*B1*B2)
    let a1a2 = poly_mul64(a1, a2);
    let b1b2 = poly_mul64(b1, b2);
    let outer = poly_mul64(a1, b2) ^ poly_mul64(b1, a2);
    (a1a2 ^ b1b2, outer ^ poly_mul64(X, b1b2))
}

fn s_norm(alpha: SElement) -> Poly {
    let (a, b) = alpha;
    // norm = A^2 + x*A*B + B^2 (all in GF(2) polynomial arithmetic)
    // Since we're working with small polynomials in the iteration, use u64 versions
    poly_sq64(a) ^ poly_mul64(poly_mul64(X, a), b) ^ poly_sq64(b)
}

fn s_conjugate(alpha: SElement) -> SElement {
    let (a, b) = alpha;
    (a ^ poly_mul64(X, b), b)
}

fn s_divmod(alpha: SElement, beta: SElement) -> (SElement, SElement) {
    let beta_conj = s_conjugate(beta);
    let num = s_mul(alpha, beta_conj);
    let den = s_norm(beta);
    if den == 0 { panic!("division by zero s-element"); }
    let (u, v) = num;
    let (qu, _) = poly_divmod(u, den);
    let (qv, _) = poly_divmod(v, den);
    let q = (qu, qv);
    let prod = s_mul(q, beta);
    (q, (alpha.0 ^ prod.0, alpha.1 ^ prod.1))
}

fn s_gcd(mut alpha: SElement, mut beta: SElement) -> SElement {
    if alpha == (0, 0) { return beta; }
    if beta == (0, 0) { return alpha; }
    while beta != (0, 0) {
        let (_, r) = s_divmod(alpha, beta);
        alpha = beta;
        beta = r;
    }
    alpha
}

// ---- Irreducible polynomial generation over GF(2) ----

fn get_irreducibles(max_deg: i32) -> Vec<Poly> {
    let mut irreds: Vec<Poly> = Vec::new();
    let upper = 1u64 << (max_deg + 1);
    for i in 2..upper {
        if i > 2 && (i & 1) == 0 { continue; }
        let deg = poly_deg(i);
        let mut is_irred = true;
        for &p in &irreds {
            if poly_deg(p) * 2 > deg { break; }
            if poly_mod(i, p) == 0 { is_irred = false; break; }
        }
        if is_irred { irreds.push(i); }
    }
    irreds
}

// ---- Polynomial factorization over GF(2) ----

fn factor_poly(k: Poly, irreds: &[Poly]) -> Vec<(Poly, u32)> {
    let mut factors: Vec<(Poly, u32)> = Vec::new();
    if k <= 1 { return factors; }
    let mut rem = k;
    for &p in irreds {
        if poly_deg(p) * 2 > poly_deg(rem) { break; }
        let mut count = 0u32;
        loop {
            let (q, r) = poly_divmod(rem, p);
            if r == 0 {
                count += 1;
                rem = q;
            } else {
                break;
            }
            if rem == 1 { break; }
        }
        if count > 0 { factors.push((p, count)); }
        if rem == 1 { break; }
    }
    if rem > 1 { factors.push((rem, 1)); }
    factors
}

// ---- Modular inverse of polynomial mod another polynomial ----

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
    if poly_deg(r) > 0 { panic!("not invertible"); }
    t
}

// ---- Solving x^2 + x = c (mod p) over GF(2)[x]/(p) ----

fn solve_root_quadratic(c_val: Poly, p: Poly) -> Option<Poly> {
    let d = poly_deg(p);
    if d % 2 == 1 {
        // Odd degree: use the trace-based formula
        let mut z: Poly = 0;
        let mut term = c_val;
        for _ in 0..((d + 1) / 2) {
            z ^= term;
            term = poly_sq64(term);
            term = poly_mod(term, p);
            term = poly_sq64(term);
            term = poly_mod(term, p);
        }
        return Some(z);
    }
    // Even degree: brute force
    let limit = 1u64 << d;
    for z in 0..limit {
        let val = poly_sq64(z) ^ z;
        if poly_mod(val, p) == c_val {
            return Some(z);
        }
    }
    None
}

// ---- Find the prime generator for an irreducible polynomial p ----

fn find_prime_generator(p: Poly) -> Option<SElement> {
    if p == X { return Some((1, 1)); }
    let x_sq = poly_sq64(X); // = 4 in bit representation (x^2)
    let inv_x2 = inverse_poly_mod(x_sq, p);
    let z = solve_root_quadratic(inv_x2, p)?;
    let val = poly_sq64(z) ^ z;
    if poly_mod(val, p) != inv_x2 { return None; }
    let t0 = poly_mod(poly_mul64(z, X), p);
    let pg = s_gcd((p, 0), (t0, 1));
    Some(pg)
}

// ---- Main counting logic ----

fn iterate_orbit(start: SElement, step: SElement, n_limit: Poly, n_deg: i32) -> u64 {
    let mut count = 0u64;
    let mut curr = start;
    loop {
        let da = poly_deg(curr.0);
        let db = poly_deg(curr.1);
        if da > n_deg + 2 && db > n_deg + 2 { break; }
        let (val_a, val_b) = curr;
        if val_a <= n_limit && val_b <= n_limit && val_a <= val_b {
            count += 1;
        } else if val_a > n_limit && val_b > n_limit && da > n_deg && db > n_deg {
            break;
        }
        curr = s_mul(curr, step);
    }
    count
}

fn count_for_k(
    k: Poly,
    n_limit: Poly,
    n_deg: i32,
    irreds: &[Poly],
    prime_gen_cache: &mut HashMap<Poly, Option<SElement>>,
) -> u64 {
    if k == 0 { return 1; }

    let factors = factor_poly(k, irreds);

    let mut current_gens: Vec<SElement> = vec![(1, 0)];

    for &(p, e) in &factors {
        if p == X {
            let pg = get_prime_gen(p, prime_gen_cache);
            if let Some(g) = pg {
                // Compute g^e
                let mut term: SElement = (1, 0);
                let mut base = g;
                let mut exp = e;
                while exp > 0 {
                    if exp & 1 != 0 { term = s_mul(term, base); }
                    base = s_mul(base, base);
                    exp >>= 1;
                }
                current_gens = current_gens.iter().map(|&cg| s_mul(cg, term)).collect();
            } else {
                current_gens = current_gens.iter().map(|&cg| s_mul(cg, (0, 0))).collect();
            }
            continue;
        }

        let pg = get_prime_gen(p, prime_gen_cache);
        if pg.is_none() {
            if e % 2 != 0 { return 0; }
            let half = e / 2;
            let mut scale: Poly = 1;
            for _ in 0..half { scale = poly_mul64(scale, p); }
            current_gens = current_gens.iter().map(|&(ga, gb)| {
                (poly_mul64(ga, scale), poly_mul64(gb, scale))
            }).collect();
            continue;
        }

        let pg = pg.unwrap();
        let pg_conj = s_conjugate(pg);

        // Precompute powers of pg and pg_conj up to e
        let eu = e as usize;
        let mut pow_pi = vec![(1u64, 0u64); eu + 1];
        let mut pow_bar = vec![(1u64, 0u64); eu + 1];
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

        let mut next_gens: Vec<SElement> = Vec::new();
        for &g in &current_gens {
            for a in 0..=eu {
                let b = eu - a;
                let factor = s_mul(pow_pi[a], pow_bar[b]);
                let combined = s_mul(g, factor);
                next_gens.push(combined);
            }
        }
        current_gens = next_gens;
    }

    let mut total_count = 0u64;
    for &g in &current_gens {
        total_count += iterate_orbit(g, OMEGA, n_limit, n_deg);
        total_count += iterate_orbit(s_mul(g, OMEGA_INV), OMEGA_INV, n_limit, n_deg);
    }
    total_count
}

fn get_prime_gen(p: Poly, cache: &mut HashMap<Poly, Option<SElement>>) -> Option<SElement> {
    if let Some(&cached) = cache.get(&p) {
        return cached;
    }
    let res = if p == X {
        Some((1, 1))
    } else {
        find_prime_generator(p)
    };
    cache.insert(p, res);
    res
}

fn solve(n: u64, m: u64) -> u64 {
    let irreds = get_irreducibles(10);
    let n_deg = 64 - n.leading_zeros() as i32; // bit_length
    let mut prime_gen_cache: HashMap<Poly, Option<SElement>> = HashMap::new();

    let mut total: u64 = 1; // k=0 contributes 1
    for k in 1..=m {
        total += count_for_k(k, n, n_deg, &irreds, &mut prime_gen_cache);
    }
    total
}

fn main() {
    let answer = solve(100_000_000_000_000_000, 1_000_000);
    println!("{}", answer);
}
