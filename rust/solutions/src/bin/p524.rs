// Project Euler 524 - First Sort II
//
// Find Q(n, N) for N = 12^12 using DFS with pruning over permutations.

use std::sync::atomic::{AtomicU64, Ordering};

fn compute_n() -> u64 {
    let mut r = 1u64;
    for _ in 0..12 { r *= 12; }
    r
}

fn ilog2(n: u64) -> usize {
    let mut b = 0;
    while (1u64 << (b + 1)) <= n { b += 1; }
    b
}

fn factorial(n: usize) -> u64 {
    let mut r = 1u64;
    for i in 2..=n { r *= i as u64; }
    r
}

static ANSWER: AtomicU64 = AtomicU64::new(0);

fn helper(remaining: &mut Vec<usize>, rks: &mut Vec<usize>, order_index: u64, l_val: usize) {
    if ANSWER.load(Ordering::Relaxed) != 0 { return; }

    if remaining.is_empty() {
        if rks.is_empty() {
            ANSWER.store(order_index, Ordering::Relaxed);
        }
        return;
    }

    let rem_len = remaining.len();
    let rks_len = rks.len();

    // Pruning
    let check = rem_len.min(rks_len);
    for i in 0..check {
        if remaining[i] > rks[i] + i { return; }
    }

    for i in 0..rem_len {
        if ANSWER.load(Ordering::Relaxed) != 0 { break; }

        let el = remaining[i];
        let rank = el.wrapping_sub(i);

        let bit_index = rks.iter().position(|&r| r == rank);

        let target_rank = l_val - (rem_len - 1);
        if rank != target_rank {
            if let Some(bi) = bit_index {
                let saved_rk = rks[bi];
                rks.remove(bi);
                let saved_el = remaining.remove(i);

                helper(remaining, rks, order_index + i as u64 * factorial(rem_len - 1), l_val);

                remaining.insert(i, saved_el);
                rks.insert(bi, saved_rk);
            }
        } else {
            let saved_el = remaining.remove(i);
            helper(remaining, rks, order_index + i as u64 * factorial(rem_len - 1), l_val);
            remaining.insert(i, saved_el);
        }
    }
}

fn main() {
    let n = compute_n();
    let l_val = ilog2(n) + 1;

    let ranks: Vec<usize> = (0..=l_val)
        .filter(|&i| n & (1u64 << i) != 0)
        .collect();

    let mut remaining: Vec<usize> = (0..=l_val).collect();
    let mut rks = ranks.clone();

    ANSWER.store(0, Ordering::Relaxed);
    helper(&mut remaining, &mut rks, 1, l_val);

    println!("{}", ANSWER.load(Ordering::Relaxed));
}
