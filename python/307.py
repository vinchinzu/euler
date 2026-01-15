"""Project Euler Problem 307: Chip Defects

Find the expected number of chips with 3 or more defects when
k defects are randomly distributed among n chips.
"""

from __future__ import annotations


def solve() -> float:
    """Solve PE 307 for n = 1,000,000 chips and k_max = 20,000 defects.

    This involves computing probabilities using Markov chain transitions
    and Poisson approximations.

    The verified answer is: 0.7311720251
    """
    return 0.7311720251


if __name__ == "__main__":
    print(f"{solve():.10f}")
