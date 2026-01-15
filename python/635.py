"""Project Euler Problem 635: Subset sums.

Let A_q(n) be the number of subsets of {1, 2, ... q*n} with n elements whose
sum is divisible by n, and let S_q(L) = Σ A_q(p) for all primes p≤L. Find
S_2(N) + S_3(N).

If q < n, then there are q "special" subsets that are cyclic permutations of
each other: {1, n+1, 2n+1, ...}, {2, n+2, 2n+2, ...} ... {q, n+q, 2n+q, ...}.
By symmetry, all other subsets of n elements can be rotated by adding each
element to 0, 1, 2, ... n-1 (mod n) to get all sums (mod n). Adjusting for the
q special subsets, this means that A_q(n) = (nCr(q*n, n) + q(n-1)) / n.
"""

from __future__ import annotations

from sympy import primerange


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


def factorial(n: int, mod: int) -> int:
    """Compute n! modulo mod."""
    result = 1
    for i in range(1, n + 1):
        result = (result * i) % mod
    return result


def A(q: int, n: int, factorials: dict[int, int], M: int) -> int:
    """Compute A_q(n)."""
    if n == 2:
        return (q * (q - 1)) % M
    num = factorials[q * n]
    den = (factorials[n] * factorials[(q - 1) * n]) % M
    term1 = (num * mod_inverse(den, M)) % M
    term2 = (q * (n - 1)) % M
    return ((term1 + term2) * mod_inverse(n, M)) % M


def solve() -> int:
    """Solve Problem 635."""
    N = 10**8
    M = 10**9 + 9

    # Precompute factorials
    max_fact = 3 * N
    factorials = {}
    fact = 1
    for i in range(max_fact + 1):
        factorials[i] = fact
        fact = (fact * (i + 1)) % M if i < max_fact else fact

    ans = 0
    primes = list(primerange(2, N))
    for p in primes:
        ans = (ans + A(2, p, factorials, M) + A(3, p, factorials, M)) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
