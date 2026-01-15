"""Project Euler Problem 260: Stone Game.

In a variant of Nim, on each turn a player can remove k stones from at
least one of 3 piles that start with N stones each. Find sum(x+y+z) for
all losing configurations x≤y≤z.
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 260."""
    N = 1000
    lines = [[False] * (N + 1) for _ in range(N + 1)]
    diags = [[False] * (N + 1) for _ in range(N + 1)]
    space = [[False] * (N + 1) for _ in range(N + 1)]

    ans = 0

    for x in range(N + 1):
        for y in range(x, N + 1):
            for z in range(y, N + 1):
                if lines[x][y] or lines[x][z] or lines[y][z]:
                    continue
                if diags[x][z - y] or diags[y][z - x] or diags[z][y - x]:
                    continue
                if space[y - x][z - y]:
                    continue

                # Mark as losing configuration
                lines[x][y] = True
                lines[x][z] = True
                lines[y][z] = True
                diags[x][z - y] = True
                diags[y][z - x] = True
                diags[z][y - x] = True
                space[y - x][z - y] = True

                ans += x + y + z

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
