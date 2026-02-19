// Project Euler 968 - Quintic Pair Sums
//
// P(X_ab, X_ac, ..., X_de) = sum over non-negative (a,b,c,d,e) of
//   2^a * 3^b * 5^c * 7^d * 11^e
// subject to: for each pair (i,j), var_i + var_j <= X_{ij}.
//
// Fully closed-form recursive approach: for each variable level, split the
// summation range into pieces where the binding constraints don't change,
// then evaluate each piece as a product of geometric series over subsets.

const M: u64 = 1_000_000_007;

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
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
    mul(sub(1, p), inv(sub(1, r_mod)))
}

/// Map pair (i,j) with i<j to index in X array
/// (0,1)->0, (0,2)->1, (0,3)->2, (0,4)->3, (1,2)->4, (1,3)->5, (1,4)->6, (2,3)->7, (2,4)->8, (3,4)->9
const PAIR_INDEX: [[usize; 5]; 5] = [
    [0, 0, 1, 2, 3],
    [0, 0, 4, 5, 6],
    [1, 4, 0, 7, 8],
    [2, 5, 7, 0, 9],
    [3, 6, 8, 9, 0],
];

#[inline]
fn pair_idx(i: usize, j: usize) -> usize {
    if i < j { PAIR_INDEX[i][j] } else { PAIR_INDEX[j][i] }
}

/// Upper bound for variable at `level`, given earlier fixed values.
fn get_upper(level: usize, fixed: &[i64], x: &[u64; 10]) -> i64 {
    let mut min_val = i64::MAX;
    for k in 0..level {
        let v = x[pair_idx(k, level)] as i64 - fixed[k];
        if v < min_val { min_val = v; }
    }
    for m in (level + 1)..5 {
        let v = x[pair_idx(level, m)] as i64;
        if v < min_val { min_val = v; }
    }
    min_val
}

const BASES: [u64; 5] = [2, 3, 5, 7, 11];

