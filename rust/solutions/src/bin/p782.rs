// Project Euler 782 - Distinct Rows and Columns
// C(n) = 3n^2 - 1 - N2 + N4 via bitarray sieve for achievability.

fn main() {
    let n: i64 = 10000;
    let big_n = n * n;

    let mut achievable = vec![false; (big_n + 1) as usize];
    achievable[0] = true;
    achievable[big_n as usize] = true;

    // S2: comp=2 values from 2x2 block matrices
    let mut is_s2 = vec![false; (big_n + 1) as usize];

    for c in 1..n {
        let v = c * c;
        if v > 0 && v < big_n { is_s2[v as usize] = true; }
        let w = big_n - v;
        if w > 0 && w < big_n { is_s2[w as usize] = true; }
    }
    for x in 1..n {
        let y = n - x;
        let v1 = x * x + y * y;
        let v2 = 2 * x * y;
        if v1 > 0 && v1 < big_n { is_s2[v1 as usize] = true; }
        if v2 > 0 && v2 < big_n { is_s2[v2 as usize] = true; }
    }

    let mut n2: i64 = 0;
    for k in 1..big_n as usize {
        if is_s2[k] {
            n2 += 1;
            achievable[k] = true;
        }
    }
    drop(is_s2);

    // Construction 1: Products d*m with 1 <= d,m <= n-1
    for d in 1..n {
        let mut k = d;
        while k < d * n {
            achievable[k as usize] = true;
            k += d;
        }
    }

    // Construction 2: Complement symmetry
    for k in 1..big_n as usize {
        let comp = big_n as usize - k;
        if achievable[k] || achievable[comp] {
            achievable[k] = true;
            achievable[comp] = true;
        }
    }

    // Construction 3: Kernel 3x3 matrices
    let rows_3: [[i64; 3]; 8] = {
        let mut r = [[0i64; 3]; 8];
        for i in 0..8 {
            r[i][0] = (i >> 2) as i64 & 1;
            r[i][1] = (i >> 1) as i64 & 1;
            r[i][2] = i as i64 & 1;
        }
        r
    };

    struct KForm {
        aa: i64, bb: i64, ab: i64, a1: i64, b1: i64, c0: i64,
    }
    let mut forms: Vec<KForm> = Vec::new();

    for r0i in 0..8usize {
        for r1i in 0..8usize {
            for r2i in 0..8usize {
                let m = [rows_3[r0i], rows_3[r1i], rows_3[r2i]];

                // Check that every column is also a row
                let mut ok = true;
                for j in 0..3 {
                    let col = [m[0][j], m[1][j], m[2][j]];
                    let mut found = false;
                    for ri in 0..3 {
                        if m[ri] == col { found = true; break; }
                    }
                    if !found { ok = false; break; }
                }
                if !ok { continue; }

                let (a_coeff, b_coeff, c_coeff) = (m[0][0], m[1][1], m[2][2]);
                let d01 = m[0][1] + m[1][0];
                let d02 = m[0][2] + m[2][0];
                let d12 = m[1][2] + m[2][1];
                let aa = a_coeff + c_coeff - d02;
                let bb = b_coeff + c_coeff - d12;
                let ab = d01 + 2 * c_coeff - d02 - d12;
                let a1 = n * (d02 - 2 * c_coeff);
                let b1 = n * (d12 - 2 * c_coeff);
                let c0 = c_coeff * n * n;

                let dup = forms.iter().any(|f| {
                    f.aa == aa && f.bb == bb && f.ab == ab
                        && f.a1 == a1 && f.b1 == b1 && f.c0 == c0
                });
                if !dup {
                    forms.push(KForm { aa, bb, ab, a1, b1, c0 });
                }
            }
        }
    }

    for f in &forms {
        for a in 0..=n {
            let b_max = n - a;
            for b in 0..=b_max {
                let k = f.aa * a * a + f.bb * b * b + f.ab * a * b
                    + f.a1 * a + f.b1 * b + f.c0;
                if k > 0 && k < big_n {
                    achievable[k as usize] = true;
                }
            }
        }
    }

    let mut n4: i64 = 0;
    for k in 1..big_n as usize {
        if !achievable[k] { n4 += 1; }
    }

    println!("{}", 3 * big_n - 1 - n2 + n4);
}
