const M: u64 = 1_000_000_007;

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
    // M < 2^30, so a,b < M means a*b < 2^60 < 2^63, fits in u64
    // Actually M ~ 10^9, so a*b ~ 10^18, which is < 2^63 ~ 9.2*10^18. Safe.
    (a as u128 * b as u128 % M as u128) as u64
}

#[inline(always)]
fn add(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= M { s - M } else { s }
}

#[inline(always)]
fn sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { M - b + a }
}

fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    base %= M;
    let mut result = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul(result, base);
        }
        base = mul(base, base);
        exp >>= 1;
    }
    result
}

fn inv(a: u64) -> u64 {
    pow_mod(a, M - 2)
}

/// Geometric series: sum_{i=0}^{n} r^i mod M
fn geo(r: u64, n: i64) -> u64 {
    if n < 0 {
        return 0;
    }
    let n = n as u64;
    let r_mod = r % M;
    if r_mod == 1 {
        return (n + 1) % M;
    }
    let p = pow_mod(r_mod, n + 1);
    let one_minus_r = sub(1, r_mod); // (1 - r) mod M
    let inv_val = inv(one_minus_r);
    mul(sub(1, p), inv_val)
}

/// sum_{i=0}^{n} base^(offset + k*i) = base^offset * sum_{i=0}^{n} (base^k)^i
fn geo2(base: u64, n: i64, k: u64, offset: u64) -> u64 {
    if n < 0 {
        return 0;
    }
    let base_offset = pow_mod(base, offset);
    let base_k = pow_mod(base, k);
    mul(base_offset, geo(base_k, n))
}

/// sum_{i=L}^{R} base^(offset + k*i) = geo2(R) - geo2(L-1)
fn geo3(base: u64, l: i64, r: i64, k: u64, offset: u64) -> u64 {
    if l > r || r < 0 {
        return 0;
    }
    let l = if l < 0 { 0 } else { l };
    if l == 0 {
        return geo2(base, r, k, offset);
    }
    let total = geo2(base, r, k, offset);
    let subtract = geo2(base, l - 1, k, offset);
    sub(total, subtract)
}

/// For a segment [d_lo, d_hi], sum over d of 7^d * geo(11, A - B*d)
fn sum_with_linear_max(base_outer: u64, base_inner: u64, l: i64, r: i64, a: i64, b: i64) -> u64 {
    if l > r {
        return 0;
    }
    if b == 0 {
        let geo_inner = geo(base_inner, a);
        return mul(geo_inner, geo3(base_outer, l, r, 1, 0));
    }
    let mut result = 0u64;
    for i in l..=r {
        let max_val = a - b * i;
        if max_val < 0 {
            break;
        }
        let contrib = mul(pow_mod(base_outer, i as u64), geo(base_inner, max_val));
        result = add(result, contrib);
    }
    result
}

