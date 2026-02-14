// Project Euler Problem 101: Optimum Polynomial
// u(n) = 1 - n + n^2 - n^3 + ... + n^10
// Lagrange interpolation to find first incorrect term (FIT).

fn u(n: i64) -> i64 {
    let mut result: i64 = 0;
    let mut power: i64 = 1;
    let neg_n = -n;
    for _ in 0..=10 {
        result += power;
        power *= neg_n;
    }
    result
}

fn op(k: usize, n_val: i64, seq: &[i64]) -> i64 {
    let mut total: f64 = 0.0;
    for j in 0..k {
        let xj = (j + 1) as f64;
        let mut num = 1.0f64;
        let mut den = 1.0f64;
        for i in 0..k {
            if i == j {
                continue;
            }
            let xi = (i + 1) as f64;
            num *= n_val as f64 - xi;
            den *= xj - xi;
        }
        total += seq[j] as f64 * (num / den);
    }
    if total > 0.0 {
        (total + 0.5) as i64
    } else {
        (total - 0.5) as i64
    }
}

fn main() {
    let mut seq = [0i64; 11];
    let mut sum_fits: i64 = 0;

    for k in 1..=10 {
        seq[k - 1] = u(k as i64);
        let predicted = op(k, (k + 1) as i64, &seq);
        let actual = u((k + 1) as i64);
        if predicted != actual {
            sum_fits += predicted;
        }
    }

    println!("{}", sum_fits);
}
