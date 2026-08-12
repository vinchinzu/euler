// Problem 989: Fibonacci Sum
// Ported from the Python reference: phi/psi modular sums with Mobius + nonprimitive pairs.
// S = sum_{n=1}^{10^14} F_n * g(n)  mod 10^9+9.
//
// Hybrid parallelization: expensive small-g terms via rayon; large-g uses sequential
// incremental powers (mod_pow-per-g is too costly when nonprimitive work is tiny).

use euler_utils::mod_pow;
use rayon::prelude::*;

const MOD: i64 = 1_000_000_009;
const TARGET_LIMIT: i64 = 100_000_000_000_000; // 10^14
const SMALL_NONPRIMITIVE_LIMIT: usize = 8;
/// g below this are parallelized (scaled_limit is large). Above: sequential powers.
const PARALLEL_G_MAX: usize = 5_000;

fn tonelli_shanks(n: i64, p: i64) -> i64 {
    if n == 0 {
        return 0;
    }
    if mod_pow(n as u64, ((p - 1) / 2) as u64, p as u64) as i64 != 1 {
        panic!("not a quadratic residue");
    }
    if p % 4 == 3 {
        return mod_pow(n as u64, ((p + 1) / 4) as u64, p as u64) as i64;
    }

    let mut q = p - 1;
    let mut s = 0i32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    let mut z = 2i64;
    while mod_pow(z as u64, ((p - 1) / 2) as u64, p as u64) as i64 != p - 1 {
        z += 1;
    }

    let mut m = s;
    let mut c = mod_pow(z as u64, q as u64, p as u64) as i64;
    let mut t = mod_pow(n as u64, q as u64, p as u64) as i64;
    let mut r = mod_pow(n as u64, ((q + 1) / 2) as u64, p as u64) as i64;

    while t != 1 {
        let mut i = 1i32;
        let mut t2i = t * t % p;
        while t2i != 1 {
            t2i = t2i * t2i % p;
            i += 1;
        }
        let b = mod_pow(c as u64, 1u64 << (m - i - 1) as u32, p as u64) as i64;
        r = r * b % p;
        c = b * b % p;
        t = t * c % p;
        m = i;
    }

    r
}

fn isqrt(n: i64) -> i64 {
    if n < 0 {
        panic!("isqrt of negative");
    }
    let mut x = (n as f64).sqrt() as i64;
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    while x * x > n {
        x -= 1;
    }
    x
}

struct Constants {
    inv_sqrt5: i64,
    phi: i64,
    phi_inv: i64,
    psi: i64,
    phi_sq: i64,
    phi_inv_sq: i64,
    small_terms: Vec<Vec<i64>>,
}

impl Constants {
    fn new() -> Self {
        let sqrt5 = tonelli_shanks(5, MOD);
        let inv_sqrt5 = mod_pow(sqrt5 as u64, (MOD - 2) as u64, MOD as u64) as i64;
        let inv2 = (MOD + 1) / 2;
        let phi = (1 + sqrt5) * inv2 % MOD;
        let phi_inv = mod_pow(phi as u64, (MOD - 2) as u64, MOD as u64) as i64;
        let psi = (1 - sqrt5).rem_euclid(MOD) * inv2 % MOD;
        let phi_sq = phi * phi % MOD;
        let phi_inv_sq = phi_inv * phi_inv % MOD;
        let small_terms = build_small_nonprimitive_terms(SMALL_NONPRIMITIVE_LIMIT);
        Self {
            inv_sqrt5,
            phi,
            phi_inv,
            psi,
            phi_sq,
            phi_inv_sq,
            small_terms,
        }
    }
}

fn build_small_nonprimitive_terms(max_limit: usize) -> Vec<Vec<i64>> {
    let mut values = Vec::new();
    let max_a = 2 * isqrt(max_limit as i64) + 2;
    for a in 2..=max_a {
        for b in 1..=(a / 2) {
            let q = a * a - a * b - b * b;
            if q > 0 && q <= max_limit as i64 {
                values.push(q);
            }
        }
    }
    values.sort_unstable();

    let mut terms = vec![Vec::new(); max_limit + 1];
    let mut prefix = Vec::new();
    let mut index = 0usize;
    let total = values.len();
    for limit in 0..=max_limit {
        while index < total && values[index] <= limit as i64 {
            prefix.push(values[index]);
            index += 1;
        }
        terms[limit] = prefix.clone();
    }
    terms
}

