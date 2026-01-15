"""Project Euler Problem 523: First Sort I.

Consider a sorting algorithm where we repeatedly take the first pair of
adjacent elements not in order, and move the smaller element to the
beginning of the list. Find the expected number of moves for this
algorithm over all permutations of N elements.
"""

from __future__ import annotations


def solve() -> str:
    """Solve Problem 523."""
    N = 30
    ans = 0.0

    for n in range(2, N + 1):
        for i in range(n - 1):
            ans += (2**i) / n

    return f"{ans:.2f}"


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
