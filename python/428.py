"""Project Euler Problem 428: Necklace triplets.

Given three positive integers a, b, c, consider four collinear points W, X, Y, Z
such that the distances between adjacent points are a, b, c respectively. Draw
a circle C_in with diameter XY, and a circle C_out with diameter WZ. The
triplet (a, b, c) is a "necklace triplet" if you can place k ≥ 3 distinct
circles C_1, C_2, ... C_k such that they are all tangent to C_in and C_out, and
all adjacent circles are tangent to each other.

Find the number of necklace triplets such that b ≤ N.

This problem uses Dirichlet series to efficiently compute divisor sums.
"""

from __future__ import annotations

import math
from typing import Callable, List


def isqrt(n: int) -> int:
    """Integer square root."""
    if n < 0:
        return 0
    if n == 0:
        return 0
    x = int(math.sqrt(n))
    while x * x < n:
        x += 1
    while x * x > n:
        x -= 1
    return x


def num_divisors(limit: int) -> List[int]:
    """Compute number of divisors for each number up to limit."""
    result = [0] * (limit + 1)
    for i in range(1, limit + 1):
        for j in range(i, limit + 1, i):
            result[j] += 1
    return result


class QuotientValues:
    """Stores values of f(⌊n/k⌋) for all integer k.

    Values are stored in big[k] directly for small k, and for large k they are
    stored implicitly as small[i] = f(i).
    """

    def __init__(self, n: int, big: List[int], small: List[int]) -> None:
        """Initialize with n and arrays."""
        self.n = n
        self.big = big
        self.small = small

    def get(self, i: int) -> int:
        """Return f(i)."""
        return self.small[i]

    def div(self, k: int) -> int:
        """Return f(⌊n/k⌋)."""
        if k < len(self.big):
            return self.big[k]
        return self.small[self.n // k]


class DirichletSeriesContext:
    """Context for Dirichlet series operations.

    A Dirichlet series is an infinite series of the form f(s) = Σ_n (a_n) / n^s.
    It is represented internally by a QuotientValues where f(k) is the cumulative
    sum of the numerators, f(k) = Σ_{n=1}^k a_n, because this allows efficient
    multiplication/division of Dirichlet series in O(N^{2/3}) time.
    """

    def __init__(self, N: int, M: int = 2**63 - 1) -> None:
        """Initialize with N and modulus M."""
        self.N = N
        self.M = M
        self.L = int((N * math.log(N + 1)) ** (1 / 3)) + 1
        self.L2 = int(N / self.L)

        num_divs = num_divisors(self.L2)

        start_indices = [0] * (self.L2 + 2)
        for i in range(1, self.L2 + 1):
            start_indices[i + 1] = start_indices[i] + num_divs[i]

        curr_indices = start_indices[:]

        divisors: List[int] = [0] * start_indices[self.L2 + 1]
        for d in range(1, self.L2 + 1):
            mult = 1
            while d * mult <= self.L2:
                divisors[curr_indices[d * mult]] = d
                curr_indices[d * mult] += 1
                mult += 1

        self.num_divisors = num_divs
        self.start_indices = start_indices
        self.divisors = divisors

    def create(self, *a_n: int) -> QuotientValues:
        """Create from explicit values."""
        cum_sums = [0] * (len(a_n) + 1)
        for i in range(len(a_n)):
            cum_sums[i + 1] = cum_sums[i] + a_n[i]

        def cum_sum_func(n: int) -> int:
            return cum_sums[min(n, len(a_n))]

        return self.create_from_func(cum_sum_func)

    def create_from_func(self, cum_sum_func: Callable[[int], int]) -> QuotientValues:
        """Create from cumulative sum function."""
        big = [0] * self.L
        small = [0] * (self.L2 + 1)
        for i in range(1, self.L2 + 1):
            small[i] = cum_sum_func(i) % self.M
        for i in range(self.L - 1, 0, -1):
            big[i] = cum_sum_func(self.N // i) % self.M
        return QuotientValues(self.N, big, small)

    def add(
        self, ds1: QuotientValues, ds2: QuotientValues, *more: QuotientValues
    ) -> QuotientValues:
        """Add Dirichlet series."""
        result = self._add_two(ds1, ds2)
        for ds in more:
            result = self._add_two(result, ds)
        return result

    def _add_two(
        self, ds1: QuotientValues, ds2: QuotientValues
    ) -> QuotientValues:
        """Add two Dirichlet series."""
        big = [0] * self.L
        small = [0] * (self.L2 + 1)
        for i in range(1, self.L2 + 1):
            small[i] = (ds1.get(i) + ds2.get(i)) % self.M
        for i in range(self.L - 1, 0, -1):
            big[i] = (ds1.div(i) + ds2.div(i)) % self.M
        return QuotientValues(self.N, big, small)

    def multiply(self, ds: QuotientValues, scale: int) -> QuotientValues:
        """Multiply Dirichlet series by scalar."""
        big = [0] * self.L
        small = [0] * (self.L2 + 1)
        for i in range(1, self.L2 + 1):
            small[i] = (ds.get(i) * scale) % self.M
        for i in range(self.L - 1, 0, -1):
            big[i] = (ds.div(i) * scale) % self.M
        return QuotientValues(self.N, big, small)

    def multiply_series(
        self, ds1: QuotientValues, ds2: QuotientValues, *more: QuotientValues
    ) -> QuotientValues:
        """Multiply Dirichlet series."""
        result = self._multiply_two(ds1, ds2)
        for ds in more:
            result = self._multiply_two(result, ds)
        return result

    def _multiply_two(
        self, ds1: QuotientValues, ds2: QuotientValues
    ) -> QuotientValues:
        """Multiply two Dirichlet series."""
        big = [0] * self.L
        small = [0] * (self.L2 + 1)
        for i in range(1, self.L2 + 1):
            small[i] = small[i - 1]
            for j in range(self.num_divisors[i]):
                d = self.divisors[self.start_indices[i] + j]
                term = (
                    (ds1.get(d) - ds1.get(d - 1))
                    * (ds2.get(i // d) - ds2.get(i // d - 1))
                ) % self.M
                small[i] = (small[i] + term) % self.M

        for i in range(self.L - 1, 0, -1):
            limit = isqrt(self.N // i)
            for x in range(1, limit + 1):
                term = (ds1.get(x) - ds1.get(x - 1)) * ds2.div(i * x)
                big[i] = (big[i] + term) % self.M
            for y in range(1, limit + 1):
                term = (ds2.get(y) - ds2.get(y - 1)) * ds1.div(i * y)
                big[i] = (big[i] + term) % self.M
            big[i] = (big[i] - ds1.get(limit) * ds2.get(limit)) % self.M

        return QuotientValues(self.N, big, small)

    def reciprocal(self, ds: QuotientValues) -> QuotientValues:
        """Compute reciprocal of Dirichlet series."""
        if abs(ds.get(1)) != 1:
            raise ValueError("Dirichlet series is not invertible")
        big = [0] * self.L
        small = [0] * (self.L2 + 1)
        small[1] = ds.get(1)
        for i in range(2, self.L2 + 1):
            small[i] = small[i - 1]
            for j in range(1, self.num_divisors[i]):
                d = self.divisors[self.start_indices[i] + j]
                term = (
                    -ds.get(1)
                    * (ds.get(d) - ds.get(d - 1))
                    * (small[i // d] - small[i // d - 1])
                ) % self.M
                small[i] = (small[i] + term) % self.M

        for i in range(self.L - 1, 0, -1):
            limit = isqrt(self.N // i)
            for x in range(2, limit + 1):
                idx = i * x
                if idx < self.L:
                    val = big[idx]
                else:
                    val = small[self.N // idx]
                term = (ds.get(x) - ds.get(x - 1)) * val
                big[i] = (big[i] + term) % self.M
            for y in range(1, limit + 1):
                term = (small[y] - small[y - 1]) * (
                    ds.div(i * y) - ds.get(1)
                )
                big[i] = (big[i] + term) % self.M
            big[i] = (big[i] - (ds.get(limit) - ds.get(1)) * small[limit]) % self.M
            big[i] = ds.get(1) * (1 - big[i]) % self.M

        return QuotientValues(self.N, big, small)

    def divide(
        self, ds1: QuotientValues, ds2: QuotientValues
    ) -> QuotientValues:
        """Divide two Dirichlet series."""
        return self.multiply_series(ds1, self.reciprocal(ds2))


def solve() -> int:
    """Solve Problem 428."""
    N = 10**9

    dsc = DirichletSeriesContext(N)

    # Create constant 1
    ONE = dsc.create(1)

    # C(2) = (1 - 2^-s) / (1 + 2^-s)
    # Represented as series with values at powers of 2
    C2_num = dsc.create(1, -1)  # 1 - 2^-s
    C2_den = dsc.create(1, 1)  # 1 + 2^-s
    C2 = dsc.divide(C2_num, C2_den)

    # C(3) = (1 - 3^-s) / (1 + 3^-s)
    C3_num = dsc.create(1, 0, -1)  # 1 - 3^-s
    C3_den = dsc.create(1, 0, 1)  # 1 + 3^-s
    C3 = dsc.divide(C3_num, C3_den)

    # tau_n2_terms combines the various terms
    term1 = dsc.multiply(dsc.add(ONE, C2), 2)
    term2_part1 = dsc.multiply(dsc.add(ONE, C3), 2)
    term2_part2 = dsc.multiply(dsc.create(0, 0, 1), 2)  # 2 * 3^-s
    term2_part3 = dsc.multiply_series(C3, dsc.create(1, 0, -1))
    term2 = dsc.multiply_series(
        dsc.add(ONE, dsc.multiply(C2, 2)),
        dsc.add(term2_part1, term2_part2, term2_part3),
    )
    tau_n2_terms = dsc.add(term1, term2)

    # Correction term: reciprocal of (1 + 3^-s)
    correction_term = dsc.reciprocal(dsc.create(1, 0, 1))

    # zeta(s) = Σ_n n / n^s where value at n is n
    def zeta_func(n: int) -> int:
        return n

    zeta_s = dsc.create_from_func(zeta_func)

    # zeta(2s) = Σ_n isqrt(n) / n^s where value at n is isqrt(n)
    def zeta_2s_func(n: int) -> int:
        return isqrt(n)

    zeta_2s = dsc.create_from_func(zeta_2s_func)

    # chi(s): 1 if n ≡ 1 (mod 3), 0 otherwise
    def chi_func(n: int) -> int:
        return 1 if n % 3 == 1 else 0

    chi_s = dsc.create_from_func(chi_func)

    # Compute final result
    # Note: multiply with multiple args means multiply all series together
    term_a = dsc.multiply_series(
        dsc.multiply_series(tau_n2_terms, zeta_s), zeta_s
    )
    term_b = dsc.multiply_series(
        dsc.multiply_series(correction_term, chi_s), chi_s
    )
    diff = dsc.add(term_a, dsc.multiply(term_b, -1))
    res = dsc.multiply_series(diff, dsc.divide(zeta_s, zeta_2s))

    # Extract value at n=1 and divide by 2
    ans = res.get(1) // 2
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
