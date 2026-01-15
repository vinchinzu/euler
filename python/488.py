"""Project Euler Problem 488: Unbalanced Nim.

Find Σ a+b+c for all losing positions (a,b,c) with 0 < a < b < c < N in a
variant of three-heap Nim where a player can never make two heaps the same
size.
"""

from __future__ import annotations

from typing import Dict, Tuple


def solve() -> int:
    """Solve Problem 488."""
    N = 10**18
    M = 10**9
    cache: Dict[Tuple[int, Tuple[bool, bool, bool]], Tuple[int, int]] = {}

    def f0(n: int, g: Tuple[bool, bool, bool]) -> Tuple[int, int]:
        """Count and sum of losing positions."""
        if n == 0:
            if any(g):
                return (0, 0)
            # Check if a^b^c = 0
            count = 0
            total = 0
            for a in range(2):
                for b in range(2):
                    for c in range(2):
                        if (a ^ b ^ c) == 0:
                            count += 1
                            total += a + b + c
            return (count, total)

        key = (n, g)
        if key in cache:
            return cache[key]

        count = 0
        total = 0
        for ba in range(2):
            for bb in range(2):
                for bc in range(2):
                    if (ba ^ bb ^ bc) != 0:
                        continue
                    new_g = (
                        (ba > (n % 2)) or (ba == (n % 2) and g[0]),
                        (bb > (n % 2)) or (bb == (n % 2) and g[1]),
                        (bc > (n % 2)) or (bc == (n % 2) and g[2]),
                    )
                    if not any(new_g):
                        sub_count, sub_total = f0(n // 2, new_g)
                        count += sub_count
                        total += 2 * sub_total + (ba + bb + bc) * sub_count

        cache[key] = (count, total)
        return (count, total)

    count, total = f0(N, (False, False, False))
    # Subtract positions with at least one zero
    count0, total0 = f0(N - 1, (False, False, False))
    ans = (total - total0) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
