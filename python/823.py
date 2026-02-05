"""Project Euler Problem 823: Factor Shuffle.

A list starts with [2,3,...,n]. Each round: divide each number by its smallest
prime factor, collect all those SPFs into a product (new number), remove 1s.
Find S(10^4, 10^16) mod 1234567891.

Key insight from Java solution: Track each prime factor individually with a
tiebreak value. After 2*num_factors rounds, we can compute periods analytically.
Each factor's period = position_in_number + numbers_with_more_factors.
"""

from __future__ import annotations
from functools import lru_cache


def sieve(n: int) -> list[int]:
    """Compute smallest prime factor for each number up to n."""
    spf = list(range(n + 1))
    for i in range(2, int(n**0.5) + 1):
        if spf[i] == i:
            for j in range(i * i, n + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def prime_factorization(n: int, spf: list[int]) -> list[int]:
    """Return list of prime factors of n (with multiplicity)."""
    factors = []
    while n > 1:
        p = spf[n]
        factors.append(p)
        n //= p
    return factors


def shuffle(all_factors: list[list[tuple[int, int]]]) -> list[list[tuple[int, int]]]:
    """Perform one round of the shuffle operation."""
    new_all_factors = []
    new_factors = []

    for factors in all_factors:
        if factors:
            # Remove the last (smallest by our sorting) factor
            new_factors.append(factors.pop())
            if factors:
                new_all_factors.append(factors)

    # Sort new factors by (value, tiebreak) descending
    new_factors.sort(key=lambda x: (x[0], x[1]), reverse=True)
    new_all_factors.insert(0, new_factors)

    return new_all_factors


def solve() -> int:
    """Solve Problem 823 using factor tracking approach."""
    N = 10000
    K = 10**16
    MOD = 1234567891

    # Compute smallest prime factors
    spf = sieve(N)

    # Initialize: for each number 2..N, get its prime factors
    # Each factor gets (prime_value, tiebreak) where tiebreak ensures uniqueness
    all_factors = []
    num_factors = 0

    for n in range(2, N + 1):
        factors = []
        pf = prime_factorization(n, spf)
        for i, p in enumerate(pf):
            # Tiebreak: i * N + n ensures unique identification
            factors.append((p, i * N + n))
        # Sort by (value, tiebreak) descending to match Java's COMPARATOR
        factors.sort(key=lambda x: (x[0], x[1]), reverse=True)
        all_factors.append(factors)
        num_factors += len(factors)

    # Shuffle for 2 * num_factors rounds to reach stable cycle state
    shuffle_rounds = 2 * num_factors
    for _ in range(shuffle_rounds):
        all_factors = shuffle(all_factors)

    # Compute period for each factor
    # period = position_of_number + (number_of_factors_in_number - position_in_number)
    factor_periods = {}  # Maps (k % period) -> list of factors

    for pos, factors in enumerate(all_factors):
        for j, factor in enumerate(factors):
            # Period calculation: factors move through the list cyclically
            # A factor at position j in a number at position pos with len factors
            # will take (pos + len - j) rounds to return to the same state
            period = pos + len(factors) - j
            k_mod = (K - shuffle_rounds) % period
            if k_mod not in factor_periods:
                factor_periods[k_mod] = []
            factor_periods[k_mod].append(factor)

    # Now shuffle k more times for each period k
    max_period_mod = max(factor_periods.keys()) if factor_periods else 0

    # Track where each factor ends up
    final_factors = []  # List of lists of prime values

    for k in range(max_period_mod + 1):
        if k in factor_periods:
            for factor in factor_periods[k]:
                # Find which number contains this factor
                pos = None
                for i, factors in enumerate(all_factors):
                    if factor in factors:
                        pos = i
                        break

                if pos is not None:
                    # Ensure final_factors has enough lists
                    while len(final_factors) <= pos:
                        final_factors.append([])
                    final_factors[pos].append(factor[0])  # Just the prime value

        # Shuffle for next iteration
        if k < max_period_mod:
            all_factors = shuffle(all_factors)

    # Compute the sum
    total = 0
    for factors in final_factors:
        if factors:
            prod = 1
            for p in factors:
                prod = (prod * p) % MOD
            total = (total + prod) % MOD

    return total


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
