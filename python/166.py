"""Project Euler Problem 166: Criss Cross."""

from typing import Dict, List

MIN_DIGIT = 0
MAX_DIGIT = 9
GRID_SIZE = 4

ROW_CACHE: Dict[int, List[List[int]]] = {}


def rows_for_sum(s: int) -> List[List[int]]:
    """Generate all valid rows with sum s."""
    if s in ROW_CACHE:
        return ROW_CACHE[s]

    rows: List[List[int]] = []
    for a in range(MIN_DIGIT, MAX_DIGIT + 1):
        if a > s:
            break
        for b in range(MIN_DIGIT, MAX_DIGIT + 1):
            sum_ab = a + b
            if sum_ab > s:
                break
            for c in range(MIN_DIGIT, MAX_DIGIT + 1):
                sum_abc = sum_ab + c
                if sum_abc > s:
                    break
                d = s - sum_abc
                if d > MAX_DIGIT:
                    continue
                rows.append([a, b, c, d])

    ROW_CACHE[s] = rows
    return rows


def count_grids_for_sum(s: int) -> int:
    """Count valid 4x4 grids with all row/column/diagonal sums equal to s."""
    if s == 0:
        return 1

    rows = rows_for_sum(s)
    if not rows:
        return 0

    count = 0

    for row0 in rows:
        a, b, c, d = row0

        for row1 in rows:
            e, f, g, h = row1

            if a + e > s or b + f > s or c + g > s or d + h > s:
                continue

            col0_sum = a + e
            i_min = max(MIN_DIGIT, s - MAX_DIGIT - col0_sum)
            i_max = min(MAX_DIGIT, s - col0_sum)
            if i_min > i_max:
                continue

            for i in range(i_min, i_max + 1):
                j = a + e + i - d - g
                if j < MIN_DIGIT or j > MAX_DIGIT:
                    continue

                k = b + c + 2 * d - f - e - i
                if k < MIN_DIGIT or k > MAX_DIGIT:
                    continue

                l = s - i - j - k
                if l < MIN_DIGIT or l > MAX_DIGIT:
                    continue

                m = s - (a + e + i)
                if m < MIN_DIGIT or m > MAX_DIGIT:
                    continue

                n = s - (b + f + j)
                if n < MIN_DIGIT or n > MAX_DIGIT:
                    continue

                o = s - (c + g + k)
                if o < MIN_DIGIT or o > MAX_DIGIT:
                    continue

                p = s - (d + h + l)
                if p < MIN_DIGIT or p > MAX_DIGIT:
                    continue

                if a + f + k + p != s:
                    continue
                if d + g + j + m != s:
                    continue

                count += 1

    return count


def main() -> int:
    """Main function."""
    total = 0
    for s in range(37):
        total += count_grids_for_sum(s)
    return total


if __name__ == "__main__":
    print(main())
