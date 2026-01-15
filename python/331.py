"""Project Euler Problem 331 - Python implementation.

This module provides a direct and idiomatic Python 3.12 translation of the
Ruby script for exploring configuration-based toggling on an N x N board.

Key points:
- Uses binary (mod 2) linear algebra over GF(2) implemented with plain lists.
- Avoids external dependencies; relies only on Python's standard library.
- Public API:
  - generate_configuration(n): build the initial board configuration.
  - solve_system(n): compute T(n) exactly (or 0 if unsolvable) via Gaussian
    elimination over GF(2).
  - main(): run a small driver that mirrors the Ruby script behavior, including
    approximations for large n.

The Gaussian elimination is O(n^6) in the board dimension n (since the matrix
size is (n^2) x (n^2)), so it is only feasible for small n.
"""

from __future__ import annotations

from math import hypot
from typing import List


def generate_configuration(n: int) -> List[List[int]]:
    """Generate the initial N x N configuration matrix.

    A cell at (x, y) is black (1) if its distance from the origin satisfies
    ``n - 1 <= sqrt(x^2 + y^2) < n``. Otherwise it is white (0).

    Coordinates follow the original convention with (0, 0) at bottom-left,
    but the condition is radially symmetric, so we implement it directly.
    """

    if n < 0:
        msg = "Board dimension must be non-negative."
        raise ValueError(msg)

    board: List[List[int]] = [[0] * n for _ in range(n)]

    for x in range(n):
        for y in range(n):
            dist = hypot(x, y)
            board[x][y] = int(n - 1 <= dist < n)

    return board


def _build_system_matrix(n: int) -> list[list[int]]:
    """Build the (n^2 x n^2) matrix A over GF(2) for the flip operations.

    Column k encodes choosing disk (i, j) with k = i * n + j; this flips all
    disks in row i and column j. The resulting linear system A x = b is solved
    over GF(2).
    """

    size = n * n
    a: list[list[int]] = [[0] * size for _ in range(size)]

    for i in range(n):
        for j in range(n):
            k = i * n + j

            # Flip all in row i.
            for col in range(n):
                target_idx = i * n + col
                a[target_idx][k] = 1

            # Flip all in column j.
            for row in range(n):
                target_idx = row * n + j
                a[target_idx][k] = 1

            # Note: Cell (i,j) is set to 1 by both row and column operations,
            # so it gets flipped when choosing position (i,j)

    return a


def solve_system(n: int) -> int:
    """Solve the configuration system for board size ``n`` over GF(2).

    Returns the minimal number of moves T(n) if the system is solvable,
    otherwise 0.

    This is an exact translation of the Ruby Gaussian elimination approach.
    It operates on a (n^2 x n^2) matrix and thus is only practical for small
    ``n``. For larger ``n``, consider using more efficient linear algebra or
    problem-specific mathematics instead.
    """

    if n < 0:
        msg = "Board dimension must be non-negative."
        raise ValueError(msg)

    size = n * n
    if size == 0:
        return 0

    a = _build_system_matrix(n)
    b_flat: list[int] = [0] * size

    config = generate_configuration(n)
    for i in range(n):
        for j in range(n):
            k = i * n + j
            b_flat[k] = config[i][j]

    pivot_positions: list[int] = [-1] * size

    # Gaussian elimination over GF(2).
    for p in range(size):
        pivot_row = None
        for i in range(p, size):
            if a[i][p] == 1:
                pivot_row = i
                break

        if pivot_row is None:
            # No pivot in this column; free variable.
            continue

        if pivot_row != p:
            a[p], a[pivot_row] = a[pivot_row], a[p]
            b_flat[p], b_flat[pivot_row] = b_flat[pivot_row], b_flat[p]

        pivot_positions[p] = p

        # Eliminate this column from all other rows.
        for i in range(size):
            if i != p and a[i][p] == 1:
                row_i = a[i]
                row_p = a[p]
                for j in range(p, size):
                    row_i[j] = (row_i[j] + row_p[j]) % 2
                b_flat[i] = (b_flat[i] + b_flat[p]) % 2

    # Check for inconsistency: 0 = 1 rows.
    for i in range(size):
        if all(val == 0 for val in a[i]) and b_flat[i] == 1:
            return 0

    # Back substitution for one solution (free vars assumed 0).
    x: list[int] = [0] * size

    for p in range(size - 1, -1, -1):
        if pivot_positions[p] == -1:
            continue
        total = b_flat[p]
        row_p = a[p]
        for j in range(p + 1, size):
            total = (total + row_p[j] * x[j]) % 2
        x[p] = total

    # Minimal number of moves equals Hamming weight of chosen flips.
    return int(sum(x))


def _approximate_T(n: int) -> int:
    """Approximate T(n) for large n.

    This mirrors the heuristic from the original Ruby script and is not
    mathematically justified. It is retained here solely for behavioral
    parity with the source script.
    """

    if n == 5:
        return 3
    if n == 10:
        return 29
    if n == 1000:
        return 395_253

    if n <= 1000:
        ratio = (n - 10) / (1000 - 10)
        return round(29 + ratio * (395_253 - 29))

    growth_rate = 395_253 / 1000
    return round(growth_rate * n)


def main() -> None:
    """Run a small demonstration mirroring the Ruby script's behavior.

    For small boards where exact computation is feasible, uses ``solve_system``.
    For larger boards, falls back to a heuristic approximation to avoid the
    prohibitive O(n^6) cost.
    """

    print("Computing T(2^i - i) for i from 3 to 31...")
    print(
        "Note: For large n, exact computation is infeasible with this simple "
        "O(n^6) solver"
    )

    total_sum = 0
    max_computable_n = 12

    for i in range(3, 32):
        n = (1 << i) - i
        print(f"Computing T({n}) for i={i} (n={n})...")

        if n <= max_computable_n:
            t_n = solve_system(n)
        else:
            t_n = _approximate_T(n)
            print(f"  Using approximation for large n: T({n}) ≈ {t_n}")

        total_sum += t_n
        print(f"  T({n}) = {t_n}")
        print(f"  Running sum: {total_sum}")

    print("\nFinal result:", total_sum)

    print("\nVerification with known values:")
    print(f"T(5) = {solve_system(5)} (expected: 3)")
    print(f"T(10) = {solve_system(10)} (expected: 29)")

    # Print only final answer for test harness
    print()
    print(total_sum)


if __name__ == "__main__":  # pragma: no cover - manual execution entrypoint
    main()
