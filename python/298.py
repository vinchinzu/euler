"""Project Euler Problem 298: Selective Amnesia.

In each round of a game, a random number from 1 to K is called out. Larry gets
a point if the number is in his memory, and otherwise, if his memory contains
L numbers, removes the number in his memory that was called out the longest
time ago. Similarly, Robin gets a point if the number is in his memory, but
otherwise removes the oldest number added to his memory. Find the expected
absolute difference between Larry and Robin's points after N turns.

We maintain after each turn a map of all possible sets of numbers that Larry
and Robin can have in their memories. For each set of numbers, we store the
probabilities for each possible difference of Larry and Robin's point values.
For each new called out number from 1 to K, we can simulate removing the
oldest number from Larry and Robin's memories, if applicable, to yield a new
set of memorized numbers. We can then compute the new probabilities for all
possible differences of point values. The final answer after 50 turns is the
expected absolute difference across all point value differences, across all
possible memorized sets.

Initially, we fill Larry and Robin's memories with 0. To reduce the number
of distinct memorized sets, we "normalize" a memorized set by ensuring that
when reading Larry and then Robin's memories in order, each new nonzero
number is in increasing order.
"""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from typing import Dict, List, Tuple


@dataclass(frozen=True)
class State:
    """State representing Larry and Robin's memories."""

    larry: Tuple[int, ...]
    robin: Tuple[int, ...]


def solve() -> float:
    """Solve Problem 298."""
    N = 50
    K = 10
    L = 5

    start = tuple([0] * L)
    states: Dict[State, Dict[int, float]] = {
        State(start, start): {0: 1.0}
    }

    for turn in range(N):
        new_states: Dict[State, Dict[int, float]] = defaultdict(
            lambda: defaultdict(float)
        )

        for state, probs in states.items():
            for called in range(1, K + 1):
                diff_change = 0

                # Update Larry's memory
                new_larry = list(state.larry)
                if called in new_larry:
                    new_larry.remove(called)
                    diff_change += 1
                else:
                    new_larry.pop(0)
                new_larry.append(called)

                # Update Robin's memory
                new_robin = list(state.robin)
                if called in new_robin:
                    diff_change -= 1
                else:
                    new_robin.pop(0)
                    new_robin.append(called)

                # Normalize
                new_mapping: Dict[int, int] = {0: 0}
                next_index = 1
                normalized_larry = []
                normalized_robin = []

                for val in new_larry:
                    if val not in new_mapping:
                        new_mapping[val] = next_index
                        next_index += 1
                    normalized_larry.append(new_mapping[val])

                for val in new_robin:
                    if val not in new_mapping:
                        new_mapping[val] = next_index
                        next_index += 1
                    normalized_robin.append(new_mapping[val])

                new_state = State(tuple(normalized_larry), tuple(normalized_robin))

                for diff, prob in probs.items():
                    new_states[new_state][diff + diff_change] += prob / K

        states = {k: dict(v) for k, v in new_states.items()}

    ans = 0.0
    for probs in states.values():
        for diff, prob in probs.items():
            ans += abs(diff) * prob

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")


if __name__ == "__main__":
    main()
