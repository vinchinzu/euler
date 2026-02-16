// Project Euler 913 - Matrix Shuffles
// For (n,m) with 2<=n<=m<=100, compute S(n,m) for the n^4,m^4 case.
// S = (nm)^4 - cycle_count - 1
// cycle_count = sum phi(d)/ord_d(multiplier) over divisors of L=(nm)^4-1.

fn gcd_ll(mut a: i64, mut b: i64) -> i64 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(Clone, Copy)]
struct Factor {
    prime: i64,
    exp: i32,
}

fn factorize(mut n: i64) -> Vec<Factor> {
    let mut factors = Vec::new();
    let mut d: i64 = 2;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 {
                e += 1;
                n /= d;
            }
            factors.push(Factor { prime: d, exp: e });
        }
        d += 1;
    }
    if n > 1 {
        factors.push(Factor { prime: n, exp: 1 });
    }
    factors
}

fn pow_mod_ll(mut a: i64, mut b: i64, m: i64) -> i64 {
    let mut res: i64 = 1;
    a %= m;
    if a < 0 { a += m; }
    while b > 0 {
        if b & 1 == 1 {
            res = (res as i128 * a as i128 % m as i128) as i64;
        }
        a = (a as i128 * a as i128 % m as i128) as i64;
        b >>= 1;
    }
    res
}

/// Compute order of a mod p^k
fn get_order_mod_pk(a: i64, p: i64, k: i32) -> i64 {
    let mut pk: i64 = 1;
    for _ in 0..k {
        pk *= p;
    }

    if p == 2 {
        let mut order: i64 = 1;
        let mut curr = a % pk;
        if curr < 0 { curr += pk; }
        if curr == 1 { return 1; }
        for _ in 1..=k {
            order *= 2;
            curr = (curr as i128 * curr as i128 % pk as i128) as i64;
            if curr == 1 { return order; }
        }
        return order;
    }

    // For odd p: find order mod p first
    let p_minus_1 = p - 1;
    let pf = factorize(p_minus_1);

    let mut order = p_minus_1;
    for f in &pf {
        let q = f.prime;
        while order % q == 0 {
            let temp_order = order / q;
            if pow_mod_ll(a, temp_order, p) == 1 {
                order = temp_order;
            } else {
                break;
            }
        }
    }

    // Lift to p^k
    let mut curr_order = order;
    loop {
        if pow_mod_ll(a, curr_order, pk) == 1 {
            return curr_order;
        }
        curr_order *= p;
    }
}

struct PrimeData {
    num_entries: usize,
    phi_val: [i64; 64],
    ord_val: [i64; 64],
}

fn count_cycles(n: i32, m: i32) -> i64 {
    let x = n as i64 * m as i64;
    // L = X^4 - 1 = (X-1)(X+1)(X^2+1)
    let parts = [x - 1, x + 1, x * x + 1];

    // Merge all prime factors
    let mut merged: Vec<Factor> = Vec::new();
    for &part in &parts {
        let facs = factorize(part);
        for f in facs {
            let mut found = false;
            for mf in merged.iter_mut() {
                if mf.prime == f.prime {
                    mf.exp += f.exp;
                    found = true;
                    break;
                }
            }
            if !found {
                merged.push(f);
            }
        }
    }

    let multiplier_raw = (n as i64).pow(4);

    // Precompute data for each prime
    let mut pd: Vec<PrimeData> = Vec::with_capacity(merged.len());
    for mf in &merged {
        let p = mf.prime;
        let e = mf.exp as usize;
        let mut data = PrimeData {
            num_entries: e + 1,
            phi_val: [0; 64],
            ord_val: [0; 64],
        };

        // j=0: phi=1, ord=1
        data.phi_val[0] = 1;
        data.ord_val[0] = 1;

        // j=1
        let ord_p = get_order_mod_pk(multiplier_raw, p, 1);
        let mut curr_ord = ord_p;
        data.phi_val[1] = p - 1;
        data.ord_val[1] = curr_ord;

        let mut current_p_pow = p;
        for j in 2..=e {
            current_p_pow *= p;
            if pow_mod_ll(multiplier_raw, curr_ord, current_p_pow) != 1 {
                curr_ord *= p;
            }
            let mut pp: i64 = 1;
            for _ in 0..j - 1 {
                pp *= p;
            }
            let phi_j = pp * (p - 1);
            data.phi_val[j] = phi_j;
            data.ord_val[j] = curr_ord;
        }

        pd.push(data);
    }

    // DFS to sum phi(d)/ord_d(mult)
    struct StackEntry {
        idx: usize,
        lcm_ord: i64,
        phi_prod: i64,
    }

    let nmerged = merged.len();
    let mut stack: Vec<StackEntry> = Vec::with_capacity(1_000_000);
    stack.push(StackEntry { idx: 0, lcm_ord: 1, phi_prod: 1 });

    let mut total_sum: i64 = 0;

    while let Some(se) = stack.pop() {
        if se.idx == nmerged {
            total_sum += se.phi_prod / se.lcm_ord;
            continue;
        }
        let idx = se.idx;
        for j in 0..pd[idx].num_entries {
            let phi_part = pd[idx].phi_val[j];
            let ord_part = pd[idx].ord_val[j];
            let g = gcd_ll(se.lcm_ord, ord_part);
            let new_lcm = se.lcm_ord / g * ord_part;
            stack.push(StackEntry {
                idx: idx + 1,
                lcm_ord: new_lcm,
                phi_prod: se.phi_prod * phi_part,
            });
        }
    }

    total_sum
}

fn main() {
    let mut total: i64 = 0;
    for n in 2..=100 {
        for m in n..=100 {
            let mut size: i64 = 1;
            for _ in 0..4 {
                size *= n as i64 * m as i64;
            }
            if size <= 2 {
                continue;
            }
            let cycle_term = count_cycles(n, m);
            let val = size - cycle_term - 1;
            total += val;
        }
    }
    println!("{}", total);
}
