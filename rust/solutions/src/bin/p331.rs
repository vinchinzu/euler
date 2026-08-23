// Project Euler 331: Cross flips
//
// T(n) for even n from an octant walk of the annulus
// (n-1)^2 <= x^2 + y^2 < n^2. Incremental squares; y-chunks via isqrt restart.

use rayon::prelude::*;

fn start_state(n: u64, y: u64) -> (u64, bool) {
    let n2 = n * n;
    let nm1_2 = (n - 1) * (n - 1);
    if y == n - 1 {
        return (0, false);
    }
    let yp = y + 1;
    let rem = n2 - yp * yp;
    let x = if rem > 1 { (rem - 1).isqrt() } else { 0 };
    let lb = x * x + y * y >= nm1_2;
    (x, lb)
}

fn walk(
    n: u64,
    mut y: u64,
    mut x: u64,
    mut left_border: bool,
    y_last: u64,
    handle_diag: bool,
) -> (i64, i64) {
    let n2 = n * n;
    let nm1_2 = (n - 1) * (n - 1);
    let mut x2 = x * x;
    let mut y2 = y * y;
    let mut num_odd_rows = 0i64;
    let mut correction = 0i64;

    loop {
        if !handle_diag && y < y_last {
            break;
        }

        if x2 + y2 < nm1_2 {
            x2 += (x << 1) + 1;
            x += 1;
        }

        let prev_x = x;
        while x2 + (x << 1) + 1 + y2 < n2 {
            x2 += (x << 1) + 1;
            x += 1;
        }

        y2 -= (y << 1) - 1;
        y -= 1;

        let right_border = x2 + y2 >= nm1_2;
        let dx = (x - prev_x) as i64;
        let odd = (dx + 1) & 1;
        let l = i64::from(left_border);
        let r = i64::from(right_border);
        num_odd_rows += dx + 1 - l - r + odd;
        correction += (dx + 1 - ((l + r) << 1)) * ((odd << 2) - 2);

        if handle_diag && y <= x {
            if y == x {
                if x2 + y2 >= nm1_2 {
                    correction += 1;
                }
            } else {
                correction -= 1;
                if l == 0 && r == 0 {
                    num_odd_rows -= 1;
                }
            }
            break;
        }

        if y == 0 {
            break;
        }
        left_border = right_border;
    }

    (num_odd_rows, correction)
}

fn t_seq(n: u64) -> i64 {
    let (a, c) = walk(n, n - 1, 0, false, 0, true);
    2 * a * (n as i64 - a) + c
}

fn t(n: i64) -> i64 {
    let n = n as u64;
    if n < 2_000_000 {
        return t_seq(n);
    }

    let n2 = n * n;
    let y_hi = n - 1;
    let y_diag = (n2 / 2).isqrt();
    let y_cut = (y_diag + 32).min(y_hi);
    let y_par_lo = y_cut + 1;

    if y_par_lo > y_hi {
        return t_seq(n);
    }

    let total = y_hi - y_par_lo + 1;
    let nchunks = (rayon::current_num_threads() * 4).max(8) as u64;
    let nchunks = nchunks.min(total).max(1);

    let (a_par, c_par) = (0..nchunks)
        .into_par_iter()
        .map(|i| {
            let start_off = total * i / nchunks;
            let end_off = total * (i + 1) / nchunks;
            if end_off <= start_off {
                return (0, 0);
            }
            let y_start = y_hi - start_off;
            let y_last = y_hi - end_off + 1;
            let (x, lb) = start_state(n, y_start);
            walk(n, y_start, x, lb, y_last, false)
        })
        .reduce(|| (0i64, 0i64), |p, q| (p.0 + q.0, p.1 + q.1));

    let (x, lb) = start_state(n, y_cut);
    let (a_tail, c_tail) = walk(n, y_cut, x, lb, 0, true);
    let a = a_par + a_tail;
    let c = c_par + c_tail;
    2 * a * (n as i64 - a) + c
}

fn main() {
    let mut ans: i64 = 0;

    for i in 3..=31 {
        let n = (1i64 << i) - i;
        if n == 5 {
            ans += 3;
        } else if n % 2 == 0 {
            ans += t(n);
        }
    }

    println!("{}", ans);
}
