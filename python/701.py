"""Project Euler Problem 701: Random connected area.

Find the expected maximum area of an orthogonally contiguous group of black
cells, if each cell in an N x N square is randomly colored white or black.

We build up board configurations, starting from the top left corner and going
in reading order, handling whether the current cell is white or black.

We memoize the configurations by only keeping track of the following state:
for each of the previous N cells, which are connected to which (the "profile",
which is 0 for each cell, and a distinct positive integer starting at 1 for
each set of black cells that are connected), the areas of the contiguous
regions containing those cells, and the total maximum area.

For handling the cell being white, we need only shift the profile and areas
left by 1. For handling the cell being black, we need to combine the groups
containing the cell above and to the left, if both are black, and sum their
areas if they are in different groups.

We store the number of configurations with each of the states, and at the end
multiply each maximum area by the corresponding probability (which is the
number of configurations for that state divided by 2^{N²}).
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Dict, Tuple


@dataclass(frozen=True)
class State:
    """State representing a board configuration profile."""

    profile: Tuple[int, ...]
    areas: Tuple[int, ...]
    max_area: int


def canonicalize(profile: list[int], n: int) -> tuple[int, ...]:
    """Canonicalize a profile by remapping group IDs."""
    mapping: list[int] = [0] * (n + 1)
    new_profile: list[int] = [0] * n
    index = 0
    for i in range(n):
        num = profile[i]
        if num > 0 and mapping[num] == 0:
            index += 1
            mapping[num] = index
        new_profile[i] = mapping[num]
    return tuple(new_profile)


def solve() -> float:
    """Solve Problem 701."""
    n = 7
    states: Dict[State, int] = {
        State(tuple([0] * n), tuple([0] * n), 0): 1
    }

    for row in range(n):
        for col in range(n):
            new_states: Dict[State, int] = {}
            for state, count in states.items():
                # Handle white cell
                new_profile = [state.profile[i] for i in range(1, n)]
                new_profile.append(0)
                new_areas = [state.areas[i] for i in range(1, n)]
                new_areas.append(0)
                canonical_profile = canonicalize(new_profile, n)
                new_state = State(canonical_profile, tuple(new_areas), state.max_area)
                new_states[new_state] = new_states.get(new_state, 0) + count

                # Handle black cell
                new_profile = [state.profile[i] for i in range(1, n)]
                new_profile.append(0)
                new_areas = [state.areas[i] for i in range(1, n)]
                new_areas.append(0)
                new_area = (
                    1
                    + state.areas[0]
                    + (
                        state.areas[n - 1]
                        if state.profile[0] != state.profile[n - 1] and col > 0
                        else 0
                    )
                )
                for i in range(n):
                    if (
                        new_profile[i] > 0
                        and (
                            new_profile[i] == state.profile[0]
                            or (new_profile[i] == state.profile[n - 1] and col > 0)
                        )
                    ) or i == n - 1:
                        new_profile[i] = n
                        new_areas[i] = new_area
                canonical_profile = canonicalize(new_profile, n)
                new_state = State(
                    canonical_profile,
                    tuple(new_areas),
                    max(state.max_area, new_area),
                )
                new_states[new_state] = new_states.get(new_state, 0) + count

            states = new_states

    ans = 0.0
    for state, count in states.items():
        ans += count * state.max_area
    ans /= 2 ** (n * n)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")
    return int(result * 1e8)  # Return as integer for comparison


if __name__ == "__main__":
    main()
