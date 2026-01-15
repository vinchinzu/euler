"""Project Euler Problem 685: Inverse Digit Sum II.

Let f(n,m) be the mth largest number with digit sum n. Find Σ_{n=1}^N f(n³,n⁴).

For small n and m, we can generate the numbers directly. Just iteratively increase
the total number of digits, and for each digit count, generate all numbers with
the desired sum by starting with lowest numbers first.

For higher n and m, note that the smallest number is [n%9]99...99 (with ⌊n/9⌋ 9s).
Then for n>9, there are ⌊n/9⌋ numbers of the form [n%9+1]89...99, where one of
the 9s is an 8, followed by roughly nCr(⌊n/9⌋,2) numbers of the form [n%9+2]79...99,
where either one of the 9s is a 7 or two 9s are 8s. (In the special case where
n%9=8, the numbers are of the form 179...99, but we can compute it similarly,
just noting that there are ⌊n/9⌋+1 places to decrement the 2 digits instead.)
For high enough n and m, we have ⌊n/9⌋ < m < nCr(⌊n/9⌋,2), so the desired number
is one of these with two digits decremented. We iteratively try the place value
to change the first 9 to an 8 (chunk1), and either go to the next batch or find
where the second digit is decremented (chunk2).
"""

from __future__ import annotations


def cb(n: int) -> int:
    """Cube of n."""
    return n * n * n


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def nCr(n: int, r: int) -> int:
    """Binomial coefficient."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    result = 1
    for i in range(min(r, n - r)):
        result = result * (n - i) // (i + 1)
    return result


def solve() -> int:
    """Solve Problem 685."""
    N = 10000
    M = 10**9 + 7
    B = 10

    def f(n: int, m: int) -> int:
        """Find mth largest number with digit sum n."""
        if m > n // (B - 1) and m < nCr(n // (B - 1), 2):
            m -= n // (B - 1) + 1
            length = (n + 1) // (B - 1)
            chunk1 = length - 1
            while m > chunk1 + 1:
                m -= chunk1 + 1
                chunk1 -= 1
            chunk2 = chunk1 - m + 1
            begin = (n + 1) % (B - 1) + 2
            return (
                begin * pow_mod(B, length, M)
                - 1
                - pow_mod(B, chunk1, M)
                - pow_mod(B, chunk2, M)
            ) % M
        else:
            nums: list[int] = []
            num_digits = 1
            while len(nums) < m:
                helper(num_digits, n, 0, m, nums, B, M)
                num_digits += 1
            return nums[m - 1] % M

    ans = 0
    for n in range(1, N + 1):
        ans = (ans + f(cb(n), pow(n, 4))) % M

    return ans


def helper(
    num_digits: int,
    sum_digits: int,
    n: int,
    limit: int,
    nums: list[int],
    B: int,
    M: int,
) -> None:
    """Helper function to generate numbers."""
    if len(nums) > limit or (B - 1) * num_digits < sum_digits:
        return
    if num_digits == 0:
        if sum_digits == 0:
            nums.append(n)
        return
    start_digit = 1 if n == 0 else 0
    for d in range(start_digit, B):
        helper(
            num_digits - 1,
            sum_digits - d,
            (n * B + d) % M,
            limit,
            nums,
            B,
            M,
        )


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
