"""Project Euler Problem 497: Drunken Tower of Hanoi.

In a variant of the Tower of Hanoi, the three rods are placed at positions
a, b, c on a row n units long. Bob randomly moves left or right along the row,
but picks up or puts down a disk whenever it is optimal to do so. Find Σ_n
E(n, 10^n, 3^n, 6^n, 9^n).
"""

from __future__ import annotations

from typing import List


def solve() -> int:
    """Solve Problem 497."""
    N = 10_000
    M = 10**9

    # numMoves[n][start][end][s][e] = number of moves from s to e
    num_moves: List[List[List[List[List[int]]]]] = [
        [[[[0 for _ in range(3)] for _ in range(3)] for _ in range(3)] for _ in range(3)]
        for _ in range(N + 1)
    ]

    # Base case: n=1
    for start in range(3):
        for end in range(3):
            if start != end:
                num_moves[1][start][end][start][end] = 1

    # Recurrence
    for n in range(2, N + 1):
        for start in range(3):
            for end in range(3):
                if start != end:
                    other = 3 - start - end
                    for s in range(3):
                        for e in range(3):
                            num_moves[n][start][end][s][e] += num_moves[n - 1][
                                start
                            ][other][s][e]
                    num_moves[n][start][end][other][start] += 1
                    num_moves[n][start][end][start][end] += 1
                    num_moves[n][start][end][end][other] += 1
                    for s in range(3):
                        for e in range(3):
                            num_moves[n][start][end][s][e] += num_moves[n - 1][
                                other
                            ][end][s][e]

    ans = 0
    for n in range(1, N + 1):
        k = 10**n
        a = 3**n
        b = 6**n
        c = 9**n

        # Expected moves
        E = 0
        for s in range(3):
            for e in range(3):
                positions = [a, b, c]
                moves = (
                    abs(positions[e] - positions[s]) ** 2
                    - abs(positions[s] - positions[s]) ** 2
                )
                E += num_moves[n][0][2][s][e] * moves

        ans = (ans + E) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
