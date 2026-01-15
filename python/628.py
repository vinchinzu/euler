"""Project Euler Problem 628: Open chess positions.

Find the number of open chess positions on a NxN board consisting of N pawns
with exactly one pawn per row and column, such that a rook can move from the
bottom left corner to the top right.

The number of pawn positions is N!. We then subtract the positions where there
are k pawns on a diagonal connecting two sides of the board. There is one
position where all pawns are on a diagonal, and k! positions for the two
possible positions of (N - k) pawns on a diagonal. Finally, we double-counted
the positions where there are a pawns on one diagonal and b pawns on another
diagonal.
"""

from __future__ import annotations


def factorial(n: int, mod: int) -> int:
    """Compute n! modulo mod."""
    result = 1
    for i in range(1, n + 1):
        result = (result * i) % mod
    return result


def solve() -> int:
    """Solve Problem 628."""
    N = 10**8
    M = 1008691207

    factorials = [1] * (N + 1)
    for i in range(1, N + 1):
        factorials[i] = (factorials[i - 1] * i) % M

    ans = (factorials[N] - 1) % M
    for k in range(1, N):
        ans = (ans - 2 * factorials[k]) % M
    for k in range(N - 1):
        ans = (ans + (N - 1 - k) * factorials[k]) % M
    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