fn eval_small_nonprimitive_pair(
    limit: usize,
    z1: i64,
    z2: i64,
    small_terms: &[Vec<i64>],
) -> (i64, i64) {
    let terms = &small_terms[limit];
    let mut total1 = 0i64;
    let mut total2 = 0i64;
    let mut power1 = 1i64;
    let mut power2 = 1i64;
    let mut exponent = 0i64;

    for &target in terms {
        while exponent < target {
            power1 = power1 * z1 % MOD;
            power2 = power2 * z2 % MOD;
            exponent += 1;
        }
        total1 += power1;
        if total1 >= MOD {
            total1 -= MOD;
        }
        total2 += power2;
        if total2 >= MOD {
            total2 -= MOD;
        }
    }
    (total1, total2)
}

fn mobius_sieve(limit: usize) -> Vec<i8> {
    let mut mu = vec![1i8; limit + 1];
    let mut is_prime = vec![true; limit + 1];
    if !is_prime.is_empty() {
        is_prime[0] = false;
    }
    if limit >= 1 {
        is_prime[1] = false;
    }

    for p in 2..=limit {
        if !is_prime[p] {
            continue;
        }
        for multiple in (p..=limit).step_by(p) {
            mu[multiple] = -mu[multiple];
        }
        let square = p * p;
        if square <= limit {
            for multiple in (square..=limit).step_by(square) {
                mu[multiple] = 0;
            }
            for multiple in (square..=limit).step_by(p) {
                is_prime[multiple] = false;
            }
        }
        for multiple in ((p + p)..=limit).step_by(p) {
            is_prime[multiple] = false;
        }
    }
    mu
}

