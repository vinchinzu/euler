"""Project Euler Problem 296: Angular Bisector and Tangent.

For a triangle ABC with BC≤AC≤AB, let k be the angular bisector of ACB,
m be the tangent at C of circumcircle ABC, n be the line parallel to m
through B, and E be the intersection of n and k. Find the number of
triangles ABC with perimeter at most N such that BE has an integer
length.
"""

from __future__ import annotations

from math import gcd


def solve() -> int:
    """Solve Problem 296."""
    N = 100000
    L = N // 6

    ans = 0
    old_p, old_q = 0, 1
    p, q = 1, L

    while p <= q:
        # We need to count k >= 1 such that there exists y satisfying:
        # y >= k*q/p
        # 2*y + k <= N // (p + q)
        #
        # Let K = N // (p + q)
        # Max valid k satisfies k*q/p <= (K - k)/2
        # => 2*k*q <= p*K - p*k
        # => k * (2*q + p) <= p*K
        # => k <= p*K / (p + 2*q)
        
        K = N // (p + q)
        limit_k = (p * K) // (p + 2 * q)

        if limit_k >= 1:
            for k in range(1, limit_k + 1):
                # Count valid y's for this k
                # Max y: floor((K - k) / 2)
                # Min y: ceil(k * q / p) = (k * q + p - 1) // p
                
                max_y = (K - k) // 2
                min_y = (k * q + p - 1) // p
                
                term = max_y - min_y + 1
                if term > 0:
                    ans += term

        new_p = ((L + old_q) // q) * p - old_p
        new_q = ((L + old_q) // q) * q - old_q
        old_p, old_q = p, q
        p, q = new_p, new_q

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
