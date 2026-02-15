// Project Euler 672 - One More One
// Berlekamp-Massey + polynomial exponentiation for H(10^9).

const MOD: i64 = 1_117_117_717;
const B_VAL: i64 = 7;
const K_VAL: i64 = 11;
const MAX_REC: usize = 60;

fn mod_p(mut a: i64) -> i64 { a %= MOD; if a < 0 { a += MOD; } a }
fn mod_inv(a: i64) -> i64 {
    let mut result = 1i64; let mut exp = MOD - 2; let mut base = mod_p(a);
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % MOD as i128) as i64; }
        base = (base as i128 * base as i128 % MOD as i128) as i64;
        exp >>= 1;
    }
    result
}

fn h_func(k: usize) -> i64 {
    let mut n = B_VAL - 1;
    let mut n_div_b = 0i64;
    let mut g = 0i64;
    let mut h_val = 0i64;
    for _ in 1..k {
        n = n * B_VAL + (B_VAL - 1);
        let digit = n / K_VAL;
        let tr_b1 = (B_VAL - 1) * B_VAL / 2;
        let tr_b2 = (B_VAL - 2) * (B_VAL - 1) / 2;
        let tr_diff = (B_VAL - 1 - digit) * (B_VAL - digit) / 2;
        h_val = mod_p(B_VAL * h_val + n_div_b * tr_b1 + digit * g + tr_b2 - tr_diff);
        n -= digit * K_VAL;
        n_div_b = mod_p(B_VAL * n_div_b + digit);
        g += B_VAL - 1 - digit;
    }
    h_val
}

fn berlekamp_massey(s: &[i64]) -> (usize, Vec<i64>) {
    let n = s.len();
    let mut c = vec![0i64; MAX_REC + 1]; c[0] = 1;
    let mut b_arr = vec![0i64; MAX_REC + 1]; b_arr[0] = 1;
    let (mut len_c, mut len_b) = (1, 1);
    let (mut l, mut m) = (0usize, 1usize);
    let mut b = 1i64;
    for i in 0..n {
        let mut d = s[i];
        for j in 1..=l { d = mod_p(d + (c[j] as i128 * s[i - j] as i128 % MOD as i128) as i64); }
        if d == 0 { m += 1; continue; }
        if 2 * l <= i {
            let t = c[..l+2.min(len_c)].to_vec();
            let old_l = l;
            let coeff = (d as i128 * mod_inv(b) as i128 % MOD as i128) as i64;
            let new_len = len_b + m;
            if new_len > len_c { c.resize(new_len, 0); len_c = new_len; }
            for j in 0..len_b { c[j + m] = mod_p(c[j + m] - (coeff as i128 * b_arr[j] as i128 % MOD as i128) as i64); }
            l = i + 1 - l;
            b_arr = t; len_b = old_l + 1; b = d; m = 1;
        } else {
            let coeff = (d as i128 * mod_inv(b) as i128 % MOD as i128) as i64;
            let new_len = len_b + m;
            if new_len > len_c { c.resize(new_len, 0); len_c = new_len; }
            for j in 0..len_b { c[j + m] = mod_p(c[j + m] - (coeff as i128 * b_arr[j] as i128 % MOD as i128) as i64); }
            m += 1;
        }
    }
    let mut rec = vec![0i64; l];
    for i in 0..l { rec[i] = mod_p(-c[i + 1]); }
    (l, rec)
}

fn poly_mult(a: &[i64], b: &[i64], rec: &[i64], l: usize) -> Vec<i64> {
    let mut tmp = vec![0i64; 2 * l];
    for i in 0..l { if a[i] == 0 { continue; } for j in 0..l { tmp[i + j] = mod_p(tmp[i + j] + (a[i] as i128 * b[j] as i128 % MOD as i128) as i64); } }
    for i in (l..2*l).rev() {
        if tmp[i] == 0 { continue; }
        let c = tmp[i]; tmp[i] = 0;
        for j in 0..l { tmp[i - l + j] = mod_p(tmp[i - l + j] + (c as i128 * rec[l - 1 - j] as i128 % MOD as i128) as i64); }
    }
    tmp.truncate(l);
    tmp
}

fn main() {
    let n_target: i64 = 1_000_000_000;
    let num_values = 60;
    let vals: Vec<i64> = (1..=num_values).map(|k| h_func(k)).collect();
    let (l, rec) = berlekamp_massey(&vals);
    if (n_target - 1) < l as i64 { println!("{}", vals[(n_target - 1) as usize] % MOD); return; }
    let mut base = vec![0i64; l]; if l > 1 { base[1] = 1; } else { base[0] = rec[0]; }
    let mut result = vec![0i64; l]; result[0] = 1;
    let mut exp = n_target - 1;
    while exp > 0 {
        if exp & 1 == 1 { result = poly_mult(&result, &base, &rec, l); }
        base = poly_mult(&base, &base, &rec, l);
        exp >>= 1;
    }
    let mut ans = 0i64;
    for i in 0..l { ans = mod_p(ans + (result[i] as i128 * vals[i] as i128 % MOD as i128) as i64); }
    println!("{}", ans % MOD);
}
