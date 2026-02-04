"""Project Euler Problem 247: Squares under a hyperbola.

Take the region constrained by 1 <= x and 0 <= y <= 1/x, and repeatedly add the
largest square that does not overlap with any of the previous squares. Find
the largest n such that S_n has IX squares to its left and IY squares below it.
"""

import heapq
from math import sqrt, comb


def solve():
    IX = 3
    IY = 3
    num_at_index = comb(IX + IY, IX)

    # Each entry: (-s, ix, iy, x0, y0)  (negate s for max-heap via min-heap)
    squares = []

    def make_entry(ix, iy, x0, y0):
        s = (sqrt(x0 * x0 + y0 * y0 - 2 * x0 * y0 + 4) - (x0 + y0)) / 2
        return (-s, ix, iy, x0, y0)

    heapq.heappush(squares, make_entry(0, 0, 1.0, 0.0))
    ans = 0

    while num_at_index > 0:
        neg_s, ix, iy, x0, y0 = heapq.heappop(squares)
        s = -neg_s
        if ix == IX and iy == IY:
            num_at_index -= 1
        heapq.heappush(squares, make_entry(ix + 1, iy, x0 + s, y0))
        heapq.heappush(squares, make_entry(ix, iy + 1, x0, y0 + s))
        ans += 1

    return ans


if __name__ == "__main__":
    print(solve())
