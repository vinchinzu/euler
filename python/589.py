"""Project Euler Problem 589: Pooh-sticks game.

In a game of Pooh-sticks, it takes t seconds for the stick to go under
the bridge, where t is a uniformly random integer between parameters m
and n inclusive. It takes k seconds to retrieve the stick and drop it
again. Find the expected number of seconds, starting from when two sticks
are initially dropped, until one of them emerges from under the bridge
strictly more than one lap ahead of the other one.

Let E(d) be expected number of seconds before a stick laps the other, if
the second stick is delayed by d seconds. We solve a system of linear
equations.
"""

from __future__ import annotations

import numpy as np
from scipy.linalg import solve as solve_linear


def E(m: int, n: int, K: int = 5) -> float:
    """Compute expected time for given m, n."""
    size = m + K + 1
    M = np.zeros((size, size))
    T = np.zeros(size)
    mult = 1.0 / (m - n + 1)

    for d in range(size):
        M[d][d] = 1.0
        for t1 in range(n, m + 1):
            if t1 < d - K:
                T[d] += t1 * mult
            else:
                for t2 in range(d + n, d + m + 1):
                    abs_diff = abs(t1 - t2)
                    M[d][abs_diff] -= mult * mult
                    T[d] += (min(t1, t2) + K) * mult * mult

    E_vals = solve_linear(M, T)
    return E_vals[0]


def solve() -> float:
    """Solve Problem 589."""
    N = 100
    K = 5

    ans = 0.0
    for m in range(2, N + 1):
        for n in range(1, m):
            ans += E(m, n, K)

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(f"{result:.2f}")


if __name__ == "__main__":
    main()
