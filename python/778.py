"""Project Euler Problem 778: Per-Digit Mod Product.

Let a☒b be the per-digit mod 10 product of each corresponding digit of a and b.
Find the sum of x_1 ☒ x_2 ☒ ... x_K for all 0 ≤ x_i ≤ N.

Consider each place value separately. For each place value and for each digit
d, we can compute the number of integers with d at that place value. We can
then use matrix exponentiation to compute the number of K-tuples with mod 10
product d. Finally, add up these counts multiplied by the appropriate d and
place value.
"""

from __future__ import annotations

from typing import List


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def matrix_multiply(A: List[List[int]], B: List[List[int]], mod: int) -> List[List[int]]:
    """Multiply two matrices."""
    n = len(A)
    m = len(B[0])
    p = len(B)
    result = [[0] * m for _ in range(n)]
    for i in range(n):
        for j in range(m):
            for k in range(p):
                result[i][j] = (result[i][j] + A[i][k] * B[k][j]) % mod
    return result


def matrix_pow(A: List[List[int]], exp: int, mod: int) -> List[List[int]]:
    """Raise matrix to exp-th power."""
    n = len(A)
    result = [[1 if i == j else 0 for j in range(n)] for i in range(n)]
    base = [row[:] for row in A]
    while exp > 0:
        if exp & 1:
            result = matrix_multiply(result, base, mod)
        base = matrix_multiply(base, base, mod)
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 778."""
    N = 765432
    K = 234567
    M = 10**9 + 9
    B = 10

    ans = 0

    pow_B = 1
    while pow_B < N:
        # Count numbers with each digit d at this place value
        counts = [0] * B
        for d in range(B):
            diff = d - (N // pow_B) % B
            base_count = (N // B) // pow_B
            if diff > 0:
                counts[d] = base_count * pow_B
            elif diff == 0:
                counts[d] = base_count * pow_B + (N % pow_B) + 1
            else:
                counts[d] = (base_count + 1) * pow_B

        # Build transition matrix
        A = [[0] * B for _ in range(B)]
        for d in range(B):
            for d2 in range(B):
                A[(d * d2) % B][d] = (A[(d * d2) % B][d] + counts[d2]) % M

        # Compute A^K
        A_pow = matrix_pow(A, K, M)

        # Sum contributions
        for d in range(B):
            ans = (ans + A_pow[d][1] * pow_B * d) % M

        pow_B *= B

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
