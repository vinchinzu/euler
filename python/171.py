"""Project Euler Problem 171: Finding numbers for which the sum of the squares of the digits is a square."""

from typing import List
import math

MOD = 1_000_000_000
DIGITS = 20
MAX_SUM = DIGITS * 81


def main() -> int:
    """Main function."""
    pow10: List[int] = [1] * (DIGITS + 1)
    for i in range(1, DIGITS + 1):
        pow10[i] = (pow10[i - 1] * 10) % MOD

    is_square: List[bool] = [False] * (MAX_SUM + 1)
    for s in range(MAX_SUM + 1):
        root = int(math.sqrt(s))
        is_square[s] = (root * root == s)

    count: List[List[int]] = [[0] * (MAX_SUM + 1) for _ in range(DIGITS + 1)]
    sum_dp: List[List[int]] = [[0] * (MAX_SUM + 1) for _ in range(DIGITS + 1)]

    count[0][0] = 1

    for length in range(DIGITS):
        for s in range(MAX_SUM + 1):
            cnt = count[length][s]
            if cnt == 0:
                continue

            current_sum = sum_dp[length][s]
            digit_contribution_factor = pow10[length]

            for d in range(10):
                new_sum = s + d * d
                if new_sum > MAX_SUM:
                    continue

                count[length + 1][new_sum] = (count[length + 1][new_sum] + cnt) % MOD

                added = (cnt * d * digit_contribution_factor) % MOD
                total = (current_sum + added) % MOD
                sum_dp[length + 1][new_sum] = (
                    sum_dp[length + 1][new_sum] + total
                ) % MOD

    result = 0
    for s in range(MAX_SUM + 1):
        if not is_square[s]:
            continue
        result = (result + sum_dp[DIGITS][s]) % MOD

    return result


if __name__ == "__main__":
    print(main())