fn nonprimitive_pair(
    limit: i64,
    z1: i64,
    z1_inv: i64,
    z2: i64,
    z2_inv: i64,
    small_terms: &[Vec<i64>],
) -> (i64, i64) {
    if limit <= SMALL_NONPRIMITIVE_LIMIT as i64 {
        return eval_small_nonprimitive_pair(limit as usize, z1, z2, small_terms);
    }

    let mod_ = MOD;
    let mut total1 = 0i64;
    let mut total2 = 0i64;

    let z1_sq = z1 * z1 % mod_;
    let z2_sq = z2 * z2 % mod_;

    let z1_inv_sq = z1_inv * z1_inv % mod_;
    let z2_inv_sq = z2_inv * z2_inv % mod_;
    let z1_inv_4 = z1_inv_sq * z1_inv_sq % mod_;
    let z2_inv_4 = z2_inv_sq * z2_inv_sq % mod_;
    let z1_inv_5 = z1_inv_4 * z1_inv % mod_;
    let z2_inv_5 = z2_inv_4 * z2_inv % mod_;
    let z1_inv_10 = z1_inv_5 * z1_inv_5 % mod_;
    let z2_inv_10 = z2_inv_5 * z2_inv_5 % mod_;
    let z1_inv_15 = z1_inv_10 * z1_inv_5 % mod_;
    let z2_inv_15 = z2_inv_10 * z2_inv_5 % mod_;

    let mut even_weight1 = z1_inv_5;
    let mut even_weight2 = z2_inv_5;
    let mut even_delta1 = z1_inv_15;
    let mut even_delta2 = z2_inv_15;

    let mut add_index = 0i64;
    let mut add_term1 = 1i64;
    let mut add_term2 = 1i64;
    let mut add_step1 = z1;
    let mut add_step2 = z2;

    let mut drop_index = 0i64;
    let mut drop_term1 = 1i64;
    let mut drop_term2 = 1i64;
    let mut drop_step1 = z1;
    let mut drop_step2 = z2;

    let mut window1 = 0i64;
    let mut window2 = 0i64;
    let mut t = 1i64;
    let mut lower = 3i64;
    let mut upper = 0i64;
    let mut rhs = limit + 5;

    while (upper + 1) * (upper + 1) <= rhs {
        upper += 1;
    }

    while lower <= upper {
        while add_index <= upper {
            window1 += add_term1;
            if window1 >= mod_ {
                window1 -= mod_;
            }
            window2 += add_term2;
            if window2 >= mod_ {
                window2 -= mod_;
            }

            add_term1 = add_term1 * add_step1 % mod_;
            add_step1 = add_step1 * z1_sq % mod_;
            add_term2 = add_term2 * add_step2 % mod_;
            add_step2 = add_step2 * z2_sq % mod_;
            add_index += 1;
        }

        while drop_index < lower {
            window1 -= drop_term1;
            if window1 < 0 {
                window1 += mod_;
            }
            window2 -= drop_term2;
            if window2 < 0 {
                window2 += mod_;
            }

            drop_term1 = drop_term1 * drop_step1 % mod_;
            drop_step1 = drop_step1 * z1_sq % mod_;
            drop_term2 = drop_term2 * drop_step2 % mod_;
            drop_step2 = drop_step2 * z2_sq % mod_;
            drop_index += 1;
        }

        total1 = (total1 + window1 * even_weight1) % mod_;
        total2 = (total2 + window2 * even_weight2) % mod_;

        even_weight1 = even_weight1 * even_delta1 % mod_;
        even_delta1 = even_delta1 * z1_inv_10 % mod_;
        even_weight2 = even_weight2 * even_delta2 % mod_;
        even_delta2 = even_delta2 * z2_inv_10 % mod_;

        rhs += 10 * t + 5;
        t += 1;
        lower += 3;
        while (upper + 1) * (upper + 1) <= rhs {
            upper += 1;
        }
    }

    let mut odd_weight1 = z1_inv;
    let mut odd_weight2 = z2_inv;
    let mut odd_delta1 = z1_inv_10;
    let mut odd_delta2 = z2_inv_10;

    add_index = 0;
    add_term1 = 1;
    add_term2 = 1;
    add_step1 = z1_sq;
    add_step2 = z2_sq;

    drop_index = 0;
    drop_term1 = 1;
    drop_term2 = 1;
    drop_step1 = z1_sq;
    drop_step2 = z2_sq;

    window1 = 0;
    window2 = 0;
    t = 0;
    lower = 1;
    upper = 0;
    rhs = limit + 1;

    while (upper + 1) * (upper + 2) <= rhs {
        upper += 1;
    }

    while lower <= upper {
        while add_index <= upper {
            window1 += add_term1;
            if window1 >= mod_ {
                window1 -= mod_;
            }
            window2 += add_term2;
            if window2 >= mod_ {
                window2 -= mod_;
            }

            add_term1 = add_term1 * add_step1 % mod_;
            add_step1 = add_step1 * z1_sq % mod_;
            add_term2 = add_term2 * add_step2 % mod_;
            add_step2 = add_step2 * z2_sq % mod_;
            add_index += 1;
        }

        while drop_index < lower {
            window1 -= drop_term1;
            if window1 < 0 {
                window1 += mod_;
            }
            window2 -= drop_term2;
            if window2 < 0 {
                window2 += mod_;
            }

            drop_term1 = drop_term1 * drop_step1 % mod_;
            drop_step1 = drop_step1 * z1_sq % mod_;
            drop_term2 = drop_term2 * drop_step2 % mod_;
            drop_step2 = drop_step2 * z2_sq % mod_;
            drop_index += 1;
        }

        total1 = (total1 + window1 * odd_weight1) % mod_;
        total2 = (total2 + window2 * odd_weight2) % mod_;

        odd_weight1 = odd_weight1 * odd_delta1 % mod_;
        odd_delta1 = odd_delta1 * z1_inv_10 % mod_;
        odd_weight2 = odd_weight2 * odd_delta2 % mod_;
        odd_delta2 = odd_delta2 * z2_inv_10 % mod_;

        rhs += 10 * t + 10;
        t += 1;
        lower += 3;
        while (upper + 1) * (upper + 2) <= rhs {
            upper += 1;
        }
    }

    (total1, total2)
}

#[inline]
fn contrib_for_g(
    g: usize,
    limit: i64,
    phi_pow_g2: i64,
    phi_inv_pow_g2: i64,
    mu_g: i8,
    small_terms: &[Vec<i64>],
) -> (i64, i64) {
    let g_square = (g as i64) * (g as i64);
    let scaled_limit = limit / g_square;

    let (psi_pow_g2, psi_inv_pow_g2) = if g & 1 == 1 {
        ((MOD - phi_inv_pow_g2) % MOD, (MOD - phi_pow_g2) % MOD)
    } else {
        (phi_inv_pow_g2, phi_pow_g2)
    };

    let (np_phi, np_psi) = nonprimitive_pair(
        scaled_limit,
        phi_pow_g2,
        phi_inv_pow_g2,
        psi_pow_g2,
        psi_inv_pow_g2,
        small_terms,
    );

    let sign = mu_g as i64;
    (np_phi * sign, np_psi * sign)
}

