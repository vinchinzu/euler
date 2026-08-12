// Problem 993: Banana Beaver
// Simulate BB prefix until period starts at n=514, period 71, then closed form for 10^18.

use std::collections::HashSet;

const PERIOD_START: usize = 514;
const PERIOD: usize = 71;
const DELTA_PATTERN: [i64; 71] = [
    17, -2, -8, -2, -2, -14, -2, -2, -17, -8, -5, -8, -5, -2, -2, -5, -8, 50, -8, 23, -13, -2, 67,
    -5, -2, -2, -5, -8, -5, 21, 29, -11, -2, -2, 6, -11, 31, -2, -11, 17, -2, -8, -2, -2, -14, -2,
    -2, -17, -8, -5, -8, -8, 8, -13, -5, -2, -2, -5, -2, -11, -8, -8, -5, -2, -11, -8, -8, -5, -2,
    -11, 216,
];

fn pattern_sum() -> i64 {
    DELTA_PATTERN.iter().sum()
}

/// Apply one game step. Returns None if the game halts before making a move.
fn step_state(pos: i64, carry: i64, bananas: &HashSet<i64>) -> Option<(i64, i64, HashSet<i64>)> {
    let has_x = bananas.contains(&pos);
    let has_x1 = bananas.contains(&(pos + 1));

    if has_x && has_x1 {
        let mut bananas = bananas.clone();
        bananas.remove(&(pos + 1));
        return Some((pos - 1, carry + 1, bananas));
    }

    if has_x && !has_x1 {
        let mut bananas = bananas.clone();
        bananas.remove(&pos);
        return Some((pos + 2, carry + 1, bananas));
    }

    if !has_x && has_x1 {
        let mut bananas = bananas.clone();
        bananas.remove(&(pos + 1));
        bananas.insert(pos);
        return Some((pos + 2, carry, bananas));
    }

    if carry >= 3 {
        let mut bananas = bananas.clone();
        bananas.insert(pos - 1);
        bananas.insert(pos);
        bananas.insert(pos + 1);
        return Some((pos - 2, carry - 3, bananas));
    }

    None
}

/// Directly simulate BB(0), BB(1), ..., BB(limit).
fn simulate_bb_values(limit: usize) -> Vec<i64> {
    let mut bb = vec![0i64];
    let mut pos = 0i64;
    let mut carry = 0i64;
    let mut bananas: HashSet<i64> = HashSet::new();

    for _n in 1..=limit {
        carry += 1;
        loop {
            match step_state(pos, carry, &bananas) {
                None => {
                    bb.push(pos);
                    break;
                }
                Some((p, c, b)) => {
                    pos = p;
                    carry = c;
                    bananas = b;
                }
            }
        }
    }
    bb
}

fn build_prefix() -> Vec<i64> {
    simulate_bb_values(PERIOD_START + PERIOD)
}

fn bb(n: u64, bb_prefix: &[i64]) -> i64 {
    if n as usize <= PERIOD_START {
        return bb_prefix[n as usize];
    }

    let remaining = n - PERIOD_START as u64;
    let whole_periods = remaining / PERIOD as u64;
    let tail = (remaining % PERIOD as u64) as usize;
    let psum = pattern_sum();
    let tail_sum: i64 = DELTA_PATTERN[..tail].iter().sum();

    bb_prefix[PERIOD_START] + whole_periods as i64 * psum + tail_sum
}

fn main() {
    let bb_prefix = build_prefix();
    println!("{}", bb(10u64.pow(18), &bb_prefix));
}
