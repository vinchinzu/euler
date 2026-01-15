"""Project Euler Problem 822: Square the Smallest.

Starting with a list of integers from 2 to K, repeatedly remove the
smallest element and add back its square. Find the sum of the elements
after doing this N times.

We do this until the square of the smallest element is larger than the
current largest element. After that, it's clear that the smallest element
just cycles through each the list, so once the number of steps remaining T
is divisible by (N-1), we just square every element T/(N-1) times.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import List


@dataclass
class Number:
    """Number with log, mod, and original value."""

    log: float
    mod: int
    original: int

    def __lt__(self, other: Number) -> bool:
        """Compare by log, then by original."""
        if self.log != other.log:
            return self.log < other.log
        return self.original < other.original


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 822."""
    import math

    N = 10**16
    K = 10**4
    M = 1234567891

    numbers: List[Number] = []
    for i in range(2, K + 1):
        numbers.append(Number(math.log(i), i, i))

    numbers.sort()

    T = N
    while T % (K - 1) != 0 or numbers[0].log * 2 < numbers[-1].log:
        number = numbers.pop(0)
        new_mod = pow_mod(number.mod, 2, M)
        numbers.append(Number(number.log * 2, new_mod, number.original))
        numbers.sort()
        T -= 1

    ans = 0
    exp = pow_mod(2, T // (K - 1), M - 1)
    for number in numbers:
        ans = (ans + pow_mod(number.mod, exp, M)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
