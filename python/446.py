#!/usr/bin/env python3
"""
Project Euler 446 - Retractions B

F(N) = sum_{n=1}^N R(n^4 + 4)
R(n) = prod(1 + p^e) - n for factorization n = prod(p^e)

Key: n^4 + 4 = ((n-1)^2 + 1) * ((n+1)^2 + 1)

So we factorize all k^2 + 1 for k = 0 to N+1, then combine.
"""

def solve():
    N = 10**7
    MOD = 10**9 + 7

    # Tonelli-Shanks: find sqrt of a mod p
    def tonelli_shanks(a, p):
        """Find x such that x^2 = a mod p, or return None if no solution."""
        if a % p == 0:
            return 0
        if pow(a, (p - 1) // 2, p) != 1:
            return None

        # Factor p - 1 = q * 2^s
        q, s = p - 1, 0
        while q % 2 == 0:
            q //= 2
            s += 1

        # Find quadratic non-residue
        z = 2
        while pow(z, (p - 1) // 2, p) != p - 1:
            z += 1

        m = s
        c = pow(z, q, p)
        t = pow(a, q, p)
        r = pow(a, (q + 1) // 2, p)

        while True:
            if t == 1:
                return r
            # Find smallest i such that t^(2^i) = 1
            i = 1
            temp = (t * t) % p
            while temp != 1:
                temp = (temp * temp) % p
                i += 1
            # Update
            b = pow(c, 1 << (m - i - 1), p)
            m = i
            c = (b * b) % p
            t = (t * c) % p
            r = (r * b) % p

    # Sieve primes up to N
    is_prime = [True] * (N + 2)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int((N + 1) ** 0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, N + 2, i):
                is_prime[j] = False

    # factors[k] = (k^2 + 1) with small primes divided out
    factors = [k * k + 1 for k in range(N + 2)]

    # res[k] will be the product of (1 + p^e) contributions from (k-1)^2+1 and (k+1)^2+1
    # for computing R((k-1)^2+1 * (k+1)^2+1)
    res = [1] * (N + 3)

    # Handle factor of 2: k^2 + 1 is odd if k is even, has one factor of 2 if k is odd
    # n^4 + 4 = ((n-1)^2+1)*((n+1)^2+1)
    # If n is even: both (n-1)^2+1 and (n+1)^2+1 are even (n-1 and n+1 odd)
    # If n is odd: both are odd
    for k in range(1, N + 2, 2):  # k odd means k^2+1 even
        factors[k] //= 2

    # Now n^4+4 has factor 4 when n is even, factor 1 when n is odd
    # For n even: (n-1)^2+1 and (n+1)^2+1 each contribute 2, so product has 4
    # R(...) includes (1 + 4) = 5 factor for the 2^2
    for n in range(2, N + 1, 2):
        res[n] = 5

    # Process primes p = 1 mod 4 (these can divide k^2+1)
    for p in range(5, N + 2):
        if not is_prime[p] or p % 4 != 1:
            continue

        # Find sqrt(-1) mod p using Tonelli-Shanks
        sqrt_neg1 = tonelli_shanks(p - 1, p)
        if sqrt_neg1 is None:
            continue

        # k^2 + 1 = 0 mod p means k^2 = -1 mod p, so k = ±sqrt(-1)
        roots = [sqrt_neg1 % p, (-sqrt_neg1) % p]
        if roots[0] == roots[1]:
            roots = [roots[0]]

        for start in roots:
            k = start
            while k < N + 2:
                # Divide out all factors of p from factors[k]
                pw = 1
                while factors[k] % p == 0:
                    factors[k] //= p
                    pw *= p

                # (k^2+1) contributes to n = k-1 and n = k+1
                # res[n] *= (1 + pw)
                term = (1 + pw) % MOD
                if k >= 1:
                    res[k - 1] = res[k - 1] * term % MOD
                if k + 1 <= N + 2:
                    res[k + 1] = res[k + 1] * term % MOD

                k += p

    # Handle remaining large prime factors
    for k in range(N + 2):
        if factors[k] > 1:
            term = (1 + factors[k]) % MOD
            if k >= 1:
                res[k - 1] = res[k - 1] * term % MOD
            if k + 1 <= N + 2:
                res[k + 1] = res[k + 1] * term % MOD

    # Compute F(N) = sum_{n=1}^N (res[n] - (n^4 + 4))
    ans = 0
    for n in range(1, N + 1):
        n4_plus_4 = (pow(n, 4, MOD) + 4) % MOD
        ans = (ans + res[n] - n4_plus_4) % MOD

    return ans

if __name__ == "__main__":
    print(solve())
