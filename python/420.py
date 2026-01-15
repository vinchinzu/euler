"""Project Euler Problem 420: 2x2 positive integer matrix.

Find the number of 2x2 matrices with positive integer entries with trace
less than N and that can be expressed as a square of a 2x2 matrix with
positive integer entries in two different ways.

Suppose that a 2x2 matrix X² is the square of a 2x2 matrix X. Then we have
the following explicit formula for their entries:
⌈ A  B ⌉    ⌈ a  b ⌉²  ⌈ a²+b*c  b(a+d) ⌉
|      | = |      | = |                |
⌊ C  D ⌋    ⌊ c  d ⌋   ⌊ c(a+d)  d²+b*c ⌋

Note that (a+d)² - (A+D) = (a² + 2a*d + d²) - (a² + 2b*c + d²) = 2(a*d -
b*c) is twice the discriminant of X. This means that given a matrix X²,
the discriminant of X must be ±√|X²|, so the trace of X, a+d, must be one
of two values: √(A+D ± (√|X²| / 2)).

Suppose we only know the traces of the two square root matrices, T1 < T2,
and we want to compute the number of valid square matrices [A B | C D]. From
the previous formula, T1² + T2² = 2(A+D) ≤ 2N. And using the explicit
formula above, A-D = (a²+b*c) - (d²+b*c) = a²-d² = (a-d)(a+d) = (a-d) T1.
This means that T1 | (A-D), and by the same logic so does T2, so (A-D) =
r LCM(T1, T2). For a fixed r, we can now derive formulas for A, D, and BC.

The number of ways to split the factors of BC into positive B and C is just
the number of divisors of (GCD(T1, T2)² - r²) / 4.

We can let r≥0 without loss of generality, so a-d ≥ 0. This means that for
strictly positive r, we need to multiply the number of results by 2.

Since all entries must be positive, we must also have a-d < a+d. This means
r LCM(T1, T2) / T1 < T1 => r T2 / GCD(T1, T2) < T1 => r T2 < T1 GCD(T1, T2).

Finally, we need to include parity restrictions:
- T1² + T2² = 2(A+D) must be even, so T1 and T2 must be the same parity.
- GCD(T1, T2) and r must have the same parity in order to give valid values
  for B and C.
"""

from __future__ import annotations

from math import gcd, isqrt


def sq(n: int) -> int:
    """Return n squared."""
    return n * n


def num_divisors(limit: int) -> list[int]:
    """Count divisors for each number up to limit."""
    result = [0] * (limit + 1)
    for i in range(1, limit + 1):
        for j in range(i, limit + 1, i):
            result[j] += 1
    return result


def solve() -> int:
    """Solve Problem 420."""
    N = 10**7

    num_divs = num_divisors(2 * N)
    ans = 0

    for T1 in range(1, isqrt(N) + 1):
        for T2 in range(T1 + 2, isqrt(2 * N - sq(T1)) + 1, 2):
            g = gcd(T1, T2)
            for r in range(g % 2, T1 * g // T2 + 1, 2):
                val = (sq(g) - sq(r)) // 4
                if val > 0 and val < len(num_divs):
                    ans += (1 if r == 0 else 2) * num_divs[val]

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
