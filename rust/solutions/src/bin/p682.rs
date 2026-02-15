// Project Euler 682 - 5-Smooth Pairs
// Berlekamp-Massey + polynomial exponentiation for linear recurrence.

const MOD: i64 = 1_000_000_007;
const MAX_REC: usize = 80;

fn power_mod(mut base: i64, mut exp: i64) -> i64 {
    let mut r = 1i64;
    base = ((base % MOD) + MOD) % MOD;
    while exp > 0 {
        if exp & 1 == 1 { r = (r as i128 * base as i128 % MOD as i128) as i64; }
        base = (base as i128 * base as i128 % MOD as i128) as i64;
        exp >>= 1;
    }
    r
}

fn inv_mod(a: i64) -> i64 { power_mod(a, MOD - 2) }

fn berlekamp_massey(s: &[i64]) -> Vec<i64> {
    let n = s.len();
    let mut c = vec![0i64; n + 1]; c[0] = 1;
    let mut b = vec![0i64; n + 1]; b[0] = 1;
    let mut t = vec![0i64; n + 1];
    let (mut big_l, mut m) = (0usize, 1usize);
    let mut bv = 1i64;
    let mut len_c = 1usize;
    let mut len_b = 1usize;

    for i in 0..n {
        let mut d = s[i];
        for j in 1..=big_l {
            d = (d as i128 + c[j] as i128 * s[i - j] as i128 % MOD as i128) as i64;
        }
        d = ((d % MOD) + MOD) % MOD;

        if d == 0 {
            m += 1;
        } else if 2 * big_l <= i {
            t[..len_c].copy_from_slice(&c[..len_c]);
            let old_len_c = len_c;
            let coeff = (d as i128 * inv_mod(bv) as i128 % MOD as i128) as i64;
            if len_b + m > len_c { len_c = len_b + m; }
            for j in 0..len_b {
                c[j + m] = ((c[j + m] as i128 - coeff as i128 * b[j] as i128 % MOD as i128 + MOD as i128) % MOD as i128) as i64;
            }
            big_l = i + 1 - big_l;
            b[..old_len_c].copy_from_slice(&t[..old_len_c]);
            len_b = old_len_c;
            bv = d;
            m = 1;
        } else {
            let coeff = (d as i128 * inv_mod(bv) as i128 % MOD as i128) as i64;
            if len_b + m > len_c { len_c = len_b + m; }
            for j in 0..len_b {
                c[j + m] = ((c[j + m] as i128 - coeff as i128 * b[j] as i128 % MOD as i128 + MOD as i128) % MOD as i128) as i64;
            }
            m += 1;
        }
    }

    let mut rec = vec![0i64; big_l];
    for i in 0..big_l {
        rec[i] = (MOD - c[i + 1]) % MOD;
    }
    rec
}

fn poly_mult(a: &[i64], b: &[i64], rec: &[i64], l: usize) -> Vec<i64> {
    let mut tmp = vec![0i64; 2 * l];
    for i in 0..l {
        if a[i] == 0 { continue; }
        for j in 0..l {
            tmp[i + j] = (tmp[i + j] as i128 + a[i] as i128 * b[j] as i128 % MOD as i128) as i64 % MOD;
        }
    }
    for i in (l..2 * l).rev() {
        if tmp[i] == 0 { continue; }
        let c = tmp[i];
        tmp[i] = 0;
        for j in 0..l {
            tmp[i - l + j] = (tmp[i - l + j] as i128 + c as i128 * rec[l - 1 - j] as i128 % MOD as i128) as i64 % MOD;
        }
    }
    tmp.truncate(l);
    tmp
}

fn linear_recurrence_nth(init: &[i64], rec: &[i64], n: i64) -> i64 {
    let l = rec.len();
    if (n as usize) < l { return init[n as usize] % MOD; }

    let mut base = vec![0i64; l];
    let mut result = vec![0i64; l];
    result[0] = 1;
    if l > 1 { base[1] = 1; } else { base[0] = rec[0]; }

    let mut exp = n;
    while exp > 0 {
        if exp & 1 == 1 {
            result = poly_mult(&result, &base, rec, l);
        }
        base = poly_mult(&base, &base, rec, l);
        exp >>= 1;
    }

    let mut ans = 0i64;
    for i in 0..l {
        ans = (ans as i128 + result[i] as i128 * init[i] as i128 % MOD as i128) as i64 % MOD;
    }
    ans
}

fn f(n: i32) -> i64 {
    let mut count = 0i64;
    let mut p2 = 0;
    while 2 * p2 < n {
        let mut p3 = 0;
        while 2 * p2 + 3 * p3 < n {
            let mut p5 = 0;
            while 2 * p2 + 3 * p3 + 5 * p5 < n {
                let mut q2 = 0;
                while 2 * p2 + 3 * p3 + 5 * p5 + 2 * q2 <= n {
                    let mut q3 = 0;
                    while 2 * p2 + 3 * p3 + 5 * p5 + 2 * q2 + 3 * q3 <= n {
                        let q5 = p2 + p3 + p5 - q2 - q3;
                        if q5 >= 0 && 2 * p2 + 3 * p3 + 5 * p5 + 2 * q2 + 3 * q3 + 5 * q5 == n {
                            count += 1;
                        }
                        q3 += 1;
                    }
                    q2 += 1;
                }
                p5 += 1;
            }
            p3 += 1;
        }
        p2 += 1;
    }
    count
}

fn main() {
    let big_n: i64 = 10_000_000;
    let num_values = MAX_REC;

    let mut vals = vec![0i64; num_values];
    for i in 0..num_values {
        vals[i] = f(i as i32) % MOD;
    }

    let rec = berlekamp_massey(&vals);
    let ans = linear_recurrence_nth(&vals, &rec, big_n);
    println!("{}", ans);
}
