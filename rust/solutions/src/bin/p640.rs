// Project Euler 640 - Shut the Box
// Value iteration over 4096 states (cards 1..12)

const ND: usize = 6;
const CARDS: usize = 12;
const GOAL: usize = (1 << CARDS) - 1;
const NUM_STATES: usize = 1 << CARDS;

fn main() {
    let mut e = vec![100.0f64; NUM_STATES];
    e[GOAL] = 0.0;

    for _ in 0..1_000_000 {
        let mut max_change = 0.0f64;
        for s in 0..NUM_STATES {
            if s == GOAL { continue; }
            let mut total = 0.0;
            for x in 1..=ND {
                for y in 1..=ND {
                    let opts = [x, y, x + y];
                    let mut best = 1e18f64;
                    for &c in &opts {
                        if c >= 1 && c <= CARDS {
                            let val = e[s ^ (1 << (c - 1))];
                            if val < best { best = val; }
                        }
                    }
                    total += best;
                }
            }
            let new_val = 1.0 + total / (ND * ND) as f64;
            let change = (new_val - e[s]).abs();
            if change > max_change { max_change = change; }
            e[s] = new_val;
        }
        if max_change < 1e-12 { break; }
    }

    println!("{:.6}", e[0]);
}
