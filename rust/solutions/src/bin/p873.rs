// Project Euler 873 - W(p,q,r) words with separation constraint
// Stars-and-bars approach iterating over run counts
// Optimization: precompute modular inverse table (linear sieve);
// pure u64 modular mul (MOD < 2^30).

const MOD: u64 = 1_000_000_007;

fn main() {
    let p: i64 = 1_000_000;
    let q: i64 = 10_000_000;
    let r: i64 = 100_000_000;

    let k_sb = p + q;
    let mut curr_n_sb = r - 2 + p + q;

    // Inverse table: covers m in 1..=q and curr_n_sb values (start ~1.11e8 down).
    let inv_limit = (curr_n_sb as usize) + 2;
    let mut inv = vec![0u64; inv_limit];
    inv[1] = 1;
    for i in 2..inv_limit {
        // SAFETY: i < MOD so MOD % i < i, already computed
        inv[i] = (MOD - (MOD / i as u64) * inv[(MOD % i as u64) as usize] % MOD) % MOD;
    }

    // Initial binom(curr_n_sb, k_sb) via successive multiply + table inv
    let mut curr_sb_val: u64 = 1;
    for i in 0..k_sb {
        let num = (curr_n_sb - i).rem_euclid(MOD as i64) as u64;
        curr_sb_val = curr_sb_val * num % MOD * inv[(i + 1) as usize] % MOD;
    }

    let mut comb_p: u64 = 1;
    let mut comb_q: u64 = 1;
    let mut ans: u64 = 0;
    let mut m: i64 = 1;

    loop {
        // Case k = 2m
        if m <= p && m <= q {
            let term = 2 * comb_p % MOD * comb_q % MOD * curr_sb_val % MOD;
            ans = (ans + term) % MOD;
        }

        // Update SB: decrease N by 2
        for _ in 0..2 {
            if curr_n_sb - k_sb < 0 {
                curr_sb_val = 0;
            } else if curr_sb_val != 0 {
                let num = (curr_n_sb - k_sb).rem_euclid(MOD as i64) as u64;
                let den = curr_n_sb.rem_euclid(MOD as i64) as usize;
                curr_sb_val = curr_sb_val * num % MOD * inv[den] % MOD;
            }
            curr_n_sb -= 1;
        }

        if curr_sb_val == 0 {
            break;
        }

        let inv_m = inv[m as usize];

        let next_comb_p = if m <= p - 1 {
            comb_p * ((p - m).rem_euclid(MOD as i64) as u64) % MOD * inv_m % MOD
        } else {
            0
        };

        let next_comb_q = if m <= q - 1 {
            comb_q * ((q - m).rem_euclid(MOD as i64) as u64) % MOD * inv_m % MOD
        } else {
            0
        };

        let mut term_odd: u64 = 0;
        if m + 1 <= p && m <= q {
            term_odd = (term_odd + next_comb_p * comb_q % MOD) % MOD;
        }
        if m <= p && m + 1 <= q {
            term_odd = (term_odd + comb_p * next_comb_q % MOD) % MOD;
        }
        term_odd = term_odd * curr_sb_val % MOD;
        ans = (ans + term_odd) % MOD;

        // Update SB for next m
        for _ in 0..2 {
            if curr_n_sb <= 0 || curr_n_sb - k_sb < 0 {
                curr_sb_val = 0;
            } else if curr_sb_val != 0 {
                let num = (curr_n_sb - k_sb).rem_euclid(MOD as i64) as u64;
                let den = curr_n_sb.rem_euclid(MOD as i64) as usize;
                curr_sb_val = curr_sb_val * num % MOD * inv[den] % MOD;
            }
            curr_n_sb -= 1;
        }

        comb_p = next_comb_p;
        comb_q = next_comb_q;
        m += 1;

        if (comb_p == 0 && comb_q == 0) || (m > p && m > q) {
            break;
        }
    }

    println!("{}", ans);
}
