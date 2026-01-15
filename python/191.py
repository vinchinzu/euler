"""Project Euler Problem 191: Prize Strings."""

from typing import List

CONSEC_A_MAX = 2
L_COUNT_MAX = 1


def prize_strings(n: int) -> int:
    """Count prize strings of length n."""
    prev: List[List[int]] = [[0] * (L_COUNT_MAX + 1) for _ in range(CONSEC_A_MAX + 1)]
    prev[0][0] = 1

    for _ in range(n):
        curr: List[List[int]] = [
            [0] * (L_COUNT_MAX + 1) for _ in range(CONSEC_A_MAX + 1)
        ]

        for consec_a in range(CONSEC_A_MAX + 1):
            for l_count in range(L_COUNT_MAX + 1):
                ways = prev[consec_a][l_count]
                if ways == 0:
                    continue

                curr[0][l_count] += ways
                if consec_a < CONSEC_A_MAX:
                    curr[consec_a + 1][l_count] += ways
                if l_count == 0:
                    curr[0][1] += ways

        prev = curr

    return sum(sum(row) for row in prev)


def main() -> int:
    """Main function."""
    return prize_strings(30)


if __name__ == "__main__":
    print(main())
