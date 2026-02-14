// Project Euler 502 - Counting Castles
// Berlekamp-Massey + Kitamasa to extrapolate linear recurrences mod 10^9+7.

const MOD: i64 = 1_000_000_007;

fn modd(x: i64) -> i64 {
    ((x % MOD) + MOD) % MOD
}

fn power(mut base: i64, mut exp: i64) -> i64 {
    let mut result = 1i64;
    base = modd(base);
    while exp > 0 {
        if exp & 1 == 1 { result = modd(result * base); }
        base = modd(base * base);
        exp >>= 1;
    }
    result
}

fn inv(x: i64) -> i64 {
    power(modd(x), MOD - 2)
}

fn num_castles(w: usize, h: usize) -> i64 {
    if h == 0 { return 0; }
    let mut dp_prev0 = vec![0i64; h + 1];
    let mut dp_prev1 = vec![0i64; h + 1];
    let mut dp_curr0 = vec![0i64; h + 1];
    let mut dp_curr1 = vec![0i64; h + 1];

    for y in (0..=h).step_by(2) {
        dp_prev0[y] = 1;
    }

    for _x in 1..=w {
        dp_curr0.iter_mut().for_each(|v| *v = 0);
        dp_curr1.iter_mut().for_each(|v| *v = 0);

        for y in 1..=h {
            let mut val0 = dp_prev0[h] + dp_prev0[h - 1] - dp_prev0[y - 1] + dp_prev1[y - 1];
            if y >= 2 { val0 += dp_curr0[y - 2]; }
            dp_curr0[y] = modd(val0);

            let mut val1 = dp_prev1[h] + dp_prev1[h - 1] - dp_prev1[y - 1] + dp_prev0[y - 1];
            if y >= 2 { val1 += dp_curr1[y - 2]; }
            dp_curr1[y] = modd(val1);
        }

        std::mem::swap(&mut dp_prev0, &mut dp_curr0);
        std::mem::swap(&mut dp_prev1, &mut dp_curr1);
    }

    modd(dp_prev0[h] + dp_prev0[h - 1])
}

fn berlekamp_massey(s: &[i64]) -> Vec<i64> {
    let n = s.len();
    let mut c = vec![0i64; n + 2];
    let mut b = vec![0i64; n + 2];
    c[0] = 1; b[0] = 1;
    let mut rec_len = 0usize;
    let mut m = 1usize;
    let mut bv = 1i64;

    for i in 0..n {
        let mut d = s[i];
        for j in 1..=rec_len {
            d = modd(d + c[j] * s[i - j]);
        }

        if d == 0 {
            m += 1;
        } else if 2 * rec_len <= i {
            let t = c.clone();
            let coef = modd(d * inv(bv));
            for j in m..=n {
                c[j] = modd(c[j] - coef * b[j - m]);
            }
            b = t;
            rec_len = i + 1 - rec_len;
            bv = d;
            m = 1;
        } else {
            let coef = modd(d * inv(bv));
            for j in m..=n {
                c[j] = modd(c[j] - coef * b[j - m]);
            }
            m += 1;
        }
    }

    // Return coefficients: c[1..=rec_len] negated
    let mut coeffs = vec![0i64; rec_len + 1];
    for i in 1..=rec_len {
        coeffs[i] = modd(-c[i]);
    }
    coeffs
}

fn linear_recurrence(coeffs: &[i64], a: &[i64], n: i64) -> i64 {
    let rec_len = coeffs.len() - 1;
    if (n as usize) < rec_len { return a[n as usize]; }
    if rec_len == 0 { return 0; }

    let mut q = vec![0i64; rec_len];
    let mut r = vec![0i64; rec_len];
    q[0] = 1;
    if rec_len > 1 { r[1] = 1; } else { r[0] = coeffs[1] % MOD; }

    let poly_mult = |a: &[i64], b: &[i64], coeffs: &[i64]| -> Vec<i64> {
        let len = rec_len;
        let mut tmp = vec![0i64; 2 * len + 2];
        for i in 0..len {
            for j in 0..len {
                tmp[i + j] = modd(tmp[i + j] + a[i] * b[j]);
            }
        }
        for i in (len..=(2 * len - 2)).rev() {
            for j in 1..=len {
                tmp[i - j] = modd(tmp[i - j] + tmp[i] * coeffs[j]);
            }
            tmp[i] = 0;
        }
        tmp[..len].to_vec()
    };

    let mut exp = n;
    while exp > 0 {
        if exp & 1 == 1 {
            q = poly_mult(&q, &r, coeffs);
        }
        r = poly_mult(&r, &r, coeffs);
        exp >>= 1;
    }

    let mut result = 0i64;
    for i in 0..rec_len {
        result = modd(result + q[i] * a[i]);
    }
    result
}

