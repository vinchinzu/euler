"""Project Euler Problem 181: Investigating in how many ways objects of two different colours can be grouped."""

from typing import List

BLACK = 60
WHITE = 40


def main() -> int:
    """Main function."""
    dp: List[List[int]] = [[0] * (WHITE + 1) for _ in range(BLACK + 1)]
    dp[0][0] = 1

    for b in range(BLACK + 1):
        for w in range(WHITE + 1):
            if b == 0 and w == 0:
                continue

            for i in range(b, BLACK + 1):
                for j in range(w, WHITE + 1):
                    dp[i][j] += dp[i - b][j - w]

    return dp[BLACK][WHITE]


if __name__ == "__main__":
    print(main())
