"""Project Euler Problem 580: Hilbert Numbers.

A Hilbert number is an integer of the form 4k+1. Find the number of Hilbert
numbers that are not divisible by the square of any Hilbert numbers other than 1.

We use Inclusion Exclusion, except the coefficient has, in addition to the
factor of (-1)^t where t is the number of 4k+1 primes, a factor of (-1)^r (r-1)
where r is the number of 4k+3 primes if all the 4k+3 primes are distinct, and
(-1)^(r-1) if there is a factor that is the square of a 4k+3 prime. Numbers
that are even or have a factor that is the square of a 4k+1 prime have
coefficient zero.

For performance, we store all the above info in 4 byte arrays: table[0] and
table[2] are the number of 4k+1 and 4k+3 prime factors respectively, and
table[1] and table[3] are the number of duplicated 4k+1 and 4k+3 prime factors
respectively.

If we call this coefficient µ'(n), then the final answer is
Σ_n µ'(n) ⌈(N/n²)/4⌉, where the ceiling term is the number of Hilbert numbers
up to N that are multiples of n².
"""

from __future__ import annotations

from math import ceil, isqrt
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


def solve() -> int:
    """Solve Problem 580."""
    N = 10**16
    L = isqrt(N)

    ff = sieve_with_smallest_factor(L)

    # table[0] = count of 4k+1 prime factors
    # table[1] = count of duplicated 4k+1 prime factors
    # table[2] = count of 4k+3 prime factors
    # table[3] = count of duplicated 4k+3 prime factors
    table = [[0] * (L + 1) for _ in range(4)]

    for i in range(3, L + 1, 2):
        d = ff[i]
        if d == 0:
            d = i
        for j in range(4):
            table[j][i] = table[j][i // d]

        rem_type = d % 4 - 1  # 0 for 4k+1, 2 for 4k+3
        is_square = i % (d * d) == 0
        table[rem_type + (0 if is_square else 1)][i] += 1

    # Compute Hilbert Möbius function
    hilbert_mobius = [0] * (L + 1)
    for i in range(1, L + 1, 2):
        if table[1][i] == 0:  # No duplicated 4k+1 primes
            r = table[2][i]  # Count of 4k+3 primes
            if table[3][i] == 0:  # No duplicated 4k+3 primes
                hilbert_mobius[i] = (1 if r % 2 == 0 else -1) * (r - 1)
            elif table[3][i] == 1:  # One duplicated 4k+3 prime
                hilbert_mobius[i] = 1 if (r - 1) % 2 == 0 else -1
            hilbert_mobius[i] *= 1 if table[0][i] % 2 == 0 else -1

    ans = 0
    for i in range(1, L + 1, 2):
        if hilbert_mobius[i] != 0:
            count = ceil((N / (i * i)) / 4)
            ans += hilbert_mobius[i] * count

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
