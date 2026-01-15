"""Project Euler Problem 406: Guessing game.

Let C(n, a, b) be the minimum worst-case cost of an optimal strategy to guess
a number from 1 to n, if any guess lower than the correct number incurs a
cost of a, and any guess greater than the correct number incurs a cost of b.
Find sum_{k=1}^K C(N, √k, √F_k), where F_k is the k'th Fibonacci number.

Let f(c) be the largest n such that it is possible to guess any number from 1
to n with cost at most c. If c is negative, then we let f(c) = 0, i.e. it is
impossible to guess any number. Then for any c, we have f(c) = 1 + f(c - a)
+ f(c - b), because if our guess g is too low, we can still guess up to
f(c - a) numbers above g, and if our guess is too high, we can guess up to
f(c - b) numbers below.

We can iteratively find all c where f(c) changes value, in increasing order
of c. These c are all the linear combinations of a and b, which we can build
up lazily. For each c, we can use the above recurrence to reliably compute
f(c), because we have already found all smaller c where f(c) changes value.
Finally, the minimum cost C(n, a, b) is the smallest c such that f(c) ≥ N.
"""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from math import gcd, isqrt
from typing import Dict


def prime_factorization(n: int) -> list[tuple[int, int]]:
    """Return prime factorization as list of (prime, exponent)."""
    if n <= 1:
        return []
    factors: list[tuple[int, int]] = []
    count = 0

    # Factor out 2s
    while n % 2 == 0:
        n //= 2
        count += 1
    if count:
        factors.append((2, count))

    # Odd factors
    p = 3
    limit = isqrt(n)
    while p <= limit and n > 1:
        if n % p == 0:
            count = 0
            while n % p == 0:
                n //= p
                count += 1
            factors.append((p, count))
            limit = isqrt(n)
        p += 2

    if n > 1:
        factors.append((n, 1))

    return factors


@dataclass(frozen=True)
class RadicalSum:
    """Represents Σ c√r over all entries (r, c) in the map."""

    radical_to_coefficients: Dict[int, int]

    def __init__(self, radical_to_coefficients: Dict[int, int]) -> None:
        """Initialize RadicalSum, simplifying radicals."""
        simplified: Dict[int, int] = {}
        for r, c in radical_to_coefficients.items():
            if r > 0 and c != 0:
                # Factor out perfect squares
                factors = prime_factorization(r)
                coeff = 1
                remaining = 1
                for p, e in factors:
                    coeff *= p ** (e // 2)
                    remaining *= p ** (e % 2)
                simplified[remaining] = simplified.get(remaining, 0) + c * coeff

        # Remove zeros
        object.__setattr__(
            self,
            "radical_to_coefficients",
            {r: c for r, c in simplified.items() if c != 0},
        )

    @staticmethod
    def sqrt(n: int) -> RadicalSum:
        """Create RadicalSum representing √n."""
        return RadicalSum({n: 1})

    def double_value(self) -> float:
        """Compute the numeric value as a float."""
        result = 0.0
        for r, c in self.radical_to_coefficients.items():
            result += c * isqrt(r)
        return result

    def add(self, other: RadicalSum) -> RadicalSum:
        """Add two RadicalSums."""
        new_map: Dict[int, int] = dict(self.radical_to_coefficients)
        for r, c in other.radical_to_coefficients.items():
            new_map[r] = new_map.get(r, 0) + c
        return RadicalSum(new_map)

    def negate(self) -> RadicalSum:
        """Negate this RadicalSum."""
        new_map = {r: -c for r, c in self.radical_to_coefficients.items()}
        return RadicalSum(new_map)

    def subtract(self, other: RadicalSum) -> RadicalSum:
        """Subtract another RadicalSum."""
        return self.add(other.negate())

    def __lt__(self, other: RadicalSum) -> bool:
        """Compare by numeric value."""
        return self.double_value() < other.double_value()

    def __le__(self, other: RadicalSum) -> bool:
        """Compare by numeric value."""
        return self.double_value() <= other.double_value()


def fibonacci(n: int) -> int:
    """Compute nth Fibonacci number."""
    if n <= 1:
        return n
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


def C(n: int, a: RadicalSum, b: RadicalSum) -> float:
    """Compute C(n, a, b) - minimum worst-case cost."""
    # Use a sorted set (simulated with sorted list) for c values
    cs: list[RadicalSum] = [RadicalSum({})]
    # f maps c to f(c)
    f: Dict[RadicalSum, int] = {a.add(b).negate(): 0}

    while True:
        # Get smallest c
        cs.sort()
        c = cs.pop(0)

        # Add c+a and c+b to candidate set
        cs.append(c.add(a))
        cs.append(c.add(b))

        # Compute f(c) using recurrence
        c_minus_a = c.subtract(a)
        c_minus_b = c.subtract(b)

        # Find floor entries
        f_c_minus_a = 0
        f_c_minus_b = 0

        for prev_c, prev_f in sorted(f.items()):
            if prev_c <= c_minus_a:
                f_c_minus_a = prev_f
            if prev_c <= c_minus_b:
                f_c_minus_b = prev_f

        length = 1 + f_c_minus_a + f_c_minus_b

        if length >= n:
            return c.double_value()

        f[c] = length


def solve() -> float:
    """Solve Problem 406."""
    N = 10**12
    K = 30

    ans = 0.0
    for k in range(1, K + 1):
        a = RadicalSum.sqrt(k)
        b = RadicalSum.sqrt(fibonacci(k))
        ans += C(N, a, b)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")
    return int(result * 100000000)  # For comparison


if __name__ == "__main__":
    main()
