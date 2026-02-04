"""Project Euler Problem 279: Triangles with Integral Sides and an Integral Angle.

How many triangles are there with integral sides, at least one integral angle
(measured in degrees), and a perimeter that does not exceed 10^8?

By Niven's theorem, the only integer degree angles with rational cosine are
60, 90, and 120 degrees. We enumerate primitive triangles for each family
using known parameterizations (m > n > 0, gcd(m,n) = 1):

  Equilateral:     perimeter = 3
  60-deg Type I:   (m-n) % 3 != 0, perimeter = 2m^2 + 2n^2 + 5mn
  60-deg Type II:  (m-n) % 3 != 0, perimeter = 3m^2 + 3mn
  120-deg:         (m-n) % 3 != 0, perimeter = 2m^2 + n^2 + 3mn
  90-deg:          (m-n) odd,       perimeter = 2m^2 + 2mn

For each primitive with perimeter p, there are floor(N/p) similar triangles.
No triangle belongs to more than one family (no integer-sided triangle can
simultaneously have two of these angles), so we simply sum all contributions.
"""

from math import gcd, isqrt


def solve() -> int:
    N = 100000000  # 10^8

    ans = N // 3  # equilateral triangles

    m_limit = isqrt(N >> 1) + 2
    _gcd = gcd

    # 60-degree Type I: perimeter = 2m^2 + 2n^2 + 5mn
    for m in range(2, m_limit + 1):
        mm2 = 2 * m * m
        m5 = 5 * m
        if mm2 + m5 + 2 > N:
            break
        bad3 = m % 3
        for n in range(1, m):
            p = mm2 + 2 * n * n + m5 * n
            if p > N:
                break
            if n % 3 != bad3 and _gcd(m, n) == 1:
                ans += N // p

    # 60-degree Type II: perimeter = 3m(m+n)
    for m in range(2, m_limit + 1):
        m3 = 3 * m
        if m3 * (m + 1) > N:
            break
        bad3 = m % 3
        for n in range(1, m):
            p = m3 * (m + n)
            if p > N:
                break
            if n % 3 != bad3 and _gcd(m, n) == 1:
                ans += N // p

    # 120-degree: perimeter = 2m^2 + n^2 + 3mn
    for m in range(2, m_limit + 1):
        mm2 = 2 * m * m
        m3 = 3 * m
        if mm2 + m3 + 1 > N:
            break
        bad3 = m % 3
        for n in range(1, m):
            p = mm2 + n * n + m3 * n
            if p > N:
                break
            if n % 3 != bad3 and _gcd(m, n) == 1:
                ans += N // p

    # 90-degree (Pythagorean): perimeter = 2m(m+n), (m-n) must be odd
    for m in range(2, m_limit + 1):
        m2 = 2 * m
        if m2 * (m + 1) > N:
            break
        n_start = 2 if m & 1 else 1
        for n in range(n_start, m, 2):
            p = m2 * (m + n)
            if p > N:
                break
            if _gcd(m, n) == 1:
                ans += N // p

    return ans


def main() -> None:
    print(solve())


if __name__ == "__main__":
    main()
