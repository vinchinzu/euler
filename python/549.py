"""Project Euler Problem 549: Divisibility of Factorials.

Let s(n) be the smallest m such that n | m!. Find sum_{i=2}^N s(i).

If n = prod (p_i)^(e_i), then s(n) = max s((p_i)^(e_i)), and we can compute
s((p_i)^(e_i)) quickly by checking multiples of p.

We search over all n by trying all prime factorizations recursively.

Optimization: when the remaining factorization is a single prime p > s,
then s(n*p) = p, so we sum all such primes p in [current_prime, N/n]
using a precomputed prime-sum lookup (Lucy_Hedgehog algorithm).
"""

from __future__ import annotations

from math import isqrt


def solve() -> int:
    N = 10**8
    sqrt_n = isqrt(N)

    # --- Sieve primes up to 2*sqrt(N) for the recursive enumeration ---
    limit = 2 * sqrt_n
    is_prime = bytearray(b'\x01') * (limit + 1)
    is_prime[0] = is_prime[1] = 0
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = 0
    primes = [i for i in range(limit + 1) if is_prime[i]]

    # --- Lucy_Hedgehog algorithm for sum-of-primes up to N ---
    # Computes S(v) = sum of primes <= v, for all v in the set of
    # "quotient values" { floor(N/k) : k = 1..N } union { 1..sqrt(N) }.
    #
    # We store results in two arrays:
    #   small[i] = S(i)          for i <= sqrt_n
    #   large[i] = S(floor(N/i)) for i >= 1, where floor(N/i) > sqrt_n

    small = [0] * (sqrt_n + 1)
    large = [0] * (sqrt_n + 1)

    # Initialize: S(v) = sum of integers from 2..v = v*(v+1)/2 - 1
    for i in range(1, sqrt_n + 1):
        small[i] = i * (i + 1) // 2 - 1
    for i in range(1, sqrt_n + 1):
        v = N // i
        large[i] = v * (v + 1) // 2 - 1

    # Sieve: for each prime p (in order), subtract contribution of composites
    # whose smallest prime factor is p.
    for p in range(2, sqrt_n + 1):
        if small[p] == small[p - 1]:
            # p is not prime (its contribution was already removed)
            continue
        sp1 = small[p - 1]  # S(p-1) = sum of primes < p
        p2 = p * p

        # Update large[] entries
        upper = min(sqrt_n, N // p2)
        for i in range(1, upper + 1):
            # large[i] stores S(N//i)
            # We need S(N//(i*p)) which is:
            #   large[i*p] if i*p <= sqrt_n (i.e. N//(i*p) > sqrt_n)
            #   small[N//(i*p)] otherwise
            ip = i * p
            if ip <= sqrt_n:
                large[i] -= p * (large[ip] - sp1)
            else:
                large[i] -= p * (small[N // ip] - sp1)

        # Update small[] entries
        for i in range(min(sqrt_n, N // p2), p2 - 1, -1):
            small[i] -= p * (small[i // p] - sp1)

    def sum_primes_up_to(x: int) -> int:
        """Return sum of all primes <= x, for any x <= N."""
        if x <= 0:
            return 0
        if x <= sqrt_n:
            return small[x]
        # x > sqrt_n, so x = N // k for some k = N // x
        k = N // x
        return large[k]

    # --- Recursive helper ---
    ans = 0

    def num_factors_in_factorial(m: int, p: int) -> int:
        """Count factors of p in m! (Legendre's formula)."""
        count = 0
        power = p
        while power <= m:
            count += m // power
            power *= p
        return count

    def helper(min_index: int, n: int, s: int) -> None:
        nonlocal ans
        if n > 1:
            ans += s

        for index in range(min_index, len(primes)):
            p = primes[index]
            if p > s and n * p * p > N:
                # Only single-prime factors remain: for each prime q >= p
                # with n*q <= N, we have s(n*q) = q (since q > s).
                # So add sum of primes in [p, N//n].
                if p <= N // n:
                    # sum of primes from p to N//n
                    # = sum_primes_up_to(N//n) - sum_primes_up_to(primes[index-1])
                    sp_upper = sum_primes_up_to(N // n)
                    if index > 0:
                        sp_lower = sum_primes_up_to(primes[index - 1])
                    else:
                        sp_lower = 0
                    ans += sp_upper - sp_lower
                return

            new_n = n
            e = 1
            while True:
                new_n *= p
                if new_n > N:
                    break
                mult = p
                while True:
                    if num_factors_in_factorial(mult, p) >= e:
                        helper(index + 1, new_n, max(mult, s))
                        break
                    mult += p
                e += 1

    helper(0, 1, 0)
    return ans


if __name__ == "__main__":
    print(solve())
