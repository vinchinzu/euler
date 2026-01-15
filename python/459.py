"""Project Euler Problem 459: Flipping game.

A NxN board starts with all white squares. Two players take turns choosing a
W x H rectangle, where W is a perfect square, H is a triangular number, and
the top right corner is white, and flips all squares in that rectangle from
white to black or vice versa. Find the number of starting moves such that the
first player can guarantee being the one to flip the entire board black.
"""

from __future__ import annotations

from typing import List


def isq(n: int) -> int:
    """Perfect square."""
    return n * n


def itr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def solve() -> int:
    """Solve Problem 459."""
    N = 1_000_000
    L = 512

    # Step sizes for each dimension
    step_sizes = [[0] * N for _ in range(2)]
    for i in range(1, N):
        step_sizes[0][i] = isq(i)
        step_sizes[1][i] = itr(i)

    # Range nimbers
    range_nimbers = [[0] * (N + 1) for _ in range(2)]
    for i in range(2):
        for j in range(1, N + 1):
            used = [False] * L
            k = 1
            while step_sizes[i][k] <= j:
                used[
                    range_nimbers[i][j - 1]
                    ^ range_nimbers[i][j - step_sizes[i][k]]
                ] = True
                k += 1
            nimber = 0
            while used[nimber]:
                nimber += 1
            range_nimbers[i][j] = range_nimbers[i][j - 1] ^ nimber

    # Count ranges with each nimber
    counts = [[0] * L for _ in range(2)]
    for i in range(2):
        for j in range(1, N + 1):
            k = 1
            while step_sizes[i][k] <= j:
                nim_val = (
                    range_nimbers[i][j] ^ range_nimbers[i][j - step_sizes[i][k]]
                )
                counts[i][nim_val] += 1
                k += 1

    # Nim product
    nim_prod = [[0] * L for _ in range(L)]
    for a in range(L):
        for b in range(L):
            if (b & (b - 1)) > 0:
                low_b = b & -b
                nim_prod[a][b] = nim_prod[a][low_b] ^ nim_prod[a][low_b ^ b]
            elif (a & (a - 1)) > 0:
                low_a = a & -a
                nim_prod[a][b] = nim_prod[low_a][b] ^ nim_prod[low_a ^ a][b]
            else:
                used = [False] * N
                for ap in range(a):
                    for bp in range(b):
                        used[
                            nim_prod[ap][b]
                            ^ nim_prod[a][bp]
                            ^ nim_prod[ap][bp]
                        ] = True
                while used[nim_prod[a][b]]:
                    nim_prod[a][b] += 1

    target_nim = nim_prod[range_nimbers[0][N]][range_nimbers[1][N]]
    ans = 0
    for n0 in range(L):
        for n1 in range(L):
            if nim_prod[n0][n1] == target_nim:
                ans += counts[0][n0] * counts[1][n1]

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
