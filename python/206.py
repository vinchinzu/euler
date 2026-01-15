"""Project Euler Problem 206: Concealed Square.

Find the unique positive integer whose square is of the form
1_2_3_4_5_6_7_8_9_0.
"""

from __future__ import annotations


def sq(n: int) -> int:
    """Return n squared."""
    return n * n


def pow_base(base: int, exp: int) -> int:
    """Return base raised to exp."""
    return base**exp


def solve() -> int:
    """Solve Problem 206."""
    S = "1_2_3_4_5_6_7_8_9_0"
    B = 10
    ans = [0]

    def match(num: int, num_end_digits: int) -> bool:
        """Check if last num_end_digits of num match pattern S."""
        temp = num
        for i in range(1, num_end_digits + 1):
            c = S[len(S) - i]
            if c != "_" and temp % B != int(c):
                return False
            temp //= B
        return True

    def helper(num_end_digits: int, num: int) -> None:
        """Recursively build number digit by digit."""
        if match(sq(num), len(S)):
            ans[0] = num
            return
        for candidate in range(num, pow_base(B, num_end_digits), pow_base(B, num_end_digits - 1)):
            if match(sq(candidate), num_end_digits):
                helper(num_end_digits + 1, candidate)

    helper(1, 0)
    return ans[0]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
