"""Project Euler Problem 149 Solution.

Generate a 2000x2000 table using the specified Lagged Fibonacci Generator
and find the maximum sum of adjacent entries in any direction (horizontal,
vertical, diagonal, or anti-diagonal).
"""

from typing import List

# Constants for clarity and maintainability
MOD = 1_000_000
OFFSET = 500_000
N = 2000
TOTAL_CELLS = N * N


def kadane(arr: List[int]) -> int:
    """Kadane's algorithm for 1D maximum subarray sum.
    
    Handles empty arrays and all-negative cases correctly.
    """
    if not arr:
        return -float('inf')

    max_ending_here = max_so_far = arr[0]
    for i in range(1, len(arr)):
        max_ending_here = max(arr[i], max_ending_here + arr[i])
        max_so_far = max(max_so_far, max_ending_here)
    return max_so_far


def generate_sequence() -> List[int]:
    """Generate the Lagged Fibonacci sequence."""
    s: List[int] = [0] * TOTAL_CELLS

    # Generate first 55 values (1-based indices 1 to 55, 0-based indices 0 to 54)
    for i in range(55):
        k = i + 1
        temp = (100003 - 200003 * k + 300007 * k**3) % MOD
        s[i] = temp - OFFSET

    # Generate remaining values (1-based indices 56 to 4000000, 0-based 55 to 3999999)
    for i in range(55, TOTAL_CELLS):
        k1 = s[i - 24]
        k2 = s[i - 55]
        # Add MOD to ensure non-negative before modulo operation
        temp = (k1 + k2 + MOD) % MOD
        s[i] = temp - OFFSET

    return s


def main() -> int:
    """Main function."""
    s = generate_sequence()

    # Create the 2D table from the 1D sequence (row-major order)
    table: List[List[int]] = []
    for i in range(N):
        row = s[i * N:(i + 1) * N]
        table.append(row)

    max_sum = -float('inf')

    # Check horizontal lines
    for row in table:
        max_sum = max(max_sum, kadane(row))

    # Check vertical lines
    for col in range(N):
        column = [table[row][col] for row in range(N)]
        max_sum = max(max_sum, kadane(column))

    # Check main diagonals (top-left to bottom-right)
    for start_row in range(N):
        diag = []
        r, c = start_row, 0
        while r < N and c < N:
            diag.append(table[r][c])
            r += 1
            c += 1
        max_sum = max(max_sum, kadane(diag))

    for start_col in range(1, N):
        diag = []
        r, c = 0, start_col
        while r < N and c < N:
            diag.append(table[r][c])
            r += 1
            c += 1
        max_sum = max(max_sum, kadane(diag))

    # Check anti-diagonals (top-right to bottom-left)
    for start_col in range(N):
        diag = []
        r, c = 0, start_col
        while r < N and c >= 0:
            diag.append(table[r][c])
            r += 1
            c -= 1
        max_sum = max(max_sum, kadane(diag))

    for start_row in range(1, N):
        diag = []
        r, c = start_row, N - 1
        while r < N and c >= 0:
            diag.append(table[r][c])
            r += 1
            c -= 1
        max_sum = max(max_sum, kadane(diag))

    return max_sum


if __name__ == "__main__":
    print(main())
