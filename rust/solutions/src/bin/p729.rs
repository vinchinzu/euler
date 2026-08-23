// Project Euler 729 - Range of Periodic Sequence
//
// Binary Lyndon words (FKM); Newton for the composed inverse fixed point.

const MAXN: usize = 25;
const JOIN_REMAIN: usize = 12;
const NEWTON_ITERS: usize = 8;

#[inline(always)]
fn process_lyndon(word: u32, len: usize) -> f64 {
    let mut x = 0.0f64;
    for _ in 0..NEWTON_ITERS {
        let mut y = x;
        let mut dy = 1.0f64;
        let mut w = word;
        for _ in 0..len {
            let s = y.mul_add(y, 4.0).sqrt();
            if w & 1 == 0 {
                dy *= 0.5 * (1.0 + y / s);
                y = 0.5 * (y + s);
            } else {
                dy *= 0.5 * (1.0 - y / s);
                y = 0.5 * (y - s);
            }
            w >>= 1;
        }
        let denom = dy - 1.0;
        let next = if denom.abs() < 1e-18 {
            y
        } else {
            x - (y - x) / denom
        };
        if (next - x).abs() <= 1e-15 * (1.0 + next.abs()) {
            x = next;
            break;
        }
        x = next;
    }

    let mut y = x;
    let mut mn = y;
    let mut mx = y;
    let mut w = word;
    for _ in 1..len {
        let s = y.mul_add(y, 4.0).sqrt();
        y = if w & 1 == 0 {
            0.5 * (y + s)
        } else {
            0.5 * (y - s)
        };
        if y < mn {
            mn = y;
        }
        if y > mx {
            mx = y;
        }
        w >>= 1;
    }
    len as f64 * (mx - mn)
}

fn generate(word: u32, t: usize, p: usize, n: usize) -> f64 {
    if t > n {
        return if p == n {
            process_lyndon(word, n)
        } else {
            0.0
        };
    }

    // Position t uses bit t-1; FKM sentinel w[0] is 0 (only when t == p).
    let src = if t == p {
        0
    } else {
        (word >> (t - p - 1)) & 1
    };
    let w_copy = word | (src << (t - 1));
    if src == 0 {
        let w_inc = word | (1 << (t - 1));
        if n + 1 - t >= JOIN_REMAIN {
            let (a, b) = rayon::join(
                || generate(w_copy, t + 1, p, n),
                || generate(w_inc, t + 1, t, n),
            );
            a + b
        } else {
            generate(w_copy, t + 1, p, n) + generate(w_inc, t + 1, t, n)
        }
    } else {
        generate(w_copy, t + 1, p, n)
    }
}

fn main() {
    let mut ans = 0.0;
    for n in 2..=MAXN {
        ans += generate(0, 1, 1, n);
    }
    println!("{:.4}", ans);
}
