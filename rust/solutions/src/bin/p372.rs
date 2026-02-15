// Project Euler 372: Pencils of Rays
fn isqrt_ll(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn exact_floor_n_alpha(n: i64, d: i64, p: i64, q: i64) -> i64 {
    if n == 0 { return 0; }
    let nd = n as i128 * n as i128 * d as i128;
    let mut s;
    if nd <= 9_000_000_000_000_000_000i128 {
        s = isqrt_ll(nd as i64);
    } else {
        s = (n as f64 * (d as f64).sqrt()) as i64;
    }
    while (s as i128) * (s as i128) > nd { s -= 1; }
    while ((s + 1) as i128) * ((s + 1) as i128) <= nd { s += 1; }

    let total_int = n as i128 * p as i128 + s as i128;
    let q_div = (total_int / q as i128) as i64;
    let r = total_int - q_div as i128 * q as i128;
    let need = q as i128 - r;
    if need <= 0 { return q_div + 1; }
    let rhs = s as i128 + need;
    if nd >= rhs * rhs { return q_div + 1; }
    q_div
}

fn sum_floor(n: i64, d: i64) -> i64 {
    if n <= 0 { return 0; }
    let s = isqrt_ll(d);
    if s * s == d {
        return s * n * (n + 1) / 2;
    }

    let base = s as i128 * n as i128 * (n + 1) as i128 / 2;
    let mut p_cur = -s;
    let mut q_cur = 1i64;
    let mut n_cur = n;
    let mut sign = 1i128;
    let mut result = base;

    while n_cur > 0 {
        let m = exact_floor_n_alpha(n_cur, d, p_cur, q_cur);
        if m == 0 { break; }

        let denom = d as i128 - p_cur as i128 * p_cur as i128;
        let q_new = (denom / q_cur as i128) as i64;
        let p_inv = -p_cur;

        let a = exact_floor_n_alpha(1, d, p_inv, q_new);

        let term = n_cur as i128 * m as i128 - a as i128 * m as i128 * (m + 1) as i128 / 2;
        result += sign * term;

        p_cur = p_inv - a * q_new;
        q_cur = q_new;
        n_cur = m;
        sign = -sign;
    }

    result as i64
}

fn floor_div_sqrt(num: i64, den: i64) -> i64 {
    if den == 0 || num <= 0 { return 0; }
    let num2 = num as i128 * num as i128;
    let val = (num2 / den as i128) as i64;
    let mut z = isqrt_ll(val);
    while (z + 1) as i128 * (z + 1) as i128 * den as i128 <= num2 { z += 1; }
    while z as i128 * z as i128 * den as i128 > num2 { z -= 1; }
    z
}

fn sum_upper(ll: i64, rr: i64, n_val: i64, d: i64) -> i64 {
    if ll > rr { return 0; }
    let s = isqrt_ll(d);
    if s * s == d {
        let x_split = (n_val + s) / s;
        let mut res = 0i64;
        let left_end = rr.min(x_split - 1);
        if ll <= left_end {
            let cnt = left_end - ll + 1;
            let sum_x = cnt * (ll + left_end) / 2;
            res += s * sum_x - cnt;
        }
        let right_start = ll.max(x_split);
        if right_start <= rr {
            let cnt = rr - right_start + 1;
            res += n_val * cnt;
        }
        res
    } else {
        let x_split = floor_div_sqrt(n_val, d) + 1;
        let left_end = rr.min(x_split - 1);
        let mut res = 0i64;
        if ll <= left_end {
            res += sum_floor(left_end, d) - sum_floor(ll - 1, d);
        }
        let right_start = ll.max(x_split);
        if right_start <= rr {
            let cnt = rr - right_start + 1;
            res += n_val * cnt;
        }
        res
    }
}

fn sum_lower_max(ll: i64, rr: i64, l_val: i64, d: i64) -> i64 {
    if ll > rr { return 0; }
    let s = isqrt_ll(d);
    if s * s == d {
        let x_split = (l_val + s - 1) / s;
        let mut res = 0i64;
        let left_end = rr.min(x_split - 1);
        if ll <= left_end {
            let cnt = left_end - ll + 1;
            res += l_val * cnt;
        }
        let right_start = ll.max(x_split);
        if right_start <= rr {
            let cnt = rr - right_start + 1;
            let sum_x = cnt * (right_start + rr) / 2;
            res += s * sum_x;
        }
        res
    } else {
        let x_split = if l_val <= 1 { 1 } else { floor_div_sqrt(l_val - 1, d) + 1 };
        let mut res = 0i64;
        let left_end = rr.min(x_split - 1);
        if ll <= left_end {
            let cnt = left_end - ll + 1;
            res += l_val * cnt;
        }
        let right_start = ll.max(x_split);
        if right_start <= rr {
            let cnt = rr - right_start + 1;
            let sf = sum_floor(rr, d) - sum_floor(right_start - 1, d);
            res += sf + cnt;
        }
        res
    }
}

fn compute_r(m: i64, n_val: i64) -> i64 {
    let l_val = m + 1;
    if l_val > n_val { return 0; }
    let mut total = 0i64;
    let max_ratio = n_val / l_val + 1;
    let max_k = 2 * max_ratio * max_ratio;
    let mut k = 1i64;
    while k <= max_k {
        let x_lo = floor_div_sqrt(l_val - 1, k + 1) + 1;
        let x_start = l_val.max(x_lo);
        let x_end_bound = floor_div_sqrt(n_val, k);
        let x_end = n_val.min(x_end_bound);

        if x_start <= x_end {
            let su = sum_upper(x_start, x_end, n_val, k + 1);
            let sl = sum_lower_max(x_start, x_end, l_val, k);
            let num_x = x_end - x_start + 1;
            let contrib = su - sl + num_x;
            if contrib > 0 { total += contrib; }
        }
        k += 2;
    }
    total
}

fn main() {
    println!("{}", compute_r(2_000_000, 1_000_000_000));
}
