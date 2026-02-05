"""Project Euler Problem 642: Sum of largest prime factors."""

from math import isqrt
import sys
sys.setrecursionlimit(100000)

def compute_prime_sums(N, mod):
    """
    Compute sum of primes up to each quotient n//i using Lucy_Hedgehog algorithm.
    Returns two arrays: small[k] = sum of primes <= k for k <= sqrt(N)
                       big[i] = sum of primes <= N//i for i <= sqrt(N)
    """
    r = isqrt(N)

    # small[k] = sum of primes <= k
    small = [0] * (r + 1)
    # big[i] = sum of primes <= N // i
    big = [0] * (r + 1)

    for k in range(2, r + 1):
        small[k] = (k * (k + 1) // 2 - 1) % mod
    for i in range(1, r + 1):
        v = N // i
        big[i] = (v * (v + 1) // 2 - 1) % mod

    for p in range(2, r + 1):
        if small[p] == small[p - 1]:
            continue  # p is not prime
        sp = small[p - 1]
        p2 = p * p

        # Update big values
        for i in range(1, min(r, N // p2) + 1):
            d = N // i // p
            if d <= r:
                big[i] = (big[i] - p * (small[d] - sp)) % mod
            else:
                # d > r, so we need big[N // d]
                idx = N // d
                big[i] = (big[i] - p * (big[idx] - sp)) % mod

        # Update small values
        for k in range(r, p2 - 1, -1):
            small[k] = (small[k] - p * (small[k // p] - sp)) % mod

    return small, big


def solve():
    N = 201820182018
    M = 10**9
    L = isqrt(N)

    # Sieve primes up to L
    sieve = [True] * (L + 1)
    sieve[0] = sieve[1] = False
    for i in range(2, int(L**0.5) + 1):
        if sieve[i]:
            for j in range(i*i, L + 1, i):
                sieve[j] = False

    primes = [i for i in range(2, L + 1) if sieve[i]]

    # Compute sum of primes efficiently
    small, big = compute_prime_sums(N, M)

    def get_sum(v):
        """Get sum of primes <= v."""
        if v <= L:
            return small[v]
        else:
            # v = N // (N // v)
            idx = N // v
            return big[idx]

    ans = [0]

    def helper(min_index, n):
        # Get N // n (the maximum prime we can use)
        max_p = N // n
        # Sum of primes from primes[min_index] to max_p
        if min_index >= len(primes):
            return
        min_p = primes[min_index]
        if min_p > max_p:
            return

        contrib = (get_sum(max_p) - get_sum(min_p - 1)) % M
        ans[0] = (ans[0] + contrib) % M

        for index in range(min_index, len(primes)):
            p = primes[index]
            if n * p * p > N:
                break
            helper(index, n * p)

    helper(0, 1)
    return ans[0]


if __name__ == "__main__":
    print(solve())
