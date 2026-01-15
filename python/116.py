"""Project Euler Problem 116: Single-colour oblong tiles.

Count the number of ways to replace grey tiles in a row of length 50
using red (length 2), green (length 3), or blue (length 4) tiles when
only one colour is used at a time and at least one coloured tile appears.
"""

ROW_LENGTH = 50
COLOUR_TILE_LENGTHS = [2, 3, 4]


def ways_for_colour(row_length: int, tile_length: int) -> int:
    """Dynamic programming for a fixed colour.
    
    dp[i] counts arrangements for length i allowing the all-grey case, so we
    subtract 1 at the end to enforce the "at least one coloured tile" rule.
    """
    dp = [0] * (row_length + 1)
    dp[0] = 1

    for length in range(1, row_length + 1):
        dp[length] = dp[length - 1]
        if length >= tile_length:
            dp[length] += dp[length - tile_length]

    return dp[row_length] - 1


def total_replacements(row_length: int) -> int:
    """Calculate total replacements for all colours."""
    return sum(ways_for_colour(row_length, tile_length) for tile_length in COLOUR_TILE_LENGTHS)


def main() -> int:
    """Main function."""
    return total_replacements(ROW_LENGTH)


if __name__ == "__main__":
    print(main())
