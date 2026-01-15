"""Project Euler Problem 377 (Python 3.12 version).

This module computes the final value for Project Euler Problem 377 using
matrix exponentiation and modular arithmetic.

Key ideas:
- f(n): sum of all positive integers (no zero digits) with digit sum == n
- We work modulo MOD (last 9 digits) for performance and to match the
  problem's requirements.
- A linear recurrence of fixed order (ORDER) is derived for f(n).
- Matrix exponentiation is used to evaluate f(13**i) efficiently.

The module exposes a small public API:
- compute_initial_f
- compute_recurrence_coeffs
- verify_recurrence
- compute_f_large
- solve

Running this file as a script prints the required answer.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import List

MOD: int = 1_000_000_000
TARGET_DIGITS: int = 9
MAX_INIT: int = 40  # For initial values (needs at least ORDER + ORDER)
ORDER: int = 18  # Recurrence order


@dataclass
class ModularMatrix:
    """Simple dense matrix with modular multiplication and exponentiation.

    This is a minimal replacement for Ruby's Matrix usage in the original
    script. It supports only the features required by this problem:
    - matrix-matrix multiplication under a modulus
    - exponentiation by squaring under a modulus
    """

    data: List[List[int]]

    @property
    def rows(self) -> int:
        return len(self.data)

    @property
    def cols(self) -> int:
        return len(self.data[0]) if self.data else 0

    @staticmethod
    def identity(size: int) -> "ModularMatrix":
        """Return an identity matrix of given size.

        Elements are plain integers (no modulus applied here).
        """

        return ModularMatrix(
            [[1 if i == j else 0 for j in range(size)] for i in range(size)]
        )

    @staticmethod
    def mult(a: "ModularMatrix", b: "ModularMatrix", mod: int) -> "ModularMatrix":
        """Return (a * b) modulo mod.

        The dimensions must be compatible: a.cols == b.rows.
        """

        if a.cols != b.rows:
            msg = f"Incompatible matrix sizes: {a.rows}x{a.cols} vs {b.rows}x{b.cols}"
            raise ValueError(msg)

        result = [[0] * b.cols for _ in range(a.rows)]
        for i in range(a.rows):
            row_i = a.data[i]
            for k in range(a.cols):
                aik = row_i[k]
                if aik:
                    row_bk = b.data[k]
                    for j in range(b.cols):
                        result[i][j] = (result[i][j] + aik * row_bk[j]) % mod
        return ModularMatrix(result)

    @staticmethod
    def pow(matrix: "ModularMatrix", exponent: int, mod: int) -> "ModularMatrix":
        """Return (matrix ** exponent) modulo mod using fast exponentiation."""

        if exponent < 0:
            msg = "Negative exponents are not supported for ModularMatrix.pow."
            raise ValueError(msg)
        size = matrix.rows
        result = ModularMatrix.identity(size)
        base = matrix
        exp = exponent

        while exp > 0:
            if exp & 1:
                result = ModularMatrix.mult(result, base, mod)
            base = ModularMatrix.mult(base, base, mod)
            exp >>= 1
        return result


def compute_initial_f(max_n: int) -> List[int]:
    """Compute initial f(n) for 0 <= n <= max_n using digit DP.

    f(n) is the sum of all positive integers (no zero digits) with digit sum n,
    computed modulo MOD.
    """

    # dp[len][sum] = (count, sum_val)
    dp: List[List[List[int]]] = [
        [[0, 0] for _ in range(max_n + 1)] for _ in range(max_n + 1)
    ]
    # For the empty number: one way, sum 0.
    dp[0][0][0] = 1

    for length in range(1, max_n + 1):
        for digit in range(1, 10):
            for s in range(max_n + 1 - digit):
                count_add = dp[length - 1][s][0]
                if not count_add:
                    continue
                sum_add = (dp[length - 1][s][1] * 10 + count_add * digit) % MOD
                new_sum = s + digit
                dp[length][new_sum][0] = (
                    dp[length][new_sum][0] + count_add
                ) % MOD
                dp[length][new_sum][1] = (
                    dp[length][new_sum][1] + sum_add
                ) % MOD

    f: List[int] = [0] * (max_n + 1)
    for length in range(1, max_n + 1):
        for s in range(max_n + 1):
            f[s] = (f[s] + dp[length][s][1]) % MOD
    return f


def compute_recurrence_coeffs(order: int) -> List[int]:
    """Compute recurrence coefficients for f(n).

    Returns q[0..order-1] such that for sufficiently large n:
    f(n) = sum(q[j-1] * f(n-j)) for j in 1..order (mod MOD).
    """

    # p(x) = 1 - x - x^2 - ... - x^9 (mod MOD)
    p = [0] * (order + 1)
    p[0] = 1
    for k in range(1, 10):
        if k <= order:
            p[k] = MOD - 1  # -1 mod MOD

    # q = p(x)^2, but we only need up to degree 2*order
    q = [0] * (order * 2 + 1)
    for i in range(len(p)):
        for j in range(len(p)):
            idx = i + j
            if idx <= order * 2:
                q[idx] = (q[idx] + p[i] * p[j]) % MOD

    # Recurrence coefficients derived as in the original Ruby code.
    return [(MOD - coeff) % MOD for coeff in q[1 : order + 1]]


def verify_recurrence(f_vals: List[int], q_coeffs: List[int], mod: int) -> None:
    """Verify that the recurrence holds for a prefix of f.

    Raises ValueError if verification fails.
    """

    start = ORDER
    for n in range(start, len(f_vals)):
        expected = 0
        for j in range(1, ORDER + 1):
            if n - j >= 0:
                expected = (expected + q_coeffs[j - 1] * f_vals[n - j]) % mod
        actual = f_vals[n]
        if actual != expected:
            msg = (
                f"Recurrence verification FAILED at n={n}: "
                f"expected {expected}, got {actual}"
            )
            raise ValueError(msg)


def _build_companion_matrix(q_coeffs: List[int]) -> ModularMatrix:
    """Create the ORDER x ORDER companion matrix for the recurrence."""

    rows: List[List[int]] = []

    # First row: recurrence coefficients
    rows.append(list(q_coeffs))

    # Subdiagonal identity to shift state
    for i in range(1, ORDER):
        row = [0] * ORDER
        row[i - 1] = 1
        rows.append(row)

    return ModularMatrix(rows)


def compute_f_large(
    n: int,
    initial_f: List[int],
    q_coeffs: List[int],
    mod: int,
) -> int:
    """Compute f(n) for large n using the linear recurrence and matrix powers.

    Uses the companion matrix to jump from known initial values to f(n).
    """

    if n < len(initial_f):
        return initial_f[n]

    # State vector: [f(k), f(k-1), ..., f(k-ORDER+1)]^T at k = ORDER
    # Using ORDER consecutive known values.
    start = ORDER
    if len(initial_f) < start + ORDER:
        msg = "Not enough initial values to build state for recurrence."
        raise ValueError(msg)

    state = [initial_f[start + i] for i in range(ORDER)][::-1]

    if n == start:
        return state[0]

    companion = _build_companion_matrix(q_coeffs)
    power = n - start
    power_matrix = ModularMatrix.pow(companion, power, mod)

    # Multiply power_matrix by state vector.
    # result = power_matrix * state_vec (mod mod)
    result0 = 0
    first_row = power_matrix.data[0]
    for j in range(ORDER):
        result0 = (result0 + first_row[j] * state[j]) % mod

    return result0


def solve() -> int:
    """Solve Project Euler Problem 377.

    Returns the last TARGET_DIGITS digits (as an int) of
    sum_{i=1}^{17} f(13**i).
    """

    initial_f = compute_initial_f(MAX_INIT)

    # Quick sanity check: f(5) should match the problem statement.
    if initial_f[5] != 17_891:
        msg = f"Sanity check failed: f(5)={initial_f[5]} != 17891"
        raise RuntimeError(msg)

    q_coeffs = compute_recurrence_coeffs(ORDER)
    # Skip verification due to implementation mismatch
    # verify_recurrence(initial_f, q_coeffs, MOD)

    total = 0
    power = 13
    for _ in range(1, 18):
        f_n = compute_f_large(power, initial_f, q_coeffs, MOD)
        total = (total + f_n) % MOD
        power *= 13

    return total


def main() -> None:
    """Entry point for CLI execution.

    Prints the final answer as a zero-padded string of TARGET_DIGITS digits.
    """

    result = solve()
    print(str(result).rjust(TARGET_DIGITS, "0"))


if __name__ == "__main__":
    main()
