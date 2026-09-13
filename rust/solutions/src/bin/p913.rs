// Project Euler 913 - Matrix Shuffles
// For (n,m) with 2<=n<=m<=100, compute S(n,m) for the n^4,m^4 case.
// S = (nm)^4 - cycle_count - 1
// cycle_count = sum phi(d)/ord_d(multiplier) over divisors of L=(nm)^4-1.

#[inline(always)]
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
    let mut factors = Vec::with_capacity(8);
    
    if n & 1 == 0 {
        let mut e = 0;
        while n & 1 == 0 {
            e += 1;
            n >>= 1;
        }
        factors.push(Factor { prime: 2, exp: e });
    }
    
    let mut d: i64 = 3;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 {
                e += 1;
                n /= d;
            }
            factors.push(Factor { prime: d, exp: e });
        }
        d += 2;
    }
    
    if n > 1 {
        factors.push(Factor { prime: n, exp: 1 });
    }
    factors
}

#[inline(always)]
fn pow_mod_u64(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut res: u64 = 1;
    a %= m;
    while b > 0 {
        if b & 1 == 1 {
            res = ((res as u128 * a as u128) % m as u128) as u64;
        }
        a = ((a as u128 * a as u128) % m as u128) as u64;
        b >>= 1;
    }
    res
}

#[inline(always)]
fn pow_mod_ll(a: i64, b: i64, m: i64) -> i64 {
    pow_mod_u64(a as u64, b as u64, m as u64) as i64
}

#[inline]
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
        let pk_u = pk as u64;
        for _ in 1..=k {
            order <<= 1;
            curr = ((curr as u128 * curr as u128) % pk_u as u128) as i64;
            if curr == 1 { return order; }
        }
        return order;
    }

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

#[inline]
fn count_cycles(n: i32, m: i32) -> i64 {
    let x = n as i64 * m as i64;
    let x_sq = x * x;
    
    let mut merged: Vec<Factor> = Vec::with_capacity(32);
    
    for &part in &[x - 1, x + 1, x_sq + 1] {
        let facs = factorize(part);
        for f in facs {
            if let Some(mf) = merged.iter_mut().find(|mf| mf.prime == f.prime) {
                mf.exp += f.exp;
            } else {
                merged.push(f);
            }
        }
    }

    let multiplier_raw = (n as i64).pow(4);

    let mut pd: Vec<PrimeData> = Vec::with_capacity(merged.len());
    for mf in &merged {
        let p = mf.prime;
        let e = mf.exp as usize;
        let mut data = PrimeData {
            num_entries: e + 1,
            phi_val: [0; 64],
            ord_val: [0; 64],
        };

        data.phi_val[0] = 1;
        data.ord_val[0] = 1;
        
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
            data.phi_val[j] = pp * (p - 1);
            data.ord_val[j] = curr_ord;
        }

        pd.push(data);
    }

    struct StackEntry {
        idx: usize,
        lcm_ord: i64,
        phi_prod: i64,
    }

    let nmerged = merged.len();
    let mut stack: Vec<StackEntry> = Vec::with_capacity(16384);
    stack.push(StackEntry { idx: 0, lcm_ord: 1, phi_prod: 1 });

    let mut total_sum: i64 = 0;

    while let Some(se) = stack.pop() {
        if se.idx == nmerged {
            total_sum += se.phi_prod / se.lcm_ord;
            continue;
        }
        let idx = se.idx;
        let pd_idx = unsafe { pd.get_unchecked(idx) };
        for j in 0..pd_idx.num_entries {
            let phi_part = unsafe { *pd_idx.phi_val.get_unchecked(j) };
            let ord_part = unsafe { *pd_idx.ord_val.get_unchecked(j) };
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
