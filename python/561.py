"""Project Euler Problem 561: Divisor pairs.

Let S(n) be the number of pairs (a, b) of distinct divisors of n, with a|b.
Let E(m, n) be the largest integer k such that 2^k | S((2*3*...*p_m)^n).
Find Σ_{n=1}^N E(K, n).

Given (2*3*...*p_m)^n, we choose a divisor b = 2^{e_1} * 3^{e_2} * ... *
(p_m)^{e_m}. Then there are Π_i (e_i + 1) divisors of b, and a can be any of
them other than b itself. Expanding this over all possible b gives

S((2*3*...*p_K)^n) = (Π_i (1+2+...+(K+1))) - (Π_i K+1)
                  = tr(n+1)^K - (n+1)^K.

If n≡1 or 2 (mod 4), then tr(n+1)^K and (n+1)^K are opposite sign and
E(K, n) = 0. If n=4t-1, then the expression becomes 2^K t^K ((n+1)^K - 2^K),
and the rightmost term is odd, so E(K, n) is one plus K times the number of
factors of 2 in t. And finally if n=4t, then the expression becomes
(n+1)^K ((2t+1)^K - 1), the leftmost term is odd, and E(K, n) is the number
of factors of 2 in (2t+1)^K - 1 = (2t) ((2t+1)^{K-1}+...+1). Since K is odd,
the right term is odd, so this is one plus the number of factors of 2 in t.

The sum of the number of factors of 2 for all t≤N/4 can be found by adding
the number of t with at least one factor of 2, the number of t with at least
two factors, etc. For each t, we have K from the n=4t-1 case and 1 from the
n=4t case, for a multiplier of K+1.
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 561."""
    N = 10**12
    K = 904961

    ans = 0
    n = N // 4
    while n > 0:
        ans += (K + 1) * n
        n //= 2

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
