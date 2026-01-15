"""Project Euler Problem 637: Flexible Digit Sum.

Let f(n, B) be the smallest number of steps to turn n into a single-digit
number in base B, using operations of inserting addition signs between some
of the digits of n repeatedly, e.g. 123 -> 1 + 2 + 3 = 6. Find the sum of all
n that satisfies f(n, B1) = f(n, B2).

For N = 10^7, the digit sum of n can never be larger than 63 in base 10 or
larger than 29 in base 3. We can verify that all these values can be turned
into a single-digit number in at most 2 steps, so f(n, B) ≤ 3 for all n.
"""

from __future__ import annotations


def sum_digits(n: int, base: int) -> int:
    """Sum of digits in given base."""
    result = 0
    while n > 0:
        result += n % base
        n //= base
    return result


def good(sum_val: int, remaining: int, base: int, sum_digits_arr: list[int]) -> bool:
    """Check if we can reach single digit."""
    if remaining == 0:
        return sum_digits_arr[sum_val] < base

    pow_base = base
    while pow_base <= base * remaining:
        if good(
            sum_val + remaining % pow_base,
            remaining // pow_base,
            base,
            sum_digits_arr,
        ):
            return True
        pow_base *= base
    return False


def f(n: int, b: int) -> list[int]:
    """Compute f values for all numbers up to n."""
    sum_digits_arr = [0] * (n + 1)
    f_arr = [0] * (n + 1)

    for i in range(n + 1):
        sum_digits_arr[i] = sum_digits(i, b)
        if i < b:
            f_arr[i] = 0
        elif sum_digits_arr[i] < b:
            f_arr[i] = 1
        elif good(0, i, b, sum_digits_arr):
            f_arr[i] = 2
        else:
            f_arr[i] = 3

    return f_arr


def solve() -> int:
    """Solve Problem 637."""
    N = 10**7
    B1 = 10
    B2 = 3

    f1 = f(N, B1)
    f2 = f(N, B2)

    ans = 0
    for i in range(1, N + 1):
        if f1[i] == f2[i]:
            ans += i
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
