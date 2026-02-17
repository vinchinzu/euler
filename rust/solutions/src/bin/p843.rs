// Project Euler 843 - Periodic Circles
//
// The iteration on a circle of n integers (replace each by |left - right|)
// corresponds to multiplication by (1+x) in GF(2)[x]/(x^n - 1).
// Via CRT: GF(2)[x]/(x^n - 1) = prod_{d|n_odd} GF(2)[x]/(Phi_d(x)^{2^s})
// where n = 2^s * n_odd.
//
// For each divisor d of n_odd, Phi_d factors over GF(2) into phi(d)/ord_2(d)
// irreducible polynomials, each of degree ord_2(d).
// The element (x + x^{n-1}) has potentially different periods in each factor.
// We compute periods from ALL factors, take LCMs across factors of same d
// (Cartesian product), then across all d values.

use std::collections::HashSet;

// GF(2) polynomial represented as u128 bitmask: bit i = coefficient of x^i
type Poly = u128;

fn poly_deg(p: Poly) -> i32 {
    if p == 0 { return -1; }
    127 - p.leading_zeros() as i32
}

fn poly_mod(a: Poly, b: Poly) -> Poly {
    if b == 0 { panic!("division by zero"); }
    let db = poly_deg(b);
    let mut r = a;
    loop {
        let dr = poly_deg(r);
        if dr < db { return r; }
        r ^= b << (dr - db);
    }
}

fn poly_mul_mod(a: Poly, b: Poly, m: Poly) -> Poly {
    // Multiply a*b mod m in GF(2)
    // Since degrees can be up to ~96, we need to be careful about overflow in u128.
    // We'll do shift-and-add with reduction.
    let mut result: Poly = 0;
    let mut aa = poly_mod(a, m);
    let mut bb = b;
    while bb != 0 {
        if bb & 1 != 0 {
            result ^= aa;
        }
        bb >>= 1;
        aa <<= 1;
        // Reduce aa mod m if degree exceeds
        if poly_deg(aa) >= poly_deg(m) {
            aa ^= m;
        }
    }
    poly_mod(result, m)
}

// Polynomial exponentiation mod m in GF(2), with u128 exponent
fn poly_pow_mod_big(base: Poly, exp: u128, m: Poly) -> Poly {
    let mut result: Poly = 1;
    let mut b = poly_mod(base, m);
    let mut e = exp;
    while e > 0 {
        if e & 1 != 0 {
            result = poly_mul_mod(result, b, m);
        }
        b = poly_mul_mod(b, b, m);
        e >>= 1;
    }
    result
}

fn poly_gcd(mut a: Poly, mut b: Poly) -> Poly {
    while b != 0 {
        let t = poly_mod(a, b);
        a = b;
        b = t;
    }
    a
}

// Make polynomial monic (in GF(2), leading coeff is always 1 if nonzero)
// This is a no-op for GF(2) but included for clarity

// Compute cyclotomic polynomial Phi_d(x) with coefficients reduced mod 2
// We use the relation: x^d - 1 = prod_{k|d} Phi_k(x)
// So Phi_d(x) = (x^d - 1) / prod_{k|d, k<d} Phi_k(x)
// Over integers first, then reduce mod 2.
// Actually, for GF(2), x^d - 1 = x^d + 1 (since -1 = 1 mod 2).
// We compute Phi_d over Z using integer polynomial division, then reduce mod 2.
fn cyclotomic_poly_gf2(d: usize) -> Poly {
    // Compute Phi_d(x) over Z, then reduce mod 2
    // For d up to 100, degrees up to 96, coefficients stay small enough
    // Actually the coefficients of cyclotomic polynomials can be large for big d,
    // but for d <= 100 they're manageable.

    // Use the formula: Phi_d(x) = prod_{k|d} (x^k - 1)^{mu(d/k)}
    // where mu is the Mobius function.
    // Equivalently: x^d - 1 = prod_{k|d} Phi_k(x)
    // So Phi_d = (x^d - 1) / prod_{k|d, k<d} Phi_k(x)

    // We'll compute over Z using Vec<i64> coefficients, then reduce mod 2

    // Get divisors of d
    let divs = get_divisors(d);

    // Start with x^d - 1 = x^d + (-1)
    let mut num = vec![0i64; d + 1];
    num[0] = -1;
    num[d] = 1;

    // Divide by Phi_k for each proper divisor k of d
    for &k in &divs {
        if k == d { continue; }
        let phi_k = cyclotomic_poly_z(k);
        num = poly_div_z(&num, &phi_k);
    }

    // Convert to GF(2) polynomial
    let mut result: Poly = 0;
    for (i, &c) in num.iter().enumerate() {
        if c.rem_euclid(2) == 1 {
            result |= 1u128 << i;
        }
    }
    result
}

