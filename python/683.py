"""Project Euler Problem 683: The Chase II.

In a game of n players and n-1 rounds, in each round two dice are each given to
a random player, and on each turn both players roll the die and each one passes
the die to the left with probability 1/3, to the right with probability 1/3,
and keeps it with probability 1/3. The round ends when both die land on the
same player after k turns, and that player puts k^E money in the pot and is
eliminated for future rounds. Find the expected number of money in the pot at
the end of the game.
"""

from __future__ import annotations

from collections import defaultdict


def nCr(n: int, r: int) -> float:
    """Binomial coefficient."""
    if r < 0 or r > n:
        return 0.0
    if r == 0 or r == n:
        return 1.0
    result = 1.0
    for i in range(min(r, n - r)):
        result = result * (n - i) / (i + 1)
    return result


class BandMatrix:
    """Band matrix for linear system solving."""

    def __init__(self, n: int, max_diff: int) -> None:
        """Initialize band matrix."""
        self.max_diff = max_diff
        self.values: dict[int, list[float]] = {}
        for d in range(-max_diff, max_diff + 1):
            self.values[d] = [0.0] * n

    def get(self, row: int, diff: int) -> float:
        """Get value at (row, row+diff)."""
        return self.values[diff][row]

    def add(self, row: int, diff: int, val: float) -> None:
        """Add value to (row, row+diff)."""
        self.values[diff][row] += val


def linear_system(A: BandMatrix, B: list[float]) -> list[float]:
    """Solve linear system Ax = B."""
    n = len(B)
    # Forward elimination
    for i in range(n):
        for j in range(1, A.max_diff + 1):
            if i + j >= n:
                break
            pivot = A.get(i, 0)
            if abs(pivot) < 1e-15:
                continue
            ratio = A.get(i + j, -j) / pivot
            for k in range(A.max_diff + 1):
                A.add(i + j, k - j, -ratio * A.get(i, k))
            B[i + j] -= ratio * B[i]

    # Back substitution
    res = [0.0] * n
    for i in range(n - 1, -1, -1):
        res[i] = B[i]
        for j in range(1, A.max_diff + 1):
            if i + j < n:
                res[i] -= A.get(i, j) * res[i + j]
        pivot = A.get(i, 0)
        if abs(pivot) > 1e-15:
            res[i] /= pivot

    return res


def expected_round_money(n: int, E: int) -> float:
    """Compute expected money for one round."""
    l = n // 2 + 1
    X: list[list[float]] = [[0.0] * l for _ in range(E + 1)]

    # Initialize X[0][d] = 1 for all d
    for d in range(l):
        X[0][d] = 1.0

    for e in range(1, E + 1):
        A = BandMatrix(l, 2)
        B = [0.0] * l

        for d in range(l):
            A.add(d, 0, 1.0)

        for d in range(1, l):
            for da in range(-1, 2):
                for db in range(-1, 2):
                    new_d = abs((d + da + db) % n)
                    new_d = min(new_d, n - new_d)
                    A.add(d, new_d - d, -1.0 / 9.0)
                    for ep in range(e):
                        B[d] += nCr(e, ep) * X[ep][new_d] / 9.0

        X[e] = linear_system(A, B)

    expected = 0.0
    for d in range(n):
        expected += X[E][min(d, n - d)]
    return expected / n


def solve() -> float:
    """Solve Problem 683."""
    N = 500
    E = 2

    ans = 0.0
    for n in range(2, N + 1):
        ans += expected_round_money(n, E)

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(f"{result:.8e}".replace("+", ""))


if __name__ == "__main__":
    main()
