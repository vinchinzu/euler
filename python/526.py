"""Project Euler Problem 526: Maximum Sum of Largest Prime Factors.

Find the maximum possible value of Σ_{k=0}^{K-1} f(n+k) for n ≤ N, where
f(n) is the largest prime factor of n.

For each small prime p, we figure out the possible residue classes of n
(mod p) that result in the largest possible sum after dividing the
appropriate terms by p, assuming all numbers are constant. Then we search
only those residue classes for a valid n.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import isqrt
from typing import Dict, List


@dataclass
class State:
    """State for residue class search."""

    a: int
    ratios: List[float]


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def primes_in_range(low: int, high: int, small_primes: List[int]) -> List[int]:
    """Get primes in range [low, high]."""
    if high < low:
        return []
    result = []
    for p in small_primes:
        if p > high:
            break
        if p >= low:
            result.append(p)
    # Use segmented sieve for large range
    if high > len(small_primes):
        segment_size = max(10000, isqrt(high))
        low_seg = max(low, len(small_primes) + 1)
        while low_seg <= high:
            high_seg = min(low_seg + segment_size - 1, high)
            is_prime_seg = [True] * (high_seg - low_seg + 1)
            for p in small_primes:
                if p * p > high_seg:
                    break
                start = max(p * p, ((low_seg + p - 1) // p) * p)
                for j in range(start, high_seg + 1, p):
                    is_prime_seg[j - low_seg] = False
            for i in range(high_seg - low_seg + 1):
                if is_prime_seg[i]:
                    result.append(low_seg + i)
            low_seg = high_seg + 1
    return result


def prime_factor(n: int) -> Dict[int, int]:
    """Get prime factorization of n."""
    factors: Dict[int, int] = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors


def round_down(n: int, mod: int) -> int:
    """Round n down to nearest multiple of mod."""
    return (n // mod) * mod


def solve() -> int:
    """Solve Problem 526."""
    N = 10**16
    K = 9
    L = 30

    sqrt_n = isqrt(N)
    small_primes_list = sieve_primes(L)
    large_primes = primes_in_range(L + 1, sqrt_n, small_primes_list)

    start_ratios = [1.0] * K
    best_states: List[State] = [State(0, start_ratios)]
    mod = 1

    for pe in range(2, L + 1):
        prime_factors = prime_factor(pe)
        if len(prime_factors) > 1:
            continue
        p = list(prime_factors.keys())[0]

        max_sum_ratios = 0.0
        new_best_states: List[State] = []
        for state in best_states:
            for i in range(p):
                start = state.a + i * mod
                new_ratios = state.ratios.copy()
                for j in range(K):
                    if (start + j) % pe == 0:
                        new_ratios[j] /= p
                sum_ratios = sum(new_ratios)
                if sum_ratios > max_sum_ratios:
                    new_best_states.clear()
                    max_sum_ratios = sum_ratios
                if sum_ratios == max_sum_ratios:
                    new_best_states.append(State(start, new_ratios))
        best_states = new_best_states
        mod *= p

    ans = 0
    for start in range(round_down(N, mod), -1, -mod):
        if ans > 0:
            break
        for state in best_states:
            num = start + state.a
            if num > N:
                continue
            good = True
            for p in large_primes:
                if num % p == 0 or num % p > p - K:
                    good = False
                    break
            if good:
                h = 0
                for i in range(K):
                    h += int(state.ratios[i] * (num + i))
                if h > ans:
                    ans = h

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
