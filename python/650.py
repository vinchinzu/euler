"""Project Euler Problem 650: Divisors of Binomial Product.

Let B(n) be the product of nCr(n, k) over all k, and let D(n) be the sum of
the divisors of B(n). Find sum_{k=1}^n D(k).

Since B(n) / B(n-1) = n^(n-1) / (n-1)!, we can start with B(0) = 0 and
repeatedly compute the prime factorization of increasing B(n). For efficiency,
we also maintain the prime factorization of n!, and update it for increasing n.
The sum of divisors can be computed from the prime factorization.
"""

from __future__ import annotations

from collections import defaultdict

from sympy import factorint, primerange


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


def mod_invs(n: int, m: int) -> list[int]:
    """Precompute modular inverses for 1..n modulo m."""
    invs = [0] * (n + 1)
    invs[1] = 1
    for i in range(2, n + 1):
        invs[i] = (m - (m // i) * invs[m % i] % m) % m
    return invs


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 650."""
    N = 20000
    M = 10**9 + 7

    primes = list(primerange(2, N + 1))
    mod_invs_list = mod_invs(N, M)

    factorial_exp = defaultdict(int)
    B_exp = defaultdict(int)
    ans = 0

    for n in range(1, N + 1):
        # Update factorial factorization
        factors = factorint(n)
        for p, e in factors.items():
            factorial_exp[p] += e

        # Update B factorization: B(n) / B(n-1) = n^(n-1) / (n-1)!
        # So B_exp[p] += (n-1)*e - factorial_exp[p] for each prime factor of n
        for p, e in factors.items():
            B_exp[p] += (n - 1) * e
        for p in primes:
            if p <= n:
                B_exp[p] -= factorial_exp[p]

        # Compute D(n) = sum of divisors
        D = 1
        for p in primes:
            if B_exp[p] > 0:
                num = (pow_mod(p, B_exp[p] + 1, M) - 1) % M
                den = mod_invs_list[p - 1] if p - 1 < len(mod_invs_list) else mod_inverse(p - 1, M)
                D = (D * num * den) % M

        ans = (ans + D) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
