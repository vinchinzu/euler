"""Project Euler Problem 770: Deliberate Strategy.

A and B play a game with 2R turns; A starts with 1 gram of gold, and on
each turn wagers some amount that A has. B can either GIVE that amount to
A or TAKE that amount from A, but with the requirement that out of all
2R turns B will GIVE R times and TAKE R times. Find the minimum amount
that A can guarantee having at the end of the game.

Let f(a,b) be the amount that A can guarantee if B must GIVE a times
and TAKE b times. If a=0, then f(a,b)=1. If b=0, then f(a,b)=2^a. We
can see that if A wagers r, then either B does a GIVE, so A will
eventually end up with f(a-1,b)(1+r), or B does a TAKE, causing A to
eventually end up with f(a,b-1)(1-r). The optimum has to occur when
these values are equal, which lets us solve for r.

Solving small values tells us that f(n,n) = 2 - 2/(1+4^n/nCr(2n,n))
≈ 2 - 2/(1+√(π*n)). Setting this to be greater than R gives
n ≥ (2/R-1)⁻²/π.
"""

from __future__ import annotations

import math


def solve() -> int:
    """Solve Problem 770."""
    R = 1.9999
    n = 1.0 / (2 / R - 1) ** 2 / math.pi
    return math.ceil(n)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
