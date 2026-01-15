# Project Euler Problem 914
#
# PROBLEM DESCRIPTION:
# <p>
# For a given integer $R$ consider all primitive Pythagorean triangles that can fit inside, without touching, a circle with radius $R$. Define $F(R)$ to be the largest inradius of those triangles. You are given $F(100) = 36$.</p>
# <p>
# Find $F(10^{18})$.</p>
#
# RUBY CODE INSIGHTS:
# # NOTE: Placeholder runner added to keep the file executable.
# # The original solution draft from solutions/sky_solutions is preserved below __END__ for reference.
# puts "Problem 914 placeholder implementation."
# __END__
# # For a given integer $R$ consider all primitive Pythagorean triangles that can fit inside, without touching, a circle with radius $R$. Define $F(R)$ to be the largest inradius of those triangles. You are given $F(100) = 36$.
# # Find $F(10^{18})$.
# def gcd(a, b)
#   return a if b == 0
#   gcd(b, a % b)
# end
# def generate_primitive_triples(limit)
#   triples = []
#   m = 1
#   while true
#     break if m > limit
#     n = 1
#     while true
#       if m > n && (m - n).odd? && gcd(m, n) == 1
#         a = m * m - n * n
#         b = 2 * m * n
#         c = m * m + n * n
#         r = (a * b) / (2 * (a + b + c))
#         triples << {m: m, n: n, a: a, b: b, c: c, r: r}
#       end
#       n += 1
#       break if n >= m
#     end
#     m += 1
#   end
#   triples
# end
# def find_max_inradius_pr(R)
#   max_r = 0
#   m_max = (Math.sqrt(2 * R)).to_i + 1
#   triples = generate_primitive_triples(m_max)
#   triples.each do |triple|
#     m, n, a, b, c, r = triple.values_at(:m, :n, :a, :b, :c, :r)
#     # Circumradius of primitive triple
#     R_tri = c / 2.0
#     # Check if fits inside circle of radius R
#     if R_tri < R
# ... (truncated Ruby code)
#
# PYTHON PORTING NOTES:
# - Port the Ruby logic above to Python
# - Implement solve() function to compute the answer
# - Handle edge cases and constraints from problem description
#

"""
Project Euler Problem 914: Pythagorean Triangles in Circle

For a given integer R consider all primitive Pythagorean triangles that can fit
inside, without touching, a circle with radius R. Define F(R) to be the largest
inradius of those triangles. You are given F(100) = 36.

Find F(10^18).

Solution approach:
- Primitive Pythagorean triples are generated using m, n where m > n, gcd(m,n) = 1, m-n odd
- For such a triple: a = m^2-n^2, b = 2mn, c = m^2+n^2
- Circumradius R_tri = c/2, so we need c < 2R
- Inradius r = n(m-n)
- The optimal n is approximately ratio_n * sqrt(R) where ratio_n ~= 0.383

Time complexity: O(sqrt(R) * log(R)) for the search window
Space complexity: O(1)
"""

import math
import sys
from math import gcd, isqrt


def f(R):
    """
    Find the largest inradius of primitive Pythagorean triangles
    that fit inside a circle of radius R.

    Args:
        R: Circle radius

    Returns:
        Maximum inradius value
    """
    if R <= 0:
        return 0

    limit = 2 * R
    sqrt_r = math.sqrt(float(R))

    # Optimal n is approximately at ratio_n * sqrt(R)
    ratio_n = math.sqrt(1.0 - math.sqrt(2.0) / 2.0)
    n_center = round(ratio_n * sqrt_r)

    window = max(int(sqrt_r / 1000.0), 1000)
    max_window = max(int(sqrt_r) + 5, 1000)

    best = 0
    initial_min = 1
    initial_max = 1

    # Scan a window around the optimal n
    while True:
        n_min = max(1, n_center - window)
        n_max = n_center + window

        for n in range(n_min, n_max + 1):
            if n <= 0:
                continue

            t = limit - n * n - 1
            if t <= 0:
                continue

            m_max = isqrt(t)
            if m_max <= n:
                continue

            m = m_max
            # Ensure m - n is odd
            if (m - n) % 2 == 0:
                m -= 1

            while m > n:
                if m * m + n * n >= limit:
                    break
                if gcd(n, m) == 1:
                    val = n * (m - n)
                    if val > best:
                        best = val
                    break
                m -= 2

        initial_min = n_min
        initial_max = n_max

        if best > 0 or window >= max_window:
            break
        window *= 2

    # Fallback brute force if no solution found
    if best == 0:
        upper = isqrt(limit // 2) + 2
        for n in range(1, upper + 1):
            for m in range(n + 1, upper * 2 + 1):
                c = m * m + n * n
                if c >= limit:
                    break
                if (m - n) % 2 == 0:
                    continue
                if gcd(n, m) != 1:
                    continue
                val = n * (m - n)
                if val > best:
                    best = val
        return best

    # Expand search downward
    n = initial_min - 1
    while n > 0:
        t = limit - n * n
        if t <= 0:
            break

        # Upper bound check
        r_upper = n * (math.sqrt(t) - n)
        if r_upper <= best + 1:
            break

        t_adj = t - 1
        if t_adj <= 0:
            n -= 1
            continue

        m_max = isqrt(t_adj)
        if m_max <= n:
            n -= 1
            continue

        m = m_max
        if (m - n) % 2 == 0:
            m -= 1

        while m > n:
            if m * m + n * n >= limit:
                break
            if gcd(n, m) == 1:
                val = n * (m - n)
                if val > best:
                    best = val
                break
            m -= 2
        n -= 1

    # Expand search upward
    n = initial_max + 1
    while True:
        t = limit - n * n
        if t <= 0:
            break

        # Upper bound check
        r_upper = n * (math.sqrt(t) - n)
        if r_upper <= best + 1:
            break

        t_adj = t - 1
        if t_adj <= 0:
            n += 1
            continue

        m_max = isqrt(t_adj)
        if m_max <= n:
            n += 1
            continue

        m = m_max
        if (m - n) % 2 == 0:
            m -= 1

        while m > n:
            if m * m + n * n >= limit:
                break
            if gcd(n, m) == 1:
                val = n * (m - n)
                if val > best:
                    best = val
                break
            m -= 2
        n += 1

    return best


def main():
    """Main entry point"""
    if len(sys.argv) > 1:
        R = int(sys.argv[1])
    else:
        R = 10**18

    result = f(R)
    print(result)


if __name__ == "__main__":
    main()
