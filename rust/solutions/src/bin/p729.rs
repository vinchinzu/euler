// Project Euler 729 - Range of Periodic Sequence
//
// Lyndon word enumeration with fixed-point iteration.

const MAXN: usize = 25;

static mut ANS: f64 = 0.0;

fn process_lyndon(w: &[i32]) {
    let len = w.len();
    if len < 2 {
        return;
    }

    let mut word: u32 = 0;
    for i in 0..len {
        if w[i] != 0 {
            word |= 1 << i;
        }
    }

    // Find fixed point by iterating the composed map
    let mut d: f64 = 1.0;
    for _ in 0..300 {
        let prev = d;
        for i in 0..len {
            let bit = (word >> i) & 1;
            let sign: f64 = if bit == 0 { 1.0 } else { -1.0 };
            d = (d + sign * (d * d + 4.0).sqrt()) / 2.0;
        }
        if (d - prev).abs() < 1e-13 {
            break;
        }
    }

    // Compute range
    let mut min_val = d;
    let mut max_val = d;
    for i in 0..len {
        let bit = (word >> i) & 1;
        let sign: f64 = if bit == 0 { 1.0 } else { -1.0 };
        d = (d + sign * (d * d + 4.0).sqrt()) / 2.0;
        if d < min_val {
            min_val = d;
        }
        if d > max_val {
            max_val = d;
        }
    }

    unsafe {
        ANS += len as f64 * (max_val - min_val);
    }
}

fn generate(w_buf: &mut [i32], t: usize, p: usize, n: usize) {
    if t > n {
        if p == n {
            process_lyndon(&w_buf[1..=n]);
        }
    } else {
        w_buf[t] = w_buf[t - p];
        generate(w_buf, t + 1, p, n);
        for j in (w_buf[t - p] + 1)..=1 {
            w_buf[t] = j;
            generate(w_buf, t + 1, t, n);
        }
    }
}

fn main() {
    let mut w_buf = [0i32; MAXN + 2];

    for n in 2..=MAXN {
        w_buf.fill(0);
        generate(&mut w_buf, 1, 1, n);
    }

    println!("{:.4}", unsafe { ANS });
}
