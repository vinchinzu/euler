"""Project Euler Problem 809: Rational Recurrence Relation.

Define f(x) to be x if x is integral, f(1/(1-x)) if x<1, and
f(1/(⌈x⌉-x)-1+f(x-1)) otherwise. Find f(A+1/B) mod M.

We see that f(A+1/B) = A(B-1,A) where A is the Ackermann function. This
is just 2^2^2^... - 3 (for a very large tower of 2s), which quickly
reaches a constant value mod M.
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 809."""
    A = 3
    B = 7
    M = 10**15

    # Find fixed point of 2^x mod M
    b = 0
    while True:
        next_b = pow(2, b, M)
        if next_b == b:
            break
        b = next_b

    return b - 3


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
