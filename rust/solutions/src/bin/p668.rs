// Project Euler 668 - Square Root Smooth Numbers
// Lucy DP for prime counting, then count n <= N with largest prime factor <= sqrt(n).

fn main() {
    let n: i64 = 10_000_000_000;
    let l = {
        let mut l = (n as f64).sqrt() as i64;
        while l * l > n { l -= 1; }
        l as usize
    };
    let l_i64 = l as i64;
    let mut small_vals = vec![0i64; l + 2];
    let mut large_vals = vec![0i64; l + 2];
    for i in 1..=l {
        let i_i64 = i as i64;
        small_vals[i] = i_i64 - 1;
        large_vals[i] = n / i_i64 - 1;
    }
    for p in 2..=l {
        // SAFETY: p >= 2, so p-1 >= 1 is in bounds; arrays have size l+2
        if unsafe { *small_vals.get_unchecked(p) == *small_vals.get_unchecked(p - 1) } { continue; }
        let pi_pm1 = small_vals[p - 1];
        let p_i64 = p as i64;
        let p2 = p_i64 * p_i64;
        for i in 1..=l {
            let i_i64 = i as i64;
            let v = n / i_i64;
            if v < p2 { break; }
            let v_div_p = v / p_i64;
            // SAFETY: i <= l, so i < l+2; v_div_p calculation is safe
            let sub = if v_div_p <= l_i64 {
                unsafe { *small_vals.get_unchecked(v_div_p as usize) }
            } else {
                unsafe { *large_vals.get_unchecked((n / v_div_p) as usize) }
            };
            unsafe { *large_vals.get_unchecked_mut(i) -= sub - pi_pm1; }
        }
        let p2_usize = p2 as usize;
        for i in (p2_usize..=l).rev() {
            let idx = i / p;
            // SAFETY: i >= p2_usize >= p^2, idx = i/p <= l; arrays have size l+2
            unsafe {
                let val = *small_vals.get_unchecked(idx);
                *small_vals.get_unchecked_mut(i) -= val - pi_pm1;
            }
        }
    }
    let mut ans = n;
    let prime_limit = (n / l_i64) as usize;
    let mut is_composite = vec![false; prime_limit + 1];
    let mut sieve_primes = Vec::new();
    for i in 2..=prime_limit {
        if !is_composite[i] {
            sieve_primes.push(i as i64);
            if (i as u64) * (i as u64) <= prime_limit as u64 {
                for j in (i*i..=prime_limit).step_by(i) { is_composite[j] = true; }
            }
        }
    }
    for &p in &sieve_primes { ans -= p; }
    for d in 1..l {
        let d_i64 = d as i64;
        let v1 = n / d_i64;
        let v2 = n / (d_i64 + 1);
        let pi1 = if v1 <= l_i64 { small_vals[v1 as usize] } else { large_vals[(n / v1) as usize] };
        let pi2 = if v2 <= l_i64 { small_vals[v2 as usize] } else { large_vals[(n / v2) as usize] };
        ans -= d_i64 * (pi1 - pi2);
    }
    println!("{}", ans);
}