fn solve(limit: i64, c: &Constants) -> i64 {
    let root = isqrt(limit) as usize;
    let mu = mobius_sieve(root);

    let par_max = PARALLEL_G_MAX.min(root);

    // --- Parallel region: small g (expensive nonprimitive_pair) ---
    let small_gs: Vec<usize> = (1..=par_max).filter(|&g| mu[g] != 0).collect();
    let (mut p_phi, mut p_psi) = small_gs
        .par_iter()
        .map(|&g| {
            let g_square = (g as i64) * (g as i64);
            let phi_pow_g2 = mod_pow(c.phi as u64, g_square as u64, MOD as u64) as i64;
            let phi_inv_pow_g2 =
                mod_pow(c.phi_inv as u64, g_square as u64, MOD as u64) as i64;
            contrib_for_g(g, limit, phi_pow_g2, phi_inv_pow_g2, mu[g], &c.small_terms)
        })
        .reduce(|| (0i64, 0i64), |a, b| (a.0 + b.0, a.1 + b.1));

    // --- Sequential region: large g with incremental powers ---
    // Start powers at g = par_max: need phi^(par_max^2) then step.
    // Incremental recurrence used in the original:
    //   after processing g, advance to g+1:
    //   pow *= forward_step; forward_step *= phi_sq
    // where before loop pow=1, forward=phi, and at start of g-body we first update.
    // So after full sequential 1..=G, phi_pow = phi^(G^2).
    //
    // We seed at g = par_max by computing phi^(par_max^2), then run the same
    // step as the original loop from g = par_max+1 ..= root.

    if par_max < root {
        let g0 = par_max as i64;
        let mut phi_pow_g2 = if par_max == 0 {
            1
        } else {
            mod_pow(c.phi as u64, (g0 * g0) as u64, MOD as u64) as i64
        };
        let mut phi_inv_pow_g2 = if par_max == 0 {
            1
        } else {
            mod_pow(c.phi_inv as u64, (g0 * g0) as u64, MOD as u64) as i64
        };
        // forward_step after processing g is phi^(2g+1), because:
        // start: pow=1, step=phi
        // for g=1: pow*=step => phi, step*=phi_sq => phi^3
        // for g=2: pow*=step => phi^4 = phi^(2^2), step => phi^5
        // for g: after update, pow = phi^(g^2), step = phi^(2g+1)
        let mut forward_step =
            mod_pow(c.phi as u64, (2 * g0 + 1) as u64, MOD as u64) as i64;
        let mut backward_step =
            mod_pow(c.phi_inv as u64, (2 * g0 + 1) as u64, MOD as u64) as i64;

        for g in (par_max + 1)..=root {
            phi_pow_g2 = phi_pow_g2 * forward_step % MOD;
            forward_step = forward_step * c.phi_sq % MOD;
            phi_inv_pow_g2 = phi_inv_pow_g2 * backward_step % MOD;
            backward_step = backward_step * c.phi_inv_sq % MOD;

            let mu_g = mu[g];
            if mu_g != 0 {
                let (dphi, dpsi) = contrib_for_g(
                    g,
                    limit,
                    phi_pow_g2,
                    phi_inv_pow_g2,
                    mu_g,
                    &c.small_terms,
                );
                p_phi += dphi;
                p_psi += dpsi;
            }
        }
    }

    let p_phi = p_phi.rem_euclid(MOD);
    let p_psi = p_psi.rem_euclid(MOD);

    ((p_phi - p_psi).rem_euclid(MOD) * c.inv_sqrt5) % MOD
}

fn main() {
    let c = Constants::new();
    debug_assert_eq!(c.psi, (MOD - c.phi_inv) % MOD);
    debug_assert_eq!(solve(1, &c), 1);
    debug_assert_eq!(solve(1000, &c), 190_950_976);
    println!("{}", solve(TARGET_LIMIT, &c));
}
