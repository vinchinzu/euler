// Project Euler 912 - Digit DP with no consecutive three 1s
// F(N) = sum of n^2 for n <= N where s_n is odd.
// Binary strings without three consecutive 1s, ranked and summed mod 10^9+7.

const MOD: i64 = 1_000_000_007;

/// Count valid completions (no three consecutive 1s) with state s, r bits remaining
fn exact_dp(memo: &mut [[u64; 80]; 3], s: usize, r: usize) -> u64 {
    if memo[s][r] != u64::MAX {
        return memo[s][r];
    }
    if r == 0 {
        memo[s][r] = 1;
        return 1;
    }
    let mut result = exact_dp(memo, 0, r - 1);
    if s < 2 {
        result += exact_dp(memo, s + 1, r - 1);
    }
    memo[s][r] = result;
    result
}

#[derive(Clone, Copy)]
struct DPResult {
    cnt: i64,
    odd: i64,
    sop: i64,
    sopq: i64,
}

impl Default for DPResult {
    fn default() -> Self {
        DPResult { cnt: 0, odd: 0, sop: 0, sopq: 0 }
    }
}

/// Mod DP: returns (count, odd_count, sum_odd_positions, sum_odd_positions_squared)
fn dp(memo: &mut [[Option<DPResult>; 80]; 3], s: usize, r: usize) -> DPResult {
    if let Some(res) = memo[s][r] {
        return res;
    }
    let res;
    if r == 0 {
        let is_odd = if s >= 1 { 1i64 } else { 0i64 };
        res = DPResult { cnt: 1, odd: is_odd, sop: is_odd, sopq: is_odd };
    } else {
        let l = dp(memo, 0, r - 1);
        let r_val = if s < 2 {
            dp(memo, s + 1, r - 1)
        } else {
            DPResult::default()
        };

        let cnt = (l.cnt + r_val.cnt) % MOD;
        let odd = (l.odd + r_val.odd) % MOD;
        let cl = l.cnt % MOD;
        let sop = (l.sop + r_val.sop + cl % MOD * r_val.odd % MOD) % MOD;
        let sopq = ((l.sopq + r_val.sopq
            + 2 * cl % MOD * r_val.sop % MOD
            + cl * cl % MOD * r_val.odd % MOD) % MOD + MOD) % MOD;

        res = DPResult { cnt, odd, sop, sopq };
    }
    memo[s][r] = Some(res);
    res
}

fn main() {
    let n: u64 = 10_000_000_000_000_000; // 10^16

    let mut exact_memo = [[u64::MAX; 80]; 3];
    let mut mod_memo = [[None::<DPResult>; 80]; 3];

    // Precompute
    for b in 1..80 {
        exact_dp(&mut exact_memo, 1, b - 1);
    }

    let mut remaining_n = n;
    let mut r_offset: u64 = 0;
    let mut ans: i64 = 0;

    for b in 1..80 {
        let total_b = exact_dp(&mut exact_memo, 1, b - 1);

        if total_b <= remaining_n {
            let d = dp(&mut mod_memo, 1, b - 1);
            let r_off_mod = (r_offset % MOD as u64) as i64;
            let contribution = (d.odd * r_off_mod % MOD * r_off_mod % MOD
                + 2 * r_off_mod % MOD * d.sop % MOD
                + d.sopq) % MOD;
            ans = (ans + contribution) % MOD;
            r_offset += total_b;
            remaining_n -= total_b;
        } else {
            // Partial processing of b-bit numbers
            let mut s: usize = 1;
            let mut local_offset: u64 = 0;

            for bit_pos in (0..=(b as i32 - 2)).rev() {
                let bit_pos = bit_pos as usize;
                let cnt_0 = exact_dp(&mut exact_memo, 0, bit_pos);

                if remaining_n <= cnt_0 {
                    s = 0;
                    continue;
                } else {
                    if cnt_0 > 0 {
                        let d = dp(&mut mod_memo, 0, bit_pos);
                        let r_off_mod = ((r_offset + local_offset) % MOD as u64) as i64;
                        let contribution = (d.odd * r_off_mod % MOD * r_off_mod % MOD
                            + 2 * r_off_mod % MOD * d.sop % MOD
                            + d.sopq) % MOD;
                        ans = (ans + contribution) % MOD;
                    }
                    remaining_n -= cnt_0;
                    local_offset += cnt_0;

                    if s >= 2 {
                        break;
                    }
                    s += 1;
                }
            }

            if remaining_n > 0 {
                let is_odd = s >= 1;
                if is_odd {
                    let rank = ((r_offset + local_offset + 1) % MOD as u64) as i64;
                    ans = (ans + rank * rank % MOD) % MOD;
                }
                let _ = remaining_n;
            }
            break;
        }
    }

    ans = ((ans % MOD) + MOD) % MOD;
    println!("{}", ans);
}
