"""Project Euler Problem 150."""

from typing import List

MOD = 1 << 20
MASK = MOD - 1
OFFSET = 1 << 19
N = 1_000
TOTAL = N * (N + 1) // 2


def main() -> int:
    """Main function."""
    sequence: List[int] = [0] * TOTAL
    t = 0
    index = 0
    while index < TOTAL:
        t = (615_949 * t + 797_807) & MASK
        sequence[index] = t - OFFSET
        index += 1

    prefix_rows: List[List[int]] = []
    row = 0
    seq_index = 0
    while row < N:
        length = row + 1
        prefix = [0] * (length + 1)
        c = 0
        while c < length:
            val = sequence[seq_index]
            seq_index += 1
            prefix[c + 1] = prefix[c] + val
            c += 1
        prefix_rows.append(prefix)
        row += 1

    sequence = None

    def range_min(prefix_rows: List[List[int]], start_row: int, end_row: int) -> int:
        """Find minimum sum in range."""
        rows = prefix_rows
        n = len(rows)
        min_sum = 1 << 62
        row = start_row
        while row < end_row:
            start_col = 0
            while start_col <= row:
                current_sum = 0
                curr_row = row
                right = start_col + 1
                while curr_row < n:
                    curr_prefix = rows[curr_row]
                    left_val = curr_prefix[start_col]
                    right_val = curr_prefix[right] if right < len(curr_prefix) else curr_prefix[-1]
                    current_sum += right_val - left_val
                    if current_sum < min_sum:
                        min_sum = current_sum
                    curr_row += 1
                    right += 1
                start_col += 1
            row += 1
        return min_sum

    return range_min(prefix_rows, 0, N)


if __name__ == "__main__":
    print(main())
