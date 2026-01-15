"""Project Euler Problem 509: Divisor Nim.

Find the number of winning positions in a variant of Nim where given a
pile of size n, only a proper divisor of n stones can be removed from
the pile.
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 509."""
    N = 123456787654321
    M = 1234567890

    counts = []
    k = 0
    while True:
        count = ((N >> k) - (N >> (k + 1))) % M
        counts.append(count)
        if count == 0:
            break
        k += 1

    ans = 0
    for k1 in range(len(counts)):
        for k2 in range(len(counts)):
            for k3 in range(len(counts)):
                if (k1 ^ k2 ^ k3) != 0:
                    ans = (ans + counts[k1] * counts[k2] % M * counts[k3]) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
