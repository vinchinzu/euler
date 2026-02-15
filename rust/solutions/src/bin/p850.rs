// Project Euler 850 - Fractional parts sum S(N)

const MOD: i64 = 977676779;
const MOD2: i64 = 2 * MOD;
const N_VAL: i64 = 33557799775533;
const SQRT_N_MAX: usize = 5900000;
const SMALL_PRIME_LIMIT: usize = 32000;
const MAX_SMALL_K: i32 = 45;

fn main() {
    // Sieve
    let mut is_prime = vec![true; SQRT_N_MAX + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let limit = (SQRT_N_MAX as f64).sqrt() as usize + 1;
    for i in 2..=limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= SQRT_N_MAX { is_prime[j] = false; j += i; }
        }
    }

    let mut p_sum = vec![0i64; SQRT_N_MAX + 1];
    let mut small_primes = Vec::new();
    let mut cs: i64 = 0;
    for i in 2..=SQRT_N_MAX {
        if is_prime[i] {
            cs += (i as i64 - 1);
            if i <= SMALL_PRIME_LIMIT { small_primes.push(i as i32); }
        }
        p_sum[i] = cs;
    }

    // Build small odd ks
    let mut small_odd_ks = Vec::new();
    let mut k = 3;
    while k <= MAX_SMALL_K + 1 { small_odd_ks.push(k); k += 2; }
    let num_k = small_odd_ks.len();

    fn ipow(p: i64, e: i32) -> i64 {
        let mut r = 1i64;
        for _ in 0..e { r *= p; }
        r
    }

    fn get_c_k_val(p: i32, e: i32, k: i32) -> i64 {
        let ceil_val = (e + k - 1) / k;
        ipow(p as i64, e - ceil_val)
    }

    fn get_c_k_inf(p: i32, e: i32) -> i64 {
        ipow(p as i64, e - 1)
    }

    let calc_tail_sum = |m: i64, is_prime: &[bool], p_sum: &[i64]| -> i64 {
        let mut res = m;
        if (SMALL_PRIME_LIMIT as i64) * (SMALL_PRIME_LIMIT as i64) >= m { return res; }

        let min_p_bound = SMALL_PRIME_LIMIT;
        let mut k = 1i64;
        loop {
            let upper_bound_val = m / k;
            let lower_bound_val = m / (k + 1);

            let mut upper_p = (upper_bound_val as f64).sqrt() as i64;
            while (upper_p + 1) * (upper_p + 1) <= upper_bound_val { upper_p += 1; }
            while upper_p * upper_p > upper_bound_val { upper_p -= 1; }

            let mut lower_p = (lower_bound_val as f64).sqrt() as i64;
            while (lower_p + 1) * (lower_p + 1) <= lower_bound_val { lower_p += 1; }
            while lower_p * lower_p > lower_bound_val { lower_p -= 1; }

            let eff_upper = upper_p as usize;
            let eff_lower = if lower_p as usize > min_p_bound { lower_p as usize } else { min_p_bound };

            if eff_upper > eff_lower && eff_upper <= SQRT_N_MAX {
                let term_sum = p_sum[eff_upper] - p_sum[eff_lower];
                res += term_sum * k;
            }

            if eff_upper <= min_p_bound { break; }
            k += 1;
        }

        // Part 2: p^3 terms
        let mut limit_p3 = (m as f64).cbrt() as i64;
        while (limit_p3 + 1) * (limit_p3 + 1) * (limit_p3 + 1) <= m { limit_p3 += 1; }
        while limit_p3 * limit_p3 * limit_p3 > m { limit_p3 -= 1; }

        if limit_p3 > SMALL_PRIME_LIMIT as i64 && limit_p3 <= SQRT_N_MAX as i64 {
            for p in (SMALL_PRIME_LIMIT + 1)..=limit_p3 as usize {
                if is_prime[p] {
                    let val = (p as i64) * (p as i64) - p as i64;
                    let term = m / ((p as i64) * (p as i64) * (p as i64));
                    res += val * term;
                }
            }
        }

        res
    };

    let mut total_sums_k = vec![0i64; num_k];
    let mut total_sums_inf: i64 = 0;

    // DFS
    struct DfsState<'a> {
        small_primes: &'a [i32],
        small_odd_ks: &'a [i32],
        is_prime: &'a [bool],
        p_sum: &'a [i64],
        total_sums_k: Vec<i64>,
        total_sums_inf: i64,
    }

    fn dfs(state: &mut DfsState, idx: usize, current_d: i64,
           curr_vals_k: &[i64], curr_vals_inf: i64,
           calc_tail: &dyn Fn(i64, &[bool], &[i64]) -> i64) {
        let m = N_VAL / current_d;
        let tail_mult = calc_tail(m, state.is_prime, state.p_sum);

        let num_k = state.small_odd_ks.len();
        for ki in 0..num_k {
            state.total_sums_k[ki] = (state.total_sums_k[ki]
                + curr_vals_k[ki] % MOD2 * (tail_mult % MOD2)) % MOD2;
        }
        state.total_sums_inf = (state.total_sums_inf
            + curr_vals_inf % MOD2 * (tail_mult % MOD2)) % MOD2;

        for i in idx..state.small_primes.len() {
            let p = state.small_primes[i];
            if current_d > N_VAL / (p as i64 * p as i64) { break; }

            let mut pe = p as i64 * p as i64;
            let mut e = 2i32;

            loop {
                let new_d = current_d * pe;
                if new_d > N_VAL { break; }

                let mut new_vals_k = curr_vals_k.to_vec();
                for ki in 0..num_k {
                    let term = get_c_k_val(p, e, state.small_odd_ks[ki])
                             - get_c_k_val(p, e - 1, state.small_odd_ks[ki]);
                    new_vals_k[ki] = (curr_vals_k[ki] * (term % MOD2)) % MOD2;
                }
                let term_inf = get_c_k_inf(p, e) - get_c_k_inf(p, e - 1);
                let new_inf = (curr_vals_inf * (term_inf % MOD2)) % MOD2;

                dfs(state, i + 1, new_d, &new_vals_k, new_inf, calc_tail);

                pe *= p as i64;
                e += 1;
            }
        }
    }

    let mut state = DfsState {
        small_primes: &small_primes,
        small_odd_ks: &small_odd_ks,
        is_prime: &is_prime,
        p_sum: &p_sum,
        total_sums_k: total_sums_k.clone(),
        total_sums_inf: 0,
    };

    let init_k = vec![1i64; num_k];
    dfs(&mut state, 0, 1, &init_k, 1, &calc_tail_sum);

    total_sums_k = state.total_sums_k;
    total_sums_inf = state.total_sums_inf;

    // Calculate final S
    let num_odd = (N_VAL + 1) / 2;
    let n_mod = N_VAL % MOD2;
    let np1_mod = (N_VAL + 1) % MOD2;
    let half_nn = (n_mod * np1_mod / 2) % MOD2;
    let term_doubled = (num_odd % MOD2 * half_nn) % MOD2;

    let mut sum_sigma_ck = N_VAL % MOD2;
    for ki in 0..num_k {
        sum_sigma_ck = (sum_sigma_ck + total_sums_k[ki]) % MOD2;
    }

    let num_small = (num_k as i64) + 1; // +1 for K=1
    let num_large = num_odd - num_small;
    sum_sigma_ck = (sum_sigma_ck + (num_large % MOD2) * (total_sums_inf % MOD2)) % MOD2;

    let two_s = ((term_doubled - sum_sigma_ck) % MOD2 + MOD2) % MOD2;
    let ans = (two_s / 2) % MOD;

    println!("{}", ans);
}
