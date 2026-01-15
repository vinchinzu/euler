"""Project Euler Problem 255: Rounded Square Roots.

An iterative method to compute √n is to start with x_0 and repeatedly
compute x_{k+1} = ⌊(x_k + ⌈n/x_k⌉) / 2⌋, until x_{k+1} = x_k. Find the
average number of iterations needed to compute √n for L ≤ n < H.
"""

from __future__ import annotations

from math import ceil


def solve() -> float:
    """Solve Problem 255."""
    L = 10**13
    H = 10**14
    X0 = 7 * 10**6

    total_iterations = sum_iterations(L, H, X0, 1)
    return total_iterations / (H - L)


def sum_iterations(l: int, h: int, x: int, k: int) -> int:
    """Sum iterations for interval [l, h) with current x and iteration k."""
    x2l = bound(x * (x - 1) + 1, l, h)
    x2h = bound(x * (x + 1) + 1, l, h)
    return (
        sum_remaining_iterations(l, x2l, x, k)
        + (x2h - x2l) * k
        + sum_remaining_iterations(x2h, h, x, k)
    )


def sum_remaining_iterations(l: int, h: int, x: int, k: int) -> int:
    """Sum remaining iterations."""
    total = 0
    while l < h:
        next_x = (x + ceil_div(l, x)) // 2
        next_l = min((next_x * 2 + 1 - x) * x + 1, h)
        total += sum_iterations(l, next_l, next_x, k + 1)
        l = next_l
    return total


def ceil_div(a: int, b: int) -> int:
    """Ceiling division."""
    return (a + b - 1) // b


def bound(n: int, low: int, high: int) -> int:
    """Bound n between low and high."""
    if n < low:
        return low
    if n > high:
        return high
    return n


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return result


if __name__ == "__main__":
    main()
