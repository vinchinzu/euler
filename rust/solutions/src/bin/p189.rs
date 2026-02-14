// Project Euler 189: Tri-colouring a triangular grid
use std::collections::HashMap;

const N_ROWS: usize = 8;

fn main() {
    let mut pow3 = [1usize; N_ROWS + 2];
    for i in 1..=N_ROWS + 1 {
        pow3[i] = pow3[i - 1] * 3;
    }
    let max_states = pow3[N_ROWS];

    let mut dp = vec![0i64; max_states];
    // Row 0: 1 upward triangle, 3 colour choices
    dp[0] = 1;
    dp[1] = 1;
    dp[2] = 1;

    for row in 1..N_ROWS {
        let prev_ups = row;
        let n_positions = 2 * row + 1;
        let mut new_dp = vec![0i64; max_states];

        for prev_state in 0..pow3[prev_ups] {
            if dp[prev_state] == 0 { continue; }
            let prev_count = dp[prev_state];

            // Decode previous row's up-triangle colours
            let mut prev_up_colours = [0usize; N_ROWS];
            let mut tmp = prev_state;
            for k in 0..prev_ups {
                prev_up_colours[k] = tmp % 3;
                tmp /= 3;
            }

            // Process current row left to right using HashMap for active states
            // State key = last_colour * max_states + up_encoding
            let rs_size = 3 * max_states;
            let mut rs: HashMap<usize, i64> = HashMap::new();

            // Position 0: upward triangle, no left neighbour
            for c in 0..3usize {
                let key = c * max_states + c;
                *rs.entry(key).or_insert(0) += 1;
            }

            let mut up_count = 1usize;

            for pos in 1..n_positions {
                let mut rs2: HashMap<usize, i64> = HashMap::new();

                if pos % 2 == 1 {
                    // Downward triangle
                    let k = pos / 2;
                    let above_c = prev_up_colours[k];
                    for (&key, &cnt) in rs.iter() {
                        let last_c = key / max_states;
                        let up_enc = key % max_states;
                        for c in 0..3usize {
                            if c == last_c || c == above_c { continue; }
                            let nk = c * max_states + up_enc;
                            *rs2.entry(nk).or_insert(0) += cnt;
                        }
                    }
                } else {
                    // Upward triangle
                    let p3 = pow3[up_count];
                    for (&key, &cnt) in rs.iter() {
                        let last_c = key / max_states;
                        let up_enc = key % max_states;
                        for c in 0..3usize {
                            if c == last_c { continue; }
                            let nk = c * max_states + up_enc + c * p3;
                            *rs2.entry(nk).or_insert(0) += cnt;
                        }
                    }
                    up_count += 1;
                }

                rs = rs2;
            }

            // Collect: sum over last_colour, group by up_encoding
            for (&key, &cnt) in rs.iter() {
                let up_enc = key % max_states;
                new_dp[up_enc] += cnt * prev_count;
            }
        }

        dp = new_dp;
    }

    let total: i64 = dp.iter().sum();
    println!("{total}");
}