fn cyclotomic_poly_z(d: usize) -> Vec<i64> {
    if d == 1 {
        return vec![-1, 1]; // x - 1
    }
    let divs = get_divisors(d);
    let mut num = vec![0i64; d + 1];
    num[0] = -1;
    num[d] = 1;
    for &k in &divs {
        if k == d { continue; }
        let phi_k = cyclotomic_poly_z(k);
        num = poly_div_z(&num, &phi_k);
    }
    // Trim trailing zeros
    while num.len() > 1 && *num.last().unwrap() == 0 {
        num.pop();
    }
    num
}

// Exact polynomial division over Z (assumes divisor divides evenly)
fn poly_div_z(num: &[i64], den: &[i64]) -> Vec<i64> {
    if den.is_empty() || *den.last().unwrap() == 0 {
        panic!("division by zero polynomial");
    }
    let n = num.len();
    let d = den.len();
    if n < d {
        return vec![0];
    }
    let mut rem = num.to_vec();
    let mut quot = vec![0i64; n - d + 1];
    let lead = *den.last().unwrap();
    for i in (0..=(n - d)).rev() {
        let q = rem[i + d - 1] / lead;
        quot[i] = q;
        for j in 0..d {
            rem[i + j] -= q * den[j];
        }
    }
    // Trim trailing zeros
    while quot.len() > 1 && *quot.last().unwrap() == 0 {
        quot.pop();
    }
    quot
}

fn get_divisors(n: usize) -> Vec<usize> {
    let mut divs = Vec::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
        i += 1;
    }
    divs.sort_unstable();
    divs
}

// Factor a square-free polynomial over GF(2) into irreducible factors
// All factors have the same degree d (= ord_2 of the cyclotomic index)
// Uses Berlekamp or Cantor-Zassenhaus for GF(2)
fn factor_gf2(f: Poly, factor_degree: usize) -> Vec<Poly> {
    let deg_f = poly_deg(f);
    if deg_f <= 0 {
        return vec![];
    }
    if deg_f as usize == factor_degree {
        return vec![f];
    }

    // Equal-degree factorization for GF(2)
    // For GF(2^k), we use the trace map: Tr(a) = a + a^2 + a^4 + ... + a^(2^(d-1))
    // gcd(Tr(random), f) gives a non-trivial split with probability ~1/2

    let num_factors = deg_f as usize / factor_degree;
    if num_factors == 1 {
        return vec![f];
    }

    let mut factors = vec![f];

    // Try different random elements (we'll use x, x+1, x^2, x^2+x, etc.)
    // In GF(2)[x]/(f), try elements a = x^i for various i, compute trace
    let mut trial = 0u128;

    while factors.len() < num_factors {
        trial += 1;
        // Use trial as a polynomial
        let a = trial;
        if poly_deg(a) >= deg_f {
            continue;
        }

        // For each not-yet-split factor, try to split it
        let mut new_factors = Vec::new();
        for &g in &factors {
            if poly_deg(g) as usize == factor_degree {
                new_factors.push(g);
                continue;
            }

            // Compute trace of a mod g: Tr(a) = a + a^2 + a^4 + ... + a^(2^(factor_degree-1))
            let a_mod = poly_mod(a, g);
            let mut trace = a_mod;
            let mut pow = a_mod;
            for _ in 1..factor_degree {
                pow = poly_mul_mod(pow, pow, g); // pow = pow^2 mod g
                trace ^= pow; // trace += pow (GF(2) addition = XOR)
            }

            let gcd = poly_gcd(trace, g);
            let gcd_deg = poly_deg(gcd);
            if gcd_deg > 0 && gcd_deg < poly_deg(g) {
                // Non-trivial split!
                new_factors.push(gcd);
                let other = poly_exact_div_gf2(g, gcd);
                new_factors.push(other);
            } else {
                new_factors.push(g);
            }
        }
        factors = new_factors;

        // Recursively split any remaining composites
        if factors.len() < num_factors {
            let mut refined = Vec::new();
            for &g in &factors {
                if poly_deg(g) as usize == factor_degree {
                    refined.push(g);
                } else {
                    // Try to split further recursively
                    refined.push(g);
                }
            }
            factors = refined;
        }
    }

    factors
}

