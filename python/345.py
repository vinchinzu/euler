"""Project Euler Problem 345 - Matrix Sum (assignment problem via DP with bitmasks).

This module computes the maximum possible sum of matrix elements such that no two
chosen elements share the same row or column. It provides:

- validate_matrix: validate that a matrix is a non-empty square numeric matrix
- matrix_sum: compute the matrix sum using dynamic programming with bitmasks
- A small CLI / script entrypoint when run as __main__
- Lightweight unit-style tests runnable without external dependencies

The implementation is self-contained and uses only the Python standard library.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import inf
from time import perf_counter
from typing import Iterable, List, Sequence

Matrix = List[List[float]]


def validate_matrix(matrix: object) -> bool:
    """Return True if ``matrix`` is a non-empty square matrix of numeric values.

    This function is intentionally permissive on numeric types, accepting any
    values that ``float()`` can handle.
    """

    if not isinstance(matrix, Sequence):  # type: ignore[unreachable]
        return False

    if not matrix:
        return False

    # Ensure matrix is a proper 2D structure
    if not all(isinstance(row, Sequence) for row in matrix):
        return False

    rows = len(matrix)
    cols = len(matrix[0])

    # Must be non-empty and square
    if cols == 0 or rows != cols:
        return False

    try:
        for row in matrix:
            if len(row) != cols:
                return False
            for val in row:
                float(val)
    except (TypeError, ValueError):
        return False

    return True


def matrix_sum(matrix: Matrix) -> float:
    """Compute the maximum matrix sum with distinct rows and columns.

    The input must be a non-empty, square matrix of numeric values.

    The algorithm uses dynamic programming with bitmasks and runs in
    O(n^2 * 2^n) time and O(n * 2^n) space, which is suitable for n up to 15-16.
    """

    if not validate_matrix(matrix):
        raise ValueError("Invalid matrix provided")

    n = len(matrix)
    total_states = 1 << n

    # Use -inf as the sentinel for unreachable states
    neg_inf = -inf
    dp: List[List[float]] = [[neg_inf] * total_states for _ in range(n + 1)]
    dp[0][0] = 0.0

    for row in range(n):
        row_vals = matrix[row]
        for mask in range(total_states):
            current = dp[row][mask]
            if current == neg_inf:
                continue

            free = (~mask) & (total_states - 1)
            while free:
                # Extract lowest set bit as candidate column
                bit = free & -free
                free ^= bit

                col = bit.bit_length() - 1
                new_mask = mask | bit
                new_sum = current + float(row_vals[col])

                if new_sum > dp[row + 1][new_mask]:
                    dp[row + 1][new_mask] = new_sum

    return max(dp[n])


# Embedded problem matrix (same as in the Ruby source)
MATRIX: Matrix = [
    [7, 53, 183, 439, 863, 497, 383, 563, 79, 973, 287, 63, 343, 169, 583],
    [
        627,
        343,
        773,
        959,
        943,
        767,
        473,
        103,
        699,
        303,
        957,
        703,
        583,
        639,
        913,
    ],
    [447, 283, 463, 29, 23, 487, 463, 993, 119, 883, 327, 493, 423, 159, 743],
    [217, 623, 3, 399, 853, 407, 103, 983, 89, 463, 290, 516, 212, 462, 350],
    [
        960,
        376,
        682,
        962,
        300,
        780,
        486,
        502,
        912,
        800,
        250,
        346,
        172,
        812,
        350,
    ],
    [
        870,
        456,
        192,
        162,
        593,
        473,
        915,
        45,
        989,
        873,
        823,
        965,
        425,
        329,
        803,
    ],
    [
        973,
        965,
        905,
        919,
        133,
        673,
        665,
        235,
        509,
        613,
        673,
        815,
        165,
        992,
        326,
    ],
    [
        322,
        148,
        972,
        962,
        286,
        255,
        941,
        541,
        265,
        323,
        925,
        281,
        601,
        95,
        973,
    ],
    [
        445,
        721,
        11,
        525,
        473,
        65,
        511,
        164,
        138,
        672,
        18,
        428,
        154,
        448,
        848,
    ],
    [
        414,
        456,
        310,
        312,
        798,
        104,
        566,
        520,
        302,
        248,
        694,
        976,
        430,
        392,
        198,
    ],
    [
        184,
        829,
        373,
        181,
        631,
        101,
        969,
        613,
        840,
        740,
        778,
        458,
        284,
        760,
        390,
    ],
    [
        821,
        461,
        843,
        513,
        17,
        901,
        711,
        993,
        293,
        157,
        274,
        94,
        192,
        156,
        574,
    ],
    [
        34,
        124,
        4,
        878,
        450,
        476,
        712,
        914,
        838,
        669,
        875,
        299,
        823,
        329,
        699,
    ],
    [
        815,
        559,
        813,
        459,
        522,
        788,
        168,
        586,
        966,
        232,
        308,
        833,
        251,
        631,
        107,
    ],
    [
        813,
        883,
        451,
        509,
        615,
        77,
        281,
        613,
        459,
        205,
        380,
        274,
        302,
        35,
        805,
    ],
]


# ------------- Lightweight tests (no external framework required) -------------

@dataclass
class _TestCase:
    name: str
    func: callable


def _assert_equal(expected: float, actual: float, message: str = "") -> None:
    if expected != actual:
        raise AssertionError(
            f"{message} (expected {expected}, got {actual})".strip()
        )


def _assert_raises(exc: type[BaseException], func: callable, *args, **kwargs) -> None:
    try:
        func(*args, **kwargs)
    except exc:  # type: ignore[misc]
        return
    except BaseException as caught:  # pragma: no cover - defensive
        raise AssertionError(
            f"Expected {exc.__name__}, got {type(caught).__name__}"
        ) from caught
    raise AssertionError(f"Expected {exc.__name__} to be raised")


def _tests() -> list[_TestCase]:
    """Return built-in test cases mirroring the Ruby tests."""

    tests: list[_TestCase] = []

    def test_single_element() -> None:
        _assert_equal(5.0, matrix_sum([[5]]))

    tests.append(_TestCase("single_element", test_single_element))

    def test_two_by_two() -> None:
        mat = [[1, 2], [3, 4]]
        _assert_equal(5.0, matrix_sum(mat))

    tests.append(_TestCase("two_by_two", test_two_by_two))

    def test_three_by_three() -> None:
        mat = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        _assert_equal(15.0, matrix_sum(mat))

    tests.append(_TestCase("three_by_three", test_three_by_three))

    def test_sample_from_problem() -> None:
        sample = [[863, 383], [343, 959]]
        _assert_equal(1822.0, matrix_sum(sample))

    tests.append(_TestCase("sample_from_problem", test_sample_from_problem))

    def test_all_zero_matrix() -> None:
        mat = [[0, 0], [0, 0]]
        _assert_equal(0.0, matrix_sum(mat))

    tests.append(_TestCase("all_zero_matrix", test_all_zero_matrix))

    def test_negative_numbers() -> None:
        mat = [[-1, -2], [-3, -4]]
        _assert_equal(-5.0, matrix_sum(mat))

    tests.append(_TestCase("negative_numbers", test_negative_numbers))

    def test_empty_matrix() -> None:
        _assert_raises(ValueError, matrix_sum, [])

    tests.append(_TestCase("empty_matrix", test_empty_matrix))

    def test_non_square_matrix() -> None:
        mat = [[1, 2, 3], [4, 5]]
        _assert_raises(ValueError, matrix_sum, mat)

    tests.append(_TestCase("non_square_matrix", test_non_square_matrix))

    def test_invalid_matrix() -> None:
        _assert_raises(ValueError, matrix_sum, [[1, "a"], [3, 4]])
        _assert_raises(ValueError, matrix_sum, None)

    tests.append(_TestCase("invalid_matrix", test_invalid_matrix))

    return tests


def run_tests() -> None:
    """Run built-in tests and print a concise summary.

    This is a minimal replacement for the Ruby Minitest usage in the original
    code, designed to keep the module self-contained.
    """

    tests = _tests()
    passed = 0
    for case in tests:
        case.func()
        passed += 1
    print(f"Ran {passed} tests: all passed")


def _compute_default_matrix_sum() -> None:
    """Compute and print the matrix sum for the embedded MATRIX constant."""

    if not validate_matrix(MATRIX):
        raise SystemExit(
            "Problem matrix is invalid (not square or contains non-numeric values)"
        )

    n = len(MATRIX)
    total_ops_approx = n * (1 << n) * n
    print(f"Computing Matrix Sum for {n}x{n} matrix...")
    print(
        "Using DP with bitmasks: "
        f"{n} * 2^{n} * {n}  approx {total_ops_approx} operations"
    )

    start = perf_counter()
    result = matrix_sum(MATRIX)
    duration = perf_counter() - start

    print(f"Calculation completed in {duration:.3f} seconds")
    print(f"Maximum Matrix Sum: {int(result)}")


if __name__ == "__main__":  # pragma: no cover - CLI behavior
    if not validate_matrix(MATRIX):
        raise SystemExit(
            "Problem matrix is invalid (not square or contains non-numeric values)"
        )
    result = matrix_sum(MATRIX)
    print(int(result))
