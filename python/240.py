"""Project Euler Problem 240: Top Dice.

Find the number of ways that N S-sided dice can be rolled so that the top K
values sum to T.
"""

from __future__ import annotations

from collections import OrderedDict
from typing import Dict, List


def n_cr(n: int, k: int) -> int:
    """Compute binomial coefficient C(n, k)."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    result = 1
    for i in range(min(k, n - k)):
        result = result * (n - i) // (i + 1)
    return result


def pow_int(base: int, exp: int) -> int:
    """Return base^exp."""
    return base**exp


def solve() -> int:
    """Solve Problem 240."""
    T = 70
    N = 20
    K = 10
    S = 12

    ans = [0]

    def helper(dice: Dict[int, int], num_dice: int, sum_val: int) -> None:
        """Recursive helper function."""
        if num_dice == K:
            if sum_val == T:
                lowest_die = min(dice.keys())
                for i in range(N - K + 1):
                    num_dice_remaining = N
                    num_ways = 1

                    for die in sorted(dice.keys()):
                        count = dice[die]
                        if die == lowest_die:
                            count += i
                        num_ways *= n_cr(num_dice_remaining, count)
                        num_dice_remaining -= count

                    ans[0] += num_ways * pow_int(lowest_die - 1, num_dice_remaining)
            return

        start_die = max(dice.keys()) + 1 if dice else 1
        for die in range(start_die, S + 1):
            for count in range(1, K - num_dice + 1):
                if sum_val + count * die > T:
                    break
                dice[die] = count
                helper(dice, num_dice + count, sum_val + count * die)
                del dice[die]

    helper(OrderedDict(), 0, 0)
    return ans[0]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
