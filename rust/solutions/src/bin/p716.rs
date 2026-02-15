// Project Euler 716 - Grid Graphs
//
// Compute C(h, w) for small h and w by brute force, use Berlekamp-Massey to
// find recurrence in w, extrapolate to W=20000 via Kitamasa. Then find
// recurrence in h and extrapolate to H=10000.

const MOD: i64 = 1_000_000_007;
const MAX_H: usize = 10;
const MAX_W: usize = 10;

fn modd(a: i64) -> i64 {
    ((a % MOD) + MOD) % MOD
}

fn power(mut base: i64, mut exp: i64, m: i64) -> i64 {
    base = ((base % m) + m) % m;
    let mut result: i64 = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % m as i128) as i64;
        }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn inv(a: i64) -> i64 {
    power(a, MOD - 2, MOD)
}

fn berlekamp_massey(seq: &[i64]) -> Vec<i64> {
    let n = seq.len();
    let mut c = vec![0i64; n + 2];
    let mut b_vec = vec![0i64; n + 2];
    c[0] = 1;
    b_vec[0] = 1;
    let mut clen = 1;
    let mut blen = 1;
    let mut big_l = 0;
    let mut m = 1;
    let mut b: i64 = 1;

    for i in 0..n {
        let mut d = seq[i];
        for j in 1..=big_l {
            d = (d as i128 + c[j] as i128 * seq[i - j] as i128 % MOD as i128) as i64 % MOD;
        }
        d = modd(d);

        if d == 0 {
            m += 1;
        } else if 2 * big_l <= i {
            let t: Vec<i64> = c[..clen].to_vec();
            let coeff = (d as i128 * inv(b) as i128 % MOD as i128) as i64;
            let newlen = blen + m;
            while clen < newlen {
                c[clen] = 0;
                clen += 1;
            }
            for j in 0..blen {
                c[j + m] = modd(c[j + m] - (coeff as i128 * b_vec[j] as i128 % MOD as i128) as i64);
            }
            big_l = i + 1 - big_l;
            b_vec[..t.len()].copy_from_slice(&t);
            blen = t.len();
            b = d;
            m = 1;
        } else {
            let coeff = (d as i128 * inv(b) as i128 % MOD as i128) as i64;
            let newlen = blen + m;
            while clen < newlen {
                c[clen] = 0;
                clen += 1;
            }
            for j in 0..blen {
                c[j + m] = modd(c[j + m] - (coeff as i128 * b_vec[j] as i128 % MOD as i128) as i64);
            }
            m += 1;
        }
    }

    let mut rec = vec![0i64; big_l];
    for i in 0..big_l {
        rec[i] = modd(-c[i + 1]);
    }
    rec
}

fn poly_mult_mod(a: &[i64], b: &[i64], rec: &[i64]) -> Vec<i64> {
    let big_l = rec.len();
    let mut raw = vec![0i64; a.len() + b.len()];
    for i in 0..a.len() {
        if a[i] == 0 {
            continue;
        }
        let ai = a[i] as i128;
        for j in 0..b.len() {
            raw[i + j] = (raw[i + j] as i128 + ai * b[j] as i128) as i64 % MOD;
        }
    }

    let mut rep = vec![0i64; big_l];
    for i in 0..big_l {
        rep[i] = rec[big_l - 1 - i];
    }

    for i in (big_l..raw.len()).rev() {
        if raw[i] == 0 {
            continue;
        }
        let c = raw[i];
        raw[i] = 0;
        for j in 0..big_l {
            raw[i - big_l + j] = (raw[i - big_l + j] as i128 + c as i128 * rep[j] as i128) as i64 % MOD;
        }
    }

    raw.truncate(big_l);
    for v in &mut raw {
        *v = modd(*v);
    }
    raw
}

