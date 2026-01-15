"""Project Euler Problem 655: Divisible Palindromes.

Find the number of palindromes with up to N digits and divisible by K.

We can find the number of palindromes with exactly n digits via dynamic
programming: let dp[r] be the number of palindromes with remainder r when
divided by K. We can iteratively update dp[r] by adding the base palindrome
100...001 up to 9 times, for all symmetric values 100...001.

If we count palindromes starting with 0, then we implicitly also count all
palindromes with n-2k digits, because e.g. 545 will be counted as 05450.
Subtracting one for the zero palindrome, the answer is dp[0] - 1.
"""

from __future__ import annotations


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


def imod(n: int, mod: int) -> int:
    """Integer modulo (non-negative result)."""
    return ((n % mod) + mod) % mod


def num_palindromes(num_digits: int, K: int, B: int) -> int:
    """Count palindromes with exactly num_digits digits divisible by K."""
    dp = [0] * K
    dp[0] = 1

    for i in range((num_digits + 1) // 2):
        if 2 * i + 1 == num_digits:
            mult = pow_mod(B, i, K)
        else:
            mult = (pow_mod(B, i, K) + pow_mod(B, num_digits - 1 - i, K)) % K

        new_dp = [0] * K
        for d in range(B):
            for j in range(K):
                new_j = imod(mult * d + j, K)
                new_dp[new_j] += dp[j]
        dp = new_dp

    return dp[0] - 1


def solve() -> int:
    """Solve Problem 655."""
    N = 32
    K = 10**7 + 19
    B = 10

    ans = 0
    for num_digits in [N - 1, N]:
        ans += num_palindromes(num_digits, K, B)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