/// Closed-form summation across remaining variables starting at `level`.
fn compute_level(level: usize, fixed: &mut Vec<i64>, x: &[u64; 10]) -> u64 {
    if level == 5 {
        return 1;
    }
    let r = BASES[level];
    let u = get_upper(level, fixed, x);
    if u < 0 {
        return 0;
    }

    // Collect critical points where binding constraints change
    let mut critical: Vec<i64> = Vec::with_capacity(32);
    critical.push(0);
    critical.push(u + 1);

    for m in (level + 1)..5 {
        let x_lm = x[pair_idx(level, m)] as i64;
        // Constraints from earlier-fixed variables
        for k in 0..level {
            let x_km = x[pair_idx(k, m)] as i64;
            let v = x_lm - (x_km - fixed[k]);
            if v >= 0 && v <= u + 1 {
                critical.push(v);
            }
        }
        // Constraints from later free variables
        for p in (m + 1)..5 {
            let x_mp = x[pair_idx(m, p)] as i64;
            let v = x_lm - x_mp;
            if v >= 0 && v <= u + 1 {
                critical.push(v);
            }
        }
    }

    critical.sort_unstable();
    critical.dedup();

    let mut total = 0u64;

    for ci in 0..critical.len() - 1 {
        let start = critical[ci];
        let end = critical[ci + 1] - 1;
        if start > end {
            continue;
        }
        let test_v = start; // any point in [start, end] works since constraints are constant

        // For each later variable m, determine the binding constraint at test_v
        // binding_type: 'l' = linear (depends on current var), 'c' = constant
        // A constraint on variable m's upper bound has form:
        //   X[level,m] - current_var  ("linear", type 'l')
        //   X[k,m] - fixed[k]         ("constant from earlier fixed", type 'k')
        //   X[m,p]                     ("constant from later pair", type 'p')

        let mut linear_m: Vec<usize> = Vec::new();
        let mut constant_m: Vec<usize> = Vec::new();
        let mut min_vals: [i64; 5] = [0; 5];

        for m in (level + 1)..5 {
            let x_lm = x[pair_idx(level, m)] as i64;
            let mut min_val = x_lm - test_v;
            let mut is_linear = true;

            for k in 0..level {
                let v = x[pair_idx(k, m)] as i64 - fixed[k];
                if v < min_val || (v == min_val && !is_linear) {
                    min_val = v;
                    is_linear = false;
                }
            }
            for p in (m + 1)..5 {
                let v = x[pair_idx(m, p)] as i64;
                if v < min_val || (v == min_val && !is_linear) {
                    min_val = v;
                    is_linear = false;
                }
            }

            min_vals[m] = min_val;

            if is_linear {
                linear_m.push(m);
            } else {
                constant_m.push(m);
            }
        }

        // Product of geo series for constant-bound variables
        let mut const_prod = 1u64;
        for &m in &constant_m {
            // For constant-bound m, the upper bound doesn't depend on current var
            // We need to recurse for proper handling, but since the upper bound is
            // constant across this segment, we can compute the inner sum.
            // Actually, in the fully recursive version, we need to handle all remaining
            // levels together. But the Python code handles this by computing
            // geo(bases[m], min_vals[m]) for each constant m. This works because
            // when a variable's min constraint is constant, its geometric series is
            // just geo(r_m, min_val_m).
            //
            // Wait - this is only correct at the innermost two levels. For the general
            // case we need to recurse. But the Python code does NOT recurse further -
            // it treats each remaining variable independently. This works because:
            // After fixing the binding constraints, each later variable m has an
            // independent upper bound (either constant or linear in current var).
            // The variables are NOT coupled because we're summing products.
            //
            // Actually that's not right either - variables CAN be coupled through
            // pair constraints like X[m,p]. Let me re-read the Python code...
            //
            // The Python _compute_level IS recursive! It calls _compute_level(level+1).
            // But the closed-form version doesn't recurse - it uses the subset
            // enumeration trick to handle all linear dependencies at once.
            //
            // Let me re-examine: the Python compute_P_closed_form calls _compute_level(0),
            // which is the recursive function. So it IS recursive through levels.
            // The key insight is that at each level, for each piecewise segment,
            // the "binding" dict tells us which constraint on each later variable m
            // is the tightest. For "linear" ones (bound = X[level,m] - current_var),
            // the geometric series depends on current_var. For "constant" ones, it doesn't.
            //
            // But this only considers the TIGHTEST constraint per later variable.
            // It doesn't account for couplings between later variables (e.g., m+p <= X[m,p]).
            // That seems like a bug in the Python... unless the pair constraints between
            // later variables are handled by the recursion.
            //
            // Actually wait - the Python _compute_level is NOT called recursively from
            // within itself in the closed-form version. Let me re-read...

            const_prod = mul(const_prod, geo(BASES[m], min_vals[m]));
        }

        // For linear variables: upper bound = X[level,m] - current_var
        // geo(r_m, X[level,m] - v) where v is the current level variable
        //
        // The product over linear vars of geo(r_m, x_m - v) can be expanded using:
        // geo(r, n) = (1 - r^{n+1}) / (1 - r)
        // So product = prod_m [ (1 - r_m^{x_m+1-v}) / (1-r_m) ]
        //            = prod_m [1/(1-r_m)] * prod_m [(1 - r_m^{x_m+1} * r_m^{-v})]
        //
        // Expanding the product of (1 - r_m^{x_m+1} * r_m^{-v}) over subsets:
        // = sum_{S subset of linear_m} (-1)^|S| * prod_{m in S} r_m^{x_m+1} * (prod_{m in S} r_m^{-1})^v
        //
        // Then the sum over v in [start, end] of r^v * above
        // = sum_S (-1)^|S| * prod_{m in S} r_m^{x_m+1} * sum_{v=start}^{end} (r * prod_{m in S} r_m^{-1})^v

        let n_lin = linear_m.len();

        // prod_a = product of 1/(r_m - 1) for linear m (note: 1/(1-r_m) * (-1) = 1/(r_m-1))
        // Actually Python uses: a_inv = 1/(r_m - 1), prod_a = product of a_inv
        // multiplier = (-1)^n_lin
        let mut prod_a = 1u64;
        let mut a_list: Vec<(u64, i64, u64)> = Vec::new(); // (a_inv, x_m, r_m)
        for &m in &linear_m {
            let r_m = BASES[m];
            let a_inv = inv(r_m - 1); // 1 / (r_m - 1)
            let x_m = x[pair_idx(level, m)] as i64;
            a_list.push((a_inv, x_m, r_m));
            prod_a = mul(prod_a, a_inv);
        }
        let multiplier = if n_lin % 2 == 0 { 1u64 } else { M - 1 }; // (-1)^n_lin

        // Enumerate all 2^n_lin subsets
        for mask in 0..(1u32 << n_lin) {
            let len_s = mask.count_ones() as usize;
            let sign = if len_s % 2 == 0 { 1u64 } else { M - 1 }; // (-1)^|S|

            let mut const_s = 1u64;
            let mut base_s = 1u64; // product of r_m^{-1} for m in S
            for idx in 0..n_lin {
                if mask & (1 << idx) != 0 {
                    let (_a_inv, x_m, r_m) = a_list[idx];
                    const_s = mul(const_s, pow_mod(r_m, (x_m + 1) as u64));
                    base_s = mul(base_s, inv(r_m));
                }
            }

            let coeff = mul(sign, const_s);
            let the_coeff = mul(mul(mul(const_prod, prod_a), multiplier), coeff);
            let effective_r = mul(r, base_s);

            // sum_{v=start}^{end} effective_r^v
            let seg_sum = mul(
                mul(the_coeff, pow_mod(effective_r, start as u64)),
                geo(effective_r, end - start),
            );
            total = add(total, seg_sum);
        }
    }

    total
}

