"""Project Euler Problem 706: 3-Like Numbers.

Find the number of N-digit positive integers n such that f(n), the number of
substrings of n that are divisible by 3, is divisible by 3.

For a given string n, we keep track of the number of prefixes with digit sum
congruent to 0, 1, or 2 (mod 3). It is only necessary to track these numbers
(mod 3). We also keep track of the total digit sum (mod 3). Given such a
string, we can compute the new state after appending any digit d.

This allows us to create a transition matrix A from strings of d digits to
strings of d+1 digits. The final state vector is obtained by starting from
states consisting of a single digit from 1 to 9, and multiply by A^{N-1}.

Finally, the number of substrings divisible by 3 is equal to the number of
ways to choose 2 prefixes that have the same digit sum (mod 3). Summing the
values of the final state vector at only these states gives the answer.
"""

from __future__ import annotations

from dataclasses import dataclass
from itertools import product
from typing import Dict, List, Tuple


@dataclass(frozen=True)
class State:
    """State representing prefix digit sum counts."""

    num_sums: Tuple[int, ...]
    total: int


def ncr(n: int, r: int) -> int:
    """Binomial coefficient C(n, r)."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    result = 1
    for i in range(min(r, n - r)):
        result = result * (n - i) // (i + 1)
    return result


def matrix_multiply(
    a: List[List[int]], b: List[List[int]], mod: int
) -> List[List[int]]:
    """Multiply two matrices modulo mod."""
    n = len(a)
    result = [[0] * n for _ in range(n)]
    for i in range(n):
        for j in range(n):
            for k in range(n):
                result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % mod
    return result


def matrix_power(matrix: List[List[int]], exp: int, mod: int) -> List[List[int]]:
    """Matrix exponentiation modulo mod."""
    n = len(matrix)
    result = [[1 if i == j else 0 for j in range(n)] for i in range(n)]
    base = [row[:] for row in matrix]

    while exp > 0:
        if exp & 1:
            result = matrix_multiply(result, base, mod)
        base = matrix_multiply(base, base, mod)
        exp >>= 1

    return result


def solve() -> int:
    """Solve Problem 706."""
    n = 10**5
    k = 3
    m = 10**9 + 7
    b = 10

    # Generate all states
    states: List[State] = []
    for num_sums in product(range(k), repeat=k):
        for total in range(k):
            states.append(State(tuple(num_sums), total))

    ordering: Dict[State, int] = {state: i for i, state in enumerate(states)}

    # Build transition matrix
    a = [[0] * len(states) for _ in range(len(states))]
    for num_sums in product(range(k), repeat=k):
        for total in range(k):
            for d in range(b):
                new_total = (total + d) % k
                new_num_sums = list(num_sums)
                new_num_sums[new_total] = (new_num_sums[new_total] + 1) % k
                new_state = State(tuple(new_num_sums), new_total)
                old_state = State(tuple(num_sums), total)
                a[ordering[new_state]][ordering[old_state]] = (
                    a[ordering[new_state]][ordering[old_state]] + 1
                ) % m

    # Compute A^(N-1)
    an = matrix_power(a, n - 1, m)

    # Count valid final states
    ans = 0
    for num_sums in product(range(k), repeat=k):
        f = 0
        for num in num_sums:
            f += ncr(num, 2)
        if f % k == 0:
            for d in range(1, b):
                start_num_sums = [0] * k
                start_num_sums[0] = (start_num_sums[0] + 1) % k
                start_num_sums[d % k] = (start_num_sums[d % k] + 1) % k
                for total in range(k):
                    final_state = State(tuple(num_sums), total)
                    start_state = State(tuple(start_num_sums), d % k)
                    ans = (
                        ans + an[ordering[final_state]][ordering[start_state]]
                    ) % m

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
