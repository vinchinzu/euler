"""Project Euler Problem 728: Circle of Coins.

Let F(n,k) be the number of distinct possible states of a configuration of n
coins in a circle, where the only move is flipping a consecutive range of k
coins. Find Σ_{n=1}^N Σ_{k=1}^n F(n,k).

Let each configuration correspond to a polynomial with coefficients in Z_2 in
the natural way, where x^n - 1 vanishes. We can see that moves correspond to
adding multiples of x^{k-1} + ... + 1 = (x^k - 1) / (x - 1). So F(n,k) is
the number of elements in the group of polynomials in the group
(x^n - 1) \ (x^k-1)/(x-1). If g=GCD(n,k), this is equivalent to
(x-1)(1 + x^g + ... + x^{n-g}) \ (1 + x^g + ... + x^{k-g}), and the x-1
factor divides into 1 + x^g + ... + x^{k-g} if and only if k/g is even
(remember this is all in Z_2). That means the degree of the resulting space
is n-g+(1 if k/g is odd), and the number of elements is
2^{n-g+(1 if k/g is odd)}.

Now we can sum the values over all n and k:
S = Σ_{n=1}^N Σ_{g=1}^⌊N/n⌋ 2^{(n-1)g} (3ϕ(n)/2 if n is odd, else 2ϕ(n))
"""

from __future__ import annotations

from math import isqrt


def sieve_phi(limit: int) -> list[int]:
    """Compute Euler totient function for all numbers up to limit."""
    phi = list(range(limit + 1))
    for i in range(2, limit + 1):
        if phi[i] == i:  # i is prime
            for j in range(i, limit + 1, i):
                phi[j] -= phi[j] // i
    return phi


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Fast exponentiation modulo mod."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def mod_inv(a: int, m: int) -> int:
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


def solve() -> int:
    """Solve Problem 728."""
    n = 10**7
    m = 10**9 + 7
    l = n // (m.bit_length() - 1)

    phi = sieve_phi(n)
    pow2s = [pow_mod(2, i, m) for i in range(n + 1)]

    ans = 0
    for i in range(1, n + 1):
        res = 0
        if i > 1 and i < l:
            # Geometric series
            base = pow2s[i - 1]
            max_g = n // i
            if base == 1:
                res = max_g
            else:
                numerator = (pow_mod(base, max_g + 1, m) - 1) % m
                denominator = mod_inv(base - 1, m)
                res = numerator * denominator % m - 1
        else:
            # Direct computation
            for g in range(1, n // i + 1):
                res = (res + pow2s[(i - 1) * g]) % m

        multiplier = 2 * phi[i] if i == 1 or i % 2 == 0 else 3 * phi[i] // 2
        ans = (ans + res % m * multiplier) % m

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