fn extrapolate(values: &[i64], x: i64) -> i64 {
    let n = values.len();
    if x <= n as i64 { return values[(x - 1) as usize]; }

    let coeffs = berlekamp_massey(values);
    let rec_len = coeffs.len() - 1;

    // coeffs already has the recurrence coefficients
    // linear_recurrence expects coeffs[1..] as the recurrence: a[n] = sum coeffs[i]*a[n-i]
    let mut c2 = vec![0i64; rec_len + 1];
    for i in 1..=rec_len {
        c2[i] = modd(-coeffs[i]); // Wait, BM gives us negated already...
    }

    // Actually, berlekamp_massey returns coeffs with coeffs[i] = -C[i].
    // The recurrence is: s[n] = sum_{j=1}^{L} coeffs[j] * s[n-j]
    // linear_recurrence expects C[j] where the reduction uses C[j].
    // In the C code, after BM, C[i] is negated before calling linear_recurrence.
    // So linear_recurrence expects the same sign as the recurrence coefficients.
    // Let's just match the C code exactly.

    // In the C extrapolate: C[i] = -C[i] (i.e., negate)
    // But berlekamp_massey here already returns -C[i] as coeffs[i].
    // Wait no - let me re-read. In the C code's berlekamp_massey, it returns C as the BM poly.
    // Then extrapolate does C[i] = mod(-C[i]), then calls linear_recurrence(C, recLen, values, x-1).
    // Our berlekamp_massey returns coeffs[i] = -C[i] already.
    // So we need to negate them again for linear_recurrence... which would give us back C[i].
    // Actually wait: let me just be careful.

    // In C's BM: fills C such that C[0]=1, s[i] + sum_{j=1}^{L} C[j]*s[i-j] = 0
    // In C's extrapolate: for(i=1..L) C[i] = mod(-C[i]) => makes C[i] positive: s[i] = sum C[j]*s[i-j]
    // In C's linear_recurrence: uses C[j] for reduction (adding C[j] * coeff)
    // So linear_recurrence wants the positive coefficients.

    // Our BM returns coeffs[i] = mod(-C_bm[i]) = the positive coefficients.
    // Wait, no. Let me re-read our BM:
    // coeffs[i] = modd(-c[i])  where c is the BM polynomial.
    // So coeffs[i] is the positive recurrence coefficient. Good.

    // But linear_recurrence in C uses C[j] where s[n] = sum C[j]*s[n-j],
    // and for reduction: tmp[i-j] += tmp[i] * C[j].
    // Our linear_recurrence does the same.

    linear_recurrence(&coeffs, values, x - 1)
}

fn num_castles_big(w: i64, h: i64) -> i64 {
    let l = 500;
    if w <= 100 {
        let values: Vec<i64> = (1..=l).map(|hh| num_castles(w as usize, hh)).collect();
        return extrapolate(&values, h);
    }
    if h <= 100 {
        let values: Vec<i64> = (1..=l).map(|ww| num_castles(ww, h as usize)).collect();
        return extrapolate(&values, w);
    }
    num_castles(w as usize, h as usize)
}

fn main() {
    let mut ans = 0i64;

    // Case 1: W=10^12, H=100
    let w = 1_000_000_000_000i64;
    let h = 100i64;
    ans = modd(ans + num_castles_big(w, h) - num_castles_big(w, h - 1));

    // Case 2: W=10000, H=10000
    let w = 10000i64;
    let h = 10000i64;
    ans = modd(ans + num_castles_big(w, h) - num_castles_big(w, h - 1));

    // Case 3: W=100, H=10^12
    let w = 100i64;
    let h = 1_000_000_000_000i64;
    ans = modd(ans + num_castles_big(w, h) - num_castles_big(w, h - 1));

    println!("{}", ans);
}
