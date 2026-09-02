// Project Euler 712 - Exponent Difference
//
// Sum |v_p(n) - v_p(m)| over all 1 <= n,m <= N and primes p.
// Small primes: enumerate exponent counts directly.
// Large primes (p > sqrt(N)): use Lucy DP for prime counting.

const MOD: i64 = 1_000_000_007;

fn main() {
    let big_n: i64 = 1_000_000_000_000;
    let mut l = (big_n as f64).sqrt() as i64;
    while (l + 1) * (l + 1) <= big_n {
        l += 1;
    }
    while l * l > big_n {
        l -= 1;
    }

    let r = l as usize;
    let limit = (big_n / l) as usize + 1;

    // Sieve primes up to limit
    let mut sieve = vec![false; limit + 1];
    for i in 2..=limit {
        if !sieve[i] && i * i <= limit {
            let mut j = i * i;
            while j <= limit {
                sieve[j] = true;
                j += i;
            }
        }
    }
    let primes_list: Vec<i64> = (2..=limit).filter(|&i| !sieve[i]).map(|i| i as i64).collect();
    let num_small_primes = primes_list.len() as i64;

    let mut ans: i64 = 0;

    // Process small primes
    for &p in &primes_list {
        let mut counts: Vec<i64> = Vec::new();
        let mut pe: i64 = 1;
        loop {
            let pe_next = if pe > big_n / p { big_n + 1 } else { pe * p };
            let cnt = (big_n / pe) - (if pe_next <= big_n { big_n / pe_next } else { 0 });
            counts.push(cnt % MOD);
            if pe > big_n / p {
                break;
            }
            pe *= p;
        }

        for (vn, &count_vn) in counts.iter().enumerate() {
            for (vm, &count_vm) in counts.iter().enumerate() {
                let diff = vn.abs_diff(vm);
                let contribution =
                    (diff as i64 % MOD) * (count_vn % MOD) % MOD * (count_vm % MOD) % MOD;
                ans = (ans + contribution) % MOD;
            }
        }
    }

    // Lucy DP for prime counting
    let mut small_s = vec![0i64; r + 2];
    let mut big_s = vec![0i64; r + 2];

    for (v, item) in small_s.iter_mut().enumerate().take(r + 1).skip(1) {
        *item = v as i64 - 1;
    }
    for (i, item) in big_s.iter_mut().enumerate().take(r + 1).skip(1) {
        *item = big_n / i as i64 - 1;
    }

    for &p_i64 in &primes_list {
        let p = p_i64 as usize;
        let p2 = p_i64 * p_i64;
        if p2 > big_n {
            break;
        }
        let sp = unsafe { *small_s.get_unchecked(p - 1) };

        let max_i = ((big_n / p2) as usize).min(r);
        let lim1 = max_i.min(r / p);

        let mut ip = p;
        for i in 1..=lim1 {
            let sv = unsafe { *big_s.get_unchecked(ip) };
            unsafe {
                *big_s.get_unchecked_mut(i) -= sv - sp;
            }
            ip += p;
        }

        let m = big_n / p_i64;
        if p2 <= r as i64 {
            let isqrt_m = ((m as f64).sqrt() as usize).min(max_i);
            let b_ptr = big_s.as_mut_ptr();
            let s_ptr = small_s.as_ptr();
            let mut i = lim1 + 1;
            unsafe {
                while i < isqrt_m {
                    let q0 = (m / (i as i64)) as usize;
                    let q1 = (m / ((i + 1) as i64)) as usize;
                    let d0 = *s_ptr.add(q0) - sp;
                    let d1 = *s_ptr.add(q1) - sp;
                    *b_ptr.add(i) -= d0;
                    *b_ptr.add(i + 1) -= d1;
                    i += 2;
                }
                if i <= isqrt_m {
                    let q0 = (m / (i as i64)) as usize;
                    *b_ptr.add(i) -= *s_ptr.add(q0) - sp;
                    i += 1;
                }
                while i <= max_i {
                    let q = (m / (i as i64)) as usize;
                    let mut i_last = (m / (q as i64)) as usize;
                    if i_last > max_i {
                        i_last = max_i;
                    }
                    let diff = *s_ptr.add(q) - sp;
                    let ptr = b_ptr.add(i);
                    let count = i_last - i + 1;
                    for offset in 0..count {
                        *ptr.add(offset) -= diff;
                    }
                    i = i_last + 1;
                }
            }
        } else {
            let m_u32 = m as u32;
            let b_ptr = big_s.as_mut_ptr();
            let s_ptr = small_s.as_ptr();
            let mut i = lim1 + 1;
            unsafe {
                while i < max_i {
                    let q0 = (m_u32 / (i as u32)) as usize;
                    let q1 = (m_u32 / ((i + 1) as u32)) as usize;
                    let d0 = *s_ptr.add(q0) - sp;
                    let d1 = *s_ptr.add(q1) - sp;
                    *b_ptr.add(i) -= d0;
                    *b_ptr.add(i + 1) -= d1;
                    i += 2;
                }
                if i <= max_i {
                    let q0 = (m_u32 / (i as u32)) as usize;
                    let d0 = *s_ptr.add(q0) - sp;
                    *b_ptr.add(i) -= d0;
                }
            }
        }

        if p2 <= r as i64 {
            let max_k = r / p;
            let s_ptr = small_s.as_mut_ptr();
            unsafe {
                let delta = *s_ptr.add(max_k) - sp;
                for v in (max_k * p)..=r {
                    *s_ptr.add(v) -= delta;
                }
                match p {
                    2 => {
                        for k in (2..max_k).rev() {
                            let delta = *s_ptr.add(k) - sp;
                            let base = s_ptr.add(k * 2);
                            *base -= delta;
                            *base.add(1) -= delta;
                        }
                    }
                    3 => {
                        for k in (3..max_k).rev() {
                            let delta = *s_ptr.add(k) - sp;
                            let base = s_ptr.add(k * 3);
                            *base -= delta;
                            *base.add(1) -= delta;
                            *base.add(2) -= delta;
                        }
                    }
                    5 => {
                        for k in (5..max_k).rev() {
                            let delta = *s_ptr.add(k) - sp;
                            let base = s_ptr.add(k * 5);
                            *base -= delta;
                            *base.add(1) -= delta;
                            *base.add(2) -= delta;
                            *base.add(3) -= delta;
                            *base.add(4) -= delta;
                        }
                    }
                    _ => {
                        for k in (p..max_k).rev() {
                            let delta = *s_ptr.add(k) - sp;
                            let base = s_ptr.add(k * p);
                            for offset in 0..p {
                                *base.add(offset) -= delta;
                            }
                        }
                    }
                }
            }
        }
    }

    // Process large primes via floor quotient grouping
    for q in 1..l as usize {
        let nq = big_n / q as i64;
        let pi_q = if nq <= r as i64 {
            small_s[nq as usize]
        } else {
            big_s[q]
        };

        let nq1 = big_n / (q as i64 + 1);
        let pi_q1 = if nq1 <= r as i64 {
            small_s[nq1 as usize]
        } else {
            big_s[q + 1]
        };

        let mut pi_q_adj = pi_q;
        let mut pi_q1_adj = pi_q1;
        if nq <= limit as i64 && pi_q_adj > num_small_primes {
            pi_q_adj = num_small_primes;
        }
        if nq1 <= limit as i64 && pi_q1_adj > num_small_primes {
            pi_q1_adj = num_small_primes;
        }

        let num_primes_in_range = ((pi_q_adj - pi_q1_adj) % MOD + MOD) % MOD;
        let contribution = 2i64 * ((big_n - q as i64) % MOD) % MOD
            * (q as i64 % MOD)
            % MOD
            * num_primes_in_range
            % MOD;
        ans = (ans + contribution) % MOD;
    }

    println!("{}", ans);
}
