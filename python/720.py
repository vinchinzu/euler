"""Project Euler Problem 720: Unpredictable Permutations.

Find the index of the first permutation of 2^N elements such that no
subsequence of three elements forms an arithmetic sequence.

For a permutation of 2^N elements, the odd numbers must be in the same order
as the first valid permutation of 2^{N-1} elements, and similarly for the
even numbers. It turns out that the odd numbers must all be before the even
numbers, except the middle 2 and 2^N-1 can be swapped.

Starting with the first valid permutation of 4 elements, we can interactively
build up the first valid permutation of 2^N elements. We maintain both the
elements and their ranks. Then at the end we can use these to compute the
index of the permutation.
"""

from __future__ import annotations


def factorial(n: int, mod: int) -> list[int]:
    """Precompute factorials modulo mod."""
    result = [1] * (n + 1)
    for i in range(1, n + 1):
        result[i] = (result[i - 1] * i) % mod
    return result


def solve() -> int:
    """Solve Problem 720."""
    n = 25
    m = 10**9 + 7
    l = 1 << n

    elements = [1, 3, 2, 4] + [0] * (l - 4)
    ranks = [1, 2, 2, 4] + [0] * (l - 4)

    for i in range(4, l, 2):
        for j in range(i):
            ranks[i + j] = ranks[j] + elements[j]
            elements[i + j] = 2 * elements[j]
            elements[j] = 2 * elements[j] - 1
        elements[i - 1] = 2
        elements[i] = 2 * i - 1
        ranks[i - 1] = 2
        ranks[i] = i + 1

    factorials = factorial(l, m)
    ans = 1
    for i in range(l):
        ans = (ans + factorials[l - 1 - i] * (elements[i] - ranks[i])) % m

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
