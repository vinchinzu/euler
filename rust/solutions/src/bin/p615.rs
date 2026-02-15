// Project Euler 615 - The millionth number with at least one million prime factors
// DFS with pruning by log-value bounds

const N_VAL: usize = 1_000_000;
const MOD_VAL: u64 = 123_454_321;

fn main() {
    // Sieve primes
    let limit = N_VAL + 2;
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= limit { if is_prime[i] { let mut j = i*i; while j <= limit { is_prime[j] = false; j += i; } } i += 1; }

    let primes: Vec<usize> = (2..=limit).filter(|&i| is_prime[i]).collect();
    let log_primes: Vec<f64> = primes.iter().map(|&p| (p as f64).ln()).collect();

    let mut res_log = Vec::new();
    let mut res_mod = Vec::new();

    let mut limit_val = N_VAL as f64 * 2.0_f64.ln();

    loop {
        res_log.clear();
        res_mod.clear();

        struct Frame {
            min_index: usize,
            num_primes: usize,
            log_val: f64,
            mod_val: u64,
        }

        let mut stack = vec![Frame { min_index: 0, num_primes: 0, log_val: 0.0, mod_val: 1 }];

        while let Some(f) = stack.pop() {
            if f.num_primes >= N_VAL {
                res_log.push(f.log_val);
                res_mod.push(f.mod_val);
                continue;
            }
            for index in f.min_index..primes.len() {
                let lp = log_primes[index];
                let remaining = N_VAL - f.num_primes;
                let remaining = if remaining < 1 { 1 } else { remaining };
                if f.log_val + remaining as f64 * lp > limit_val { break; }
                let new_mod = (f.mod_val as u128 * primes[index] as u128 % MOD_VAL as u128) as u64;
                let mut e = 1;
                let mut cur_mod = new_mod;
                let mut cur_log = f.log_val + lp;
                while cur_log < limit_val {
                    stack.push(Frame {
                        min_index: index + 1,
                        num_primes: f.num_primes + e,
                        log_val: cur_log,
                        mod_val: cur_mod,
                    });
                    e += 1;
                    cur_mod = (cur_mod as u128 * primes[index] as u128 % MOD_VAL as u128) as u64;
                    cur_log += lp;
                }
            }
        }

        if res_log.len() >= N_VAL {
            let mut indices: Vec<usize> = (0..res_log.len()).collect();
            indices.sort_by(|&a, &b| res_log[a].partial_cmp(&res_log[b]).unwrap());
            println!("{}", res_mod[indices[N_VAL - 1]]);
            break;
        }
        limit_val += 1.0;
    }
}
