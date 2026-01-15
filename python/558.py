"""Project Euler Problem 558: Irrational base.

Let w(n) be the number of terms in the unique way to represent n as the sum of
integer powers of r, where r is the real root of x³=x²+1. Find Σ_{j=1}^N w(j²).

The powers of r are very close to integers, specifically the integers in the
recurrence relation with characteristic polynomial x³=x²+1, i.e. a_n = a_{n-1}
+ a_{n-3} with a_0 = 1, a_1 = 2, a_2 = 3. So if we multiply both sides of
j² = Σ_e r^e by a large power of r, then all terms are approximately equal to
integers. So we can start with the integer close to j²r^e, and greedily
subtract the largest valid integer in the sequence (which corresponds to some
power of r).
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 558."""
    N = 5000000
    L = 200

    # Generate recurrence sequence: a_n = a_{n-1} + a_{n-3}
    a = [0] * (2 * L)
    for i in range(2 * L):
        if i < 3:
            a[i] = i + 1
        else:
            a[i] = a[i - 1] + a[i - 3]

    ans = 0
    for j in range(1, N + 1):
        target = j * j * a[L]
        count = 0
        for i in range(2 * L - 1, -1, -1):
            if target >= a[i]:
                target -= a[i]
                count += 1
        ans += count

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
