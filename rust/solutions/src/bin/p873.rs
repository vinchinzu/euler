// Project Euler 873 - W(p,q,r) words with separation constraint
// Stars-and-bars approach iterating over run counts
// Optimization: precompute modular inverse table (linear sieve);
// pure u64 modular mul (MOD < 2^32); unsafe get_unchecked for hot paths.

const MOD: u64 = 1_000_000_007;

#[inline(always)]
fn update_sb(curr_sb_val: u64, curr_n_sb: u64, k_sb: u64, inv: &[u64]) -> u64 {
    if curr_n_sb < k_sb || curr_sb_val == 0 {
        0
    } else {
        let num = (curr_n_sb - k_sb) % MOD;
        let den_idx = (curr_n_sb % MOD) as usize;
        // SAFETY: den_idx < MOD < inv.len()
        curr_sb_val * num % MOD * unsafe { *inv.get_unchecked(den_idx) } % MOD
    }
}

fn main() {
    let p: u64 = 1_000_000;
    let q: u64 = 10_000_000;
    let r: u64 = 100_000_000;

    let k_sb = p + q;
    let mut curr_n_sb = r - 2 + p + q;

    // Inverse table: covers m in 1..=q and curr_n_sb values (start ~1.11e8 down).
    let inv_limit = (curr_n_sb + 2) as usize;
    let mut inv = vec![0u64; inv_limit];
    inv[1] = 1;
    for i in 2..inv_limit {
        let i_u64 = i as u64;
        let idx = (MOD % i_u64) as usize;
        // SAFETY: idx < i < inv_limit, already computed in previous iterations
        inv[i] = (MOD - (MOD / i_u64) * unsafe { *inv.get_unchecked(idx) } % MOD) % MOD;
    }

    // Initial binom(curr_n_sb, k_sb) via successive multiply + table inv
    let mut curr_sb_val: u64 = 1;
    for i in 0..k_sb {
        let num = (curr_n_sb - i) % MOD;
        let den_idx = (i + 1) as usize;
        // SAFETY: den_idx = i+1, i < k_sb = 11M, den_idx < inv_limit
        curr_sb_val = curr_sb_val * num % MOD * unsafe { *inv.get_unchecked(den_idx) } % MOD;
    }

    let mut comb_p: u64 = 1;
    let mut comb_q: u64 = 1;
    let mut ans: u64 = 0;
    let mut m: u64 = 1;

    loop {
        // Case k = 2m
        if m <= p && m <= q {
            let term = 2 * comb_p % MOD * comb_q % MOD * curr_sb_val % MOD;
            ans = (ans + term) % MOD;
        }

        // Update SB: decrease N by 2
        curr_sb_val = update_sb(curr_sb_val, curr_n_sb, k_sb, &inv);
        curr_n_sb -= 1;
        curr_sb_val = update_sb(curr_sb_val, curr_n_sb, k_sb, &inv);
        curr_n_sb -= 1;

        if curr_sb_val == 0 {
            break;
        }

        let m_idx = m as usize;
        // SAFETY: m starts at 1, increments; m <= min(p,q) = 1M < inv_limit
        let inv_m = unsafe { *inv.get_unchecked(m_idx) };

        let next_comb_p = if m < p {
            comb_p * ((p - m) % MOD) % MOD * inv_m % MOD
        } else {
            0
        };

        let next_comb_q = if m < q {
            comb_q * ((q - m) % MOD) % MOD * inv_m % MOD
        } else {
            0
        };

        let mut term_odd: u64 = 0;
        if m + 1 <= p && m <= q {
            term_odd = next_comb_p * comb_q % MOD;
        }
        if m <= p && m + 1 <= q {
            term_odd = (term_odd + comb_p * next_comb_q % MOD) % MOD;
        }
        ans = (ans + term_odd * curr_sb_val % MOD) % MOD;

        // Update SB for next m
        curr_sb_val = update_sb(curr_sb_val, curr_n_sb, k_sb, &inv);
        curr_n_sb -= 1;
        curr_sb_val = update_sb(curr_sb_val, curr_n_sb, k_sb, &inv);
        curr_n_sb -= 1;

        comb_p = next_comb_p;
        comb_q = next_comb_q;
        m += 1;

        if (comb_p == 0 && comb_q == 0) || (m > p && m > q) {
            break;
        }
    }

    println!("{}", ans);
}
