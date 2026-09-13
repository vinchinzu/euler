const LIMIT: usize = 46368;
const MOD: u64 = 1_000_000_000;

fn main() {
    let mut is_prime = vec![true; LIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=215 {
        if is_prime[i] {
            let mut j = i * i;
            while j <= LIMIT {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    let mut primes = Vec::with_capacity(5000);
    for p in 2..=LIMIT {
        if is_prime[p] {
            primes.push(p as u64);
        }
    }

    let mut dp = vec![0u64; LIMIT + 1];
    dp[0] = 1;

    for &p in &primes {
        let p_usize = p as usize;
        
        if p > 8 {
            let mut k = p_usize;
            while k + 7 <= LIMIT {
                unsafe {
                    // SAFETY: p > 8, so reads (k-p to k+7-p) don't overlap writes (k to k+7)
                    let prev0 = *dp.get_unchecked(k - p_usize);
                    let prev1 = *dp.get_unchecked(k + 1 - p_usize);
                    let prev2 = *dp.get_unchecked(k + 2 - p_usize);
                    let prev3 = *dp.get_unchecked(k + 3 - p_usize);
                    let prev4 = *dp.get_unchecked(k + 4 - p_usize);
                    let prev5 = *dp.get_unchecked(k + 5 - p_usize);
                    let prev6 = *dp.get_unchecked(k + 6 - p_usize);
                    let prev7 = *dp.get_unchecked(k + 7 - p_usize);
                    
                    *dp.get_unchecked_mut(k) = (*dp.get_unchecked(k) + p * prev0) % MOD;
                    *dp.get_unchecked_mut(k + 1) = (*dp.get_unchecked(k + 1) + p * prev1) % MOD;
                    *dp.get_unchecked_mut(k + 2) = (*dp.get_unchecked(k + 2) + p * prev2) % MOD;
                    *dp.get_unchecked_mut(k + 3) = (*dp.get_unchecked(k + 3) + p * prev3) % MOD;
                    *dp.get_unchecked_mut(k + 4) = (*dp.get_unchecked(k + 4) + p * prev4) % MOD;
                    *dp.get_unchecked_mut(k + 5) = (*dp.get_unchecked(k + 5) + p * prev5) % MOD;
                    *dp.get_unchecked_mut(k + 6) = (*dp.get_unchecked(k + 6) + p * prev6) % MOD;
                    *dp.get_unchecked_mut(k + 7) = (*dp.get_unchecked(k + 7) + p * prev7) % MOD;
                }
                k += 8;
            }
            
            while k <= LIMIT {
                unsafe {
                    let prev_val = *dp.get_unchecked(k - p_usize);
                    let curr_ptr = dp.get_unchecked_mut(k);
                    *curr_ptr = (*curr_ptr + p * prev_val) % MOD;
                }
                k += 1;
            }
        } else {
            for k in p_usize..=LIMIT {
                unsafe {
                    let prev_val = *dp.get_unchecked(k - p_usize);
                    let curr_ptr = dp.get_unchecked_mut(k);
                    *curr_ptr = (*curr_ptr + p * prev_val) % MOD;
                }
            }
        }
    }

    let mut fib_prev = 0usize;
    let mut fib_curr = 1usize;
    let mut ans = 0u64;
    for i in 1..=24 {
        let tmp = fib_prev + fib_curr;
        fib_prev = fib_curr;
        fib_curr = tmp;
        if i >= 2 {
            unsafe {
                ans = (ans + *dp.get_unchecked(fib_prev)) % MOD;
            }
        }
    }

    println!("{}", ans);
}
