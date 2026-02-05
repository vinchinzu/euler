"""Project Euler Problem 237: Tours on a 4 x n playing board.

Find T(10^12) mod 10^8 where T(n) is the number of tours on a 4xn board that:
- Start in top left corner
- Move up/down/left/right one square
- Visit each square exactly once
- End in bottom left corner

Solution uses the transfer matrix method with matrix exponentiation.

For a 4-row board, we can derive a linear recurrence for T(n) by computing
small values and finding the characteristic polynomial.
"""

from __future__ import annotations


def matrix_mult(A: list[list[int]], B: list[list[int]], mod: int) -> list[list[int]]:
    """Multiply two matrices modulo mod."""
    n = len(A)
    m = len(B[0])
    k = len(B)
    C = [[0] * m for _ in range(n)]
    for i in range(n):
        for j in range(m):
            for p in range(k):
                C[i][j] = (C[i][j] + A[i][p] * B[p][j]) % mod
    return C


def matrix_pow(M: list[list[int]], exp: int, mod: int) -> list[list[int]]:
    """Compute M^exp mod mod using binary exponentiation."""
    n = len(M)
    # Start with identity matrix
    result = [[1 if i == j else 0 for j in range(n)] for i in range(n)]
    base = [row[:] for row in M]  # Copy

    while exp > 0:
        if exp & 1:
            result = matrix_mult(result, base, mod)
        base = matrix_mult(base, base, mod)
        exp >>= 1

    return result


def count_paths_brute(n: int) -> int:
    """Count Hamiltonian paths on 4xn grid from (0,0) to (3,0)."""
    if n <= 0:
        return 0

    rows, cols = 4, n
    total_cells = rows * cols
    start = (0, 0)  # Top-left
    end = (3, 0)    # Bottom-left

    # DFS to count all Hamiltonian paths
    count = [0]

    def dfs(pos: tuple[int, int], visited: set) -> None:
        if len(visited) == total_cells:
            if pos == end:
                count[0] += 1
            return

        r, c = pos
        for dr, dc in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            nr, nc = r + dr, c + dc
            if 0 <= nr < rows and 0 <= nc < cols and (nr, nc) not in visited:
                if (nr, nc) == end and len(visited) < total_cells - 1:
                    continue  # Don't visit end early
                visited.add((nr, nc))
                dfs((nr, nc), visited)
                visited.remove((nr, nc))

    visited = {start}
    dfs(start, visited)
    return count[0]


def solve() -> int:
    """Solve Problem 237 using recurrence and matrix exponentiation.

    We compute T(n) for small n to find the linear recurrence,
    then use matrix exponentiation to compute T(10^12) mod 10^8.
    """
    N = 10**12
    MOD = 10**8

    # Compute T(1) through T(10) by brute force to find and verify the recurrence
    # T(n) counts Hamiltonian paths on 4xn grid from top-left to bottom-left
    #
    # For efficiency, we compute just enough values.
    # Known: T(10) = 2329

    # Computing small values by brute force
    T_values = [0]  # T[0] is unused
    for i in range(1, 11):
        T_values.append(count_paths_brute(i))

    # Verify T(10) = 2329
    assert T_values[10] == 2329, f"T(10) = {T_values[10]}, expected 2329"

    # Now find the recurrence. For 4xn Hamiltonian paths, the recurrence
    # typically has order around 6 (related to the number of valid states
    # in the transfer matrix, which is small for 4 rows).

    # Try to find a recurrence of order k such that:
    # T(n) = c1*T(n-1) + c2*T(n-2) + ... + ck*T(n-k)

    def find_recurrence(values: list[int], max_order: int = 10) -> list[int] | None:
        """Find the linear recurrence for the sequence."""
        n = len(values) - 1
        for order in range(1, min(max_order + 1, n // 2)):
            # Try to find coefficients using Gaussian elimination
            # We need 'order' equations and 'order' unknowns

            # Set up augmented matrix for the system
            # values[order+1] = c1*values[order] + ... + ck*values[1]
            # values[order+2] = c1*values[order+1] + ... + ck*values[2]
            # etc.

            # Use Fraction for exact arithmetic
            from fractions import Fraction
            matrix = []
            for i in range(order + 1, 2 * order + 1):
                row = [Fraction(values[i - j]) for j in range(1, order + 1)]
                row.append(Fraction(values[i]))  # RHS
                matrix.append(row)

            # Gaussian elimination
            for col in range(order):
                # Find pivot
                pivot_row = None
                for row in range(col, order):
                    if matrix[row][col] != 0:
                        pivot_row = row
                        break

                if pivot_row is None:
                    break  # No solution with this order

                # Swap rows
                matrix[col], matrix[pivot_row] = matrix[pivot_row], matrix[col]

                # Eliminate
                for row in range(order):
                    if row != col and matrix[row][col] != 0:
                        factor = matrix[row][col] / matrix[col][col]
                        for j in range(order + 1):
                            matrix[row][j] -= factor * matrix[col][j]

            # Extract solution
            try:
                coeffs = []
                for i in range(order):
                    coeffs.append(matrix[i][order] / matrix[i][i])

                # Check if coefficients are integers
                int_coeffs = []
                for c in coeffs:
                    if c.denominator != 1:
                        raise ValueError("Non-integer coefficient")
                    int_coeffs.append(int(c))

                # Verify with remaining values
                valid = True
                for i in range(order + 1, n + 1):
                    predicted = sum(int_coeffs[j] * values[i - j - 1] for j in range(order))
                    if predicted != values[i]:
                        valid = False
                        break

                if valid:
                    return int_coeffs
            except Exception:
                pass

        return None

    recurrence = find_recurrence(T_values)

    if recurrence is None:
        raise ValueError("Could not find recurrence")

    # Use matrix exponentiation to compute T(N)
    # T(n) = c1*T(n-1) + c2*T(n-2) + ... + ck*T(n-k)
    #
    # Matrix form:
    # [T(n)    ]   [c1 c2 ... ck-1 ck] [T(n-1)  ]
    # [T(n-1)  ] = [1  0  ... 0    0 ] [T(n-2)  ]
    # [T(n-2)  ]   [0  1  ... 0    0 ] [T(n-3)  ]
    # [...]       [...    ...     ...]  [...]
    # [T(n-k+1)]   [0  0  ... 1    0 ] [T(n-k)  ]

    k = len(recurrence)

    # Build transition matrix
    M = [[0] * k for _ in range(k)]
    for j in range(k):
        M[0][j] = recurrence[j]
    for i in range(1, k):
        M[i][i - 1] = 1

    # Initial state vector: [T(k), T(k-1), ..., T(1)]
    initial = [T_values[k - i] for i in range(k)]

    # We want T(N). If N <= k, return directly.
    if N <= k:
        return T_values[N] % MOD

    # Compute M^(N-k) * initial
    M_power = matrix_pow(M, N - k, MOD)

    # Result is the first element of M_power * initial
    result = 0
    for j in range(k):
        result = (result + M_power[0][j] * initial[j]) % MOD

    return result


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
