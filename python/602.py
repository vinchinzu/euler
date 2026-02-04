"""Project Euler Problem 602: Product of Head Counts.

Starting with Alice, Alice and n friends take turns flipping a coin that comes
up tails with probability p. Let e(n, p) be the expected product of the number
of heads that each friend flips before Alice flips heads for the first time.
For a given n, e(n, p) is a polynomial in p; find the coefficient of p^k.

If each of Alice's friends flips r times before Alice flips heads, then the
expected head count for each friend is r(1-p) and the expected product of all
head counts is (r(1-p))^n. The expected product over all r is
p(1-p)*(1-p) + p²(1-p)*(2(1-p))² + p³(1-p)*(3(1-p))³ + ...
Each p^k term is obtained by multiplying a t^n*p^t term from the right with
the p^(k-t) term on the left, which has coefficient (-1)^(k-t) nCr(n+1,k-t)
from Binomial Theorem. Summing over all 0≤t≤k gives the answer.
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


def nth_pows(limit: int, exp: int, mod: int) -> list[int]:
    """Compute 0^exp, 1^exp, 2^exp, ..., limit^exp modulo mod."""
    pows = [0] * (limit + 1)
    for i in range(limit + 1):
        pows[i] = pow(i, exp, mod)
    return pows


def solve() -> int:
    """Solve Problem 602."""
    N = 10_000_000
    K = 4_000_000
    M = 10**9 + 7

    mod_invs_list = mod_invs(K, M)
    nCrs = [0] * (K + 1)
    nCrs[0] = 1
    for i in range(1, K + 1):
        nCrs[i] = nCrs[i - 1] * (N + 2 - i) % M * mod_invs_list[i] % M

    pows = nth_pows(K, N, M)
    ans = 0
    for t in range(K + 1):
        sign = 1 if (K - t) % 2 == 0 else M - 1
        coeff = nCrs[K - t] * sign % M * pows[t] % M
        ans = (ans + coeff) % M
    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
