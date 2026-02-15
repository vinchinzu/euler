// Project Euler 861 - Bi-unitary divisors
// sum_{k=2}^{10} Q_k(10^12) using Lucy DP for pi(x) and signature enumeration

const N_LIMIT: i64 = 1_000_000_000_000;
const SIEVE_LIM: usize = 1_000_001;

static mut S_SMALL: [i64; SIEVE_LIM + 1] = [0; SIEVE_LIM + 1];
static mut S_LARGE: [i64; SIEVE_LIM + 1] = [0; SIEVE_LIM + 1];
static mut ISQRT_VAL: i64 = 0;

fn sieve_primes() -> Vec<i32> {
    let mut is_prime = vec![true; SIEVE_LIM + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 2..=SIEVE_LIM {
        if is_prime[p] {
            let mut i = p * p;
            while i <= SIEVE_LIM { is_prime[i] = false; i += p; }
        }
    }
    (2..=SIEVE_LIM).filter(|&p| is_prime[p]).map(|p| p as i32).collect()
}

fn compute_pi(n: i64, prime_list: &[i32]) {
    let mut isqrt = (n as f64).sqrt() as i64;
    while (isqrt + 1) * (isqrt + 1) <= n { isqrt += 1; }
    while isqrt > 0 && isqrt * isqrt > n { isqrt -= 1; }

    unsafe {
        ISQRT_VAL = isqrt;
        for v in 0..=isqrt as usize { S_SMALL[v] = v as i64 - 1; }
        for k in 1..=isqrt as usize { S_LARGE[k] = n / k as i64 - 1; }

        for pi in 0..prime_list.len() {
            let p = prime_list[pi] as i64;
            if p > isqrt { break; }
            let p2 = p * p;
            if p2 > n { break; }
            let sp_1 = S_SMALL[(p - 1) as usize];
            let mut k_limit = n / p2;
            if k_limit > isqrt { k_limit = isqrt; }
            for k in 1..=k_limit as usize {
                let target = (n / k as i64) / p;
                let s_target = if target <= isqrt { S_SMALL[target as usize] } else { S_LARGE[k * p as usize] };
                S_LARGE[k] -= s_target - sp_1;
            }
            for v in (p2 as usize..=isqrt as usize).rev() {
                S_SMALL[v] -= S_SMALL[v / p as usize] - sp_1;
            }
        }
    }
}

fn get_pi(x: i64) -> i64 {
    if x <= 1 { return 0; }
    unsafe {
        if x <= ISQRT_VAL { S_SMALL[x as usize] } else { S_LARGE[(N_LIMIT / x) as usize] }
    }
}

fn integer_root(n: i64, k: i32) -> i64 {
    if k == 1 { return n; }
    if n <= 1 { return n; }
    let mut lo = 1i64;
    let mut hi = (n as f64).powf(1.0 / k as f64) as i64 + 2;
    let mut ans = 1i64;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let mut p = 1i64;
        let mut over = false;
        for _ in 0..k {
            if p > n / mid { over = true; break; }
            p *= mid;
        }
        if !over && p <= n { ans = mid; lo = mid + 1; }
        else { hi = mid - 1; }
    }
    ans
}

fn get_factor_partitions(target: i32, count: i32, min_val: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if count == 1 {
        if target >= min_val {
            let mut p = current.clone();
            p.push(target);
            result.push(p);
        }
        return;
    }
    for i in min_val..=target {
        if target % i == 0 {
            current.push(i);
            get_factor_partitions(target / i, count - 1, i, current, result);
            current.pop();
        }
    }
}

fn generate_signatures(k: i32) -> Vec<Vec<i32>> {
    let mut sigs: Vec<Vec<i32>> = Vec::new();
    let mut r = 1;
    loop {
        let power_of_2 = 1i64 << (r - 1);
        if power_of_2 > k as i64 { break; }
        if k % power_of_2 as i32 == 0 {
            let target = k / power_of_2 as i32;
            let mut partitions = Vec::new();
            let mut current = Vec::new();
            get_factor_partitions(target, r, 1, &mut current, &mut partitions);

            for part in &partitions {
                let n_choices = 1 << r;
                for mask in 0..n_choices {
                    let mut a: Vec<i32> = (0..r as usize).map(|i| {
                        if (mask >> i) & 1 == 1 { 2 * part[i] } else { 2 * part[i] - 1 }
                    }).collect();
                    a.sort();
                    if !sigs.contains(&a) {
                        sigs.push(a);
                    }
                }
            }
        }
        r += 1;
    }
    sigs
}

struct BacktrackState<'a> {
    prime_list: &'a [i32],
    num_primes: usize,
    used_primes: Vec<i64>,
}

impl<'a> BacktrackState<'a> {
    fn backtrack(&mut self, group_idx: usize, current_prod: i64, groups: &[(i32, i32)]) -> i64 {
        if group_idx == groups.len() { return 1; }
        self.backtrack_inner(group_idx, groups[group_idx].1, current_prod, groups, 0)
    }

    fn backtrack_inner(&mut self, group_idx: usize, remain: i32, current_prod: i64,
                       groups: &[(i32, i32)], min_p_idx: usize) -> i64 {
        let exp = groups[group_idx].0;

        if remain == 0 {
            return self.backtrack(group_idx + 1, current_prod, groups);
        }

        let mut total = 0i64;

        // Last prime optimization
        if group_idx == groups.len() - 1 && remain == 1 {
            let rem = N_LIMIT / current_prod;
            let limit_p = integer_root(rem, exp);
            if limit_p < 2 { return 0; }
            let lower_bound = if min_p_idx < self.num_primes { self.prime_list[min_p_idx] as i64 } else { return 0; };
            if limit_p < lower_bound { return 0; }
            let mut valid_count = get_pi(limit_p) - get_pi(lower_bound - 1);
            for &up in &self.used_primes {
                if up >= lower_bound && up <= limit_p { valid_count -= 1; }
            }
            return valid_count;
        }

        for i in min_p_idx..self.num_primes {
            let p = self.prime_list[i] as i64;

            if self.used_primes.contains(&p) { continue; }

            let mut p_pow = 1i64;
            let mut over = false;
            for _ in 0..exp {
                if p_pow > N_LIMIT / p { over = true; break; }
                p_pow *= p;
            }
            if over { break; }
            if current_prod > N_LIMIT / p_pow { break; }
            let next_prod = current_prod * p_pow;
            if next_prod > N_LIMIT { break; }

            self.used_primes.push(p);
            total += self.backtrack_inner(group_idx, remain - 1, next_prod, groups, i + 1);
            self.used_primes.pop();
        }
        total
    }
}

fn main() {
    let prime_list = sieve_primes();
    let num_primes = prime_list.len();
    compute_pi(N_LIMIT, &prime_list);

    let mut total_sum = 0i64;

    for k in 2..=10 {
        let sigs = generate_signatures(k);
        let mut q_k = 0i64;

        for sig in &sigs {
            // Group exponents
            let mut groups: Vec<(i32, i32)> = Vec::new(); // (exp, count)
            for &e in sig {
                if let Some(g) = groups.iter_mut().find(|g| g.0 == e) {
                    g.1 += 1;
                } else {
                    groups.push((e, 1));
                }
            }
            // Sort by exponent descending
            groups.sort_by(|a, b| b.0.cmp(&a.0));

            let mut state = BacktrackState {
                prime_list: &prime_list,
                num_primes,
                used_primes: Vec::new(),
            };
            let count = state.backtrack(0, 1, &groups);
            q_k += count;
        }
        total_sum += q_k;
    }

    println!("{}", total_sum);
}
