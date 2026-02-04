"""Project Euler Problem 607: Marsh Crossing.

Find the minimum time to get from point A to a point B that is D units away
due east, if a marsh with width L running exactly south-west to north-east is
exactly midway between A and B, and consists of K strips dividing the map
into regions where you can go at the speed given in SPEEDS.

The problem is equivalent to finding the time for a beam of light to go from
A to B, given the same relative speeds in the different regions. Given an
initial orientation angle alpha, it is possible to trace the beam of light
originating from A at angle alpha, by using Snell's Law at every border between
two regions.
"""

from __future__ import annotations

import math


def solve() -> float:
    """Solve Problem 607."""
    D = 100
    L = 50
    K = 5
    SPEEDS = [10, 9, 8, 7, 6, 5, 10]

    border_xs = []
    for i in range(K + 1):
        border_xs.append(D / 2 - L / math.sqrt(2) + (L / K) * math.sqrt(2) * i)
    border_xs.append(float(D))

    def trace(alpha):
        """Trace a ray from A=(0,0) at angle alpha.

        Returns (time, final_y) where final_y is the y coordinate when
        hitting the last border (the vertical line through B).
        """
        a = alpha
        px, py = 0.0, 0.0
        total_time = 0.0

        for i in range(len(border_xs)):
            # Border line goes through (border_xs[i], 0) at angle pi/4
            # Parametric: border point = (bx + t*cos(pi/4), 0 + t*sin(pi/4))
            # Ray: point = (px + s*cos(a), py + s*sin(a))
            bx = border_xs[i]

            # Border direction: (cos(pi/4), sin(pi/4)) = (1/sqrt2, 1/sqrt2)
            # But the border is a line, equation: x - y = bx (since slope=1 through (bx,0))
            # Ray from (px, py) at angle a: x = px + s*cos(a), y = py + s*sin(a)
            # Intersection: px + s*cos(a) - py - s*sin(a) = bx
            # s = (bx - px + py) / (cos(a) - sin(a))

            # For the last border, it's a vertical line at x = D going at pi/4
            # Actually the Java code uses Line.of(new FPoint(borderXs.get(i), 0), Math.PI / 4)
            # which creates a line through (bx, 0) at angle pi/4

            # Line through (bx, 0) at angle pi/4:
            # A = sin(pi/4), B = -cos(pi/4), C = sin(pi/4)*bx + (-cos(pi/4))*0 = bx/sqrt2

            # Ray through (px, py) at angle a:
            # A2 = sin(a), B2 = -cos(a), C2 = sin(a)*px + (-cos(a))*py

            A1 = math.sin(math.pi / 4)
            B1 = -math.cos(math.pi / 4)
            C1 = A1 * bx + B1 * 0.0

            A2 = math.sin(a)
            B2 = -math.cos(a)
            C2 = A2 * px + B2 * py

            denom = A2 * B1 - B2 * A1
            if abs(denom) < 1e-15:
                return total_time, py  # degenerate

            nx = (C2 * B1 - B2 * C1) / denom
            ny = (C2 * A1 - A2 * C1) / (B2 * A1 - A2 * B1)

            dist = math.sqrt((nx - px)**2 + (ny - py)**2)
            total_time += dist / SPEEDS[i]
            px, py = nx, ny

            if i < len(border_xs) - 1:
                # Snell's law: sin(theta1)/v1 = sin(theta2)/v2
                # theta is the angle with the normal to the border
                # The border is at pi/4, so the normal is at pi/4 + pi/2 = 3pi/4
                # The angle of incidence is pi/4 + a
                sin_val = SPEEDS[i + 1] * math.sin(math.pi / 4 + a) / SPEEDS[i]
                if abs(sin_val) > 1:
                    return total_time, py  # total internal reflection
                a = math.asin(sin_val) - math.pi / 4

        return total_time, py

    low = 0.0
    high = math.pi / 4

    for _ in range(200):  # binary search iterations
        mid = (low + high) / 2
        t, y = trace(mid)
        if y < 0:
            low = mid
        else:
            high = mid

    ans = trace((low + high) / 2)[0]
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return result


if __name__ == "__main__":
    main()
