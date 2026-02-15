// Project Euler 771 - Increasing Sequences
// Count strictly increasing sequences with |x_i^2 - x_{i-1}*x_{i+1}| <= 2.

fn main() {
    const NN: i64 = 1_000_000_000_000_000_000;
    const M: i64 = 1_000_000_007;

    let phi_limit = (NN as f64).powf(0.25) as usize + 1;

    // Compute Euler's totient
    let mut phi = vec![0i32; phi_limit + 1];
    for i in 0..=phi_limit { phi[i] = i as i32; }
    for i in 2..=phi_limit {
        if phi[i] == i as i32 {
            for j in (i..=phi_limit).step_by(i) {
                phi[j] -= phi[j] / i as i32;
            }
        }
    }

    fn tr(n: i64) -> i64 {
        let a = n % M;
        let b = (n + 1) % M;
        (a as i128 * b as i128 % M as i128 * 500000004i128 % M as i128) as i64
    }

    fn pow_int(base: i64, exponent: i32) -> i64 {
        let mut result: i64 = 1;
        for _ in 0..exponent {
            if result > NN / base { return NN + 1; }
            result *= base;
        }
        result
    }

    let mut ans_total: i64 = 0;

    let process_seq_arr = |seq: &[i64]| -> i64 {
        let mut count = 0i64;
        for start in 0..seq.len() {
            for end in (start + 5)..=seq.len() {
                if seq[end - 1] <= NN { count += 1; }
            }
        }
        count
    };

    let process_recursive_seq = |x0: i64, x1: i64, a: i64, b: i64| -> i64 {
        let mut seq = vec![x0, x1];
        loop {
            let prev2 = seq[seq.len() - 2];
            let prev1 = seq[seq.len() - 1];
            let next_d = a as f64 * prev2 as f64 + b as f64 * prev1 as f64;
            if next_d > NN as f64 || next_d < 0.0 { break; }
            let next = a * prev2 + b * prev1;
            if next <= 0 || next > NN { break; }
            seq.push(next);
            if seq.len() >= 199 { break; }
        }
        process_seq_arr(&seq)
    };

    ans_total += process_seq_arr(&[1, 2, 3, 4, 6, 9]);
    ans_total += process_seq_arr(&[1, 2, 3, 5, 9, 16]);
    ans_total += process_seq_arr(&[1, 2, 4, 7, 12]);
    ans_total += process_seq_arr(&[1, 2, 4, 9, 20]);
    ans_total += process_seq_arr(&[1, 2, 6, 17, 48]);
    ans_total += process_seq_arr(&[1, 2, 6, 19, 60]);

    ans_total += process_recursive_seq(1, 2, 1, 1);
    ans_total += process_recursive_seq(1, 2, 1, 2);
    ans_total += process_recursive_seq(1, 2, -1, 3);
    ans_total += process_recursive_seq(1, 3, 1, 2);
    ans_total += process_recursive_seq(1, 3, -1, 4);

    let mut x1: i64 = 3;
    while pow_int(x1 - 1, 4) <= NN {
        ans_total += process_recursive_seq(1, x1, -1, x1);
        ans_total += process_recursive_seq(1, x1, 1, x1);
        x1 += 1;
    }

    {
        let mut v: i64 = 2;
        while 27 * v <= NN {
            ans_total += 1;
            v *= 3;
        }
    }

    for e in 4.. {
        if pow_int(2, e) > NN { break; }
        let mut x1: i64 = 2;
        while pow_int(x1, e) <= NN {
            let powe = pow_int(x1, e);
            ans_total += ((NN / powe) % M) * phi[x1 as usize] as i64;
            x1 += 1;
        }
    }

    let mut ans = tr(NN - 4);
    ans = (ans + ans_total % M) % M;
    println!("{}", ans);
}

const NN: i64 = 1_000_000_000_000_000_000;
const M: i64 = 1_000_000_007;
