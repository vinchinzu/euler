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
        if !sieve[i] {
            if i * i <= limit {
                let mut j = i * i;
                while j <= limit {
                    sieve[j] = true;
                    j += i;
                }
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

        for vn in 0..counts.len() {
            for vm in 0..counts.len() {
                let diff = if vn > vm { vn - vm } else { vm - vn };
                let contribution =
                    (diff as i64 % MOD) * (counts[vn] % MOD) % MOD * (counts[vm] % MOD) % MOD;
                ans = (ans + contribution) % MOD;
            }
        }
    }

    // Lucy DP for prime counting
    let mut small_s = vec![0i64; r + 2];
    let mut big_s = vec![0i64; r + 2];

    for v in 1..=r {
        small_s[v] = v as i64 - 1;
    }
    for i in 1..=r {
        big_s[i] = big_n / i as i64 - 1;
    }

    for p in 2..=r {
        if small_s[p] == small_s[p - 1] {
            continue;
        }
        let sp = small_s[p - 1];
        let p2 = (p as i64) * (p as i64);
        for i in 1..=r {
            if big_n / (i as i64) < p2 {
                break;
            }
            let v_over_p = big_n / (i as i64) / (p as i64);
            let sv = if (i as i64) * (p as i64) <= r as i64 {
                big_s[i * p]
            } else {
                small_s[(big_n / ((i as i64) * (p as i64))) as usize]
            };
            big_s[i] -= sv - sp;
        }
        let mut v = r;
        while v as i64 >= p2 {
            small_s[v] -= small_s[v / p] - sp;
            v -= 1;
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
