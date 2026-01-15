"""Project Euler Problem 129: Repunit divisibility.

A number consisting entirely of ones is called a repunit. We shall define R(k) to be a repunit of length k; for example, R(6) = 111111.

Given that n is a positive integer and gcd(n, 10) = 1, it can be shown that there always exists a value, k, for which R(k) is divisible by n, and let A(n) be the least such value of k; for example, A(7) = 6 and A(41) = 5.

The least value of n for which A(n) first exceeds ten is 17.

Find the least value of n for which A(n) first exceeds one-million.
"""

from math import gcd, sqrt
from sympy import factorint
from typing import Dict, List

LIMIT = 1_000_000
MAX_N = 20_000_000  # Increased to cover all possible m = 9 * n


def build_spf(max_n: int) -> List[int]:
    """Build smallest prime factor array."""
    spf = list(range(max_n + 1))
    for i in range(2, int(sqrt(max_n)) + 1):
        if spf[i] == i:
            for j in range(i * i, max_n + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def factor(num: int, spf: List[int]) -> Dict[int, int]:
    """Factor a number using SPF."""
    res: Dict[int, int] = {}
    while num > 1:
        p = spf[num]
        if p is None:
            raise ValueError(f"SPF not found for num={num}. Increase sieve size.")
        while num % p == 0:
            res[p] = res.get(p, 0) + 1
            num //= p
    return res


def order_10_pe(p: int, e: int, spf: List[int]) -> int:
    """Compute order of 10 modulo p^e."""
    if e == 0:
        return 1
    m = p ** e
    phi = (p - 1) * (p ** (e - 1))
    phi_f = factor(phi, spf)
    d = phi
    for q in phi_f.keys():
        while d % q == 0 and pow(10, d // q, m) == 1:
            d //= q
    return d


def lcm(a: int, b: int) -> int:
    """Compute LCM."""
    return a * b // gcd(a, b)


def compute_a(n: int, spf: List[int]) -> int:
    """Compute A(n)."""
    m = 9 * n
    factors = factor(m, spf)
    lcm_val = 1
    for p, e in factors.items():
        ord_val = order_10_pe(p, e, spf)
        lcm_val = lcm(lcm_val, ord_val)
    return lcm_val


def main() -> int:
    """Main function."""
    spf = build_spf(MAX_N)
    n = 1
    while n <= MAX_N:
        if n % 2 != 0 and n % 5 != 0:
            a = compute_a(n, spf)
            if a > LIMIT:
                return n
        n += 1
    return 0


if __name__ == "__main__":
    print(main())
