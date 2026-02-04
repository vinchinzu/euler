"""Project Euler Problem 777: Lissajous Curves.

Let the Lissajous curve C_{a,b} be a curve parameterized by x = cos(at) and
y = cos(b (t-pi/10)). Find sum(x^2+y^2) for all (x,y) at which C_{a,b} crosses
itself, summed over all relatively prime a,b with 2<=a,b<=N.

The formulas are:
  sum(x^2+y^2) = (4ab-3a-3b)/2  (generic case)
  sum(x^2+y^2) = (2ab-3a-3b+4)/4  (when 10|ab)

We sum these over coprime pairs using Mobius inversion and handle the 10|ab
special case with inclusion-exclusion over divisors of 10/gcd(10,g).
"""

from __future__ import annotations

from math import gcd
from fractions import Fraction


def pre_mobius(limit):
    """Precompute Mobius function."""
    mu = [1] * (limit + 1)
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                if j % (i * i) == 0:
                    mu[j] = 0
                else:
                    mu[j] = -mu[j]
    return mu


def sq(n):
    """Square of n."""
    return n * n


def tr(n):
    """Triangular number n*(n+1)/2."""
    return n * (n + 1) // 2


def all_divisors(n):
    """Return all divisors of n."""
    divisors = []
    for i in range(1, int(n**0.5) + 1):
        if n % i == 0:
            divisors.append(i)
            if i != n // i:
                divisors.append(n // i)
    return sorted(divisors)


def num_divisors(n):
    """Count number of divisors."""
    count = 0
    for i in range(1, int(n**0.5) + 1):
        if n % i == 0:
            count += 2
            if i * i == n:
                count -= 1
    return count


def solve():
    """Solve Problem 777."""
    N = 1000000
    mobius = pre_mobius(N)

    # Precompute num_divisors for all divisors of 10 (1, 2, 5, 10)
    nd = {}
    for d in [1, 2, 5, 10]:
        nd[d] = num_divisors(d)

    # Use Fraction for exact rational arithmetic to avoid float precision loss.
    # The answer involves a sum of terms that are rational (denominators are 1, 2, or 4).
    # We accumulate 4*ans as an integer to keep everything integral.
    # res_main = 2*sq(g*tr(n)) - 3*n*g*tr(n)  contributes to ans as res_main (integer)
    # correction contributes nd_t * (...) / 4, so 4*correction is integer.
    # Overall: 4*ans = sum of mu[g] * (4*res_main + correction_terms_times_4)

    ans_times_4 = 0

    for g in range(1, N + 1):
        if mobius[g] == 0:
            continue
        n = N // g
        if n < 1:
            continue
        t = gcd(10, g)
        trn = tr(n)

        # Main formula: 2*sq(g*trn) - 3*n*g*trn
        # Multiply by 4: 8*sq(g*trn) - 12*n*g*trn
        res4 = 8 * sq(g * trn) - 12 * n * g * trn

        # Correction for 10|ab case
        for d in all_divisors(10 // t):
            e = (10 // t) // d
            nd_t = nd[t]
            # The correction per (d,e) is:
            # nd_t * (-6*sq(g)*d*tr(n//d)*e*tr(n//e) + 3*(n//d)*g*e*tr(n//e) + 3*(n//e)*g*d*tr(n//d) + 4*(n//d)*(n//e)) / 4
            # Multiply by 4:
            # nd_t * (-6*sq(g)*d*tr(n//d)*e*tr(n//e) + 3*(n//d)*g*e*tr(n//e) + 3*(n//e)*g*d*tr(n//d) + 4*(n//d)*(n//e))
            tnd = tr(n // d)
            tne = tr(n // e)
            nd_val = n // d
            ne_val = n // e
            res4 += nd_t * (
                -6 * sq(g) * d * tnd * e * tne
                + 3 * nd_val * g * e * tne
                + 3 * ne_val * g * d * tnd
                + 4 * nd_val * ne_val
            )

        ans_times_4 += mobius[g] * res4

    # ans = ans_times_4 / 4
    # Convert to float for output
    ans = ans_times_4 / 4.0
    return ans


def main():
    """Main entry point."""
    result = solve()
    formatted = f"{result:.9e}"
    formatted = formatted.replace("+", "")
    print(formatted)
    return result


if __name__ == "__main__":
    main()
