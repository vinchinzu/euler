# Project Euler Problem 879
#
# PROBLEM DESCRIPTION:
# <p>A touch-screen device can be unlocked with a "password" consisting of a sequence of two or more distinct spots that the user selects from a rectangular grid of spots on the screen. The user enters their sequence by touching the first spot, then tracing a straight line segment to the next spot, and so on until the end of the sequence. The user's finger remains in contact with the screen throughout, and may only move in straight line segments from spot to spot.</p>
# 
# <p>If the finger traces a straight line that passes over an intermediate spot, then that is treated as two line segments with the intermediate spot included in the password sequence. For example, on a $3\times 3$ grid labelled with digits $1$ to $9$ (shown below), tracing $1-9$ is interpreted as $1-5-9$.</p>
# 
# <p>Once a spot has been selected it disappears from the screen. Thereafter, the spot may not be used as an endpoint of future line segments, and it is ignored by any future line segments which happen to pass through it. For example, tracing $1-9-3-7$ (which crosses the $5$ spot twice) will give the password $1-5-9-6-3-7$.</p>
# <img src="resources/images/0879_touchscreen_159637.png?1707555645" alt="1-5-9-6-3-7 example">
# 
# <p>There are $389488$ different passwords that can be formed on a $3 \times 3$ grid.</p>
# 
# <p>Find the number of different passwords that can be formed on a $4 \times 4$ grid.</p>
#

import math
from functools import lru_cache
import sys

# Increase recursion depth just in case
sys.setrecursionlimit(2000)

def get_intermediates(p1: tuple[int, int], p2: tuple[int, int], width: int, height: int) -> int:
    """
    Returns a bitmask of points strictly between p1 and p2.
    """
    x1, y1 = p1
    x2, y2 = p2
    dx = x2 - x1
    dy = y2 - y1
    g = math.gcd(dx, dy)

    intermediates = 0
    if g > 1:
        step_x = dx // g
        step_y = dy // g
        # Points strictly between, so k from 1 to g-1
        for k in range(1, g):
            mx = x1 + k * step_x
            my = y1 + k * step_y
            idx = mx * width + my
            intermediates |= (1 << idx)
    return intermediates

def count_passwords(width: int, height: int) -> int:
    num_points = width * height

    # Precompute intermediates mask for every pair of points
    inters = [[0] * num_points for _ in range(num_points)]

    for i in range(num_points):
        # Determine coordinates (row, col)
        r1, c1 = divmod(i, width)
        for j in range(num_points):
            r2, c2 = divmod(j, width)
            inters[i][j] = get_intermediates((r1, c1), (r2, c2), width, height)

    @lru_cache(maxsize=None)
    def dfs(current: int, used_mask: int) -> int:
        # count starts at 1 because the current path ending at 'current' is a valid password.
        count = 1

        for next_p in range(num_points):
            # Check if next_p is already used
            if not (used_mask & (1 << next_p)):
                # Logic: We can move to next_p ONLY IF all intermediate points are already used.
                required_intermediates = inters[current][next_p]
                if (required_intermediates & used_mask) == required_intermediates:
                    count += dfs(next_p, used_mask | (1 << next_p))
        return count

    total = 0
    for start_node in range(num_points):
        # We subtract 1 because dfs counts the path of length 1 (just start_node),
        # but the problem requires "sequence of two or more distinct spots".
        total += dfs(start_node, 1 << start_node) - 1

    return total

def solve() -> int:
    return count_passwords(4, 4)

if __name__ == "__main__":
    print(solve())
