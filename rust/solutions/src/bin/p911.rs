// Problem 911 - Khinchin Exceptions
//
// Compute the geometric mean of k_infinity(rho_n) for 0 <= n <= 50,
// where rho_n = sum_{i>=0} 2^n / 2^{2^i}.
//
// Uses Shallit's continued fraction recurrences (Theorems 1 and 11)
// to build CF expansions and compute the limiting geometric mean of
// the partial quotients.

fn seed_rho(n: u32) -> (Vec<u64>, bool, u64) {
    // Returns (coeffs, is_theorem11, t2)
    if n == 0 {
        return (vec![0, 1, 3], false, 0);
    }

    // Theorem 11 parameters (u=2, t=n)
    let mut vprime: u32 = 0;
    while (1u64 << vprime) <= n as u64 {
        vprime += 1;
    }

    let d = (1u64 << vprime) - n as u64;
    let ut = 1u64 << n;
    let ud = 1u64 << d;

    // c = sum_{k=0..vprime-1} 2^{n - 2^k}
    let mut c: u64 = 0;
    for k in 0..vprime {
        c += 1u64 << (n as u64 - (1u64 << k));
    }

    let coeffs = vec![c, ud - 1, 1, ut - 1, ud];
    (coeffs, true, ut - 1)
}

fn extend_theorem11(coeffs: &mut Vec<u64>, t2: u64) {
    let len = coeffs.len();
    let last = coeffs[len - 1];

    if last > 1 {
        // Append: t2, 1, last-1, then mirror of coeffs[len-2..1] reversed
        coeffs.push(t2);
        coeffs.push(1);
        coeffs.push(last - 1);
        // tail_rev = coeffs[len-2], coeffs[len-3], ..., coeffs[1]
        // But we already modified coeffs by pushing 3 elements, so original indices
        // are still valid at positions 1..len-1
        let mirror: Vec<u64> = (1..len - 1).rev().map(|i| coeffs[i]).collect();
        coeffs.extend_from_slice(&mirror);
    } else {
        // last == 1: eliminate the 0
        let prelast = coeffs[len - 2];
        coeffs.push(t2);
        coeffs.push(1 + prelast);
        // tail_rev_rest = coeffs[len-3], ..., coeffs[1]
        let mirror: Vec<u64> = (1..len - 2).rev().map(|i| coeffs[i]).collect();
        coeffs.extend_from_slice(&mirror);
    }
}

fn extend_theorem1(coeffs: &mut Vec<u64>) {
    let len = coeffs.len();
    let last = coeffs[len - 1];

    if last > 1 {
        // Replace last with: last+1, last-1, then mirror of coeffs[len-2..1]
        coeffs[len - 1] = last + 1;
        coeffs.push(last - 1);
        let mirror: Vec<u64> = (1..len - 1).rev().map(|i| coeffs[i]).collect();
        coeffs.extend_from_slice(&mirror);
    } else {
        // last == 1: eliminate 0
        let prelast = coeffs[len - 2];
        // Replace [prelast, 1] with [2 + prelast], then mirror
        coeffs[len - 2] = 2 + prelast;
        coeffs.pop(); // remove last (which was 1)
        let new_len = coeffs.len();
        let mirror: Vec<u64> = (1..new_len - 1).rev().map(|i| coeffs[i]).collect();
        coeffs.extend_from_slice(&mirror);
    }
}

fn canonicalize_tail(coeffs: &mut Vec<u64>) {
    let len = coeffs.len();
    if len > 1 && coeffs[len - 1] == 1 {
        coeffs[len - 2] += 1;
        coeffs.pop();
    }
}

fn rho_cf_prefix(n: u32, count: usize) -> Vec<u64> {
    let (mut coeffs, is_t11, t2) = seed_rho(n);

    if is_t11 {
        while coeffs.len() <= count {
            extend_theorem11(&mut coeffs, t2);
        }
        coeffs.truncate(count + 1);
        return coeffs;
    }

    // Theorem 1 for n=0
    let mut v: u32 = 1;
    canonicalize_tail(&mut coeffs);
    while (1u64 << v) < (count as u64 + 1) {
        extend_theorem1(&mut coeffs);
        canonicalize_tail(&mut coeffs);
        v += 1;
    }
    coeffs.truncate(count + 1);
    coeffs
}

fn avg_log_positive_ints(arr: &[u64]) -> f64 {
    let mut s: f64 = 0.0;
    for &a in arr {
        s += (a as f64).ln();
    }
    s / arr.len() as f64
}

fn log_khinchin_rho0() -> f64 {
    // Compute ln(k_infinity(rho_0)) for rho_0 = B(2,infinity)
    // Using Richardson extrapolation with two large prefix lengths.
    let (mut coeffs, _, _) = seed_rho(0);
    canonicalize_tail(&mut coeffs);

    let mut mu1: f64 = 0.0;
    let mut l1: usize = 0;
    let mut mu2: f64 = 0.0;
    let mut l2: usize = 0;

    for step in 1..=22 {
        extend_theorem1(&mut coeffs);
        canonicalize_tail(&mut coeffs);

        if step == 20 {
            let tail = &coeffs[1..];
            l1 = tail.len();
            mu1 = avg_log_positive_ints(tail);
        }

        if step == 22 {
            let tail = &coeffs[1..];
            l2 = tail.len();
            mu2 = avg_log_positive_ints(tail);
        }
    }

    assert!(l1 > 0 && l2 > l1, "unexpected checkpoint lengths");

    // Extrapolate: mu(L) = mu_inf + c/L
    (mu2 * l2 as f64 - mu1 * l1 as f64) / (l2 as f64 - l1 as f64)
}

fn khinchin_log_limit(n: u32, max_steps: usize) -> f64 {
    if n == 0 {
        return log_khinchin_rho0();
    }

    let (coeffs, _, t2) = seed_rho(n);
    let tail = &coeffs[1..];
    let mut l = tail.len() as u64;

    let mut mu = avg_log_positive_ints(tail);

    let first = tail[0];
    let second = tail[1];
    let mut last = tail[tail.len() - 1];
    let mut prelast = tail[tail.len() - 2];

    for _ in 0..max_steps {
        let mu_old = mu;
        if last > 1 {
            let l_new = 2 * l + 2;
            let delta = -(last as f64).ln() + (t2 as f64).ln() + ((last - 1) as f64).ln();
            mu = mu * ((2 * l) as f64 / l_new as f64) + delta / l_new as f64;
            l = l_new;
        } else {
            // last == 1
            let l_new = 2 * l;
            let delta =
                -(prelast as f64).ln() + (t2 as f64).ln() + ((prelast + 1) as f64).ln();
            mu = mu + delta / l_new as f64;
            l = l_new;
        }

        prelast = second;
        last = first;

        if (mu - mu_old).abs() < 1e-15 {
            break;
        }
    }

    mu
}

fn main() {
    // Verify against problem statement examples
    let cf = rho_cf_prefix(2, 7);
    assert_eq!(cf, vec![3, 3, 1, 3, 4, 3, 1, 3]);

    let k2 = (khinchin_log_limit(2, 80)).exp();
    assert!(
        (k2 - 2.059767).abs() < 0.000001,
        "k_infty(rho_2) = {}, expected ~2.059767",
        k2
    );

    // Geometric mean of k_infty(rho_n) for 0 <= n <= 50
    let mut total_log: f64 = 0.0;
    for n in 0..=50 {
        total_log += khinchin_log_limit(n, 80);
    }
    let avg_log = total_log / 51.0;
    let ans = avg_log.exp();

    println!("{:.6}", ans);
}
