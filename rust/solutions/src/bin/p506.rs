// Project Euler 506 - Clock Sequence
// Berlekamp-Massey + Kitamasa via CRT for composite modulus M = 123454321 = 41^2 * 271^2.

fn power_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn berlekamp_massey(seq: &[i64], m: i64) -> (usize, Vec<i64>) {
    let n = seq.len();
    let mut c = vec![0i64; n + 2];
    let mut b = vec![0i64; n + 2];
    let mut t = vec![0i64; n + 2];
    c[0] = 1; b[0] = 1;
    let mut l = 0usize;
    let mut mm = 1usize;
    let mut bv = 1i64;

    for i in 0..n {
        let mut d = seq[i];
        for j in 1..=l {
            d = (d + c[j] * seq[i - j]) % m;
        }
        d = ((d % m) + m) % m;

        if d == 0 {
            mm += 1;
        } else if 2 * l <= i {
            t[..n + 2].copy_from_slice(&c);
            let coef = d * power_mod(bv, m - 2, m) % m;
            for j in mm..=n {
                c[j] = ((c[j] - coef * b[j - mm]) % m + m) % m;
            }
            b[..n + 2].copy_from_slice(&t[..n + 2]);
            l = i + 1 - l;
            bv = d;
            mm = 1;
        } else {
            let coef = d * power_mod(bv, m - 2, m) % m;
            for j in mm..=n {
                c[j] = ((c[j] - coef * b[j - mm]) % m + m) % m;
            }
            mm += 1;
        }
    }

    let mut coeffs = vec![0i64; l + 2];
    for i in 1..=l {
        coeffs[i] = (m - c[i]) % m;
    }
    (l, coeffs)
}

fn poly_mult_mod(a: &[i64], b: &[i64], rec: &[i64], l: usize, m: i64) -> Vec<i64> {
    let mut raw = vec![0i64; 2 * l + 1];
    for i in 0..l {
        if a[i] == 0 { continue; }
        for j in 0..l {
            raw[i + j] = (raw[i + j] + a[i] * b[j]) % m;
        }
    }
    for i in (l..=(2 * l).saturating_sub(2)).rev() {
        if raw[i] == 0 { continue; }
        let c = raw[i];
        raw[i] = 0;
        for j in 0..l {
            raw[i - 1 - j] = (raw[i - 1 - j] + c * rec[j]) % m;
        }
    }
    raw[..l].to_vec()
}

fn eval_recurrence(rec: &[i64], init: &[i64], l: usize, n: i64, m: i64) -> i64 {
    if (n as usize) < l { return init[n as usize] % m; }
    if l == 0 { return 0; }

    let rec_slice = &rec[1..=l]; // [c1, c2, ..., cL]

    let mut res = vec![0i64; l];
    let mut base = vec![0i64; l];
    res[0] = 1;
    if l > 1 { base[1] = 1; } else { base[0] = rec[1] % m; }

    let mut exp = n;
    while exp > 0 {
        if exp & 1 == 1 {
            res = poly_mult_mod(&res, &base, rec_slice, l, m);
        }
        base = poly_mult_mod(&base, &base, rec_slice, l, m);
        exp >>= 1;
    }

    let mut ans = 0i64;
    for i in 0..l {
        ans = (ans + res[i] * (init[i] % m)) % m;
    }
    ans
}

fn solve_linear_mod_prime(a_mat: &[Vec<i64>], b_vec: &[i64], n: usize, p: i64) -> Option<Vec<i64>> {
    let mut aug: Vec<Vec<i64>> = (0..n).map(|i| {
        let mut row = a_mat[i].clone();
        row.push(b_vec[i]);
        row
    }).collect();

    for col in 0..n {
        let pivot = (col..n).find(|&row| aug[row][col] % p != 0);
        let pivot = match pivot {
            Some(p) => p,
            None => return None,
        };
        aug.swap(col, pivot);
        let inv_val = power_mod(aug[col][col] % p, p - 2, p);
        for j in 0..=n {
            aug[col][j] = aug[col][j] * inv_val % p;
        }
        for row in 0..n {
            if row == col { continue; }
            let factor = ((aug[row][col] % p) + p) % p;
            for j in 0..=n {
                aug[row][j] = ((aug[row][j] - factor * aug[col][j]) % p + p) % p;
            }
        }
    }
    Some((0..n).map(|i| ((aug[i][n] % p) + p) % p).collect())
}

