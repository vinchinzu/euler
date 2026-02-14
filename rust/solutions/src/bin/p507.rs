// Project Euler 507 - Shortest Lattice Vector
// L1 norm Gauss reduction on 3D lattice vectors defined by tribonacci-like sequence.

const NN: usize = 20_000_000;
const M: i64 = 10_000_000;

fn l1(v1: i64, v2: i64, v3: i64) -> i64 {
    v1.abs() + v2.abs() + v3.abs()
}

fn gauss(u1: i64, u2: i64, u3: i64, v1: i64, v2: i64, v3: i64) -> i64 {
    let u_l1 = l1(u1, u2, u3);
    let components = [
        (u1, v1), (u2, v2), (u3, v3),
    ];
    let mut candidates = Vec::with_capacity(6);
    for &(u, v) in &components {
        if u != 0 {
            candidates.push((v as f64 / u as f64).floor() as i64);
            candidates.push((v as f64 / u as f64).ceil() as i64);
        } else {
            candidates.push(0);
            candidates.push(0);
        }
    }

    let mut min_w = (0i64, 0i64, 0i64);
    let mut min_d = i64::MAX;

    for &m in &candidates {
        if u_l1 > 0 && m.unsigned_abs() < u64::MAX / std::cmp::max(u_l1 as u64, 1) {
            let w1 = v1 - m * u1;
            let w2 = v2 - m * u2;
            let w3 = v3 - m * u3;
            let d = l1(w1, w2, w3);
            if d < min_d {
                min_w = (w1, w2, w3);
                min_d = d;
            }
        }
    }

    if l1(min_w.0, min_w.1, min_w.2) < u_l1 {
        gauss(min_w.0, min_w.1, min_w.2, u1, u2, u3)
    } else {
        u_l1
    }
}

fn main() {
    let mut r = vec![0i64; 12 * NN + 1];
    r[2] = 1;
    for n in 3..=12 * NN {
        r[n] = (r[n - 1] + r[n - 2] + r[n - 3]) % M;
    }

    let mut ans: i64 = 0;
    for n in 1..=NN {
        let v1 = r[12 * n - 11] - r[12 * n - 10];
        let v2 = r[12 * n - 9] + r[12 * n - 8];
        let v3 = r[12 * n - 7] * r[12 * n - 6];
        let w1 = r[12 * n - 5] - r[12 * n - 4];
        let w2 = r[12 * n - 3] + r[12 * n - 2];
        let w3 = r[12 * n - 1] * r[12 * n];

        if l1(v1, v2, v3) < l1(w1, w2, w3) {
            ans += gauss(v1, v2, v3, w1, w2, w3);
        } else {
            ans += gauss(w1, w2, w3, v1, v2, v3);
        }
    }

    println!("{}", ans);
}
