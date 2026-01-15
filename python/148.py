"""Project Euler Problem 148.

We can easily verify that none of the entries in the first seven rows of Pascal's triangle are divisible by 7.

However, if we check the first one hundred rows, we will find that only 2361 of the 5050 entries are not divisible by 7.
Find the number of entries which are not divisible by 7 in the first one billion (10^9) rows of Pascal's triangle.
"""

BASE = 7


def to_base7(num: int) -> list:
    """Convert a number to its base-7 digits (most significant digit first)."""
    if num == 0:
        return []
    digits = []
    temp = num
    while temp > 0:
        digits.append(temp % BASE)
        temp //= BASE
    digits.reverse()
    return digits


class DigitDP:
    """Digit DP to count the total number of non-divisible entries in rows 0 to m."""

    def __init__(self, digits: list):
        """Initialize."""
        self.digits = digits
        self.memo = {}

    def dp(self, pos: int, tight: bool) -> int:
        """dp(pos, tight) returns the sum of products for all valid suffixes starting at position pos."""
        if pos == len(self.digits):  # Base case: empty suffix, product = 1
            return 1

        key = (pos, 1 if tight else 0)
        if key in self.memo:
            return self.memo[key]

        result = 0
        max_digit = self.digits[pos] if tight else (BASE - 1)

        for d in range(max_digit + 1):
            new_tight = tight and (d == max_digit)
            result += (d + 1) * self.dp(pos + 1, new_tight)

        self.memo[key] = result
        return result


def count_non_divisible(m: int) -> int:
    """Count non-divisible entries in rows 0 to m."""
    digits = to_base7(m)
    dp = DigitDP(digits)
    return dp.dp(0, True)


def main() -> int:
    """Main function."""
    # Count entries in rows 0 to n-1 (n rows total)
    # For first 10^9 rows, we need rows 0 to 10^9 - 1
    return count_non_divisible(10**9 - 1)


if __name__ == "__main__":
    print(main())
