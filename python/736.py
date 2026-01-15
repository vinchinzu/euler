"""Project Euler Problem 736: Paths to Equality.

Given a lattice point (x,y), define two operations r(x,y) = (x+1,2y) and
s(x,y) = (2x,y+1). Find the shortest sequence of operations resulting in a
lattice point where x=y.

We brute force with iterative deepening with one optimization: let t be the
number of remaining times r will be applied. Then we must have
x 2^t + depth-t ≤ (y+t) 2^{depth-t}, and similarly
y 2^{depth-t} + t ≤ (x+depth-t) 2^t. Otherwise, it is impossible for x=y
and we can exit early.
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 736."""
    a = 45
    b = 90
    ans = 0

    def search(x: int, y: int, depth: int) -> None:
        """Search for solution with iterative deepening."""
        nonlocal ans
        if depth == 0 and x == y:
            ans = x
            return

        for t in range(depth + 1):
            if (
                x * (2**t) + depth - t <= (y + t) * (2 ** (depth - t))
                and y * (2 ** (depth - t)) + t <= (x + depth - t) * (2**t)
            ):
                search(2 * x, y + 1, depth - 1)
                search(x + 1, 2 * y, depth - 1)
                if ans > 0:
                    return

    max_depth = 2
    while ans == 0:
        search(a, b, max_depth)
        max_depth += 2

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
