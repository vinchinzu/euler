"""Project Euler Problem 109: Darts Checkout Combinations.

Count distinct checkouts for scores below 100 where the final dart is a double.
"""

from dataclasses import dataclass
from typing import List


@dataclass
class Shot:
    """Represents a dart shot."""

    score: int
    is_double: bool
    label: str


ZERO_SHOT = Shot(0, False, '0')
SINGLES = [Shot(n, False, f"S{n}") for n in range(1, 21)]
DOUBLES = [Shot(2 * n, True, f"D{n}") for n in range(1, 21)]
TREBLES = [Shot(3 * n, False, f"T{n}") for n in range(1, 21)]
OUTER_BULL = Shot(25, False, 'S25')
INNER_BULL = Shot(50, True, 'D25')

PRE_SHOTS = [ZERO_SHOT] + SINGLES + DOUBLES + TREBLES + [OUTER_BULL, INNER_BULL]
FINISHING_SHOTS = DOUBLES + [INNER_BULL]

LIMIT = 99


def checkout_total(limit: int = LIMIT) -> int:
    """Evaluate all unordered pairs of opening darts and append finishing double."""
    total = 0

    for finish in FINISHING_SHOTS:
        for i, first in enumerate(PRE_SHOTS):
            for j in range(i, len(PRE_SHOTS)):
                second = PRE_SHOTS[j]
                score = first.score + second.score + finish.score
                if 1 <= score <= limit:
                    total += 1

    return total


def main() -> int:
    """Main function."""
    return checkout_total()


if __name__ == "__main__":
    print(main())
