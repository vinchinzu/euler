"""Project Euler Problem 159: Digital root sums of factorisations."""

from typing import List
import math

MAX = 1_000_000


def main() -> int:
    """Main function."""
    # Precompute divisors
    divisors: List[List[int]] = [[] for _ in range(MAX)]
    for i in range(1, MAX):
        for j in range(i, MAX, i):
            divisors[j].append(i)

    # Precompute digital roots
    dr: List[int] = [0] * MAX
    for i in range(1, MAX):
        dr[i] = 9 if i % 9 == 0 else i % 9

    # Dynamic programming
    dp: List[int] = [0] * MAX
    for n in range(2, MAX):
        dp[n] = dr[n]
        sqrt_n = int(math.sqrt(n))
        for d in divisors[n]:
            if 2 <= d <= sqrt_n:
                m = n // d
                dp[n] = max(dp[n], dr[d] + dp[m], dr[m] + dp[d])

    sum_val = 0
    for n in range(2, MAX):
        sum_val += dp[n]
    return sum_val


if __name__ == "__main__":
    print(main())