fn compute_p_efficient(x: &[u64; 10]) -> u64 {
    let mut result = 0u64;
    let max_a = x[0].min(x[1]).min(x[2]).min(x[3]);

    for a in 0..=max_a {
        let pow2a = pow_mod(2, a);
        let max_b = (x[0] - a).min(x[4]).min(x[5]).min(x[6]);

        for b in 0..=max_b {
            let pow2a3b = mul(pow2a, pow_mod(3, b));
            let max_c_val = (x[1] - a).min(x[4] - b).min(x[7]).min(x[8]);

            for c in 0..=max_c_val {
                let pow2a3b5c = mul(pow2a3b, pow_mod(5, c));
                let max_d = (x[2] - a).min(x[5] - b).min(x[7] - c).min(x[9]);

                // e_limits
                let e_lim0 = x[3] - a; // X[3] - a
                let e_lim1 = x[6] - b; // X[6] - b
                let e_lim2 = x[8] - c; // X[8] - c

                // Critical points for d segments
                let mut critical_points = Vec::with_capacity(6);
                critical_points.push(0i64);
                critical_points.push(max_d as i64 + 1);

                for &limit in &[e_lim0, e_lim1, e_lim2] {
                    // cp = X[9] - limit, if X[9] >= limit
                    if x[9] >= limit {
                        let cp = (x[9] - limit) as i64;
                        if cp >= 0 && cp <= max_d as i64 + 1 {
                            critical_points.push(cp);
                        }
                    } else {
                        // x[9] < limit means cp is negative, but we need to handle:
                        // The Python code computes cp = X[9] - limit (which could be negative in Python
                        // but X values are mod M so always positive). However logically in the problem
                        // context, X[9] and limit are both < M, so the subtraction can underflow.
                        // In the Python code, X values are < M (around 10^9), so X[9] - limit can be negative.
                        // A negative cp won't satisfy 0 <= cp <= max_d + 1, so skip.
                    }
                }

                critical_points.sort_unstable();
                critical_points.dedup();

                for i in 0..critical_points.len() - 1 {
                    let d_lo = critical_points[i];
                    let d_hi = critical_points[i + 1] - 1;
                    if d_lo > d_hi || d_lo > max_d as i64 {
                        continue;
                    }
                    let d_hi = d_hi.min(max_d as i64);

                    let d_test = d_lo as u64;
                    let max_e_test = (x[3] - a).min(x[6] - b).min(x[8] - c).min(x[9] - d_test);

                    // Check which constraint is binding
                    if max_e_test == x[9] - d_test {
                        // max_e depends on d linearly: max_e = X[9] - d
                        let contrib_de = sum_with_linear_max(7, 11, d_lo, d_hi, x[9] as i64, 1);
                        let contrib = mul(pow2a3b5c, contrib_de);
                        result = add(result, contrib);
                    } else {
                        // max_e is constant over this segment
                        let geo_e = geo(11, max_e_test as i64);
                        let geo_d = geo3(7, d_lo, d_hi, 1, 0);
                        let contrib_de = mul(geo_d, geo_e);
                        let contrib = mul(pow2a3b5c, contrib_de);
                        result = add(result, contrib);
                    }
                }
            }
        }
    }
    result
}

fn compute_p_bruteforce(x: &[u64; 10]) -> u64 {
    let mut s = 0u64;
    let max_a = x[0].min(x[1]).min(x[2]).min(x[3]);
    for a in 0..=max_a {
        let pow2a = pow_mod(2, a);
        let max_b = (x[0] - a).min(x[4]).min(x[5]).min(x[6]);
        for b in 0..=max_b {
            let pow2a3b = mul(pow2a, pow_mod(3, b));
            let max_c = (x[1] - a).min(x[4] - b).min(x[7]).min(x[8]);
            for c in 0..=max_c {
                let pow2a3b5c = mul(pow2a3b, pow_mod(5, c));
                let max_d = (x[2] - a).min(x[5] - b).min(x[7] - c).min(x[9]);
                for d in 0..=max_d {
                    let max_e = (x[3] - a).min(x[6] - b).min(x[8] - c).min(x[9] - d);
                    let term = mul(mul(pow2a3b5c, pow_mod(7, d)), geo(11, max_e as i64));
                    s = add(s, term);
                }
            }
        }
    }
    s
}

fn compute_p(x: &[u64; 10]) -> u64 {
    if x.iter().all(|&v| v < 20) {
        compute_p_bruteforce(x)
    } else {
        compute_p_efficient(x)
    }
}

fn main() {
    let mut a = vec![0u64; 1001];
    a[0] = 1;
    a[1] = 7;
    for n in 2..=1000 {
        let sq = mul(a[n - 2], a[n - 2]);
        a[n] = add(mul(7, a[n - 1]), sq);
    }

    let mut total = 0u64;
    for n in 0..100 {
        let mut xs = [0u64; 10];
        for i in 0..10 {
            xs[i] = a[10 * n + i];
        }
        let q = compute_p(&xs);
        total = add(total, q);
    }
    println!("{}", total);
}
