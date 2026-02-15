// Project Euler 438: Integer part of polynomial equation's solutions

const N: usize = 7;

fn main() {
    let mut ineq = [[[0.0f64; N + 1]; N + 1]; N + 1];

    let mut eps = 1.0f64;
    for i in 1..=N { eps /= i as f64; }

    for j in 0..=N {
        let x = (N + 1 - j) as f64 - eps;
        for k in 0..=N {
            ineq[N][j][k] = x.powi((N - k) as i32);
        }
    }

    for i in (1..N).rev() {
        for j in 0..=i {
            for k in 0..=i {
                ineq[i][j][k] = ineq[i + 1][j][k] - ineq[i + 1][j + 1][k];
            }
        }
    }

    let mut ans: i64 = 0;
    let mut t = [0i32; N];

    fn helper(
        t: &mut [i32; N], tlen: usize,
        ineq: &[[[f64; N + 1]; N + 1]; N + 1],
        ans: &mut i64,
    ) {
        let index = tlen + 1;

        let mut lower_bound = f64::NEG_INFINITY;
        let mut upper_bound = f64::INFINITY;

        for j in 0..=index {
            let mut goal = -ineq[index][j][0];
            for k in 0..index - 1 {
                goal -= ineq[index][j][k + 1] * t[k] as f64;
            }
            goal /= ineq[index][j][index];

            if j % 2 == 0 {
                let c = goal.ceil();
                if c > lower_bound { lower_bound = c; }
            } else {
                let f = goal.floor();
                if f < upper_bound { upper_bound = f; }
            }
        }

        let lb = lower_bound as i64;
        let ub = upper_bound as i64;

        if index == N {
            let num_terms = ub - lb + 1;
            if num_terms > 0 {
                let mut sum_prefix: i64 = 0;
                for k in 0..tlen {
                    sum_prefix += (t[k] as i64).abs();
                }
                let sum_abs = if lb >= 0 {
                    (ub * (ub + 1) - lb * (lb - 1)) / 2
                } else if ub <= 0 {
                    let nlb = -lb;
                    let nub = -ub;
                    (nlb * (nlb + 1) - nub * (nub - 1)) / 2
                } else {
                    let nlb = -lb;
                    nlb * (nlb + 1) / 2 + ub * (ub + 1) / 2
                };
                *ans += num_terms * sum_prefix + sum_abs;
            }
            return;
        }

        for a_r in lb..=ub {
            t[tlen] = a_r as i32;
            helper(t, tlen + 1, ineq, ans);
        }
    }

    helper(&mut t, 0, &ineq, &mut ans);
    println!("{ans}");
}