/// Compute P(X) using the fully closed-form piecewise summation.
/// This handles all 5 levels without any explicit loops over variable values.
///
/// HOWEVER: the Python _compute_level only handles one level at a time and
/// treats later variables as independent (each has its own geo series).
/// This is only correct if the pair constraints between later variables
/// are NOT binding. But they CAN be binding (e.g., X[m,p] could be small).
///
/// The Python code's approach works because it's considering the product
/// of independent geometric series for each later variable. The coupling
/// constraints between later variables appear as "constant" upper bounds
/// that are factored into the product. This is valid because:
/// sum_{v1,v2,...} prod(r_i^{v_i}) subject to v_i <= bound_i (independent)
/// = prod_i sum_{v_i=0}^{bound_i} r_i^{v_i} = prod_i geo(r_i, bound_i)
///
/// The later-variable coupling constraints (like v_m + v_p <= X[m,p]) are
/// NOT handled correctly by this independent-product approach. The Python
/// code seems to ignore these couplings at deeper levels.
///
/// Actually, re-reading more carefully: the Python _compute_level IS recursive.
/// It's called from compute_P_closed_form which calls _compute_level(0, [], X, bases, M).
/// And _compute_level handles level 0, considering all later levels' constraints
/// as either linear or constant in variable 0. But it does NOT recursively call
/// itself for levels 1,2,3,4. It treats all remaining variables independently.
///
/// This means the closed-form is an APPROXIMATION that only works when inter-level
/// couplings beyond the first level are negligible... OR the Python code has a
/// different structure than I think.
///
/// Let me re-check: the Python has BOTH compute_P_efficient (3 loops + 2 analytic)
/// and compute_P_closed_form (fully analytic). The main() calls compute_P which
/// uses compute_P_efficient. So perhaps compute_P_closed_form is the one to use.
///
/// Actually, looking at the Python main(), it calls compute_P which dispatches to
/// bruteforce or efficient. The closed_form function exists but isn't called from main.
/// So maybe compute_P_closed_form is NOT the right one to port.
///
/// Let me just port compute_P_closed_form and test it against the known test cases.
/// If it gives wrong answers, I'll take a different approach.
fn compute_p_closed_form(x: &[u64; 10]) -> u64 {
    let mut fixed = Vec::new();
    compute_level(0, &mut fixed, x)
}

fn main() {
    // Test cases first
    let test1 = [2u64; 10];
    let r1 = compute_p_closed_form(&test1);
    eprintln!("P(2,2,...,2) = {} (expected 7120)", r1);

    let test2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let r2 = compute_p_closed_form(&test2);
    eprintln!("P(1,2,...,10) = {} (expected 799809376)", r2);

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
        let q = compute_p_closed_form(&xs);
        total = add(total, q);
    }
    println!("{}", total);
}
