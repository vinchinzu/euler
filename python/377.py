"""Project Euler Problem 377.

f(n) = sum of all positive integers (digits 1-9 only) with digit sum n.
count(n) = number of such integers.

Recurrence:
  f(n) = 10*sum(f(n-k) for k=1..9) + sum(k*count(n-k) for k=1..9)
  count(n) = sum(count(n-k) for k=1..9)

Use 18x18 companion matrix exponentiation to compute f(13^i) for i=1..17.
Answer = sum(f(13^i) for i=1..17) mod 10^9.
"""
from __future__ import annotations
from typing import List

MOD = 1_000_000_000
ORDER = 18  # 9 for f values + 9 for count values
MAX_INIT = 40


def mat_mult(A: List[List[int]], B: List[List[int]], mod: int) -> List[List[int]]:
    n = len(A)
    m = len(B[0])
    k = len(B)
    C = [[0] * m for _ in range(n)]
    for i in range(n):
        Ai = A[i]
        for p in range(k):
            aip = Ai[p]
            if aip:
                Bp = B[p]
                for j in range(m):
                    C[i][j] = (C[i][j] + aip * Bp[j]) % mod
    return C


def mat_pow(M: List[List[int]], exp: int, mod: int) -> List[List[int]]:
    n = len(M)
    result = [[1 if i == j else 0 for j in range(n)] for i in range(n)]
    base = [row[:] for row in M]
    while exp > 0:
        if exp & 1:
            result = mat_mult(result, base, mod)
        base = mat_mult(base, base, mod)
        exp >>= 1
    return result


def compute_initial_values(max_n: int):
    """Compute f(n) and count(n) for 0 <= n <= max_n using digit DP."""
    # dp[length][sum] = (count, sum_of_values)
    dp = [[[0, 0] for _ in range(max_n + 1)] for _ in range(max_n + 1)]
    dp[0][0][0] = 1  # empty number: count=1, sum=0

    for length in range(1, max_n + 1):
        for digit in range(1, 10):
            for s in range(max_n + 1 - digit):
                cnt = dp[length - 1][s][0]
                if not cnt:
                    continue
                # Each number v of length-1 with digit sum s becomes 10*v + digit
                sum_val = (dp[length - 1][s][1] * 10 + cnt * digit) % MOD
                ns = s + digit
                dp[length][ns][0] = (dp[length][ns][0] + cnt) % MOD
                dp[length][ns][1] = (dp[length][ns][1] + sum_val) % MOD

    f = [0] * (max_n + 1)
    count = [0] * (max_n + 1)
    for length in range(1, max_n + 1):
        for s in range(max_n + 1):
            f[s] = (f[s] + dp[length][s][1]) % MOD
            count[s] = (count[s] + dp[length][s][0]) % MOD
    return f, count


def verify_recurrence(f, count, max_n):
    """Verify the recurrence holds for computed initial values."""
    for n in range(10, max_n + 1):
        expected_f = 0
        for k in range(1, 10):
            expected_f = (expected_f + 10 * f[n - k] + k * count[n - k]) % MOD
        if expected_f != f[n]:
            raise ValueError(f"f recurrence failed at n={n}: expected {expected_f}, got {f[n]}")

        expected_c = 0
        for k in range(1, 10):
            expected_c = (expected_c + count[n - k]) % MOD
        if expected_c != count[n]:
            raise ValueError(f"count recurrence failed at n={n}: expected {expected_c}, got {count[n]}")


def build_companion_matrix():
    """Build the 18x18 companion matrix.

    State vector: [f(n-1), f(n-2), ..., f(n-9), count(n-1), count(n-2), ..., count(n-9)]

    Row 0 computes f(n):
      f(n) = 10*f(n-1) + 10*f(n-2) + ... + 10*f(n-9) + 1*count(n-1) + 2*count(n-2) + ... + 9*count(n-9)
    Rows 1-8 shift f values: row i copies from position i-1 (identity subdiagonal)
    Row 9 computes count(n):
      count(n) = count(n-1) + count(n-2) + ... + count(n-9)
    Rows 10-17 shift count values: row i copies from position i-1
    """
    M = [[0] * ORDER for _ in range(ORDER)]

    # Row 0: f(n) = 10*f(n-1) + ... + 10*f(n-9) + 1*c(n-1) + ... + 9*c(n-9)
    for k in range(9):
        M[0][k] = 10  # coefficient for f(n-k-1)
    for k in range(9):
        M[0][9 + k] = k + 1  # coefficient for count(n-k-1)

    # Rows 1-8: shift f values (f(n-1) <- f(n-2), etc.)
    for i in range(1, 9):
        M[i][i - 1] = 1

    # Row 9: count(n) = count(n-1) + ... + count(n-9)
    for k in range(9):
        M[9][9 + k] = 1

    # Rows 10-17: shift count values
    for i in range(10, 18):
        M[i][i - 1] = 1

    return M


def compute_f_at(n, f, count):
    """Compute f(n) using matrix exponentiation."""
    if n < len(f):
        return f[n]

    # Build state vector at position 9 (using values f(1)..f(9), count(1)..count(9))
    # State = [f(9), f(8), ..., f(1), count(9), count(8), ..., count(1)]
    base_pos = 9
    state = []
    for i in range(9):
        state.append(f[base_pos - i])   # f(9), f(8), ..., f(1)
    for i in range(9):
        state.append(count[base_pos - i])  # count(9), count(8), ..., count(1)

    companion = build_companion_matrix()
    power = n - base_pos
    M = mat_pow(companion, power, MOD)

    # result = M * state
    result = 0
    for j in range(ORDER):
        result = (result + M[0][j] * state[j]) % MOD
    return result


def solve():
    f, count = compute_initial_values(MAX_INIT)

    # Sanity check
    assert f[5] == 17891, f"f(5)={f[5]} != 17891"

    # Verify recurrence on initial values
    verify_recurrence(f, count, MAX_INIT)

    total = 0
    power = 13
    for i in range(1, 18):
        fn = compute_f_at(power, f, count)
        total = (total + fn) % MOD
        power *= 13

    return total


if __name__ == "__main__":
    result = solve()
    print(str(result).rjust(9, "0"))
