#!/usr/bin/env python3
"""
Project Euler Problem 852: Coins in a Box

A game involves a box containing N unfair coins and N fair coins.
Fair coins have p=0.5 probability of heads, unfair coins have p=0.75.

Game mechanics:
- Start with score 0
- Each round, randomly select a coin from the box
- Before guessing, may toss the coin multiple times (each toss costs 1 point)
- After deciding to stop tossing, guess if coin is fair or unfair
- Correct guess: +20 points, incorrect: -50 points
- Coin is revealed and discarded
- Game continues for 2N rounds until box is empty

Calculate S(N) = expected score when playing optimally.
Given S(1) = 20.558591, find S(50) rounded to 6 decimal places.
"""

import math

# Constants
REWARD = 20.0                      # Correct guess
PENALTY = -50.0                    # Wrong guess
COST_PER_TOSS = 1.0                # Cost per toss
MAX_TOSS = 300                     # Maximum tosses to consider


def compute_posterior(fair: int, unfair: int, heads: int, tails: int) -> float:
    """
    Compute posterior probability that current coin is fair given observations.
    """
    total_coins = fair + unfair
    if total_coins == 0:
        return 0.0

    total_tosses = heads + tails
    if total_tosses == 0:
        return fair / total_coins

    # Prior probability coin is fair
    prior_fair = fair / total_coins

    # Likelihoods
    # Use log likelihoods to prevent underflow for large MAX_TOSS?
    # For MAX_TOSS=200, 0.25^200 approx 1e-120, which fits in float.
    # 0.5^200 approx 1e-60.
    # Standard float handles 1e-308. So direct mult is fine.
    
    likelihood_fair = (0.5 ** total_tosses)
    likelihood_unfair = (0.75 ** heads) * (0.25 ** tails)

    # Bayes: P(fair|data) = P(data|fair) * P(fair) / P(data)
    numerator = likelihood_fair * prior_fair
    denominator = numerator + likelihood_unfair * (1 - prior_fair)

    return numerator / denominator if denominator > 0 else 0.5


def solve_current_coin(fair: int, unfair: int, global_memo: dict) -> float:
    """
    Compute the optimal expected value for a single coin given future values.
    """
    # local_memo for tosses: (h, t) -> value
    # We only need to keep the current and next layer of total_tosses to be even more efficient,
    # but keeping all (MAX_TOSS^2 entries) is fine (40K entries for 200^2).
    local_memo = {}
    
    # Pre-fetch future values from global memo
    # If we guess fair correctly (prob_fair), we are left with fair-1, unfair.
    # If we guess fair incorrectly (1-prob_fair), it was unfair, so we are left with fair, unfair-1.
    future_fair_remains = global_memo.get((fair-1, unfair), 0.0) if fair > 0 else 0.0
    future_unfair_remains = global_memo.get((fair, unfair-1), 0.0) if unfair > 0 else 0.0
    
    # Iterate backwards from max tosses
    for total_tosses in range(MAX_TOSS, -1, -1):
        for h in range(total_tosses + 1):
            t = total_tosses - h
            
            # Calc posterior
            prob_fair = compute_posterior(fair, unfair, h, t)
            
            # Calc guess values
            # Guess Fair
            ev_guess_fair = (prob_fair * (REWARD + future_fair_remains) + 
                             (1 - prob_fair) * (PENALTY + future_unfair_remains))
            
            # Guess Unfair
            ev_guess_unfair = ((1 - prob_fair) * (REWARD + future_unfair_remains) + 
                               prob_fair * (PENALTY + future_fair_remains))
            
            best_guess = max(ev_guess_fair, ev_guess_unfair)
            
            if total_tosses >= MAX_TOSS:
                local_memo[(h, t)] = best_guess
                continue
            
            # Calc toss value
            p_heads = prob_fair * 0.5 + (1 - prob_fair) * 0.75
            
            val_h = local_memo.get((h+1, t), -float('inf')) 
            val_t = local_memo.get((h, t+1), -float('inf'))
            
            ev_toss = p_heads * val_h + (1 - p_heads) * val_t - COST_PER_TOSS
            
            local_memo[(h, t)] = max(best_guess, ev_toss)
            
    return local_memo[(0, 0)]


def compute_s_n(n: int) -> float:
    """
    Compute S(N).
    """
    if n == 0:
        return 0.0

    # Global memo: (fair, unfair) -> expected_future_score
    global_memo = {}
    
    # Base case: 0 coins left -> 0 score (already handled by .get(..., 0.0))
    global_memo[(0, 0)] = 0.0

    # Fill DP table
    for total_coins in range(1, 2 * n + 1):
        # Iterate over possible counts of fair coins
        # fair can range from 0 to total_coins, but must be <= n
        # unfair = total_coins - fair, must be <= n
        start_fair = max(0, total_coins - n)
        end_fair = min(total_coins, n)
        
        for fair in range(start_fair, end_fair + 1):
            unfair = total_coins - fair
            
            val = solve_current_coin(fair, unfair, global_memo)
            global_memo[(fair, unfair)] = val
            
    return global_memo[(n, n)]


def main():
    """Main computation."""
    print("Starting computation...")

    # Test small values first
    print("Computing S(1)...")
    s1 = compute_s_n(1)
    print(f"S(1) = {s1:.6f}")
    expected_s1 = 20.558591
    if abs(s1 - expected_s1) < 1e-6:
        print("✓ S(1) matches expected value!")
    else:
        print(f"⚠ S(1) mismatch: expected {expected_s1:.6f}")
        # If S(1) doesn't match, don't proceed to S(50)
        return

    print("Computing S(2)...")
    print(f"S(2) = {compute_s_n(2):.6f}")

    print("Computing S(5)...")
    print(f"S(5) = {compute_s_n(5):.6f}")

    # For the actual problem
    print("Computing S(50)...")
    result = compute_s_n(50)
    print(f"S(50) = {result:.6f}")


if __name__ == "__main__":
    main()