fn find_recurrence_mod_pe(seq: &[i64], l: usize, p: i64, e: usize) -> Option<Vec<i64>> {
    let mut pe = 1i64;
    for _ in 0..e { pe *= p; }

    let a_mat: Vec<Vec<i64>> = (0..l).map(|i| {
        (0..l).map(|j| ((seq[i + l - 1 - j] % p) + p) % p).collect()
    }).collect();
    let b_vec: Vec<i64> = (0..l).map(|i| ((seq[i + l] % p) + p) % p).collect();

    let mut coeffs_cur = solve_linear_mod_prime(&a_mat, &b_vec, l, p)?;

    let mut cur_mod = p;
    for _step in 1..e {
        let next_mod = cur_mod * p;
        let residuals: Vec<i64> = (0..l).map(|i| {
            let mut r = ((seq[i + l] % next_mod) + next_mod) % next_mod;
            for j in 0..l {
                let sval = ((seq[i + l - 1 - j] % next_mod) + next_mod) % next_mod;
                r = ((r as i128 - coeffs_cur[j] as i128 * sval as i128) % next_mod as i128 + next_mod as i128) as i64 % next_mod;
            }
            ((r % next_mod + next_mod) % next_mod / cur_mod) % p
        }).collect();

        let delta = solve_linear_mod_prime(&a_mat, &residuals, l, p)?;
        for j in 0..l {
            coeffs_cur[j] = (coeffs_cur[j] + cur_mod * delta[j]) % next_mod;
        }
        cur_mod = next_mod;
    }

    let mut result = vec![0i64; l + 2];
    for i in 0..l {
        result[i + 1] = (coeffs_cur[i] % pe + pe) % pe;
    }
    Some(result)
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0); }
    let (g, x1, y1) = extended_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

fn crt2(r1: i64, m1: i64, r2: i64, m2: i64) -> i64 {
    let (g, x, _) = extended_gcd(m1, m2);
    let lcm = m1 / g * m2;
    let diff = (r2 - r1) / g;
    let r = ((r1 as i128 + m1 as i128 * ((diff as i128 * x as i128) % (m2 / g) as i128)) % lcm as i128 + lcm as i128) as i64 % lcm;
    r
}

fn main() {
    let digits = [1, 2, 3, 4, 3, 2];
    let n_target: i64 = 100_000_000_000_000; // 10^14
    let m_val: i64 = 123454321;

    let num_values = 500;
    let mut values = vec![0i64; num_values];

    // Compute cumulative sums
    let mut cum_sum = 0i64;
    let mut digit_idx = 0usize;
    let mut v_buf = Vec::new();

    fn sum_digits(buf: &[u8]) -> usize {
        buf.iter().map(|&d| d as usize).sum()
    }

    for term_num in 1..=num_values {
        v_buf.clear();
        let mut vlen = 0;
        while sum_digits(&v_buf) < term_num {
            v_buf.push(digits[digit_idx % 6]);
            digit_idx += 1;
            vlen += 1;
        }
        let mut val = 0i64;
        for i in 0..vlen {
            val = (val * 10 + v_buf[i] as i64) % m_val;
        }
        cum_sum = (cum_sum + val) % m_val;
        values[term_num - 1] = cum_sum;
    }

    // M = 41^2 * 271^2
    let primes = [(41i64, 2), (271i64, 2)];
    let mut results_r = [0i64; 2];
    let mut results_m = [0i64; 2];

    for (fi, &(p, e)) in primes.iter().enumerate() {
        let mut pe = 1i64;
        for _ in 0..e { pe *= p; }

        // Find recurrence order via BM mod p
        let seq_p: Vec<i64> = values.iter().map(|&v| ((v % p) + p) % p).collect();
        let (l, _) = berlekamp_massey(&seq_p, p);

        if l == 0 {
            results_r[fi] = 0;
            results_m[fi] = pe;
            continue;
        }

        // Find recurrence mod p^e
        let coeffs = find_recurrence_mod_pe(&values, l, p, e as usize).unwrap();

        // Evaluate at N_target - 1 (0-indexed)
        let init: Vec<i64> = (0..l).map(|i| ((values[i] % pe) + pe) % pe).collect();
        let val = eval_recurrence(&coeffs, &init, l, n_target - 1, pe);
        results_r[fi] = val % pe;
        results_m[fi] = pe;
    }

    let r = crt2(results_r[0], results_m[0], results_r[1], results_m[1]);
    println!("{}", r % m_val);
}