// Exact division in GF(2)[x]
fn poly_exact_div_gf2(a: Poly, b: Poly) -> Poly {
    let da = poly_deg(a);
    let db = poly_deg(b);
    if da < db { return 0; }
    let mut rem = a;
    let mut quot: Poly = 0;
    loop {
        let dr = poly_deg(rem);
        if dr < db { break; }
        let shift = dr - db;
        quot |= 1u128 << shift;
        rem ^= b << shift;
    }
    quot
}

// Compute multiplicative order of element `elem` in GF(2)[x]/(f)
// where f is irreducible of degree d. Order divides 2^d - 1.
fn multiplicative_order_gf2(elem: Poly, f: Poly) -> u64 {
    let d = poly_deg(f) as u32;
    if d == 0 { return 1; }
    let order_bound = (1u128 << d) - 1; // 2^d - 1

    if order_bound == 0 { return 1; }

    // Factor 2^d - 1
    let factors = factorize_u128(order_bound);

    let mut m = order_bound;
    for &(p, e) in &factors {
        for _ in 0..e {
            if poly_pow_mod_big(elem, m / p, f) == 1 {
                m /= p;
            } else {
                break;
            }
        }
    }

    m as u64
}

// Factorize a u128 number (for numbers up to ~2^96)
fn factorize_u128(mut n: u128) -> Vec<(u128, u32)> {
    let mut factors = Vec::new();
    if n <= 1 { return factors; }

    let mut d = 2u128;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0u32;
            while n % d == 0 {
                n /= d;
                e += 1;
            }
            factors.push((d, e));
        }
        d += if d == 2 { 1 } else { 2 };
        // For large n, we might need to be smarter, but 2^96-1 has
        // all prime factors manageable with trial division up to ~2^48
        // Actually, we need to be careful. Let's add a limit.
        if d > 1_000_000 && n > 1 {
            // Try Pollard rho for the remaining factor
            break;
        }
    }
    if n > 1 {
        // n might be a prime or semiprime
        // Try to factor further with Pollard rho
        let remaining_factors = factor_large(n);
        factors.extend(remaining_factors);
    }
    factors
}

fn factor_large(n: u128) -> Vec<(u128, u32)> {
    if n <= 1 { return vec![]; }
    if is_prime_u128(n) {
        return vec![(n, 1)];
    }
    // Pollard rho
    let d = pollard_rho_u128(n);
    let mut result = factor_large(d);
    let mut result2 = factor_large(n / d);
    result.append(&mut result2);
    // Merge same primes
    result.sort();
    let mut merged = Vec::new();
    for (p, e) in result {
        if let Some(last) = merged.last_mut() {
            let (lp, le): &mut (u128, u32) = last;
            if *lp == p {
                *le += e;
                continue;
            }
        }
        merged.push((p, e));
    }
    merged
}

