// Project Euler 917 - Grid DP with interesting indices
// Minimum cost path on N x N grid with compressed DP.
// N = 10^7, MOD = 998388889.
// Uses i128 for exact integer DP to avoid floating-point precision loss.

const N_VAL: usize = 10_000_000;
const MOD: u64 = 998_388_889;
const S1: u64 = 102_022_661;
const INF: i128 = i128::MAX / 2;

fn main() {
    // Generate s[1..2N]
    let mut s_arr = vec![0u64; 2 * N_VAL + 1];
    s_arr[1] = S1;
    let mut curr = S1;
    for i in 2..=(2 * N_VAL) {
        curr = (curr as u128 * curr as u128 % MOD as u128) as u64;
        s_arr[i] = curr;
    }

    // Fill a[1..N], b[1..N] and prefix sums
    let mut a_arr = vec![0u64; N_VAL + 1];
    let mut b_arr = vec![0u64; N_VAL + 1];
    let mut pa_arr = vec![0i64; N_VAL + 1];
    let mut pb_arr = vec![0i64; N_VAL + 1];

    let mut cpa: i64 = 0;
    let mut cpb: i64 = 0;
    for i in 1..=N_VAL {
        a_arr[i] = s_arr[2 * i - 1];
        b_arr[i] = s_arr[2 * i];
        cpa += a_arr[i] as i64;
        pa_arr[i] = cpa;
        cpb += b_arr[i] as i64;
        pb_arr[i] = cpb;
    }
    drop(s_arr);

    let k = 2000usize;

    let rows = get_interesting_indices(&a_arr, N_VAL, k);
    let cols = get_interesting_indices(&b_arr, N_VAL, k);

    let nr = rows.len();
    let nc = cols.len();

    // DP on compressed grid using i128 for exact arithmetic
    let mut dp = vec![INF; nr * nc];
    dp[0] = a_arr[rows[0]] as i128 + b_arr[cols[0]] as i128;

    for r in 0..nr {
        let real_r = rows[r];
        let val_a_r = a_arr[real_r] as i128;

        let has_down = r + 1 < nr;
        let mut diff_r: i128 = 0;
        let mut cost_a_segment: i128 = 0;
        if has_down {
            let next_r = rows[r + 1];
            diff_r = next_r as i128 - real_r as i128;
            cost_a_segment = pa_arr[next_r] as i128 - pa_arr[real_r] as i128;
        }

        for c in 0..nc {
            let curr_val = dp[r * nc + c];
            if curr_val >= INF {
                continue;
            }

            let real_c = cols[c];

            // Move Right
            if c + 1 < nc {
                let next_c = cols[c + 1];
                let cost_move = (next_c as i128 - real_c as i128) * val_a_r
                    + (pb_arr[next_c] as i128 - pb_arr[real_c] as i128);
                let new_val = curr_val + cost_move;
                if new_val < dp[r * nc + c + 1] {
                    dp[r * nc + c + 1] = new_val;
                }
            }

            // Move Down
            if has_down {
                let cost_move =
                    cost_a_segment + diff_r * b_arr[real_c] as i128;
                let new_val = curr_val + cost_move;
                if new_val < dp[(r + 1) * nc + c] {
                    dp[(r + 1) * nc + c] = new_val;
                }
            }
        }
    }

    let result = dp[(nr - 1) * nc + nc - 1];
    println!("{}", result);
}

fn get_interesting_indices(arr: &[u64], n: usize, k: usize) -> Vec<usize> {
    let mut indices = Vec::new();

    // Always include 1 and N
    indices.push(1);
    indices.push(n);

    // Prefix minima
    let mut curr_min = u64::MAX;
    for i in 1..=n {
        if arr[i] < curr_min {
            curr_min = arr[i];
            indices.push(i);
        }
    }

    // Suffix minima
    curr_min = u64::MAX;
    for i in (1..=n).rev() {
        if arr[i] < curr_min {
            curr_min = arr[i];
            indices.push(i);
        }
    }

    // Top K smallest values by threshold
    let threshold: u64 = 1_000_000;
    let mut candidates: Vec<usize> = (1..=n).filter(|&i| arr[i] < threshold).collect();

    // If too many, take K smallest by value
    if candidates.len() > k {
        candidates.sort_by_key(|&i| arr[i]);
        candidates.truncate(k);
    }

    indices.extend_from_slice(&candidates);

    // Sort and deduplicate
    indices.sort_unstable();
    indices.dedup();

    indices
}
