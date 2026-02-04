"""Project Euler Problem 538: Maximum Quadrilateral.

Consider the numbers u_n = 2^B(3n) + 3^B(2n) + B(n+1), where B(n) is the
number of ones in the binary representation of n. Let f(U_n) be the
perimeter of the quadrilateral with side lengths equal to distinct values
of u_i for 1 <= i <= n and with the largest area. Find sum_{n=4}^N f(U_n).

We maintain the set of numbers U_n and iteratively add the next term u_n,
updating the currently best quadrilateral. The maximum area is achieved when
the quadrilateral is cyclic, and the area can be computed using
Brahmagupta's formula.
"""

from __future__ import annotations

from sortedcontainers import SortedList


def bit_count(n: int) -> int:
    """Count number of 1s in binary representation."""
    return bin(n).count("1")


def solve() -> int:
    """Solve Problem 538."""
    N = 3 * 10**6
    K = 4

    U = SortedList()
    max_area2 = 0.0
    best_perim = 0
    best_min_side = 0
    ans = 0

    for n in range(1, N + 1):
        u = (
            pow(2, bit_count(3 * n))
            + pow(3, bit_count(2 * n))
            + bit_count(n + 1)
        )
        U.add(u)

        if u >= best_min_side:
            # Get K values <= u and K-1 values > u
            idx = U.bisect_right(u)
            start = max(0, idx - K)
            end = min(len(U), idx + K - 1)
            candidates = list(U[start:end])

            for i in range(len(candidates) - K + 1):
                quad_sides = candidates[i : i + K]
                perim = sum(quad_sides)
                # Brahmagupta's formula: 16*area^2 = prod(perim - 2*side)
                area2 = 1.0
                for side in quad_sides:
                    area2 *= perim - 2 * side
                if area2 >= max_area2:
                    max_area2 = area2
                    best_perim = perim
                    best_min_side = quad_sides[0]

        ans += best_perim

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