fn is_prime_u128(n: u128) -> bool {
    if n < 2 { return false; }
    if n < 4 { return true; }
    if n % 2 == 0 { return false; }
    // Miller-Rabin with enough witnesses for correctness up to 2^128
    let d_val = n - 1;
    let mut d = d_val;
    let mut r = 0u32;
    while d % 2 == 0 { d /= 2; r += 1; }

    // For n < 3,317,044,064,679,887,385,961,981, these witnesses suffice
    let witnesses: &[u128] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    'outer: for &a in witnesses {
        if a >= n { continue; }
        let mut x = mod_pow_u128(a, d, n);
        if x == 1 || x == n - 1 { continue; }
        for _ in 0..r - 1 {
            x = mod_mul_u128(x, x, n);
            if x == n - 1 { continue 'outer; }
        }
        return false;
    }
    true
}

fn mod_pow_u128(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1u128;
    base %= modulus;
    while exp > 0 {
        if exp & 1 != 0 {
            result = mod_mul_u128(result, base, modulus);
        }
        base = mod_mul_u128(base, base, modulus);
        exp >>= 1;
    }
    result
}

fn mod_mul_u128(a: u128, b: u128, m: u128) -> u128 {
    // For u128 multiplication mod m, we need to handle overflow
    // Use the Russian peasant multiplication approach
    let mut result = 0u128;
    let mut a = a % m;
    let mut b = b % m;
    while b > 0 {
        if b & 1 != 0 {
            result = result.wrapping_add(a);
            if result >= m { result -= m; }
        }
        a = a.wrapping_add(a);
        if a >= m { a -= m; }
        b >>= 1;
    }
    result
}

fn pollard_rho_u128(n: u128) -> u128 {
    if n % 2 == 0 { return 2; }
    for c in 1u128.. {
        let mut x = 2u128;
        let mut y = 2u128;
        let mut d = 1u128;
        while d == 1 {
            x = (mod_mul_u128(x, x, n) + c) % n;
            y = (mod_mul_u128(y, y, n) + c) % n;
            y = (mod_mul_u128(y, y, n) + c) % n;
            d = gcd_u128(if x > y { x - y } else { y - x }, n);
        }
        if d != n {
            return d;
        }
    }
    unreachable!()
}

fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

