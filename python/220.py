"""Project Euler Problem 220: Heighway Dragon.

Define D_0 = "Fa" and define D_n by using string-rewriting rules. Find the
position of the cursor after N steps when following the dragon curve.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Dict, Tuple


@dataclass
class Path:
    """Represents a path with displacement and direction."""

    dx: int = 0
    dy: int = 0
    ddir: int = 0
    num_steps: int = 0


def solve() -> Tuple[int, int]:
    """Solve Problem 220."""
    N = 10**12
    K = 50
    D0 = "Fa"
    MAPPINGS = {"a": "aRbFR", "b": "LFaLb"}
    cache: Dict[Tuple[str, int], Path] = {}

    def memoize(key: Tuple[str, int], func) -> Path:
        """Memoize function results."""
        if key not in cache:
            cache[key] = func(key)
        return cache[key]

    def helper(path_str: str, level: int, num_steps: int = None) -> Path:
        """Compute path for given string and level."""
        if num_steps is None:
            key = (path_str, level)
            return memoize(key, lambda k: helper(k[0], k[1], None))

        path = Path()
        dir_dx = [0, -1, 0, 1]
        dir_dy = [1, 0, -1, 0]
        rot_dx = [[1, 0, -1, 0], [0, -1, 0, 1]]
        rot_dy = [[0, 1, 0, -1], [1, 0, -1, 0]]

        for c in path_str:
            if path.num_steps == num_steps:
                break

            if c == "F":
                path.dx += dir_dx[path.ddir]
                path.dy += dir_dy[path.ddir]
                path.num_steps += 1
            elif c == "L":
                path.ddir = (path.ddir + 1) % 4
            elif c == "R":
                path.ddir = (path.ddir + 3) % 4
            elif level > 0:
                new_path = helper(MAPPINGS[c], level - 1)
                if path.num_steps + new_path.num_steps > num_steps:
                    new_path = helper(MAPPINGS[c], level - 1, num_steps - path.num_steps)

                # Rotate new_path displacement by current direction
                new_dx = (
                    rot_dx[0][path.ddir] * new_path.dx + rot_dx[1][path.ddir] * new_path.dy
                )
                new_dy = (
                    rot_dy[0][path.ddir] * new_path.dx + rot_dy[1][path.ddir] * new_path.dy
                )
                path.dx += new_dx
                path.dy += new_dy
                path.ddir = (path.ddir + new_path.ddir) % 4
                path.num_steps += new_path.num_steps

        return path

    path = helper(D0, K, N)
    return (path.dx, path.dy)


def main() -> Tuple[int, int]:
    """Main entry point."""
    result = solve()
    print(f"{result[0]},{result[1]}")
    return result


if __name__ == "__main__":
    main()
