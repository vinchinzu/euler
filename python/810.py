"""Project Euler Problem 810: XOR-Primes.

Let the XOR product x ⊗ y be the bitwise XOR of x and y, where the columns
in long multiplication are XORed instead of added with carry. Find the Nth
XOR-prime, a number not the XOR product of two integers greater than 1.

This can be computed with a sieve in the same as way as normal primes.
"""

from __future__ import annotations

from typing import List


def xor_product(x: int, y: int) -> int:
    """Compute XOR product x ⊗ y."""
    result = 0
    while y > 0:
        if y & 1:
            result ^= x
        x <<= 1
        y >>= 1
    return result


def solve() -> int:
    """Solve Problem 810."""
    N = 5 * 10**6
    L = 1 << 27

    sieve = [True] * L
    sieve[0] = sieve[1] = False

    for i in range(2, L):
        if sieve[i]:
            j = i
            while j < L:
                m = xor_product(i, j)
                if m >= L:
                    break
                if m > 1:
                    sieve[m] = False
                j += 1

    count = 0
    ans = 0
    for i in range(2, L):
        if sieve[i]:
            count += 1
            if count == N:
                ans = i
                break

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
