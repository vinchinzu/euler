// Project Euler 892
// Modular inverse sieve with central binomial coefficients.

const MOD: i64 = 1_234_567_891;

fn main() {
    let n = 10_000_000i64;
    let limit = (n / 2 + 2) as usize;

    let mut inv = vec![0i64; limit + 1];
    inv[1] = 1;
    for i in 2..=limit {
        let i_i64 = i as i64;
        inv[i] = (MOD - MOD / i_i64) * inv[(MOD % i_i64) as usize] % MOD;
    }

    let m = (n / 2) as usize;
    let mut comb = 1i64;
    let inv2 = inv[2];
    let inv_ptr = inv.as_ptr();

    let mut acc_sum = 0i64;
    let m_minus_1 = m - 1;
    
    // Main loop: process all mi where both even and odd terms apply
    for mi in 1..m_minus_1 {
        // SAFETY: mi < m-1 < m, so mi+1 < m < limit; inv has size limit+1
        let inv_mi = unsafe { *inv_ptr.add(mi) };
        let inv_mi_plus_1 = unsafe { *inv_ptr.add(mi + 1) };
        let mi_i64 = mi as i64;
        
        let tmp = comb * 2 % MOD;
        comb = tmp * (2 * mi_i64 - 1) % MOD * inv_mi % MOD;

        let comb2 = comb * comb % MOD;
        acc_sum += comb2 * inv2 % MOD;
        acc_sum += comb2 * 2 % MOD * mi_i64 % MOD * inv_mi_plus_1 % MOD;
        
        if mi & 0xFF == 0 {
            acc_sum %= MOD;
        }
    }
    
    // Handle last few iterations that might not have odd term
    for mi in m_minus_1..=m {
        // SAFETY: mi <= m < limit; inv has size limit+1
        let inv_mi = unsafe { *inv_ptr.add(mi) };
        let mi_i64 = mi as i64;
        
        let tmp = comb * 2 % MOD;
        comb = tmp * (2 * mi_i64 - 1) % MOD * inv_mi % MOD;

        let comb2 = comb * comb % MOD;
        acc_sum += comb2 * inv2 % MOD;

        if 2 * mi + 1 <= n as usize {
            // SAFETY: mi+1 <= m+1 < limit; inv has size limit+1
            let inv_mi_plus_1 = unsafe { *inv_ptr.add(mi + 1) };
            acc_sum += comb2 * 2 % MOD * mi_i64 % MOD * inv_mi_plus_1 % MOD;
        }
    }
    
    let total_sum = acc_sum % MOD;

    println!("{}", total_sum);
}
