"""Project Euler Problem 416: A frog's trip.

Find the number of ways that a frog can jump from the leftmost square of a
row of N squares, to the rightmost square, then back to the leftmost square,
etc., making K round trips total, such that each jump is over 1, 2, or 3
squares, and at most one square is unvisited.

This is equivalent to the number of ways that 2K frogs can jump from the
leftmost square to the rightmost square. If at any point, the leftmost frogs
jump, then all frogs must always be within 3 consecutive squares, which we'll
label A, B, and C from left to right. The state can be uniquely defined by
the number of frogs on A and the number of frogs on B (after which the
number of frogs on C is fixed). We also need to maintain the number of
unvisited squares, though we can ignore states with at least 2 unvisited
squares.

Given a state, we can consider all possible ways that the frogs on A can
jump. Some of them can jump 1 square, some 2 squares, and the remaining 3
squares. This gives the new state, (A', B', C') = (B + jump1, C + jump2,
jump3), and the number of unvisited squares is only incremented if there were
no frogs on A.

In the start state, all 2K frogs are on A and there are no unvisited
squares. In the end state, we must again have all 2K frogs on A, but there
can be 0 or 1 unvisited squares. We can use matrix exponentiation to compute
the transition matrix from the start to the end.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import gcd
from typing import Dict, List


@dataclass(frozen=True)
class State:
    """State of the frog jumping problem."""

    num_a: int
    num_b: int
    num_c: int
    num_unvisited: int


def ncr(n: int, k: int) -> int:
    """Binomial coefficient C(n, k)."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    k = min(k, n - k)
    result = 1
    for i in range(k):
        result = result * (n - i) // (i + 1)
    return result


def multinomial(n: int, *ks: int) -> int:
    """Multinomial coefficient: n! / (k1! k2! ... km!)."""
    if sum(ks) != n:
        return 0
    result = 1
    remaining = n
    for k in ks:
        result = result * ncr(remaining, k)
        remaining -= k
    return result


def matrix_multiply(
    a: List[List[int]], b: List[List[int]], mod: int
) -> List[List[int]]:
    """Multiply two matrices modulo mod."""
    n = len(a)
    result = [[0] * n for _ in range(n)]
    for i in range(n):
        for k in range(n):
            if a[i][k]:
                for j in range(n):
                    result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % mod
    return result


def matrix_power(
    matrix: List[List[int]], exp: int, mod: int
) -> List[List[int]]:
    """Raise matrix to power exp modulo mod."""
    n = len(matrix)
    # Identity matrix
    result = [[0] * n for _ in range(n)]
    for i in range(n):
        result[i][i] = 1

    base = [row[:] for row in matrix]
    e = exp

    while e > 0:
        if e & 1:
            result = matrix_multiply(result, base, mod)
        base = matrix_multiply(base, base, mod)
        e >>= 1

    return result


def extended_gcd(a: int, b: int) -> tuple[int, int, int]:
    """Extended Euclidean algorithm."""
    if a == 0:
        return (b, 0, 1)
    gcd_val, x1, y1 = extended_gcd(b % a, a)
    x = y1 - (b // a) * x1
    y = x1
    return (gcd_val, x, y)


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse."""
    gcd_val, x, _y = extended_gcd(a, m)
    if gcd_val != 1:
        raise ValueError("Modular inverse does not exist")
    return (x % m + m) % m


def crt(mods: List[int], values: List[int]) -> int:
    """Chinese Remainder Theorem."""
    result = 0
    M = 1
    for m in mods:
        M *= m

    for i, (m, v) in enumerate(zip(mods, values)):
        Mi = M // m
        inv = mod_inverse(Mi, m)
        result = (result + v * Mi * inv) % M

    return result


def solve() -> int:
    """Solve Problem 416."""
    N = 10**12
    K = 10
    M1 = 2**9
    M2 = 5**9

    # Generate all states
    states: List[State] = []
    for num_a in range(2 * K + 1):
        for num_b in range(2 * K + 1 - num_a):
            for num_unvisited in range(2):
                num_c = 2 * K - (num_a + num_b)
                states.append(State(num_a, num_b, num_c, num_unvisited))

    # Create ordering map
    ordering: Dict[State, int] = {state: i for i, state in enumerate(states)}

    # Build transition matrix
    n_states = len(states)
    A = [[0] * n_states for _ in range(n_states)]

    for i, state in enumerate(states):
        new_num_unvisited = state.num_unvisited + (1 if state.num_a == 0 else 0)
        if new_num_unvisited <= 1:
            for jump1s in range(state.num_a + 1):
                for jump2s in range(state.num_a + 1 - jump1s):
                    jump3s = state.num_a - (jump1s + jump2s)
                    new_state = State(
                        state.num_b + jump1s,
                        state.num_c + jump2s,
                        jump3s,
                        new_num_unvisited,
                    )
                    if new_state in ordering:
                        j = ordering[new_state]
                        A[j][i] += multinomial(state.num_a, jump1s, jump2s, jump3s)

    # Find start and end state indices
    start_state = State(2 * K, 0, 0, 0)
    end_state_0 = State(2 * K, 0, 0, 0)
    end_state_1 = State(2 * K, 0, 0, 1)

    idx_start = ordering[start_state]
    idx_end_0 = ordering[end_state_0]
    idx_end_1 = ordering[end_state_1]

    # Compute result modulo M1 and M2 separately, then combine with CRT
    def compute_result(mod: int) -> int:
        """Compute result modulo mod."""
        An = matrix_power(A, N - 1, mod)
        return (An[idx_end_0][idx_start] + An[idx_end_1][idx_start]) % mod

    result_m1 = compute_result(M1)
    result_m2 = compute_result(M2)

    return crt([M1, M2], [result_m1, result_m2])


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
