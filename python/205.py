"""Project Euler Problem 205: Dice Game.

Find the probability that Peter, when rolling D1 dice with face values in F1,
rolls a larger sum than Colin when rolling D2 dice with face values in F2.
"""

from __future__ import annotations

from itertools import product
from typing import List


def compute_probs(num_dice: int, values: List[int]) -> List[float]:
    """Compute probability distribution for sum of num_dice rolls."""
    max_sum = num_dice * max(values)
    probs = [0.0] * (max_sum + 1)
    prob_per_outcome = 1.0 / (len(values) ** num_dice)

    for rolled_values in product(values, repeat=num_dice):
        total = sum(rolled_values)
        probs[total] += prob_per_outcome

    return probs


def solve() -> float:
    """Solve Problem 205."""
    D1 = 9
    F1 = [1, 2, 3, 4]
    D2 = 6
    F2 = [1, 2, 3, 4, 5, 6]

    peter_probs = compute_probs(D1, F1)
    colin_probs = compute_probs(D2, F2)

    ans = 0.0
    for i in range(len(peter_probs)):
        for j in range(i):
            ans += peter_probs[i] * colin_probs[j]

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.7f}")
    return result


if __name__ == "__main__":
    main()
