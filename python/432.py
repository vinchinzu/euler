#!/usr/bin/env python3
"""
Project Euler 432 - Totient sum

S(n,m) = sum of phi(n*i) for i=1 to m
Find S(510510, 10^11) mod 10^9

Key insight: 510510 = 2*3*5*7*11*13*17 (primorial)
Using the identity for phi(K*i) where K is squarefree:
S(K, N) = phi(K) * sum over all d (products of prime factors of K) of totient_sum(N/d)
"""
import sys
sys.setrecursionlimit(10000)

def solve():
    N = 10**11
    K = 510510
    MOD = 10**9

    # Prime factors of 510510
    primes_of_K = [2, 3, 5, 7, 11, 13, 17]

    # phi(K) where K = 2*3*5*7*11*13*17
    phi_K = 1
    for p in primes_of_K:
        phi_K *= (p - 1)
    # phi_K = 92160

    # Compute totient sums using Lucy_Hedgehog's algorithm with quotient values
    # For N = 10^11, we have O(sqrt(N)) ~ 316000 distinct quotient values

    # First, sieve small values of sum_phi
    # Use N^(2/3) as cutoff
    limit = int(N ** (2/3)) + 1000  # about 4.6 million

    # Compute phi for small values using sieve
    phi = list(range(limit + 1))
    for i in range(2, limit + 1):
        if phi[i] == i:  # i is prime
            for j in range(i, limit + 1, i):
                phi[j] -= phi[j] // i

    # Prefix sums of phi (exact values, Python handles big integers)
    sum_phi_small = [0] * (limit + 1)
    for i in range(1, limit + 1):
        sum_phi_small[i] = sum_phi_small[i-1] + phi[i]

    # For large values, we use quotient enumeration
    # All quotients are of the form floor(N/k) for some k
    # Compute totient sums bottom-up for all quotient values

    # Collect all quotient values
    sqrt_N = int(N ** 0.5)
    quotients = set()
    for i in range(1, sqrt_N + 2):
        quotients.add(N // i)
        if i <= N:
            quotients.add(i)

    quotients = sorted(quotients)

    # Compute totient_sum for each quotient value, bottom-up
    totient_sum_cache = {}

    for q in quotients:
        if q <= limit:
            totient_sum_cache[q] = sum_phi_small[q]
        else:
            # totient_sum(q) = q*(q+1)/2 - sum_{d=2}^{q} totient_sum(q/d)
            result = q * (q + 1) // 2
            sqrt_q = int(q ** 0.5)

            # Sum over d from 2 to sqrt_q
            for d in range(2, sqrt_q + 1):
                qd = q // d
                if qd in totient_sum_cache:
                    result -= totient_sum_cache[qd]
                elif qd <= limit:
                    result -= sum_phi_small[qd]

            # Sum over quotient values m where q//m > sqrt_q
            for m in range(1, sqrt_q + 1):
                if q // m > sqrt_q:
                    count = q // m - q // (m + 1)
                    if m in totient_sum_cache:
                        result -= count * totient_sum_cache[m]
                    elif m <= limit:
                        result -= count * sum_phi_small[m]

            totient_sum_cache[q] = result

    def totient_sum(n):
        if n in totient_sum_cache:
            return totient_sum_cache[n]
        if n <= limit:
            return sum_phi_small[n]
        return totient_sum_cache[n]

    # Now enumerate all d that are K-smooth (products of prime factors of K)
    # and d <= N
    ans = 0

    def enumerate_d(idx, d):
        nonlocal ans
        if idx == len(primes_of_K):
            ans += totient_sum(N // d)
            return
        # Don't include primes_of_K[idx]
        enumerate_d(idx + 1, d)
        # Include primes_of_K[idx] repeatedly
        d2 = d * primes_of_K[idx]
        while d2 <= N:
            enumerate_d(idx + 1, d2)
            d2 *= primes_of_K[idx]

    enumerate_d(0, 1)

    ans = (ans * phi_K) % MOD
    return ans

if __name__ == "__main__":
    print(solve())
