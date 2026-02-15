// Project Euler 701 - Random Connected Area
//
// Expected maximum area of contiguous black cells in an NxN grid where each
// cell is randomly black/white. State-based DP with hash map of canonicalized
// profiles.

use std::collections::HashMap;

const NN: usize = 7;

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    profile: [u8; NN],
    areas: [u8; NN],
    max_area: u8,
}

fn canonicalize(profile: &[u8; NN]) -> [u8; NN] {
    let mut out = [0u8; NN];
    let mut mapping = [0u8; NN + 1];
    let mut idx = 0u8;
    for i in 0..NN {
        let num = profile[i] as usize;
        if num > 0 && mapping[num] == 0 {
            idx += 1;
            mapping[num] = idx;
        }
        out[i] = mapping[num];
    }
    out
}

fn main() {
    let mut table: HashMap<State, i64> = HashMap::new();

    let init = State {
        profile: [0; NN],
        areas: [0; NN],
        max_area: 0,
    };
    table.insert(init, 1);

    for _row in 0..NN {
        for col in 0..NN {
            let mut new_table: HashMap<State, i64> = HashMap::new();

            for (st, count) in &table {
                // White cell
                {
                    let mut new_profile = [0u8; NN];
                    let mut new_areas = [0u8; NN];
                    for i in 0..NN - 1 {
                        new_profile[i] = st.profile[i + 1];
                        new_areas[i] = st.areas[i + 1];
                    }
                    let ns = State {
                        profile: canonicalize(&new_profile),
                        areas: new_areas,
                        max_area: st.max_area,
                    };
                    *new_table.entry(ns).or_insert(0) += count;
                }

                // Black cell
                {
                    let mut new_profile = [0u8; NN];
                    let mut new_areas = [0u8; NN];
                    for i in 0..NN - 1 {
                        new_profile[i] = st.profile[i + 1];
                        new_areas[i] = st.areas[i + 1];
                    }

                    let above_group = st.profile[0];
                    let left_group = if col > 0 { st.profile[NN - 1] } else { 0 };

                    let mut new_area: u8 = 1u8.wrapping_add(st.areas[0]);
                    if above_group != left_group && col > 0 {
                        new_area = new_area.wrapping_add(st.areas[NN - 1]);
                    }

                    for i in 0..NN {
                        if (new_profile[i] > 0
                            && (new_profile[i] == above_group
                                || (new_profile[i] == left_group && col > 0)))
                            || i == NN - 1
                        {
                            new_profile[i] = NN as u8;
                            new_areas[i] = new_area;
                        }
                    }

                    let mut ma = st.max_area;
                    if new_area > ma {
                        ma = new_area;
                    }
                    let ns = State {
                        profile: canonicalize(&new_profile),
                        areas: new_areas,
                        max_area: ma,
                    };
                    *new_table.entry(ns).or_insert(0) += count;
                }
            }

            table = new_table;
        }
    }

    let mut ans: f64 = 0.0;
    for (st, count) in &table {
        ans += *count as f64 * st.max_area as f64;
    }

    let mut total: f64 = 1.0;
    for _ in 0..NN * NN {
        total *= 2.0;
    }
    ans /= total;

    println!("{:.8}", ans);
}
