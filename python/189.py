"""Project Euler Problem 189: Tri-colouring a triangular grid."""

from typing import Dict

N = 8


def count_colorings(n: int) -> int:
    """Count valid colorings using recursive DP."""
    memo: Dict[tuple[int, int], int] = {}

    def dp(row: int, prev_mask: int) -> int:
        """DP function."""
        if row > n:
            return 1
        key = (row, prev_mask)
        if key in memo:
            return memo[key]

        size = row
        count = 0

        # Generate valid masks for current row: no adjacent same color
        valid_masks: list[int] = []
        for mask in range(3 ** size):
            colors: list[int] = []
            temp = mask
            valid = True
            for p in range(size):
                col = temp % 3
                colors.append(col)
                if p > 0 and col == colors[p - 1]:
                    valid = False
                    break
                temp //= 3
            if valid:
                # Check compatibility with previous row
                compatible = True
                if row > 1:
                    prev_colors: list[int] = []
                    temp_prev = prev_mask
                    prev_size = row - 1
                    for p in range(prev_size):
                        col = temp_prev % 3
                        prev_colors.append(col)
                        temp_prev //= 3
                    for p in range(size):
                        # Triangle at position p in current row is adjacent to:
                        # - Triangle at position p in previous row (if p < prev_size)
                        # - Triangle at position p-1 in previous row (if p > 0)
                        if p < prev_size and colors[p] == prev_colors[p]:
                            compatible = False
                            break
                        if p > 0 and colors[p] == prev_colors[p - 1]:
                            compatible = False
                            break
                if compatible:
                    valid_masks.append(mask)

        for mask in valid_masks:
            count += dp(row + 1, mask)

        memo[key] = count
        return count

    return dp(1, 0)


def main() -> int:
    """Main function."""
    return count_colorings(N)


if __name__ == "__main__":
    print(main())
