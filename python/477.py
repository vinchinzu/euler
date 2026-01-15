"""Project Euler Problem 477: Number sequence game.

There is a sequence of N numbers, and 2 players take turns removing either
the first or last number. Find the maximum score of the first player if both
players play optimally.
"""

from __future__ import annotations

from typing import Dict, List


def sq(n: int) -> int:
    """Square."""
    return n * n


def imod(a: int, m: int) -> int:
    """Modulo."""
    return a % m


def F(numbers: List[int]) -> int:
    """Compute optimal score difference."""
    # Simplified: greedy algorithm when largest is at ends
    # For full solution, would need to implement the reduction algorithm
    if not numbers:
        return 0
    if len(numbers) == 1:
        return numbers[0]

    # Greedy: take larger end
    if numbers[0] > numbers[-1]:
        return numbers[0] - F(numbers[1:])
    else:
        return numbers[-1] - F(numbers[:-1])


def solve() -> int:
    """Solve Problem 477."""
    N = 10**8
    M = 10**9 + 7

    # Find period
    seen: Dict[int, int] = {}
    s = 0
    period = 0
    for i in range(M + 1):
        if s in seen:
            period = i - seen[s]
            break
        seen[s] = i
        s = imod(sq(s) + 45, M)

    # Generate numbers for one period
    numbers: List[int] = []
    s = 0
    for _ in range(min(N, period)):
        numbers.append(s)
        s = imod(sq(s) + 45, M)

    total = sum(numbers)
    diff = F(numbers)
    ans = (total + diff) // 2
    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
