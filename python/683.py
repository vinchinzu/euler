"""Project Euler Problem 683: The Chase II.

In a game of n players and n-1 rounds, in each round two dice are each given to
a random player, and on each turn both players roll the die and each one passes
the die to the left with probability 1/3, to the right with probability 1/3,
and keeps it with probability 1/3. The round ends when both die land on the
same player after k turns, and that player puts k^E money in the pot and is
eliminated for future rounds. Find the expected number of money in the pot at
the end of the game.

Suppose a round has n players and let P_{d,k} be the probability that if the
dice start d players apart (0≤d≤⌊n/2⌋), then the round will end in k turns.
Then if k=0, P_{d,k} = [d=0]. Otherwise, P_{d,k} = Σ_t 1/9 P_{t(d),k-1},
where t ranges over all 9 possibilities of how the dice will move, and t(d) is
the new distance between the dice.

Let X_{d,e} = Σ_{k=0}^∞ P_{d,k} k^e. For e=0, this is just the sum of the
probabilities over all d and k, which is always 1. If d>0, then we have

X_{d,e} = Σ_{k=1}^∞ Σ_t 1/9 P_{t(d),k-1} k^e
        = 1/9 Σ_t Σ_{k=0}^∞ P_{t(d),k} (k+1)^e
        = 1/9 Σ_t Σ_{e'=0}^e nCr(e,e') X_{t(d),e'}.

So for each e, if we've computed all X_{d,e'} for e'<e, then we can compute
all the X_{d,e} for 0≤d≤⌊n/2⌋ as a system of linear equations. Also, since
t(d) never differs from d by more than 2, the system of equations can be
expressed as a band matrix with maximum offset 2, and so the equations can be
solved in linear time.

Finally, there is a 1/n chance of the second die starting any particular number
of players clockwise from the first die. Taking d to be the minimum distance
(either clockwise or counter-clockwise) and averaging X_{d,E} over all n
possibilities gives the expected money paid per round. Summing over all rounds
(2≤n≤N) gives the answer.
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
            if i + j < n:
                ratio = A.get(i + j, -j) / A.get(i, 0)
                for k in range(-A.max_diff, A.max_diff + 1):
                    if 0 <= i + j + k < n:
                        A.add(i + j, k - j, -ratio * A.get(i, k))
                B[i + j] -= ratio * B[i]

    # Back substitution
    res = [0.0] * n
    for i in range(n - 1, -1, -1):
        res[i] = B[i]
        for j in range(1, A.max_diff + 1):
            if i + j < n:
                res[i] -= A.get(i, j) * res[i + j]
        res[i] /= A.get(i, 0)

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


def main() -> int:
    """Main entry point."""
    result = solve()
    print(f"{result:.8e}".replace("+", ""))
    return int(result)


if __name__ == "__main__":
    main()
