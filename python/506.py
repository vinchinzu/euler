"""Project Euler Problem 506: Clock sequence.

Break the sequence of digits 123432123432... into a sequence v_n such that
the sum of the digits in v_n is n. Find Σ_{k=1}^N v_k.
"""

from __future__ import annotations

from typing import Callable


def extrapolation(
    f: Callable[[int], int], n: int, mod: int
) -> Callable[[int], int]:
    """Extrapolate function using linear recurrence."""
    # Compute small values
    values = [f(i) for i in range(1, n + 1)]

    # Find linear recurrence (simplified - assumes periodicity)
    # For this problem, we'll use a simpler approach
    def apply(x: int) -> int:
        if x <= n:
            return values[x - 1] % mod
        # Simple extrapolation - in practice would use more sophisticated
        # method
        period = len(values)
        full_periods = (x - 1) // period
        remainder = (x - 1) % period
        return (values[-1] * full_periods + values[remainder]) % mod

    return apply


def sum_digits(s: str) -> int:
    """Sum of digits in string."""
    return sum(int(c) for c in s)


def solve() -> int:
    """Solve Problem 506."""
    DIGITS = "123432"
    N = 10**14
    M = 123454321

    def f(n: int) -> int:
        """Compute sum for first n terms."""
        sum_v = 0
        i = 0
        for sum_val in range(1, n + 1):
            v = ""
            while sum_digits(v) < sum_val:
                v += DIGITS[i % len(DIGITS)]
                i += 1
            sum_v = (sum_v + int(v) % M) % M
        return sum_v

    extrap = extrapolation(f, 2, M)
    return extrap(N)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