// Compute periods of element P = x + x^(n-1) in GF(2)[x]/(f^{2^s})
// where f is irreducible of degree d_f.
//
// The period in GF(2)[x]/(f) is the multiplicative order m of P mod f.
// The period in GF(2)[x]/(f^{2^s}) is m * 2^j for some j in 0..=s,
// specifically the smallest j such that P^(m*2^j) = 1 mod f^{2^s}.
//
// Valid periods for this component: {1} union {m * 2^j : j = 0..=j_max}
// where j_max is determined by when we first get 1 mod the full modulus.
fn get_component_periods(elem: Poly, f: Poly, s: u32) -> Vec<u64> {
    // If element is zero mod f^{2^s}, period is 1
    let mod_poly = poly_pow_gf2(f, 1u128 << s);
    let elem_mod = poly_mod(elem, mod_poly);
    if elem_mod == 0 {
        return vec![1];
    }

    // Compute multiplicative order of elem mod f (irreducible)
    let elem_f = poly_mod(elem, f);
    if elem_f == 0 {
        // elem is divisible by f. Period is determined by nilpotent part.
        // The order of (1 + f*g) mod f^{2^s} is a power of 2.
        // We need to find the smallest k such that elem^(2^k) = 1 mod f^{2^s}.
        // Start with exponent 1 (i.e. 2^0) and double.
        let mut periods = vec![1u64];
        let mut p = elem_mod;
        for j in 0..=63 {
            if p == 1 { break; }
            let period = 1u64 << j;
            // Actually this case is tricky. Let's just compute directly.
            // elem^(2^j) mod f^{2^s}
            periods.push(period);
            p = poly_mul_mod(p, p, mod_poly);
        }
        // Actually let me reconsider. If elem_f == 0, then elem mod f = 0,
        // so in the component for f, the element is nilpotent.
        // Its "period" contribution is just 1 (it eventually becomes 0, not 1).
        // Wait, but we're looking at multiplication, not addition.
        // If elem mod f = 0 and we're in GF(2)[x]/(f^{2^s}),
        // then elem is in the maximal ideal. Its powers never reach 1 (they go to 0).
        // So this component contributes period = infinity? No...
        //
        // Actually the problem is about (1+x)^period = 1 mod (x^n - 1).
        // The element P = x + x^{n-1} = x(1 + x^{n-2}).
        // Hmm, but we need P^period = 1 in the quotient ring, not P^period = 0.
        // If P mod f = 0 in the irreducible component, then P^anything mod f = 0 ≠ 1.
        // So this component can never have P^k = 1, meaning no period works.
        // But wait, in the CRT decomposition, we need P^k = 1 in EVERY component.
        // If P mod f = 0, that means P^k mod f = 0 ≠ 1 for all k > 0.
        // So... there's no valid period? That can't be right since every circle eventually becomes periodic.
        //
        // Let me reconsider. The element is (1+x), not (x + x^{n-1}).
        // P = (1+x) in GF(2)[x]/(x^n - 1).
        // Wait, the iteration replaces a_i with |a_{i-1} - a_{i+1}|.
        // Over GF(2), this is a_{i-1} + a_{i+1} = (x^{-1} + x) * a_i.
        // So the operator is multiplication by (x + x^{-1}) = (x^2 + 1)/x.
        // Hmm, but x is invertible mod (x^n - 1) since x^n = 1.
        // Actually x + x^{n-1} = x + x^{-1} = (x^2 + 1)/x.
        //
        // For the period, we need (x + x^{n-1})^k = 1 mod component.
        // If x + x^{n-1} = 0 mod f, then no power gives 1.
        // This means this component imposes no constraint (any period works from other components).
        // Actually no: P^k = 0 mod f, and we need P^k = 1 mod f for the CRT to give 1 overall.
        // Unless P^k is never 1 mod f, which means no global period exists...
        // But wait, p = 0 in this component means x + x^{-1} = 0 mod f, i.e., x^2 + 1 = 0 mod f.
        // This means f | (x^2 + 1) = (x+1)^2. Since f is irreducible and odd degree >1 doesn't
        // divide (x+1)^2, this only works for f = x+1, i.e., d=1.
        // For d=1, Phi_1 = x - 1 = x + 1 mod 2.
        // P = x + x^{n-1} mod (x+1) = 1 + 1 = 0. So P mod (x+1) = 0.
        // P^k mod (x+1) = 0, not 1. But this is the d=1 component which we skip anyway.
        // So in practice, elem_f should never be 0 for d > 1.

        return vec![1]; // shouldn't happen for d > 1
    }

    // Multiplicative order of elem mod f
    let m = multiplicative_order_gf2(elem_f, f);

    let mut periods = vec![1];

    if s == 0 {
        // No nilpotent part, just the base order
        periods.push(m);
        return periods;
    }

    // For the full modulus f^{2^s}:
    // Period is m * 2^j for the smallest j where P^(m*2^j) = 1 mod f^{2^s}
    // Valid periods: {m * 2^j : 0 <= j <= j_max} union {1}

    // Compute P^m mod f^{2^s}
    let mut current = poly_pow_mod_big(elem_mod, m as u128, mod_poly);

    // Find j_max: smallest j where current^{2^j} = 1 mod f^{2^s}
    let mut j_max = 0;
    periods.push(m);

    while current != 1 {
        current = poly_mul_mod(current, current, mod_poly);
        j_max += 1;
        if j_max > 64 { break; } // safety limit
        periods.push(m * (1u64 << j_max));
    }

    periods
}

