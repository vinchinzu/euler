"""Project Euler Problem 306: Paper-strip Game

Count winning positions n where 1 <= n <= 10^6 in a game where players
alternately mark two adjacent unmarked squares on a strip of n squares.
A player who cannot move loses.

This is an impartial combinatorial game. We compute Grundy numbers:
- G(n) = mex{G(a) XOR G(b)} for all valid moves
- When marking squares i and i+1, the strip splits into games of size i-1 and n-i-2
- A position is losing (P-position) iff G(n) = 0
- A position is winning (N-position) iff G(n) != 0

The Grundy sequence for this game eventually becomes periodic, which allows
efficient computation.

The problem asks: how many values n in [1, 10^6] allow the first player to force a win?
(i.e., count positions where G(n) != 0)
"""

from __future__ import annotations


def compute_grundy(limit: int) -> list[int]:
    """Compute Grundy numbers for strips of length 0 to limit.

    For a strip of length n:
    - We can mark positions (i, i+1) for i in range(n-1)
    - This creates two independent games of sizes i and n-i-2
    - G(n) = mex of all G(left) XOR G(right) values

    Base cases:
    - G(0) = 0 (no squares, no moves)
    - G(1) = 0 (one square, can't mark two adjacent)
    """
    G = [0] * (limit + 1)

    # G[0] = 0, G[1] = 0 (no valid moves possible)

    for n in range(2, limit + 1):
        # Collect all possible XOR values from moves
        seen = set()

        # Mark squares at positions i and i+1 (0-indexed)
        # Left segment has i squares (indices 0 to i-1)
        # Right segment has n - i - 2 squares (indices i+2 to n-1)
        for i in range(n - 1):
            left = i
            right = n - i - 2
            xor_val = G[left] ^ G[right]
            seen.add(xor_val)

        # G[n] = mex (minimum excludant) of the seen set
        mex = 0
        while mex in seen:
            mex += 1
        G[n] = mex

    return G


def find_period(G: list[int], min_check: int = 100) -> tuple[int, int] | None:
    """Find periodicity in Grundy sequence.

    Returns (start, period) if periodic pattern found, None otherwise.
    """
    n = len(G)

    # Try different period lengths
    for period in range(1, n // 3):
        # Check if sequence is periodic starting from some point
        for start in range(n // 3):
            # Verify periodicity for enough terms
            valid = True
            check_length = min(min_check, (n - start) // period - 1)
            if check_length < min_check:
                continue

            for j in range(check_length):
                if G[start + j] != G[start + period + j]:
                    valid = False
                    break

            if valid:
                return (start, period)

    return None


def solve() -> int:
    """Solve PE 306 for n up to 10^6.

    Count how many n from 1 to 10^6 have G(n) != 0 (winning positions).
    """
    LIMIT = 10**6

    # First compute enough Grundy numbers to find the period
    # The period for this game is known to be 34 starting around n=52
    # But we'll compute enough to verify
    initial_compute = 5000
    G = compute_grundy(initial_compute)

    # Find the period
    period_info = find_period(G, min_check=100)

    if period_info is None:
        # If no period found, compute everything directly
        G = compute_grundy(LIMIT)
        return sum(1 for n in range(1, LIMIT + 1) if G[n] != 0)

    start, period = period_info

    # Count winning positions (G[n] != 0)
    count = 0

    # Count positions before the periodic part
    for n in range(1, start):
        if G[n] != 0:
            count += 1

    # Count winning positions in one full period
    nonzeros_in_period = sum(1 for i in range(period) if G[start + i] != 0)

    # How many full periods fit in [start, LIMIT]?
    remaining = LIMIT - start + 1
    full_periods = remaining // period
    leftover = remaining % period

    count += full_periods * nonzeros_in_period

    # Count the leftover part
    for i in range(leftover):
        if G[start + i] != 0:
            count += 1

    return count


if __name__ == "__main__":
    print(solve())
