// Project Euler 393: Ant Migration
const GRID: usize = 10;
const STATE_SIZE: usize = 59049 * 3; // 3^10 * 3

fn encode_state(top_flows: &[i32; GRID], left_flow: i32) -> usize {
    let mut key = 0usize;
    for i in 0..GRID {
        key = key * 3 + (top_flows[i] + 1) as usize;
    }
    key * 3 + (left_flow + 1) as usize
}

fn decode_state(mut key: usize) -> ([i32; GRID], i32) {
    let left_flow = (key % 3) as i32 - 1;
    key /= 3;
    let mut top_flows = [0i32; GRID];
    for i in (0..GRID).rev() {
        top_flows[i] = (key % 3) as i32 - 1;
        key /= 3;
    }
    (top_flows, left_flow)
}

fn main() {
    // Pre-compute zero-flow combinations
    let mut zero_flows: Vec<Vec<(i32, i32)>> = vec![Vec::new(); 9];
    for top in -1..=1i32 {
        for left in -1..=1i32 {
            let idx = ((top + 1) * 3 + (left + 1)) as usize;
            for bottom in -1..=1i32 {
                for right in -1..=1i32 {
                    let mut f = [top, left, bottom, right];
                    f.sort();
                    if f == [-1, 0, 0, 1] {
                        zero_flows[idx].push((bottom, right));
                    }
                }
            }
        }
    }

    let mut counts = vec![0i64; STATE_SIZE];
    let init_top = [0i32; GRID];
    counts[encode_state(&init_top, 0)] = 1;

    for _row in 0..GRID {
        for col in 0..GRID {
            let mut new_counts = vec![0i64; STATE_SIZE];

            for s in 0..STATE_SIZE {
                if counts[s] == 0 { continue; }

                let (top_flows, left_flow) = decode_state(s);
                let top_flow = top_flows[0];
                let fidx = ((top_flow + 1) * 3 + (left_flow + 1)) as usize;

                for &(bottom_flow, right_flow) in &zero_flows[fidx] {
                    if col == GRID - 1 && right_flow != 0 { continue; }

                    let mut new_top = [0i32; GRID];
                    for i in 0..GRID - 1 {
                        new_top[i] = top_flows[i + 1];
                    }
                    new_top[GRID - 1] = bottom_flow;

                    new_counts[encode_state(&new_top, right_flow)] += counts[s];
                }
            }

            counts = new_counts;
        }
    }

    let final_top = [0i32; GRID];
    println!("{}", counts[encode_state(&final_top, 0)]);
}
