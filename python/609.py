"""Project Euler Problem 609: π sequences.

Given a "π-sequence" u = (u_0, u_1 = π(u_0), u_2 = π(π(u_0)), ..., u_m ≥ 1)
with at least 2 elements, define c(u) to be the number of non-prime elements
of u, and define p(n,k) to be the number of π-sequences with u_0 ≤ n and
c(u)=k. Find the product of all p(N,k) ≠ 0.

For every prime u_0, we can compute all π-sequences starting with u_0, compute
c(u) for all sequences, and increment the value of p(n, c(u)) for each sequence.
"""

from __future__ import annotations

from sympy import isprime, primerange


def prime_counts(limit: int) -> list[int]:
    """Count primes up to each number."""
    is_prime_arr = [True] * (limit + 1)
    is_prime_arr[0] = is_prime_arr[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if is_prime_arr[i]:
            for j in range(i * i, limit + 1, i):
                is_prime_arr[j] = False

    counts = [0] * (limit + 1)
    count = 0
    for i in range(limit + 1):
        if is_prime_arr[i]:
            count += 1
        counts[i] = count
    return counts


def solve() -> int:
    """Solve Problem 609."""
    N = 10**8
    M = 10**9 + 7

    prime_counts_arr = prime_counts(N)

    # Find max length
    max_len = 0
    n = N
    while n > 0:
        max_len += 1
        n = prime_counts_arr[n]

    ps = [0] * max_len
    primes_list = list(primerange(2, N + 1))

    for i in range(len(primes_list)):
        count = (
            (N if i == len(primes_list) - 1 else primes_list[i + 1] - 1)
            - primes_list[i]
        )
        n = prime_counts_arr[primes_list[i]]
        c = 0
        while n > 0:
            if not isprime(n):
                c += 1
            ps[c] += 1
            ps[c + 1] += count
            n = prime_counts_arr[n]

    ans = 1
    for p in ps:
        if p != 0:
            ans = (ans * p) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
