"""Project Euler Problem 614: Special partitions II.

Find Σ_{i=1}^N P(i), where P(n) is the number of partitions of n such that
all terms are distinct and all even terms are divisible by 4.

We can find that P(i) = a_i + P(i-1) + P(i-3) - P(i-6) - P(i-10) + P(i-15)
+ ..., where a_i = ±1 if i is twice a triangular number, with +1 if i/2 is
odd and -1 otherwise, and the differences k in P(i-k) are triangular numbers.
"""

from __future__ import annotations

from math import isqrt


def is_square(n: int) -> bool:
    """Check if n is a perfect square."""
    root = isqrt(n)
    return root * root == n


def triangular(n: int) -> int:
    """Triangular number n(n+1)/2."""
    return n * (n + 1) // 2


def parity(n: int) -> int:
    """Return 1 if n is even, -1 if odd."""
    return 1 if n % 2 == 0 else -1


def solve() -> int:
    """Solve Problem 614."""
    N = 10**7
    M = 10**9 + 7
    L = 1 << 14

    P = [0] * (N + L)

    for page in range((N + L - 1) // L):
        # Process previous pages
        for prev_page in range(page):
            min_t = int(
                (isqrt(8 * (page - (prev_page + 1)) * L + 1) + 1) / 2
            )
            for t in range(
                min_t, (page + 1 - prev_page) * L + 1
            ):
                tr_val = triangular(t)
                if tr_val >= (page + 1 - prev_page) * L:
                    break
                for i in range(
                    max(page * L, prev_page * L + tr_val),
                    min((page + 1) * L, N + 1),
                ):
                    if i - tr_val >= prev_page * L and i - tr_val < (
                        prev_page + 1
                    ) * L:
                        P[i] = (
                            P[i] + parity((t - 1) // 2) * P[i - tr_val]
                        ) % M

        # Process current page
        for i in range(page * L, min((page + 1) * L, N + 1)):
            res = 4 * i + 1
            if is_square(res):
                root = isqrt(res)
                P[i] = (
                    P[i] + parity((root // 2 + 1) // 2)
                ) % M

            for t in range(1, i - page * L + 1):
                tr_val = triangular(t)
                if tr_val > i - page * L:
                    break
                P[i] = (P[i] + parity((t - 1) // 2) * P[i - tr_val]) % M

    ans = sum(P[1 : N + 1]) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
