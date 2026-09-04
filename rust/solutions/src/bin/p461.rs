use rayon::prelude::*;

fn main() {
    const N: usize = 10000;
    let pi = std::f64::consts::PI;

    let mut f = vec![0.0f64; 2 * N + 1];
    for i in 0..=2 * N {
        f[i] = (i as f64 / N as f64).exp() - 1.0;
    }

    let mut k_lim = 1;
    while f[k_lim] < pi {
        k_lim += 1;
    }

    let counts: Vec<usize> = (1..k_lim)
        .into_par_iter()
        .map(|k1| {
            let target = pi - f[k1];
            f[k1..k_lim].partition_point(|&x| x < target)
        })
        .collect();

    let mut offsets = Vec::with_capacity(k_lim);
    offsets.push(0);
    for &c in &counts {
        offsets.push(offsets.last().unwrap() + c);
    }
    let total_count = *offsets.last().unwrap();

    let mut pairs = vec![0.0f64; total_count];
    let pairs_ptr = pairs.as_mut_ptr() as usize;

    (1..k_lim).into_par_iter().for_each(|k1| {
        let fk1 = f[k1];
        let off = offsets[k1 - 1];
        let len = counts[k1 - 1];
        let ptr = unsafe { (pairs_ptr as *mut f64).add(off) };
        for i in 0..len {
            unsafe {
                *ptr.add(i) = fk1 + *f.get_unchecked(k1 + i);
            }
        }
    });

    pairs.par_sort_unstable_by(|a, b| a.total_cmp(b));

    let mut left = 0usize;
    let mut right = pairs.len() - 1;
    let mut min_error = f64::INFINITY;
    let mut best_s1 = 0.0;
    let mut best_s2 = 0.0;

    while left <= right {
        let s1 = pairs[left];
        let s2 = pairs[right];
        let error = s1 + s2 - pi;
        if error.abs() < min_error {
            min_error = error.abs();
            best_s1 = s1;
            best_s2 = s2;
        }
        if error < 0.0 {
            left += 1;
        } else if right == 0 {
            break;
        } else {
            right -= 1;
        }
    }

    let reconstruct = |target: f64| -> (usize, usize) {
        let mut l = 1;
        let mut r = k_lim - 1;
        let mut best_err = f64::INFINITY;
        let mut best_pair = (0, 0);
        while l <= r {
            let s = f[l] + f[r];
            let err = (s - target).abs();
            if err < best_err {
                best_err = err;
                best_pair = (l, r);
            }
            if s < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
        best_pair
    };

    let (a1, b1) = reconstruct(best_s1);
    let (a2, b2) = reconstruct(best_s2);

    let ans = a1 as u64 * a1 as u64
        + b1 as u64 * b1 as u64
        + a2 as u64 * a2 as u64
        + b2 as u64 * b2 as u64;
    println!("{}", ans);
}
