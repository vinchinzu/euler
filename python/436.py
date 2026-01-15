"""Project Euler Problem 436: Unfair wager.

In a game, Louise generates random numbers uniformly from 0 to 1 until
their sum is greater than 1, and takes the last number x. Then Julie
continues generating numbers in the same way until their sum is greater
than 2, and takes the last number y. Find the probability y > x.

After n+1 numbers are generated, the probability distribution for their
sum S on [0, 1] is P(S) = S^n / n! by induction. That means that the
probability that Louise's last number is x and total is T is (subject to
the constraint that 1 ≤ T ≤ 1+x):

P(x, T) = Σ_{n=0}^∞ P(T-x)
        = e^{T-x}.

Now if there are m+1 more numbers generated, then the probability that the
last number is y is the integral of the same probability distribution from
S ∈ [2-T - y, 2-T], summed over all m:

P(y) = Σ_{n=0}^∞ ∫_{2-T-y}^{2-T} S^n / n! dS
     = ∫_{2-T-y}^{2-T} e^S dS
     = e^{2-T} - e^{2-T-y}.

This is true if y ≤ 2-T. Otherwise, we only integrate from the lower
limit of S = 0, which gives e^{2-T} - 1. However, if y ≥ 2-T, it is also
possible this time for the number right after x to be y, so the actual
probability is P(y) = e^{2-T}.

The probability that y > x is therefore the integral of all valid P(x, T)
P(y) where y > x:

P(y > x) = ∫_1^2 ∫_{T-1}^1 ∫_x^1  P(x, T) P(y)  dy dx dT
         = (1 + 14e - 5e²) / 4.
"""

from __future__ import annotations

import math


def solve() -> float:
    """Solve Problem 436."""
    e = math.e
    ans = (1 + 14 * e - 5 * e * e) / 4
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return result


if __name__ == "__main__":
    main()
