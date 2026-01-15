"""Project Euler Problem 503: Alice's Game.

Alice plays a game in which she repeatedly draws a card, but is only told
how many previous cards are larger than it. If she can stop at any time
and plays optimally to get the lowest score, find this expected score.
"""

from __future__ import annotations


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def solve() -> str:
    """Solve Problem 503."""
    N = 10**6
    ans = float(N)

    for n in range(N, 0, -1):
        d = (N + 1) / (n + 1)
        k = int(ans / d)
        ans = (tr(k) * d + (n - k) * ans) / n

    return f"{ans:.10f}"


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
