"""Project Euler Problem 165: Intersections."""

from typing import Dict, List, Set, Tuple
import math

SEGMENT_COUNT = 5_000
VALUES_COUNT = SEGMENT_COUNT * 4
MOD = 50_515_093
SEED = 290_797
RANGE = 500


def generate_values() -> List[int]:
    """Generate sequence values."""
    values: List[int] = [0] * VALUES_COUNT
    v = SEED
    for i in range(VALUES_COUNT):
        v = (v * v) % MOD
        values[i] = v % RANGE
    return values


def main() -> int:
    """Main function."""
    values = generate_values()
    segments: List[Tuple[int, ...]] = []

    for idx in range(0, VALUES_COUNT, 4):
        x1 = values[idx]
        y1 = values[idx + 1]
        x2 = values[idx + 2]
        y2 = values[idx + 3]
        if x1 == x2 and y1 == y2:
            continue

        min_x = min(x1, x2)
        max_x = max(x1, x2)
        min_y = min(y1, y2)
        max_y = max(y1, y2)
        dx = x2 - x1
        dy = y2 - y1
        cross = x1 * y2 - y1 * x2

        segments.append((x1, y1, x2, y2, min_x, max_x, min_y, max_y, dx, dy, cross))

    segments.sort(key=lambda seg: seg[4])
    count = len(segments)

    intersections: Set[Tuple[int, int, int, int]] = set()

    for i in range(count):
        s1 = segments[i]
        x1, y1, x2, y2 = s1[0], s1[1], s1[2], s1[3]
        min_x1, max_x1 = s1[4], s1[5]
        min_y1, max_y1 = s1[6], s1[7]
        dx1, dy1, cross1 = s1[8], s1[9], s1[10]

        j = i + 1
        while j < count:
            s2 = segments[j]
            if s2[4] > max_x1:
                break
            j += 1

            min_y2, max_y2 = s2[6], s2[7]
            if max_y1 < min_y2 or max_y2 < min_y1:
                continue

            x3, y3, x4, y4 = s2[0], s2[1], s2[2], s2[3]
            dx2, dy2, cross2 = s2[8], s2[9], s2[10]

            o1 = dx1 * (y3 - y1) - dy1 * (x3 - x1)
            if o1 == 0:
                continue
            o2 = dx1 * (y4 - y1) - dy1 * (x4 - x1)
            if o2 == 0 or (o1 > 0) == (o2 > 0):
                continue

            o3 = dx2 * (y1 - y3) - dy2 * (x1 - x3)
            if o3 == 0:
                continue
            o4 = dx2 * (y2 - y3) - dy2 * (x2 - x3)
            if o4 == 0 or (o3 > 0) == (o4 > 0):
                continue

            den = dx1 * dy2 - dy1 * dx2
            if den == 0:
                continue

            num_x = dx1 * cross2 - cross1 * dx2
            num_y = dy1 * cross2 - cross1 * dy2

            if den < 0:
                den = -den
                num_x = -num_x
                num_y = -num_y

            g1 = math.gcd(num_x, den)
            num_x //= g1
            den_x = den // g1

            g2 = math.gcd(num_y, den)
            num_y //= g2
            den_y = den // g2

            intersections.add((num_x, den_x, num_y, den_y))

    return len(intersections)


if __name__ == "__main__":
    print(main())
