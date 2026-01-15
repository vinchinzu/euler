"""Project Euler Problem 744: What? Where? When?.

In a game, X questions are asked in order (where X is uniformly chosen from
0 to 2N inclusive), and each one answered correctly with probability P.
Find the probability that either N questions will be answered correctly or
N questions will be answered incorrectly before the game ends.

We expect P*X questions to be answered correctly. If P*X >> X-N, then it is
very likely that the game ends, and if P*X << X-N, then it is very unlikely.
This cutoff point is at N/(1-P), and the game ends when N/(1-P) ≤ X ≤ 2N.
So the probability the game ends is very close to 1 - 1/(2(1-P)), which is
accurate enough for this problem.
"""

from __future__ import annotations


def solve() -> float:
    """Solve Problem 744."""
    p = 0.4999
    ans = 1.0 - 1.0 / 2.0 / (1.0 - p)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return int(result * 1e10)


if __name__ == "__main__":
    main()
