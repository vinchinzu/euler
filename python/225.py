"""Project Euler Problem 225: Tribonacci non-divisors.

Find the Nth odd number that does not divide any term in the Tribonacci
sequence.
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 225."""
    N = 124

    d = 1
    count = 0

    while count < N:
        d += 2
        a, b, c = 1, 1, 1

        while True:
            next_val = (a + b + c) % d
            if next_val == 0:
                break
            a, b, c = b, c, next_val
            if a == 1 and b == 1 and c == 1:
                count += 1
                break

    return d


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
