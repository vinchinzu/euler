// Project Euler 272: Modular Cubes, part 2
use euler_utils::sieve;

const K: usize = 5;

fn main() {
    let big_n: i64 = 100_000_000_000;
    let small_ps: [i64; 8] = [7, 13, 19, 31, 37, 43, 61, 67];

    let mut max_p_val = big_n / 9;
    for i in 0..(K - 2) { max_p_val /= small_ps[i]; }
    let mut max_q_val = big_n / 9;
    for i in 0..(K - 1) { max_q_val /= small_ps[i]; }

    let is_p = sieve(max_p_val as usize + 10);
    let mut ps: Vec<i64> = Vec::new();
    for i in 2..=max_p_val as usize + 10 {
        if i < is_p.len() && is_p[i] && i % 3 == 1 {
            ps.push(i as i64);
        }
    }

    let max_q = max_q_val as usize;
    let max_q = if max_q < 1 { 1 } else { max_q };

    // Build prod_qs sieve
    let mut prod_qs = vec![true; max_q + 1];
    prod_qs[0] = false;
    for &p in &ps {
        if p as usize > max_q { break; }
        let mut j = p as usize;
        while j <= max_q { prod_qs[j] = false; j += p as usize; }
    }

    let build_prefix_sum = |prod_qs: &[bool]| -> Vec<i64> {
        let mut ps = vec![0i64; prod_qs.len() + 1];
        for i in 0..prod_qs.len() {
            ps[i + 1] = ps[i] + if prod_qs[i] { i as i64 } else { 0 };
        }
        ps
    };

    let mut prefix_sum = build_prefix_sum(&prod_qs);
    let mut ans: i128 = 0;

    let upper_bound = |start: usize, end: usize, val: i64| -> isize {
        let (mut lo, mut hi, mut res) = (start as isize, end as isize, start as isize - 1);
        while lo <= hi {
            let mid = (lo + hi) / 2;
            if ps[mid as usize] <= val { res = mid; lo = mid + 1; } else { hi = mid - 1; }
        }
        res
    };

    fn accumulate_last_level(
        index: usize, prod: i64, ps: &[i64], big_n: i64,
        prefix_sum: &[i64], max_q: usize, ans: &mut i128,
        upper_bound: &dyn Fn(usize, usize, i64) -> isize,
    ) {
        let max_prime = big_n / prod;
        let hi = upper_bound(index, ps.len() - 1, max_prime);
        if hi < index as isize { return; }
        let sqrt_limit = ((big_n / prod) as f64).sqrt() as i64;
        let mid = upper_bound(index, hi as usize, sqrt_limit);

        for ni in (mid + 1)..=(hi) {
            let new_prod = prod * ps[ni as usize];
            let limit = (big_n / new_prod).min(max_q as i64);
            *ans += new_prod as i128 * prefix_sum[(limit + 1) as usize] as i128;
        }
        for ni in (index as isize)..=(mid) {
            let mut new_prod = prod * ps[ni as usize];
            while new_prod <= big_n {
                let limit = (big_n / new_prod).min(max_q as i64);
                *ans += new_prod as i128 * prefix_sum[(limit + 1) as usize] as i128;
                if new_prod > big_n / ps[ni as usize] { break; }
                new_prod *= ps[ni as usize];
            }
        }
    }

    fn enumerate(
        index: usize, prod: i64, nps: usize,
        ps_slice: &[i64], big_n: i64, k: usize,
        prefix_sum: &[i64], max_q: usize, ans: &mut i128,
        upper_bound: &dyn Fn(usize, usize, i64) -> isize,
    ) {
        if nps >= k {
            let limit = (big_n / prod).min(max_q as i64);
            *ans += prod as i128 * prefix_sum[(limit + 1) as usize] as i128;
            return;
        }
        let remaining = k - nps;
        if remaining == 1 {
            accumulate_last_level(index, prod, ps_slice, big_n, prefix_sum, max_q, ans, upper_bound);
            return;
        }
        let max_start = if ps_slice.len() >= remaining { ps_slice.len() - remaining } else { return; };
        for ni in index..=max_start {
            let mut min_prod = prod;
            let mut ok = true;
            for i in 0..remaining {
                if ni + i >= ps_slice.len() { ok = false; break; }
                if min_prod > big_n / ps_slice[ni + i] { ok = false; break; }
                min_prod *= ps_slice[ni + i];
            }
            if !ok { break; }
            let mut new_prod = prod * ps_slice[ni];
            while new_prod <= big_n {
                enumerate(ni + 1, new_prod, nps + 1, ps_slice, big_n, k, prefix_sum, max_q, ans, upper_bound);
                if new_prod > big_n / ps_slice[ni] { break; }
                new_prod *= ps_slice[ni];
            }
        }
    }

    let ub = |start: usize, end: usize, val: i64| -> isize { upper_bound(start, end, val) };

    // Case 1: with factor of 9
    enumerate(0, 9, 1, &ps, big_n, K, &prefix_sum, max_q, &mut ans, &ub);

    // Case 2: without factor of 3
    let mut j = 9;
    while j <= max_q { prod_qs[j] = false; j += 9; }
    prefix_sum = build_prefix_sum(&prod_qs);

    enumerate(0, 1, 0, &ps, big_n, K, &prefix_sum, max_q, &mut ans, &ub);

    println!("{}", ans as i64);
}
