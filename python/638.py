"""Project Euler Problem 638: Weighted Paths in a Grid.

Let A(P_{a,b}) be the area under a path from (0, 0) to (a, b) that always
increases in the x or y coordinate. Find sum_{k=1}^N C(10^k + k, 10^k + k, k),
where C(a, b, k) is the sum of A(P_{a,b}) for all paths P.

C(a, b, k) is the "q binomial coefficient", or the q-analog of the binomial
coefficient. It can be computed in linear time by computing the q-analog of
the integers, [n]_q = 1 + q + q² + q³ + ..., and using the normal formula for
the binomial coefficient.
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


def C(a: int, b: int, k: int, M: int) -> int:
    """Compute q-binomial coefficient."""
    pow_k = 1
    sum_pow_k = 0
    factorials = [1] * (a + b + 1)

    for i in range(1, a + b + 1):
        sum_pow_k = (sum_pow_k + pow_k) % M
        factorials[i] = (factorials[i - 1] * sum_pow_k) % M
        pow_k = (pow_k * k) % M

    num = factorials[a + b]
    den = (factorials[a] * factorials[b]) % M
    return (num * mod_inverse(den, M)) % M


def solve() -> int:
    """Solve Problem 638."""
    N = 7
    M = 10**9 + 7

    ans = 0
    for k in range(1, N + 1):
        base = 10**k + k
        ans = (ans + C(base, base, k, M)) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
