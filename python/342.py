"""Project Euler Problem 342 - Sum of n where phi(n^2) is a cube.

Find sum of all n with 1 < n < 10^10 such that phi(n^2) is a perfect cube.

phi(n^2) = n * phi(n). For phi(n^2) to be a cube, we need the prime factorization
of n * phi(n) to have all exponents divisible by 3.

Build n by iterating over prime factors. Track which primes in phi
have exponents not divisible by 3, and resolve them.
"""
from math import isqrt

def solve():
    N = 10**10
    L = isqrt(N)

    # Sieve primes up to sqrt(N)
    sieve = bytearray(L + 1)
    primes = []
    for i in range(2, L + 1):
        if not sieve[i]:
            primes.append(i)
            for j in range(i * i, L + 1, i):
                sieve[j] = 1

    # Factorize small numbers
    def prime_factor(n):
        factors = {}
        for p in primes:
            if p * p > n:
                break
            if n % p == 0:
                e = 0
                while n % p == 0:
                    n //= p
                    e += 1
                factors[p] = e
        if n > 1:
            factors[n] = 1
        return factors

    ans = 0

    def helper(n, phi, max_prime):
        """
        n: current product
        phi: dict mapping prime -> exponent mod status for phi(n^2)
             Only primes with exponent % 3 != 0 are stored.
        max_prime: upper bound on the next prime we can add to resolve
        """
        nonlocal ans

        if not phi:
            # All exponents in phi(n^2) are divisible by 3
            if n > 1:
                ans += n
        else:
            # Must resolve the largest unresolved prime
            p = max(phi.keys())
            e_mod = phi[p] % 3
            start_e = 3 if e_mod == 1 else 1  # if mod=1, need 3 more; if mod=2, need 1 more
            add_prime(n, phi, p, start_e, p)

        # Try adding new primes
        for p in primes:
            if p >= max_prime:
                break
            if n * p * p >= N:
                break
            if n % p == 0:
                continue
            if phi and p < max(phi.keys()):
                continue
            if p in phi:
                continue
            add_prime(n, phi, p, 2, max_prime)

    def add_prime(n, phi, p, start_e, max_prime_for_recurse):
        """Add p^e to n for e = start_e, start_e+3, start_e+6, ..."""
        e = start_e
        pe = p ** e
        while n * pe < N:
            new_phi = dict(phi)
            # Remove p from phi if present
            new_phi.pop(p, None)

            # Add factors of (p-1) to phi tracking
            good = True
            for q, f in prime_factor(p - 1).items():
                new_f = new_phi.get(q, 0) + f
                if new_f % 3 != 0:
                    new_phi[q] = new_f
                    if n % q == 0:
                        good = False
                else:
                    new_phi.pop(q, None)

            if good:
                helper(n * pe, new_phi, p)

            e += 3
            pe = p ** e

    helper(1, {}, 2**31)
    return ans

if __name__ == "__main__":
    print(solve())
