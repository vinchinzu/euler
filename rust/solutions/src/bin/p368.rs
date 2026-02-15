// Project Euler 368: Kempner-like series
const NSTATES: usize = 20;

fn enc(d: usize, c: usize) -> usize { d * 2 + c - 1 }
fn dig(s: usize) -> usize { s / 2 }
fn con(s: usize) -> usize { s % 2 + 1 }

fn trans(s: usize, nd: usize) -> Option<usize> {
    let d = dig(s);
    let c = con(s);
    if nd == d {
        if c + 1 > 2 { return None; }
        Some(enc(nd, c + 1))
    } else {
        Some(enc(nd, 1))
    }
}

fn gauss(a: &mut [[f64; NSTATES + 1]; NSTATES], x: &mut [f64; NSTATES]) {
    let n = NSTATES;
    for col in 0..n {
        let mut pivot = col;
        for row in (col + 1)..n {
            if a[row][col].abs() > a[pivot][col].abs() { pivot = row; }
        }
        for j in 0..=n { let tmp = a[col][j]; a[col][j] = a[pivot][j]; a[pivot][j] = tmp; }
        for row in (col + 1)..n {
            let f = a[row][col] / a[col][col];
            for j in col..=n { a[row][j] -= f * a[col][j]; }
        }
    }
    for row in (0..n).rev() {
        let mut s = a[row][n];
        for j in (row + 1)..n { s -= a[row][j] * x[j]; }
        x[row] = s / a[row][row];
    }
}

fn main() {
    let prefix_len = 7;

    // Compute T0..T4
    let mut t = [[0.0f64; NSTATES]; 5];

    // T0
    {
        let mut a = [[0.0f64; NSTATES + 1]; NSTATES];
        for s in 0..NSTATES {
            a[s][s] = 1.0;
            let mut nv = 0;
            for nd in 0..10 {
                if let Some(tt) = trans(s, nd) { a[s][tt] -= 0.1; nv += 1; }
            }
            a[s][NSTATES] = nv as f64 / 10.0;
        }
        gauss(&mut a, &mut t[0]);
    }

    // T1
    {
        let mut a = [[0.0f64; NSTATES + 1]; NSTATES];
        for s in 0..NSTATES {
            a[s][s] = 1.0;
            let mut rhs = 0.0;
            for nd in 0..10usize {
                if let Some(tt) = trans(s, nd) {
                    a[s][tt] -= 0.01;
                    rhs += nd as f64 * (1.0 + t[0][tt]);
                }
            }
            a[s][NSTATES] = rhs / 100.0;
        }
        gauss(&mut a, &mut t[1]);
    }

    // T2
    {
        let mut a = [[0.0f64; NSTATES + 1]; NSTATES];
        for s in 0..NSTATES {
            a[s][s] = 1.0;
            let mut rhs = 0.0;
            for nd in 0..10usize {
                if let Some(tt) = trans(s, nd) {
                    a[s][tt] -= 0.001;
                    rhs += (nd * nd) as f64 * (1.0 + t[0][tt]) + 2.0 * nd as f64 * t[1][tt];
                }
            }
            a[s][NSTATES] = rhs / 1000.0;
        }
        gauss(&mut a, &mut t[2]);
    }

    // T3
    {
        let mut a = [[0.0f64; NSTATES + 1]; NSTATES];
        for s in 0..NSTATES {
            a[s][s] = 1.0;
            let mut rhs = 0.0;
            for nd in 0..10usize {
                if let Some(tt) = trans(s, nd) {
                    let d = nd as f64;
                    a[s][tt] -= 1e-4;
                    rhs += d * d * d * (1.0 + t[0][tt]) + 3.0 * d * d * t[1][tt] + 3.0 * d * t[2][tt];
                }
            }
            a[s][NSTATES] = rhs * 1e-4;
        }
        gauss(&mut a, &mut t[3]);
    }

    // T4
    {
        let mut a = [[0.0f64; NSTATES + 1]; NSTATES];
        for s in 0..NSTATES {
            a[s][s] = 1.0;
            let mut rhs = 0.0;
            for nd in 0..10usize {
                if let Some(tt) = trans(s, nd) {
                    let d = nd as f64;
                    a[s][tt] -= 1e-5;
                    rhs += d.powi(4) * (1.0 + t[0][tt]) + 4.0 * d.powi(3) * t[1][tt]
                        + 6.0 * d * d * t[2][tt] + 4.0 * d * t[3][tt];
                }
            }
            a[s][NSTATES] = rhs * 1e-5;
        }
        gauss(&mut a, &mut t[4]);
    }

    let mut direct_sum: f64 = 0.0;
    let mut tail_sum: f64 = 0.0;

    fn enumerate_short(prefix: i64, state: i32, digits_left: i32, direct_sum: &mut f64) {
        if digits_left == 0 {
            if prefix > 0 { *direct_sum += 1.0 / prefix as f64; }
            return;
        }
        for nd in 0..10usize {
            if prefix == 0 && nd == 0 { continue; }
            let ns = if state < 0 { enc(nd, 1) as i32 } else {
                match trans(state as usize, nd) { Some(v) => v as i32, None => continue }
            };
            enumerate_short(prefix * 10 + nd as i64, ns, digits_left - 1, direct_sum);
        }
    }

    fn enumerate_with_tail(prefix: i64, state: i32, digits_left: i32,
                           direct_sum: &mut f64, tail_sum: &mut f64, t: &[[f64; NSTATES]; 5]) {
        if digits_left == 0 {
            if prefix <= 0 { return; }
            *direct_sum += 1.0 / prefix as f64;
            let p = prefix as f64;
            let inv_p = 1.0 / p;
            let s = state as usize;
            let tail = t[0][s] * inv_p
                - t[1][s] * inv_p * inv_p
                + t[2][s] * inv_p.powi(3)
                - t[3][s] * inv_p.powi(4)
                + t[4][s] * inv_p.powi(5);
            *tail_sum += tail;
            return;
        }
        for nd in 0..10usize {
            if prefix == 0 && nd == 0 { continue; }
            let ns = if state < 0 { enc(nd, 1) as i32 } else {
                match trans(state as usize, nd) { Some(v) => v as i32, None => continue }
            };
            enumerate_with_tail(prefix * 10 + nd as i64, ns, digits_left - 1, direct_sum, tail_sum, t);
        }
    }

    // Exact sum for 1 to prefix_len-1 digit numbers
    for d in 1..prefix_len {
        enumerate_short(0, -1, d, &mut direct_sum);
    }

    // prefix_len digit numbers: direct + tail
    enumerate_with_tail(0, -1, prefix_len, &mut direct_sum, &mut tail_sum, &t);

    let total = direct_sum + tail_sum;
    println!("{:.10}", total);
}
