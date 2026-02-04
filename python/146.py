"""Project Euler Problem 146 - Investigating a Prime Pattern.

Find the sum of all n <= 150,000,000 where n^2+1, n^2+3, n^2+7, n^2+9,
n^2+13, n^2+27 are ALL prime, and n^2+5, n^2+11, n^2+15, n^2+17, n^2+19,
n^2+21, n^2+23, n^2+25 are all NOT prime.
"""

from sympy import isprime


def solve():
    LIMIT = 150_000_000
    must_prime = (1, 3, 7, 9, 13, 27)
    must_composite = (5, 11, 15, 17, 19, 21, 23, 25)

    # Precompute allowed residues mod 510510 = 2*3*5*7*11*13*17
    # For each small prime p, n^2+k must not be 0 mod p for k in must_prime
    sieve_primes = (2, 3, 5, 7, 11, 13, 17)
    MOD = 2 * 3 * 5 * 7 * 11 * 13 * 17  # 510510

    allowed_per_p = {}
    for p in sieve_primes:
        s = set()
        for r in range(p):
            sq = (r * r) % p
            if all((sq + k) % p != 0 for k in must_prime):
                s.add(r)
        allowed_per_p[p] = s

    allowed = []
    for r in range(MOD):
        if all(r % p in allowed_per_p[p] for p in sieve_primes):
            allowed.append(r)

    # Secondary sieve primes for quick inline filtering
    extra_primes = (19, 23, 29, 31, 37, 41, 43)
    extra_allowed = {}
    for p in extra_primes:
        s = set()
        for r in range(p):
            sq = (r * r) % p
            if all((sq + k) % p != 0 for k in must_prime):
                s.add(r)
        extra_allowed[p] = s

    # Precompute extra checks as frozensets for O(1) lookup
    extra_checks = tuple((p, frozenset(extra_allowed[p])) for p in extra_primes)

    total = 0
    for base_r in allowed:
        n = base_r
        if n == 0:
            n = MOD
        while n < LIMIT:
            # Quick modular checks against extra primes
            skip = False
            for p, ok_set in extra_checks:
                if n % p not in ok_set:
                    skip = True
                    break
            if skip:
                n += MOD
                continue

            sq = n * n
            # Check must-be-prime offsets
            if (isprime(sq + 1) and isprime(sq + 3) and isprime(sq + 7)
                    and isprime(sq + 9) and isprime(sq + 13) and isprime(sq + 27)):
                # Check must-be-composite offsets
                if (not isprime(sq + 5) and not isprime(sq + 11)
                        and not isprime(sq + 15) and not isprime(sq + 17)
                        and not isprime(sq + 19) and not isprime(sq + 21)
                        and not isprime(sq + 23) and not isprime(sq + 25)):
                    total += n
            n += MOD
    return total


if __name__ == "__main__":
    print(solve())
