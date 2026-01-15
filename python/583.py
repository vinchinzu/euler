"""Project Euler Problem 583: Heron envelopes.

Find the number of Heron envelopes, pentagons ABCDE consisting of a
rectangle ABDE below an isosceles triangle BCD where the height of BCD
is no larger than the height of ABDE, such that all sides and diagonals
are integral, and the perimeter is at most N.

     .C
    / \
 B._____.D
  |     |
  |     |
 A._____.E

Let a=AE, b1 be the height of rectangle ABDE, b2 / 2 be the height of
isosceles triangle BCD, and b3 / 2 be the height of the entire envelope.
Then we can see that (a,b1), (a,2b2) and (a,2b3) must all be legs of
Pythagorean triples. Furthermore, in order for the diagonals to not be
half-integers, a must be even. We also have the given constraint that
b1 ≥ b2 / 2.
"""

from __future__ import annotations

import math
from math import gcd, hypot, isqrt
from typing import List


def sieve_with_smallest_factor(limit: int) -> List[int]:
    """Sieve that stores smallest prime factor."""
    ff = [0] * (limit + 1)
    for i in range(2, limit + 1):
        if ff[i] == 0:
            ff[i] = i
            for j in range(i * i, limit + 1, i):
                if ff[j] == 0:
                    ff[j] = i
    return ff


def generate_pythagorean_triples(limit: int) -> List[tuple[int, int, int]]:
    """Generate all Pythagorean triples with c <= limit.

    Uses Euclid's formula: a = m² - n², b = 2mn, c = m² + n²
    where m > n > 0, gcd(m, n) = 1, and m and n have opposite parity.
    """
    triples: List[tuple[int, int, int]] = []
    m_limit = isqrt(limit)

    for m in range(2, m_limit + 1):
        for n in range(1, m):
            if (m + n) % 2 == 1 and gcd(m, n) == 1:
                a = m * m - n * n
                b = 2 * m * n
                c = m * m + n * n

                if c > limit:
                    break

                # Generate all multiples
                k = 1
                while k * c <= limit:
                    triples.append((k * a, k * b, k * c))
                    triples.append((k * b, k * a, k * c))
                    k += 1

    return triples


def solve() -> int:
    """Solve Problem 583."""
    N = 10**7

    # Precompute smallest factor array
    ff = sieve_with_smallest_factor(N)

    # Compute number of factors for each number
    num_factors = [0] * (N + 1)
    num_factors[1] = 1
    for i in range(2, N + 1):
        ii = i
        mult = 1
        while ii % ff[i] == 0:
            ii //= ff[i]
            mult += 2
        num_factors[i] = num_factors[ii] * mult

    # Compute start indices for storing other legs
    start_indices = [0] * (N + 2)
    for i in range(1, N + 1):
        start_indices[i + 1] = start_indices[i] + num_factors[i] // 2

    end_indices = start_indices[:]
    total_size = start_indices[N + 1]
    other_legs = [0] * total_size

    # Generate Pythagorean triples and store other legs
    triples = generate_pythagorean_triples(2 * N)
    for a, b, c in triples:
        if c <= N:
            if a <= N:
                other_legs[end_indices[a]] = b
                end_indices[a] += 1
            if b <= N:
                other_legs[end_indices[b]] = a
                end_indices[b] += 1

    ans = 0

    # Process each even a
    for a in range(2, N + 1, 2):
        start = start_indices[a]
        end = end_indices[a]

        # Sort other legs for this a
        other_legs[start:end] = sorted(other_legs[start:end])

        # Find valid (b1, b2, b3) where b1 + b2/2 = b3/2
        for i in range(start, end):
            b1 = other_legs[i]
            j = start
            k = start + 1

            while k < end:
                b2 = other_legs[j]
                b3 = other_legs[k]

                if b2 > 2 * b1:
                    break

                if 2 * b1 + b2 < b3:
                    j += 1
                elif 2 * b1 + b2 > b3:
                    k += 1
                else:
                    # Found valid combination
                    perim = a + 2 * b1 + round(hypot(a, b2))
                    if b2 % 2 == 0 and perim <= N:
                        ans += perim
                    j += 1
                    k += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
