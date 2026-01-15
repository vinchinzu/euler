"""Project Euler Problem 672: One More One.

Define a process where we start with n and repeatedly divide by 7 if the current
number is divisible by 7, and add one otherwise, ending at 1. Let g(n) be the
number of times when we add one, S(N) = sum_{n=1}^N g(n), and H(K) = S((7^K - 1) / 11).
Find H(N).

First consider S(N) = g(1) + ... + g(N). The values up to g(7⌊N/7⌋) can be split
into ⌊N/7⌋ groups of 7. In each group (7k+1, ..., 7k+7), the first number needs
to be incremented 6 times before it becomes a multiple of 7, the next one 5 times,
and so on down to 0 times. This gives 21 steps, plus an additional 7g(k) steps.
All groups together require 21⌊n/7⌋ + 7S(⌊n/7⌋) steps.

There are (N%7) remaining numbers. Again, the first number needs to be incremented
6 times, and so on, which in total require (21 - tr(6 - n%7)) steps, and the result
will always be 7(⌊n/7⌋ + 1), which require g(⌊n/7⌋ + 1) more steps to get to 1.

Finally, we need to make one correction: g(1) = 0, i.e. we don't need to increment
6 times like any of the other numbers that are 1 (mod 7). This means we over-counted
and need to subtract 6. The final recurrence is

S(N) = 21⌊n/7⌋ + 7S(⌊n/7⌋) + (21 - tr(6 - n%7)) + (n%7)g(⌊n/7⌋ + 1) - 6
     = 7S(⌊n/7⌋) + 21⌊n/7⌋ + (n%7)g(⌊n/7⌋ + 1) + 15 - tr(6 - n%7).

Note that g(⌊n/7⌋ + 1) is the sum of (6-b) over all digits b in the base 7
representation of ⌊n/7⌋.

To compute H(K) efficiently, we can compute the base 7 representation of
N = (7^K - 1) / 11 with long division, and then iteratively compute the values
of S for prefixes of N, keeping track of cumulative values of ⌊n/7⌋ and
g = sum (6-b) for prefixes of N for efficient computation of S.

Finally, for extra speed, we recognize that the base 7 representation of N is
periodic, so obeys a relatively small recurrence. This allows us to only compute
H(K) for relatively small values of K.
"""

from __future__ import annotations

from typing import Callable


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def extrapolation(
    f: Callable[[int], int], n_points: int, mod: int
) -> Callable[[int], int]:
    """Extrapolate function using Lagrange interpolation."""
    # Generate n_points values
    values = []
    for i in range(1, n_points + 1):
        values.append(f(i) % mod)

    def interpolate(x: int) -> int:
        """Interpolate at point x."""
        result = 0
        for i in range(n_points):
            term = values[i]
            for j in range(n_points):
                if i != j:
                    denom = (i + 1 - (j + 1)) % mod
                    if denom == 0:
                        continue
                    # Compute modular inverse
                    inv = pow(denom, mod - 2, mod)
                    term = (term * (x - (j + 1)) * inv) % mod
            result = (result + term) % mod
        return result

    return interpolate


def solve() -> int:
    """Solve Problem 672."""
    N = 10**9
    K = 11
    M = 1117117717
    B = 7

    def H(k: int) -> int:
        """Compute H(k) = S((7^k - 1) / 11)."""
        n = B - 1
        n_div_b = 0
        g = 0
        H_val = 0
        for i in range(1, k):
            n = n * B + (B - 1)
            digit = n // K
            H_val = (
                B * H_val
                + n_div_b * tr(B - 1)
                + digit * g
                + tr(B - 2)
                - tr(B - 1 - digit)
            ) % M
            n -= digit * K
            n_div_b = (B * n_div_b + digit) % M
            g += B - 1 - digit
        return H_val

    extrap_func = extrapolation(H, 3, M)
    ans = extrap_func(N) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
