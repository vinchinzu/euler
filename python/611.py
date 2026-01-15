"""Project Euler Problem 611: Hallway of square steps.

Find the number of integers up to N that can be expressed as a²+b² for
positive a < b in an odd number of ways.

We generate all values that can be expressed as a sum of two squares by
building up factors. By the sum-of-squares formula, values satisfying
⌊(P+1)/2⌋ ≠ P (mod 2), where P = Π (e_i + 1), can be expressed in an odd
number of ways.
"""

from __future__ import annotations

from math import isqrt

from sympy import primerange


class QuotientValues:
    """Helper class for quotient-based prime counting."""

    def __init__(self, n: int, big: list[int], small: list[int]):
        """Initialize with precomputed values."""
        self.n = n
        self.big = big
        self.small = small
        self.L = len(small) - 1

    def div(self, x: int) -> int:
        """Get value for quotient n/x."""
        if x <= self.L:
            return self.small[x]
        return self.big[self.n // x] if self.n // x < len(self.big) else 0

    def get(self, x: int) -> int:
        """Get value for exact x."""
        if x <= self.L:
            return self.small[x]
        return 0


def solve() -> int:
    """Solve Problem 611."""
    N = 10**12
    L = isqrt(N)

    # Initialize arrays for prime counting (simplified)
    big = [[0] * (N // L + 1) for _ in range(2)]
    small = [[0] * (L + 1) for _ in range(2)]

    for r in range(2):
        for i in range(1, min(N // L, len(big[0]))):
            big[r][i] = (N // i - 2 * r + 3) // 4
        for i in range(1, L + 1):
            small[r][i] = (i - 2 * r + 3) // 4

    primes = list(primerange(3, L + 1))
    for p in primes:
        for i in range(1, min(N // L, len(big[0]))):
            if N // i >= p * p:
                ip = i * p
                r_idx = 1 if p % 4 == 1 else 0
                if ip < N // L and ip < len(big[0]):
                    big[r_idx][i] -= big[0][ip] - small[0][p - 1]
                else:
                    big[r_idx][i] -= small[0][N // ip] - small[0][p - 1]

        for i in range(L, p * p - 1, -1):
            r_idx = 1 if p % 4 == 1 else 0
            small[r_idx][i] -= small[0][i // p] - small[0][p - 1]

    for i in range(1, min(N // L, len(big[0]))):
        big[0][i] -= 1
    for i in range(1, L + 1):
        small[0][i] -= 1

    num_primes_1mod4 = QuotientValues(N, big[0], small[0])
    ans = 0

    def helper(min_index: int, n: int, P: int, skip: bool) -> None:
        """Recursive helper."""
        nonlocal ans
        if min_index >= len(primes):
            return

        p = primes[min_index]
        if not skip and ((P + 1) // 2 - P) % 2 != 0:
            ans += 1

        if N // n >= p and ((2 * P + 1) // 2) % 2 != 0:
            ans += (
                num_primes_1mod4.div(n)
                - num_primes_1mod4.get(p)
                + (1 if p % 4 == 1 else 0)
            )

        for index in range(min_index, len(primes) - 1):
            p = primes[index]
            if n * p * p > N:
                break
            e = 2 if p % 4 == 3 else 1
            while n * (p**e) <= N:
                new_P = P * (e + 1 if p % 4 == 1 else 1)
                new_skip = p % 4 == 1 and e == 1
                helper(index + 1, n * (p**e), new_P, new_skip)
                e += 2 if p % 4 == 3 else 1

    helper(0, 1, 1, True)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
