"""Project Euler Problem 555: McCarthy 91 function.

Let SF(m,k,s) be the sum of the natural numbers n that are fixed points of the
generalized McCarthy 91 function, M_{m,k,s}(n) = n-s if n > m, and
M_{m,k,s}(M_{m,k,s}(n+k)) otherwise. Find Σ_{1≤s<k≤N} SF(M,k,s).

Let d = k-s. First we show by induction that if n > m-k-t*d for nonnegative
integer t, then M(n) = M(n+(t+1)d). We have M(n) = M(M(n+k)) =
M(M(n+k+(t-1)d) = M(n+k+(t-1)d-s) = M(n+t*d). Therefore, the values of M(n)
cycle periodically from M(m+1) to M(m+d), with a period length of d.

This means that n - M(n) (mod d) is constant, and is equal to m - M(m) = s. So
in order to have fixed points, we must have s ≡ 0 (mod d). Then all of the
values from M(m+1) = m+1-s to M(m+d) = m+d-s are fixed points, with a sum of
tr(M(m+d-s)) - tr(M-s).
"""

from __future__ import annotations


def triangular(n: int) -> int:
    """Return triangular number T(n) = n*(n+1)/2."""
    if n < 0:
        return 0
    return n * (n + 1) // 2


def solve() -> int:
    """Solve Problem 555."""
    N = 1000000
    M = 1000000  # This is the modulus, not the parameter m

    # Looking at the Java code, it uses `M` as both modulus and parameter
    # But in the problem statement, M is the parameter for SF(M,k,s)
    # The Java code seems to use M (the modulus value) as the parameter m
    # This is likely a naming conflict. Let's use M as the parameter m.
    m_param = M

    ans = 0
    for d in range(1, N // 2 + 1):
        for s in range(d, N - d + 1, d):
            # s must be divisible by d (s ≡ 0 mod d)
            # k = s + d, so we need s + d <= N, which is s <= N - d
            term1 = triangular(m_param + d - s)
            term2 = triangular(m_param - s)
            ans += term1 - term2

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
