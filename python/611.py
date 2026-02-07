#!/usr/bin/env python3
"""Project Euler Problem 611: Hallway of square steps.

Count integers up to N expressible as a^2 + b^2 (a < b > 0) in an odd number of ways.
Uses Lucy DP sieve for counting primes ≡ 1 (mod 4) in sub-linear time,
then iterates over products of prime powers.
"""

import math


def solve():
    N = 10**12
    L = int(math.isqrt(N))

    # Sieve primes up to L
    sieve = bytearray(L + 1)
    sieve[0] = sieve[1] = 1
    for i in range(2, int(math.isqrt(L)) + 1):
        if not sieve[i]:
            for j in range(i * i, L + 1, i):
                sieve[j] = 1
    sieve_primes = [i for i in range(3, L + 1) if not sieve[i]]
    all_primes = [2] + sieve_primes  # Include 2 for the helper

    # Lucy DP: count primes by residue mod 4
    big_size = N // L + 1
    big0 = [0] * big_size
    big1 = [0] * big_size
    small0 = [0] * (L + 1)
    small1 = [0] * (L + 1)

    for i in range(1, big_size):
        v = N // i
        big0[i] = (v + 3) // 4
        big1[i] = (v + 1) // 4
    for i in range(1, L + 1):
        small0[i] = (i + 3) // 4
        small1[i] = (i + 1) // 4

    for p in sieve_primes:
        p2 = p * p
        sp0 = small0[p - 1]
        sp1 = small1[p - 1]
        mod1 = (p % 4 == 1)

        # Big array updates (forward)
        i = 1
        while i < big_size and N // i >= p2:
            ip = i * p
            if ip < big_size:
                v0 = big0[ip] - sp0
                v1 = big1[ip] - sp1
            else:
                v0 = small0[N // ip] - sp0
                v1 = small1[N // ip] - sp1
            if mod1:
                big0[i] -= v0
                big1[i] -= v1
            else:
                big0[i] -= v1
                big1[i] -= v0
            i += 1

        # Small array updates (reverse)
        if p2 <= L:
            for i in range(L, p2 - 1, -1):
                v0 = small0[i // p] - sp0
                v1 = small1[i // p] - sp1
                if mod1:
                    small0[i] -= v0
                    small1[i] -= v1
                else:
                    small0[i] -= v1
                    small1[i] -= v0

    # Remove count of 1 (which is ≡ 1 mod 4 but not prime)
    for i in range(1, big_size):
        big0[i] -= 1
    for i in range(1, L + 1):
        small0[i] -= 1

    # Quotient values lookup for primes ≡ 1 mod 4
    def qv(v):
        if v <= L:
            return small0[v]
        return big0[N // v]

    # Iterate over products of prime powers (stack-based)
    ans = 0
    stack = [(0, 1, 1, True)]

    while stack:
        min_idx, n, P, skip = stack.pop()

        p0 = all_primes[min_idx]

        if not skip and ((P + 1) // 2 - P) % 2 != 0:
            ans += 1

        if N // n >= p0 and P % 2 != 0:
            ans += qv(N // n) - small0[p0] + (1 if p0 % 4 == 1 else 0)

        for idx in range(min_idx, len(all_primes) - 1):
            p = all_primes[idx]
            if n * p * p > N:
                break
            step = 2 if p % 4 == 3 else 1
            e = step
            pe = p ** step
            while n * pe <= N:
                new_P = P * ((e + 1) if p % 4 == 1 else 1)
                new_skip = (p % 4 == 1 and e == 1)
                stack.append((idx + 1, n * pe, new_P, new_skip))
                e += step
                pe *= p ** step

    return ans


if __name__ == "__main__":
    print(solve())
