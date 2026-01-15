"""Project Euler Problem 675: 2^ω(n!).

Let ω(n) be the number of distinct prime divisors of n, and let S(n) =
sum_{d|n} 2^{ω(d)}. Find sum_{i=2}^N S(i!).

Note that S(n) = prod_e (1 + 2e) for all exponents e in the prime
factorization of n. This means we can iteratively compute the prime
factorization of i!, by tacking on the prime factors of i.
"""

from __future__ import annotations


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


def preff(n: int) -> list[int]:
    """Precompute smallest prime factor."""
    ff = list(range(n + 1))
    for i in range(2, int(n**0.5) + 1):
        if ff[i] == i:
            for j in range(i * i, n + 1, i):
                if ff[j] == j:
                    ff[j] = i
    return ff


def solve() -> int:
    """Solve Problem 675."""
    N = 10**7
    M = 10**9 + 87

    ff = preff(N)
    mod_invs_list = mod_invs(2 * N, M)

    exponents = [0] * (N + 1)
    S = 1
    ans = 0

    for i in range(2, N + 1):
        ii = i
        while ii > 1:
            p = ff[ii]
            e = 0
            while ii % p == 0:
                ii //= p
                e += 1

            S = (S * mod_invs_list[1 + 2 * exponents[p]]) % M
            exponents[p] += e
            S = (S * (1 + 2 * exponents[p])) % M

        ans = (ans + S) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
