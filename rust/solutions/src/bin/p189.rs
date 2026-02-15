// Project Euler 189: Tri-colouring a triangular grid

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

    // Flat arrays for DP states: key = last_colour * max_states + up_encoding
    let rs_size = 3 * max_states;

    // Preallocate working buffers
    let mut rs_vals = vec![0i64; rs_size];
    let mut rs_active: Vec<usize> = Vec::with_capacity(rs_size);
    let mut rs2_vals = vec![0i64; rs_size];
    let mut rs2_active: Vec<usize> = Vec::with_capacity(rs_size);

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

            // Clear rs
            for &idx in &rs_active {
                rs_vals[idx] = 0;
            }
            rs_active.clear();

            // Position 0: upward triangle, no left neighbour
            for c in 0..3usize {
                let key = c * max_states + c;
                rs_vals[key] = 1;
                rs_active.push(key);
            }

            let mut up_count = 1usize;

            for pos in 1..n_positions {
                // Clear rs2
                for &idx in &rs2_active {
                    rs2_vals[idx] = 0;
                }
                rs2_active.clear();

                if pos % 2 == 1 {
                    // Downward triangle
                    let k = pos / 2;
                    let above_c = prev_up_colours[k];
                    for &key in &rs_active {
                        let cnt = rs_vals[key];
                        if cnt == 0 { continue; }
                        let last_c = key / max_states;
                        let up_enc = key % max_states;
                        for c in 0..3usize {
                            if c == last_c || c == above_c { continue; }
                            let nk = c * max_states + up_enc;
                            if rs2_vals[nk] == 0 {
                                rs2_active.push(nk);
                            }
                            rs2_vals[nk] += cnt;
                        }
                    }
                } else {
                    // Upward triangle
                    let p3 = pow3[up_count];
                    for &key in &rs_active {
                        let cnt = rs_vals[key];
                        if cnt == 0 { continue; }
                        let last_c = key / max_states;
                        let up_enc = key % max_states;
                        for c in 0..3usize {
                            if c == last_c { continue; }
                            let nk = c * max_states + up_enc + c * p3;
                            if rs2_vals[nk] == 0 {
                                rs2_active.push(nk);
                            }
                            rs2_vals[nk] += cnt;
                        }
                    }
                    up_count += 1;
                }

                // Swap rs and rs2
                std::mem::swap(&mut rs_vals, &mut rs2_vals);
                std::mem::swap(&mut rs_active, &mut rs2_active);
            }

            // Collect: sum over last_colour, group by up_encoding
            for &key in &rs_active {
                let cnt = rs_vals[key];
                if cnt == 0 { continue; }
                let up_enc = key % max_states;
                new_dp[up_enc] += cnt * prev_count;
            }

            // Clear rs for next iteration
            for &idx in &rs_active {
                rs_vals[idx] = 0;
            }
            rs_active.clear();
        }

        dp = new_dp;
    }

    let total: i64 = dp.iter().sum();
    println!("{total}");
}
