"""Project Euler Problem 277: A Modified Collatz sequence.

Find the smallest integer greater than N whose modified Collatz sequence
is the specified string.
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 277."""
    S = "UDDDUdddDDUDDddDdDddDDUDDdUUDd"
    N = 10**15

    mod = 1
    ans = 0

    # Process string in reverse
    for c in reversed(S):
        mod *= 3
        if c == "D":
            ans *= 3
        elif c == "U":
            ans = 3 * ans - 2
            while ans % 4 != 0:
                ans += mod
            ans //= 4
        elif c == "d":
            ans = 3 * ans + 1
            while ans % 2 != 0:
                ans += mod
            ans //= 2

    while ans <= N:
        ans += mod

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
