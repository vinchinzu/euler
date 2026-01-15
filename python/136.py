"""Project Euler Problem 136: Singleton difference.

How many values of n less than fifty million have exactly one solution?
"""

from typing import List

LIMIT_N = 50_000_000


def main() -> int:
    """Main function."""
    solution_counts: List[int] = [0] * LIMIT_N

    # Calculate d_max: we need the smallest possible n for a given d to be < limit_n
    # The smallest n occurs when a = d+1 or a = 4*d-1
    # For a = d+1: n = (d+1)(4d - (d+1)) = (d+1)(3d-1)
    # We need (d+1)(3d-1) < limit_n
    # 3d^2 + 2d - 1 < 50,000,000
    # Using quadratic formula: d < (-2 + sqrt(4 + 600,000,012)) / 6 ≈ 4082
    d_max = 4082

    for d in range(1, d_max + 1):
        for a in range(d + 1, 4 * d):
            n = a * (4 * d - a)
            if n < LIMIT_N:
                solution_counts[n] += 1

    # For larger d values, use the optimization from Problem 135
    for d in range(d_max + 1, LIMIT_N // 4 + 1):
        k = 1
        while True:
            n = 4 * d * k - k * k
            if n >= LIMIT_N:
                break
            solution_counts[n] += 1
            k += 1

    return sum(1 for count in solution_counts if count == 1)


if __name__ == "__main__":
    print(main())
