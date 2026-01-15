"""Project Euler Problem 184: Triangles containing the origin."""

from typing import Dict, List, Tuple
import math

RADIUS = 105
RADIUS_SQ = RADIUS * RADIUS
EPSILON = 1e-12
TWO_PI = 2.0 * math.pi
PI = math.pi


def main() -> int:
    """Main function."""
    angles: List[float] = []
    direction_counts: Dict[Tuple[int, int], List[int]] = {}

    min_coord = -(RADIUS - 1)
    max_coord = RADIUS - 1

    for x in range(min_coord, max_coord + 1):
        for y in range(min_coord, max_coord + 1):
            if x == 0 and y == 0:
                continue
            if x * x + y * y >= RADIUS_SQ:
                continue

            angles.append(math.atan2(float(y), float(x)))

            g = math.gcd(abs(x), abs(y))
            dx = x // g
            dy = y // g

            key = (dx, dy)
            if key not in direction_counts:
                direction_counts[key] = [0, 0]

            if dy > 0 or (dy == 0 and dx > 0):
                direction_counts[key][0] += 1
            else:
                # For negative direction, use opposite key
                opposite_key = (-dx, -dy)
                if opposite_key not in direction_counts:
                    direction_counts[opposite_key] = [0, 0]
                direction_counts[opposite_key][1] += 1

    angles.sort()
    count_points = len(angles)
    extended_angles = angles + [a + TWO_PI for a in angles]

    bad = 0
    j = 0
    for i in range(count_points):
        j = max(j, i + 1)
        while j < i + count_points and extended_angles[j] - angles[i] < PI - EPSILON:
            j += 1
        m = j - i - 1
        bad += m * (m - 1) // 2

    opposite = 0

    for pos, neg in direction_counts.values():
        if pos == 0 or neg == 0:
            continue
        total_on_line = pos + neg
        other_points = count_points - total_on_line

        opposite += pos * neg * other_points
        opposite += (pos * (pos - 1) // 2) * neg
        opposite += pos * (neg * (neg - 1) // 2)

    total = count_points * (count_points - 1) * (count_points - 2) // 6
    result = total - bad - opposite
    return result


if __name__ == "__main__":
    print(main())
