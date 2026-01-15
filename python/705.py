"""Project Euler Problem 705: Total Inversion Count of Divisibility.

Let G(N) be the concatenation of all primes less than N, ignoring zeros. Find
the sum of the inversion counts of all digit sequences s with the same length
as G(N) such that each digit of s is a divisor of the corresponding digit of
G(N).

We process the string G(N) from right to left, keeping track of the frequency
of each possible digit in the suffixes of all digit sequences s. For each
digit d, any digit i less than d in these suffixes will result in an
inversion. The number of strings with this inversion is the total number of
digit sequences s, with the digits d and i fixed. In other words, it is the
total number of digit sequences, which is the product of the number of
divisors over all digits of G(N), divided by the number of divisors for the
two digits of G(N) at the positions of d and i.

This means that as we keep track of digit frequencies, we need to multiply
them by the total number of sequences divided by the number of divisors of
the corresponding digit of G(N). And when we count inversions, we need to
divide by the number of divisors of the other corresponding digit of G(N).
Doing this over all digits gives the answer.
"""

from __future__ import annotations

from typing import List


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def digits(n: int) -> List[int]:
    """Get digits of a number."""
    return [int(d) for d in str(n)]


def all_divisors(d: int) -> List[int]:
    """Get all divisors of a digit (1-9)."""
    if d == 0:
        return []
    result = []
    for i in range(1, d + 1):
        if d % i == 0:
            result.append(i)
    return result


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    if m == 1:
        return 0
    t, new_t = 0, 1
    r, new_r = m, a % m
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Modular inverse does not exist")
    if t < 0:
        t += m
    return t


def mod_invs(n: int, m: int) -> List[int]:
    """Generate modular inverses for 0..n-1 modulo m."""
    return [mod_inverse(i, m) if i > 0 else 0 for i in range(n)]


def solve() -> int:
    """Solve Problem 705."""
    n = 10**8
    b = 10
    m = 10**9 + 7

    primes = sieve(n)
    primes.reverse()

    all_divisors_list: List[List[int]] = []
    for i in range(b):
        all_divisors_list.append(all_divisors(i))

    num_sequences = 1
    for p in primes:
        for k in digits(p):
            if k > 0:
                num_sequences = (num_sequences * len(all_divisors_list[k])) % m

    mod_invs_list = mod_invs(b, m)
    counts = [0] * b
    ans = 0

    for p in primes:
        for k in digits(p):
            if k > 0:
                divisors_k = all_divisors_list[k]
                inv_divisors_k = mod_invs_list[len(divisors_k)]
                for d in divisors_k:
                    for i in range(1, d):
                        ans = (ans + counts[i] * inv_divisors_k) % m
                for d in divisors_k:
                    counts[d] = (
                        counts[d] + num_sequences * inv_divisors_k
                    ) % m

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
