"""Project Euler Problem 671: Coloured Tiles II.

Find the number of ways to fill a 2 x N closed loop with 1x1, 1x2, ... 1xT tiles
such that no four tiles meet at a single point, each tile is one of K colors, and
no adjacent tiles have the same color.

The transitions are similar to problem 670, but since we need to check whether the
last tile(s) are different colors from the first tile(s), we need to keep track of
the colors. This means that the vertical "state 0" from problem 670 must be split
into K states (one for each color) and each other state (i, j) must be split into
K(K-1) states (one for each combination of two colors). The final answer is easier
to compute: we just need to make sure that after N steps, the state is the same as
the starting state. The only additional point is that rotations aren't considered
distinct, so we must divide by N.

For efficiency, we can further reduce the number of states. By symmetry, we only need
to consider the starting states for "state 0" with any specific color, and a state
(i, j) for i=0 and j=1. This means that all colors 2...K-1 can be treated together,
which we consider to all be "color 2". The logic must be more complicated to handle
this special color, but it means we can solve the problem using matrix multiplication
with only 3+9T² = 84 states instead of K+(KT)² = 910 states.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Dict


@dataclass(frozen=True)
class State:
    """State representing tile configuration with colors."""

    top: int
    top_color: int
    bottom: int
    bottom_color: int

    @staticmethod
    def vertical(color: int) -> State:
        """Create a vertical state with given color."""
        return State(-1, color, -1, color)


def mod_inv(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    if m == 1:
        return 0
    t, new_t = 0, 1
    r, new_r = m, a % m
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Modular inverse does not exist")
    if t < 0:
        t += m
    return t


def mat_mult(
    a: list[list[int]], b: list[list[int]], mod: int
) -> list[list[int]]:
    """Matrix multiplication modulo mod."""
    n = len(a)
    result = [[0] * n for _ in range(n)]
    for i in range(n):
        for j in range(n):
            for k in range(n):
                result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % mod
    return result


def mat_pow(mat: list[list[int]], exp: int, mod: int) -> list[list[int]]:
    """Matrix exponentiation modulo mod."""
    n = len(mat)
    result = [[1 if i == j else 0 for j in range(n)] for i in range(n)]
    base = [[mat[i][j] % mod for j in range(n)] for i in range(n)]

    while exp > 0:
        if exp & 1:
            result = mat_mult(result, base, mod)
        base = mat_mult(base, base, mod)
        exp >>= 1

    return result


def solve() -> int:
    """Solve Problem 671."""
    N = 10004003002001
    K = 10
    T = 3
    M = 1000004321

    # Build states
    states: list[State] = []
    for color in range(3):
        states.append(State.vertical(color))
    for top in range(T):
        for top_color in range(3):
            for bottom in range(T):
                for bottom_color in range(3):
                    states.append(State(top, top_color, bottom, bottom_color))

    ordering: Dict[State, int] = {s: i for i, s in enumerate(states)}

    # Build transition matrix
    A = [[0] * len(states) for _ in range(len(states))]

    # Transitions from vertical states
    for color1 in range(3):
        for color2 in range(K):
            if color1 != color2:
                idx = ordering[State.vertical(min(color2, 2))]
                A[idx][ordering[State.vertical(color1)]] = (
                    A[idx][ordering[State.vertical(color1)]] + 1
                ) % M

    # Transitions from vertical to horizontal
    for color1 in range(3):
        for color2 in range(3):
            for color3 in range(K):
                if (color1 != color2 or color1 == 2) and color1 != color3 and color2 != color3:
                    if color1 == 2 and color2 == 2 and color3 == 3:
                        continue
                    A[ordering[State.vertical(min(color3, 2))]][
                        ordering[State(0, color1, 0, min(color2, 2))]
                    ] = (
                        A[ordering[State.vertical(min(color3, 2))]][
                            ordering[State(0, color1, 0, min(color2, 2))]
                        ]
                        + 1
                    ) % M
                    for i in range(1, T):
                        for j in range(T):
                            A[ordering[State(i - 1, color1, j, min(color3, 2))]][
                                ordering[State(i, color1, 0, min(color2, 2))]
                            ] = (
                                A[ordering[State(i - 1, color1, j, min(color3, 2))]][
                                    ordering[State(i, color1, 0, min(color2, 2))]
                                ]
                                + 1
                            ) % M
                    for i in range(T):
                        for j in range(1, T):
                            A[ordering[State(i, min(color3, 2), j - 1, color1)]][
                                ordering[State(0, min(color2, 2), j, color1)]
                            ] = (
                                A[ordering[State(i, min(color3, 2), j - 1, color1)]][
                                    ordering[State(0, min(color2, 2), j, color1)]
                                ]
                                + 1
                            ) % M

    # Transitions from horizontal to vertical
    for color1 in range(3):
        for color2 in range(K):
            for color3 in range(K):
                if len({color1, color2, color3}) == 3:
                    for i in range(T):
                        for j in range(T):
                            A[ordering[State(i, min(color2, 2), j, min(color3, 2))]][
                                ordering[State.vertical(color1)]
                            ] = (
                                A[ordering[State(i, min(color2, 2), j, min(color3, 2))]][
                                    ordering[State.vertical(color1)]
                                ]
                                + 1
                            ) % M

    # Transitions within horizontal states
    for color1 in range(3):
        for color2 in range(3):
            for i in range(1, T):
                for j in range(1, T):
                    A[ordering[State(i - 1, color1, j - 1, min(color2, 2))]][
                        ordering[State(i, color1, j, min(color2, 2))]
                    ] = (
                        A[ordering[State(i - 1, color1, j - 1, min(color2, 2))]][
                            ordering[State(i, color1, j, min(color2, 2))]
                        ]
                        + 1
                    ) % M

    # Compute A^N
    Ae = mat_pow(A, N, M)

    ans = 0
    ans = (ans + K * Ae[ordering[State.vertical(0)]][ordering[State.vertical(0)]]) % M
    for i in range(T):
        for j in range(T):
            ans = (
                ans
                + K
                * (K - 1)
                * Ae[ordering[State(i, 0, j, 1)]][ordering[State(i, 0, j, 1)]]
            ) % M

    ans = (ans % M * mod_inv(N, M)) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
