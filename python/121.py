"""Project Euler Problem 121: Disc game prize fund."""

from math import prod
from typing import List

TURNS = 15


def main() -> int:
    """Main function."""
    weights: List[int] = [0] * (TURNS + 1)
    weights[0] = 1

    for turn in range(1, TURNS + 1):
        next_weights: List[int] = [0] * (TURNS + 1)
        for blue in range(turn):
            weight = weights[blue]
            if weight == 0:
                continue
            next_weights[blue] += weight * turn     # draw red (turn red discs)
            next_weights[blue + 1] += weight        # draw blue (always 1 blue disc)
        weights = next_weights

    winning_weight = sum(weights[blue] for blue in range((TURNS // 2) + 1, TURNS + 1))
    denominator = prod(range(2, TURNS + 2))

    return denominator // winning_weight


if __name__ == "__main__":
    print(main())
