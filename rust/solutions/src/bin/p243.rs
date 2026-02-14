// Project Euler 243: Resilience
// Find the smallest d such that phi(d)/(d-1) < 15499/94744.

fn main() {
    // Sieve small primes
    let plimit = 100;
    let mut is_prime = vec![true; plimit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= plimit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= plimit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let primes: Vec<i64> = (2..=plimit).filter(|&i| is_prime[i]).map(|i| i as i64).collect();

    let r_num: i64 = 15499;
    let r_den: i64 = 94744;

    // Find the primorial that first satisfies the condition
    let mut prod: i64 = 1;
    let mut phi: i64 = 1;
    let mut base_index = 0usize;

    for (i, &p) in primes.iter().enumerate() {
        prod *= p;
        phi *= p - 1;
        if phi * r_den < r_num * (prod - 1) {
            base_index = i;
            break;
        }
    }

    let mut best_ans = prod;

    let prev_prod = prod / primes[base_index];
    let prev_phi = phi / (primes[base_index] - 1);

    // Try multiplying prev_prod by individual primes <= primes[base_index]
    for i in 0..base_index {
        let p = primes[i];
        let test_prod = prev_prod * p;
        let test_phi = prev_phi * p;
        if test_phi * r_den < r_num * (test_prod - 1) && test_prod < best_ans {
            best_ans = test_prod;
        }
    }

    // Search: multiply prev_prod by products of primes up to primes[base_index]
    fn search(
        idx: usize, m: i64, phi_m: i64,
        base_index: usize, primes: &[i64],
        prev_prod: i64, prev_phi: i64,
        r_num: i64, r_den: i64,
        best_ans: &mut i64,
    ) {
        let d = prev_prod * m;
        if d >= *best_ans { return; }
        let phi_d = prev_phi * phi_m;
        if phi_d * r_den < r_num * (d - 1) {
            *best_ans = d;
            return;
        }
        for i in idx..=base_index {
            if i >= primes.len() { break; }
            let p = primes[i];
            let new_m = m * p;
            if prev_prod * new_m >= *best_ans { break; }
            search(i, new_m, phi_m * p, base_index, primes, prev_prod, prev_phi, r_num, r_den, best_ans);
        }
    }

    search(0, 1, 1, base_index, &primes, prev_prod, prev_phi, r_num, r_den, &mut best_ans);

    println!("{}", best_ans);
}
