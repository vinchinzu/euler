// Project Euler 873 - W(p,q,r) words with separation constraint
// Stars-and-bars approach iterating over run counts

const MOD: i64 = 1_000_000_007;

fn power(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut res: i64 = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { res = res * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    res
}

fn mod_inv(a: i64, m: i64) -> i64 { power(a, m - 2, m) }

fn main() {
    let p: i64 = 1_000_000;
    let q: i64 = 10_000_000;
    let r: i64 = 100_000_000;

    let k_sb = p + q;
    let mut curr_n_sb = r - 2 + p + q;

    // Compute initial binom(curr_n_sb, k_sb) mod MOD
    let mut sb_num: i64 = 1;
    let mut sb_den: i64 = 1;
    for i in 0..k_sb {
        sb_num = (sb_num as i128 * ((curr_n_sb - i).rem_euclid(MOD)) as i128 % MOD as i128) as i64;
        sb_den = (sb_den as i128 * ((i + 1) % MOD) as i128 % MOD as i128) as i64;
    }
    let mut curr_sb_val = (sb_num as i128 * mod_inv(sb_den, MOD) as i128 % MOD as i128) as i64;

    let mut comb_p: i64 = 1;
    let mut comb_q: i64 = 1;
    let mut ans: i64 = 0;
    let mut m: i64 = 1;

    loop {
        // Case k = 2m
        if m <= p && m <= q {
            let term = (2i128 * comb_p as i128 % MOD as i128 * comb_q as i128 % MOD as i128 * curr_sb_val as i128 % MOD as i128) as i64;
            ans = (ans + term) % MOD;
        }

        // Update SB: decrease N by 2
        for _ in 0..2 {
            if curr_n_sb - k_sb < 0 {
                curr_sb_val = 0;
            } else if curr_sb_val != 0 {
                let factor = ((curr_n_sb - k_sb).rem_euclid(MOD) as i128
                    * mod_inv(curr_n_sb.rem_euclid(MOD), MOD) as i128 % MOD as i128) as i64;
                curr_sb_val = (curr_sb_val as i128 * factor as i128 % MOD as i128) as i64;
            }
            curr_n_sb -= 1;
        }

        if curr_sb_val == 0 { break; }

        let inv_m = mod_inv(m % MOD, MOD);

        let next_comb_p = if m <= p - 1 {
            (comb_p as i128 * ((p - m).rem_euclid(MOD)) as i128 % MOD as i128 * inv_m as i128 % MOD as i128) as i64
        } else { 0 };

        let next_comb_q = if m <= q - 1 {
            (comb_q as i128 * ((q - m).rem_euclid(MOD)) as i128 % MOD as i128 * inv_m as i128 % MOD as i128) as i64
        } else { 0 };

        let mut term_odd: i64 = 0;
        if m + 1 <= p && m <= q {
            term_odd = (term_odd + (next_comb_p as i128 * comb_q as i128 % MOD as i128) as i64) % MOD;
        }
        if m <= p && m + 1 <= q {
            term_odd = (term_odd + (comb_p as i128 * next_comb_q as i128 % MOD as i128) as i64) % MOD;
        }
        term_odd = (term_odd as i128 * curr_sb_val as i128 % MOD as i128) as i64;
        ans = (ans + term_odd) % MOD;

        // Update SB for next m
        for _ in 0..2 {
            if curr_n_sb <= 0 || curr_n_sb - k_sb < 0 {
                curr_sb_val = 0;
            } else if curr_sb_val != 0 {
                let factor = ((curr_n_sb - k_sb).rem_euclid(MOD) as i128
                    * mod_inv(curr_n_sb.rem_euclid(MOD), MOD) as i128 % MOD as i128) as i64;
                curr_sb_val = (curr_sb_val as i128 * factor as i128 % MOD as i128) as i64;
            }
            curr_n_sb -= 1;
        }

        comb_p = next_comb_p;
        comb_q = next_comb_q;
        m += 1;

        if (comb_p == 0 && comb_q == 0) || (m > p && m > q) { break; }
    }

    println!("{}", ans);
}
