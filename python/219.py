"""Project Euler Problem 219: Skew-cost Coding.

Find the minimum cost of transmitting one of each character in an N-character
prefix-free code if the cost of transmitting a zero is C0 and the cost of
transmitting a one is C1.
"""

from __future__ import annotations

from typing import List


def solve() -> int:
    """Solve Problem 219."""
    N = 10**9
    C0 = 1
    C1 = 4

    dp: List[int] = [1]
    n = 1
    ans = 0

    while n < N:
        num_codes = min(dp[-1], N - n)
        n += num_codes
        ans += num_codes * (len(dp) - 1 + C0 + C1)

        next_val = 0
        if len(dp) >= C0:
            next_val += dp[-C0]
        if len(dp) >= C1:
            next_val += dp[-C1]
        dp.append(next_val)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
