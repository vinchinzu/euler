// Project Euler 470: Super Ramvok
//
// Game theory problem with subset enumeration and tridiagonal system solving.

const N: usize = 20;

fn ilog2(mut n: usize) -> usize {
    let mut r = 0;
    while n > 1 {
        n >>= 1;
        r += 1;
    }
    r
}

fn popcount(x: usize) -> usize {
    x.count_ones() as usize
}

fn r_func(subset: usize, c: f64) -> f64 {
    if c == 0.0 {
        return 1.0 + ilog2(subset) as f64;
    }

    let cnt = popcount(subset);
    let mut vals = Vec::with_capacity(cnt);
    for i in 0..N {
        if (subset & (1 << i)) != 0 {
            vals.push((i + 1) as f64);
        }
    }

    let mut best_expected_earning = 0.0;
    for t in 1.. {
        let mean: f64 = vals.iter().sum::<f64>() / cnt as f64;
        let expected_earning = mean - c * t as f64;
        if expected_earning < best_expected_earning {
            return best_expected_earning;
        }
        best_expected_earning = expected_earning;

        for v in vals.iter_mut() {
            if *v < mean {
                *v = mean;
            }
        }
    }
    unreachable!()
}

fn tridiagonal_system(a: &[f64], b: &[f64], c: &[f64], d: &[f64], n: usize) -> Vec<f64> {
    let mut c_prime = vec![0.0; n];
    let mut d_prime = vec![0.0; n];

    c_prime[0] = c[0] / b[0];
    d_prime[0] = d[0] / b[0];

    for i in 1..n {
        let denom = b[i] - a[i] * c_prime[i - 1];
        c_prime[i] = if i < n - 1 { c[i] / denom } else { 0.0 };
        d_prime[i] = (d[i] - a[i] * d_prime[i - 1]) / denom;
    }

    let mut x = vec![0.0; n];
    x[n - 1] = d_prime[n - 1];
    for i in (0..n - 1).rev() {
        x[i] = d_prime[i] - c_prime[i] * x[i + 1];
    }
    x
}

fn main() {
    let mut ans = 0.0;
    let mut r_cache = vec![0.0; 1 << N];

    for c_val in 0..=N {
        for subset in 1..(1 << N) {
            r_cache[subset] = r_func(subset, c_val as f64);
        }

        for d in 4..=N {
            let size = d + 1;
            let mut a = vec![0.0; size];
            let mut b = vec![0.0; size];
            let mut c_arr = vec![0.0; size];
            let mut d_arr = vec![0.0; size];

            for i in 1..=d {
                a[i] = -((d - i + 1) as f64) / d as f64;
            }
            for i in 0..=d {
                b[i] = 1.0;
            }
            for i in 1..d {
                c_arr[i] = -((i + 1) as f64) / d as f64;
            }
            for subset in 1..(1usize << d) {
                d_arr[popcount(subset)] += r_cache[subset];
            }

            let x = tridiagonal_system(&a, &b, &c_arr, &d_arr, size);
            ans += x[d];
        }
    }

    println!("{}", ans.round() as i64);
}
