// Test program to understand P870 Stone Game IV
// Compute L(r) for various r values and find transition values

use std::collections::BTreeSet;

/// Compute L(r) (losing positions for second player wins) up to max_n
/// by brute force game analysis.
fn compute_l(r: f64, max_n: usize) -> BTreeSet<usize> {
    // For each pile size n, determine if it's a P-position (second player wins)
    // or N-position (first player wins).
    //
    // A position (n, last_move) is a P-position if all moves from it lead to N-positions.
    // A position is an N-position if there exists a move to a P-position.
    //
    // But L(r) is defined as: initial pile sizes where second player wins.
    // On the first move, player 1 can remove any k with 1 <= k < n.
    // Then the game continues with pile n-k and last_move = k.
    //
    // So n is in L(r) if for ALL first moves k (1 <= k < n),
    //   the resulting position (n-k, k) is an N-position for the player to move
    //   (which is player 2), meaning player 2 can win.
    //
    // Wait, let me reconsider. After player 1 removes k, pile has n-k stones,
    // player 2 moves next with constraint 1 <= move <= floor(r*k).
    //
    // n in L(r) means: for ALL k in [1, n-1], position (n-k, k) is winning for player 2.
    // i.e., for ALL k, player 2 can win from (remaining=n-k, last=k).
    //
    // A state (remaining, last) is winning for the current player if there exists
    // a move m in [1, min(remaining, floor(r*last))] such that (remaining-m, m) is
    // losing for the next player.
    //
    // A state (remaining, last) is losing if remaining=0 (current player can't move)
    // or all moves lead to winning states for opponent.
    // Actually: remaining=0 means previous player took the last stone. Current player
    // can't move, so current player loses. So (0, _) is a losing position.
    //
    // Actually wait: "Whoever cannot make a legal move loses."
    // If remaining > 0 but floor(r*last) = 0 (when r*last < 1), then player can't move
    // and loses. But r > 0 and last >= 1, so r*last >= r > 0. If r < 1, then floor(r*1)=0
    // for r < 1, so the player can't move.
    //
    // Hmm, but then for r = 0.5: after first player removes k from pile n,
    // remaining = n-k, player 2 can remove at most floor(0.5*k).
    // If k=1, floor(0.5) = 0, so player 2 can't move and loses.
    // But wait, L(0.5) = {1}, meaning second player wins when n=1?
    // When n=1, first player must remove k with 1 <= k < 1, impossible. So first player
    // can't move and loses. So n=1 is always in L(r).
    //
    // For n=2, r=0.5: first player removes 1. Remaining=1, floor(0.5*1)=0.
    // Player 2 can't make a legal move (needs 1 <= k <= 0). Player 2 loses.
    // So n=2 is NOT in L(0.5). Consistent with L(0.5)={1}.
    //
    // For n=2, r=1: first player removes 1. Remaining=1, floor(1*1)=1.
    // Player 2 removes 1. Remaining=0. Player 1 can't move. Player 1 loses.
    // So n=2 IS in L(1). Consistent with L(1) = {1,2,4,8,...}.

    // Let's use memoization. State: (remaining, last_move).
    // But the state space is huge. Let's limit it.

    // is_winning[remaining][last] = true if the current player to move wins
    // from state (remaining, last_move).
    // remaining can be 0..max_n, last can be 1..max_n.

    let mut is_winning = vec![vec![false; max_n + 1]; max_n + 1];
    // (0, last) for any last: current player can't move (no stones), loses.
    // So is_winning[0][*] = false. Already initialized.

    for remaining in 1..=max_n {
        for last in 1..=max_n {
            let max_take = (r * last as f64).floor() as usize;
            let max_take = max_take.min(remaining);
            // Can take 1..=max_take
            let mut can_win = false;
            for take in 1..=max_take {
                if !is_winning[remaining - take][take] {
                    can_win = true;
                    break;
                }
            }
            is_winning[remaining][last] = can_win;
        }
    }

    // Now determine L(r): n is in L(r) if for ALL first moves k in [1, n-1],
    // player 2 is in a winning position, i.e., is_winning[n-k][k] = true for all k.
    // Equivalently: n is in L(r) if there is NO k in [1, n-1] such that
    // is_winning[n-k][k] = false.
    // Wait: n in L(r) means second player wins. After first player removes k,
    // (n-k, k) is the state for player 2. Player 2 wins if is_winning[n-k][k] = true.
    // n in L(r) means for ALL k in [1, n-1], is_winning[n-k][k] = true.
    // i.e., no matter what player 1 does, player 2 wins.

    let mut result = BTreeSet::new();
    result.insert(1); // n=1: first player can't move, loses
    for n in 2..=max_n {
        let all_p2_wins = (1..n).all(|k| is_winning[n - k][k]);
        if all_p2_wins {
            result.insert(n);
        }
    }
    result
}

fn main() {
    // Test with known values
    println!("L(0.5) up to 20:");
    let l = compute_l(0.5, 20);
    println!("{:?}", l);

    println!("\nL(1) up to 40:");
    let l = compute_l(1.0, 40);
    println!("{:?}", l);

    println!("\nL(2) up to 40:");
    let l = compute_l(2.0, 40);
    println!("{:?}", l);

    // Now find transition values by scanning r
    println!("\nSearching for transition values...");
    let max_n = 30;
    let mut prev_l = compute_l(0.01, max_n);
    let mut transitions = Vec::new();

    // Scan r from 0.01 to 10 with fine steps
    let steps = 100000;
    for i in 1..=steps {
        let r = 0.01 + (i as f64) * 10.0 / (steps as f64);
        let l = compute_l(r, max_n);
        if l != prev_l {
            transitions.push(r);
            println!("Transition near r={:.6}: {:?} -> {:?}", r, prev_l, l);
        }
        prev_l = l;
    }

    println!("\nApproximate transitions: {:?}", transitions);
}
