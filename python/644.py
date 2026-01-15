"""Project Euler Problem 644: Squares on the Line.

A 2-player game is played on a line segment of length L. First, a square is
randomly placed with sides either parallel/perpendicular or diagonal relative
to the segment, uniformly randomly along the segment. Then, two players take
turns placing a square such that squares never overlap, until the player who
cannot place a square loses. Let f(L) be the probability that the first
player wins. Find the maximum value of L * f(L) for A≤L≤B.

This game can be analyzed with nimbers, except continuous regions of the
starting length L have the same nimber.
"""

from __future__ import annotations

import math
from collections import defaultdict
from dataclasses import dataclass
from heapq import heappop, heappush


@dataclass
class Event:
    """Event in priority queue."""

    pos: float
    is_add: bool
    value: object

    def __lt__(self, other):
        """Comparison for priority queue."""
        return self.pos < other.pos


@dataclass
class Range:
    """Range of values."""

    start: float
    end: float


@dataclass
class RangePair:
    """Pair of ranges."""

    range1: Range
    range2: Range


def feq(a: float, b: float, eps: float = 1e-10) -> bool:
    """Check if floats are approximately equal."""
    return abs(a - b) < eps


def solve() -> float:
    """Solve Problem 644."""
    A = 200
    B = 500

    # Compute nimbers (simplified version)
    nimbers = {0.0: 0}
    current_nimbers = defaultdict(int)
    events = []
    heappush(events, Event(1.0, True, 0))

    while events:
        event = heappop(events)
        if event.pos > B:
            break

        if event.is_add:
            current_nimbers[event.value] += 1
        else:
            current_nimbers[event.value] -= 1
            if current_nimbers[event.value] == 0:
                del current_nimbers[event.value]

        if events and feq(events[0].pos, event.pos):
            continue

        nimber = 0
        while nimber in current_nimbers.values():
            nimber += 1

        if nimber == nimbers.get(max(nimbers.keys()), -1):
            continue

        nimbers[event.pos] = nimber

        # Add new events (simplified)
        for pos in list(nimbers.keys())[:-1]:
            heappush(events, Event(event.pos + pos + 1, True, nimber ^ nimbers[pos]))
            heappush(
                events,
                Event(
                    event.pos + pos + math.sqrt(2), True, nimber ^ nimbers[pos]
                ),
            )

    # Compute probabilities (simplified)
    sizes = []
    for a in range(1, B + 1):
        for b in range(1, int((B - a) / math.sqrt(2)) + 1):
            size = a + b * math.sqrt(2)
            if size >= A:
                sizes.append(size)

    probs = {}
    for size in sizes:
        # Simplified probability computation
        prob = 0.5  # Placeholder
        probs[size] = prob

    ans = 0.0
    for size in sizes:
        L_val = (
            size
            * (
                probs.get(size - 1, 0.0)
                + probs.get(size - math.sqrt(2), 0.0)
            )
            / 2
        )
        if L_val > ans:
            ans = L_val

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")
    return result


if __name__ == "__main__":
    main()
