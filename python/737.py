"""Project Euler Problem 737: Coin Loops.

A line is perpendicular to a table, and coins are stacked on top of each
other, one by one, such that each one touches the line. Find the minimum
number of coins needed to make a coin loop that winds around the line at
least N times (the sum of the angles between each pair of adjacent coins is
at least 2π*N).

We simulate placing the coins one by one, but in reverse order, where
(cx, cy) is the center of the current coin, and (x, y) is the center of
mass of all coins so far. In that case, the if we let O be the line (the
origin), C be the current center of mass, D be the center of the next coin,
then we have OCD is a triangle with OD = CD = 1. Let B be the midpoint of
OC. To compute the new center D, we note that OC and BD are perpendicular,
and l = BD/OC = √(1/(x²+y²) - 1/4). So the new coordinates (cx, cy) of D
can be computed by starting at B = (x/2, y/2) and translating by the
perpendicular displacement (-y*l, x*l). The new center of mass (x,y) can be
computed by adding D weighted by 1/k.

A loop is completed when the new cy changes sign from negative to positive.
When we complete N loops, the number of iterations is our answer.
"""

from __future__ import annotations

from math import sqrt


def fsq(x: float) -> float:
    """Square of x."""
    return x * x


def solve() -> int:
    """Solve Problem 737."""
    n = 2020
    x = 1.0
    y = 0.0
    last_cy = 0.0
    num_loops = 0

    k = 2
    while True:
        l = sqrt(1.0 / (fsq(x) + fsq(y)) - 0.25)
        cx = x / 2.0 - y * l
        cy = y / 2.0 + x * l
        x += (cx - x) / k
        y += (cy - y) / k

        if cy > 0 and last_cy < 0:
            num_loops += 1
        last_cy = cy

        if num_loops == n:
            return k

        k += 1


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
