"""Project Euler Problem 476: Circle packing in triangles.

Let R(a, b, c) be the maximum area of three non-overlapping circles inside a
triangle with side lengths a, b, c. Find the average of R(a, b, c) for all
1 ≤ a ≤ b ≤ c < a + b ≤ N.
"""

from __future__ import annotations

import math


def solve() -> float:
    """Solve Problem 476."""
    N = 1803
    sqrt_cache = [math.sqrt(i) for i in range(2 * N)]

    total_area = 0.0
    count = 0

    for a in range(1, N + 1):
        for b in range(a, N + 1):
            for c in range(b, min(a + b, N + 1)):
                if a + b <= c:
                    continue

                s = (a + b + c) / 2.0
                area_triangle = math.sqrt(s * (s - a) * (s - b) * (s - c))
                r = area_triangle / s  # In-radius

                # Second circle
                H = math.sqrt(r * r + (s - a) * (s - a))
                r_a = r - 2 * r / (H / r + 1)

                # Third circle - two candidates
                H2 = math.sqrt(r_a * r_a + (s - a) * (s - a))
                r_a2 = r_a - 2 * r_a / (H2 / r_a + 1)

                H3 = math.sqrt(r * r + (s - b) * (s - b))
                r_b = r - 2 * r / (H3 / r + 1)

                r3 = max(r_a2, r_b)
                total_area += math.pi * (r * r + r_a * r_a + r3 * r3)
                count += 1

    return total_area / count if count > 0 else 0.0


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return result


if __name__ == "__main__":
    main()
