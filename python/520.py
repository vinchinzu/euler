"""Project Euler Problem 520: Simbers.

Let Q(n) be the number of Simbers, a number where each odd digit appears an
odd number of times (if present) and each even digit appears an even number
of times, with at most n digits. Find Σ_{u=1}^N Q(2^n).

Given a state consisting of the number of odd digits and even digits with a
given parity, we can determine the possible new states by adding a digit. We
need to also consider the number of odd digits that appear zero times
(because that's fine) and whether the number consists only of leading zeros
(because those zeros do not count). We can then use matrix exponentiation
to determine the state counts for 2^n digits.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Dict, List


@dataclass(frozen=True)
class State:
    """State for Simbers counting."""

    zero_odd: int
    odd_odd: int
    even_odd: int
    odd_even: int
    even_even: int
    empty: bool


def matrix_multiply(
    a: List[List[int]], b: List[List[int]], mod: int
) -> List[List[int]]:
    """Multiply two matrices modulo mod."""
    n = len(a)
    m = len(b[0])
    p = len(b)
    result = [[0] * m for _ in range(n)]
    for i in range(n):
        for j in range(m):
            for k in range(p):
                result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % mod
    return result


def solve() -> int:
    """Solve Problem 520."""
    N = 39
    M = 10**9 + 123
    B = 10

    # Generate all states
    start_state = State(B // 2, 0, 0, 0, B // 2, True)
    states: List[State] = [start_state]
    for zero_odd in range(B // 2 + 1):
        for odd_odd in range(B // 2 - zero_odd + 1):
            for odd_even in range(B // 2 + 1):
                states.append(
                    State(
                        zero_odd,
                        odd_odd,
                        B // 2 - zero_odd - odd_odd,
                        odd_even,
                        B // 2 - odd_even,
                        False,
                    )
                )

    # Create ordering map
    ordering: Dict[State, int] = {state: i for i, state in enumerate(states)}
    n_states = len(states)

    # Build transition matrix
    A = [[0] * n_states for _ in range(n_states)]
    for state in states:
        state_idx = ordering[state]

        if state.zero_odd > 0:
            new_state = State(
                state.zero_odd - 1,
                state.odd_odd + 1,
                state.even_odd,
                state.odd_even,
                state.even_even,
                False,
            )
            A[ordering[new_state]][state_idx] = (
                A[ordering[new_state]][state_idx] + state.zero_odd
            ) % M

        if state.odd_odd > 0:
            new_state = State(
                state.zero_odd,
                state.odd_odd - 1,
                state.even_odd + 1,
                state.odd_even,
                state.even_even,
                False,
            )
            A[ordering[new_state]][state_idx] = (
                A[ordering[new_state]][state_idx] + state.odd_odd
            ) % M

        if state.even_odd > 0:
            new_state = State(
                state.zero_odd,
                state.odd_odd + 1,
                state.even_odd - 1,
                state.odd_even,
                state.even_even,
                False,
            )
            A[ordering[new_state]][state_idx] = (
                A[ordering[new_state]][state_idx] + state.even_odd
            ) % M

        if state.odd_even > 0:
            new_state = State(
                state.zero_odd,
                state.odd_odd,
                state.even_odd,
                state.odd_even - 1,
                state.even_even + 1,
                False,
            )
            A[ordering[new_state]][state_idx] = (
                A[ordering[new_state]][state_idx] + state.odd_even
            ) % M

        if state.even_even > 0:
            new_state = State(
                state.zero_odd,
                state.odd_odd,
                state.even_odd,
                state.odd_even + 1,
                state.even_even - 1,
                False,
            )
            if state.empty:
                A[state_idx][state_idx] = (A[state_idx][state_idx] + 1) % M
                A[ordering[new_state]][state_idx] = (
                    A[ordering[new_state]][state_idx] + state.even_even - 1
                ) % M
            else:
                A[ordering[new_state]][state_idx] = (
                    A[ordering[new_state]][state_idx] + state.even_even
                ) % M

    # Compute A^(2^u) for u = 1 to N
    An = A
    ans = 0
    start_idx = ordering[start_state]

    for u in range(1, N + 1):
        An = matrix_multiply(An, An, M)
        for state in states:
            if state.even_odd == 0 and state.odd_even == 0 and not state.empty:
                state_idx = ordering[state]
                ans = (ans + An[state_idx][start_idx]) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
