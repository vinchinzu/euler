"""
Project Euler Problem 951: Card Game

F(26) - fair starting configurations

Problem Description:
Two players play a game using a deck of 2n cards: n red and n black.
Initially the deck is shuffled into one of the C(2n, n) possible starting configurations.
Play proceeds, alternating turns.
1. Remove top card, note color.
2. If next card exists and is same color, toss fair coin.
   Heads: remove next card too.
   Tails: leave it.

Player removing final card wins.

A configuration is fair if winning prob is 0.5.
F(n) is number of fair configurations.

Analysis:
The win probability P satisfies a linear recurrence based on block decomposition of the card sequence.
A block of identical colors of length k transforms the probability p (of winning from next state) to f(k, p).
f(k, p) = A_k * p + B_k.
We found that only for k=2 (length 2 block), the transformation resets the probability to 0.5 regardless of p (specifically A_2=0, B_2=0.5).
Also f(k, 0.5) = 0.5 for all k.
Thus, a configuration is fair iff it contains at least one block of length 2 in its run-length encoding.
(The end state has p=0, so we need a reset to reach 0.5).

We count "Bad" configurations: those with no block of length 2.
Then F(n) = Total - Bad.
"""

import math
import os

def nCr(n, r):
    return math.factorial(n) // (math.factorial(r) * math.factorial(n - r))

def solve(n):
    """
    Calculates F(n) for the given n.
    F(n) = C(2n, n) - (Number of configurations with no run of length 2)
    """

    # dp[state] = count
    # state is (r, b, length, color)
    # color: 0 for Red, 1 for Black
    # We only care about avoiding run length == 2 when a run finishes.

    # Optimization: Iterate layer by layer based on total cards used (r+b).
    # Since we only need current layer to compute next, we can save memory, though for n=26 full dict is fine.

    dp = {}
    # Initial states: 1 Red card or 1 Black card
    dp[(1, 0, 1, 0)] = 1
    dp[(0, 1, 1, 1)] = 1

    # Iterate total cards k from 1 to 2n-1
    for k in range(1, 2 * n):
        new_dp = {}
        for state, count in dp.items():
            r, b, length, color = state

            # Try adding Red (0)
            if r + 1 <= n:
                if color == 0:
                    # Extend Red run
                    new_state = (r + 1, b, length + 1, 0)
                    new_dp[new_state] = new_dp.get(new_state, 0) + count
                else:
                    # Switch from Black to Red
                    # Check if finished Black run is valid (len != 2)
                    if length != 2:
                        new_state = (r + 1, b, 1, 0)
                        new_dp[new_state] = new_dp.get(new_state, 0) + count

            # Try adding Black (1)
            if b + 1 <= n:
                if color == 1:
                    # Extend Black run
                    new_state = (r, b + 1, length + 1, 1)
                    new_dp[new_state] = new_dp.get(new_state, 0) + count
                else:
                    # Switch from Red to Black
                    # Check if finished Red run is valid (len != 2)
                    if length != 2:
                        new_state = (r, b + 1, 1, 1)
                        new_dp[new_state] = new_dp.get(new_state, 0) + count
        dp = new_dp

    # Sum valid final states
    bad_count = 0
    for state, count in dp.items():
        r, b, length, color = state
        # We only care about states that used all cards
        if r == n and b == n:
            # The final run must also not be length 2
            if length != 2:
                bad_count += count

    total = nCr(2 * n, n)
    fair = total - bad_count
    return fair

def main():
    """Main function."""
    n = 26
    result = solve(n)
    print(f"F({n}) = {result}")

    # Write answer to file
    with open(os.path.join(os.path.dirname(__file__), 'answer.txt'), 'w') as f:
        f.write(str(result))

    return result

if __name__ == "__main__":
    main()
