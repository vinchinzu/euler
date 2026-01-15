"""Project Euler Problem 686: Powers of Two.

Find the Nth positive integer j such that 2^j starts with the digits of K.
"""

from __future__ import annotations


def iceil_pow(n: int, base: int) -> int:
    """Smallest power of base >= n."""
    result = 1
    while result < n:
        result *= base
    return result


def solve() -> int:
    """Solve Problem 686."""
    N = 678910
    K = 123
    B = 10
    L = iceil_pow(K, B)

    pow2 = 1.0
    ans = 0
    n = 0
    while n < N:
        ans += 1
        pow2 *= 2
        if pow2 > L:
            pow2 /= B
        if int(pow2) == K:
            n += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