fn eval_recurrence(rec: &[i64], init: &[i64], n: i64) -> i64 {
    let big_l = rec.len();
    if (n as usize) < big_l {
        return modd(init[n as usize]);
    }

    let mut result = vec![0i64; big_l];
    let mut base = vec![0i64; big_l];
    result[0] = 1;
    if big_l > 1 {
        base[1] = 1;
    } else {
        base[0] = modd(rec[0]);
    }

    let mut exp = n;
    while exp > 0 {
        if exp & 1 == 1 {
            result = poly_mult_mod(&result, &base, rec);
        }
        base = poly_mult_mod(&base, &base, rec);
        exp >>= 1;
    }

    let mut ans: i64 = 0;
    for i in 0..big_l {
        ans = (ans as i128 + result[i] as i128 * init[i] as i128) as i64 % MOD;
    }
    modd(ans)
}

struct Props {
    first0: i32,
    first1: i32,
    last0: i32,
    last1: i32,
    b0: i32,
    blast: i32,
}

fn precompute_props(n: usize) -> Vec<Props> {
    let sz = 1 << n;
    let mut props = Vec::with_capacity(sz);
    for mask in 0..sz {
        let (mut f0, mut f1, mut l0, mut l1) = (-1i32, -1, -1, -1);
        let b0_val = (mask >> 0) & 1;
        let blast_val = (mask >> (n - 1)) & 1;
        for j in 0..n {
            let bit = (mask >> j) & 1;
            if bit == 0 {
                if f0 == -1 { f0 = j as i32; }
                l0 = j as i32;
            } else {
                if f1 == -1 { f1 = j as i32; }
                l1 = j as i32;
            }
        }
        props.push(Props {
            first0: f0, first1: f1, last0: l0, last1: l1,
            b0: b0_val as i32, blast: blast_val as i32,
        });
    }
    props
}

fn imax(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn compute_c(h: usize, w: usize) -> i64 {
    let vprops = precompute_props(h);
    let hprops = precompute_props(w);

    let mut total: i64 = 0;
    for vp in &vprops {
        for hp in &hprops {
            let x1 = if vp.b0 == 0 { hp.first1 } else { hp.first0 };
            let y1 = if hp.b0 == 0 { vp.first1 } else { vp.first0 };
            let x2 = if vp.b0 == 0 { hp.last0 } else { hp.last1 };
            let y2 = if hp.blast == 0 { vp.first0 } else { vp.first1 };
            let x3 = if vp.blast == 0 { hp.first0 } else { hp.first1 };
            let y3 = if hp.b0 == 0 { vp.last0 } else { vp.last1 };
            let x4 = if vp.blast == 0 { hp.last1 } else { hp.last0 };
            let y4 = if hp.blast == 0 { vp.last1 } else { vp.last0 };

            if x1 == -1 || x2 == -1 || y1 == -1 || y3 == -1 {
                total += (w * h) as i64;
            } else {
                let mut area = x1 * y1
                    + (w as i32 - 1 - imax(x1 - 1, x2)) * y2
                    + x3 * (h as i32 - 1 - imax(y1 - 1, y3))
                    + (w as i32 - 1 - imax(x3 - 1, x4)) * (h as i32 - 1 - imax(y2 - 1, y4));
                if area < (w * h) as i32 {
                    area += 1;
                }
                total += area as i64;
            }
        }
    }
    total
}

fn main() {
    let big_h = 10000;
    let big_w = 20000;

    // Step 1: For each h, compute C(h, w) for small w, find recurrence, extrapolate to W
    let mut vals_at_w = [0i64; MAX_H];
    for h in 1..=MAX_H {
        let mut row = [0i64; MAX_W];
        for w in 1..=MAX_W {
            row[w - 1] = compute_c(h, w) % MOD;
        }

        let rec = berlekamp_massey(&row);
        let init: Vec<i64> = row[..rec.len()].to_vec();
        vals_at_w[h - 1] = eval_recurrence(&rec, &init, big_w - 1);
    }

    // Step 2: Find recurrence in h and extrapolate to H
    let rec_h = berlekamp_massey(&vals_at_w);
    let init_h: Vec<i64> = vals_at_w[..rec_h.len()].to_vec();
    let result = eval_recurrence(&rec_h, &init_h, big_h - 1);

    println!("{}", modd(result));
}