// Compute f^e in GF(2)[x] using repeated squaring
fn poly_pow_gf2(f: Poly, e: u128) -> Poly {
    if e == 0 { return 1; }
    let mut result: Poly = 1;
    let mut base = f;
    let mut exp = e;
    while exp > 0 {
        if exp & 1 != 0 {
            result = poly_mul_gf2(result, base);
        }
        if exp > 1 {
            base = poly_mul_gf2(base, base);
        }
        exp >>= 1;
    }
    result
}

// Multiply two GF(2) polynomials without modular reduction
fn poly_mul_gf2(a: Poly, b: Poly) -> Poly {
    let mut result: Poly = 0;
    let mut bb = b;
    let mut shift = 0;
    while bb != 0 {
        if bb & 1 != 0 {
            result ^= a << shift;
        }
        bb >>= 1;
        shift += 1;
    }
    result
}

// Multiplicative order of 2 modulo d
fn ord2_mod(d: usize) -> usize {
    if d <= 1 { return 1; }
    let mut x = 2u64 % d as u64;
    for i in 1..=d {
        if x == 1 { return i; }
        x = (x * 2) % d as u64;
    }
    d
}

// Euler's totient function
fn euler_phi(mut n: usize) -> usize {
    let mut result = n;
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 { n /= p; }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

fn main() {
    let mut all_periods: HashSet<u64> = HashSet::new();

    for n in 3..=100usize {
        // Factor n = 2^s * n_odd
        let mut n_odd = n;
        let mut s = 0u32;
        while n_odd % 2 == 0 {
            n_odd /= 2;
            s += 1;
        }

        let divs = get_divisors(n_odd);

        // For each divisor d, get the set of possible periods from that CRT component
        let mut component_sets: Vec<Vec<u64>> = Vec::new();

        for &d in &divs {
            if d == 1 {
                component_sets.push(vec![1]);
                continue;
            }

            let phi_d = euler_phi(d);
            let ord_d = ord2_mod(d);
            let num_factors = phi_d / ord_d;

            // Compute Phi_d(x) mod 2
            let phi_poly = cyclotomic_poly_gf2(d);

            // Element P = x + x^{n-1} in GF(2)[x]
            let p_elem: Poly = (1u128 << 1) | (1u128 << (n - 1));

            if num_factors == 1 {
                // Only one irreducible factor = phi_poly itself
                let periods = get_component_periods(p_elem, phi_poly, s);
                component_sets.push(periods);
            } else {
                // Factor phi_poly into irreducible factors over GF(2)
                let factors = factor_gf2(phi_poly, ord_d);

                // Get periods from each factor
                let mut factor_period_sets: Vec<Vec<u64>> = Vec::new();
                for &f in &factors {
                    let periods = get_component_periods(p_elem, f, s);
                    factor_period_sets.push(periods);
                }

                // Take Cartesian product and LCMs across factors
                let mut d_periods: HashSet<u64> = HashSet::new();
                cartesian_lcm(&factor_period_sets, 0, 1, &mut d_periods);

                component_sets.push(d_periods.into_iter().collect());
            }
        }

        // Take Cartesian product across all divisor components, compute LCMs
        let mut valid_periods: HashSet<u64> = HashSet::new();
        cartesian_lcm(&component_sets, 0, 1, &mut valid_periods);

        all_periods.extend(valid_periods);
    }

    let total: u64 = all_periods.iter().sum();
    println!("{}", total);
}

fn cartesian_lcm(sets: &[Vec<u64>], idx: usize, current_lcm: u64, result: &mut HashSet<u64>) {
    if idx == sets.len() {
        result.insert(current_lcm);
        return;
    }
    for &p in &sets[idx] {
        let new_lcm = lcm(current_lcm, p);
        cartesian_lcm(sets, idx + 1, new_lcm, result);
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { return 0; }
    a / gcd(a, b) * b
}
