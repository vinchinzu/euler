"""Project Euler Problem 467: Superstring of prime and composite digital roots.

Let P_D and C_D be the sequences of digital roots of the k'th prime and k'th
composite numbers, respectively. Find the smallest common super-sequence of
the first N digits of P_D and the first N digits of C_D.
"""

from __future__ import annotations

from math import isqrt
from array import array
import ctypes, tempfile, os, subprocess


def sieve_primes(limit: int) -> bytearray:
    """Sieve of Eratosthenes."""
    is_prime = bytearray(limit + 1)
    for i in range(2, limit + 1):
        is_prime[i] = 1
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = 0
    return is_prime


def digital_root(n: int) -> int:
    """Compute digital root."""
    if n == 0:
        return 0
    return 1 + (n - 1) % 9


def solve() -> int:
    """Solve Problem 467."""
    N = 10_000
    M = 10**9 + 7
    B = 10

    # Generate primes and composites
    is_prime = sieve_primes(120_000)
    P = []
    n = 2
    while len(P) < N:
        if is_prime[n]:
            P.append(digital_root(n))
        n += 1

    C = []
    n = 2
    while len(C) < N:
        if not is_prime[n]:
            C.append(digital_root(n))
        n += 1

    # Write a small C helper for the DP
    c_code = r"""
#include <string.h>
#include <limits.h>

void dp_fill(int N, const int *P, const int *C, int *dp, unsigned char *move_i) {
    int stride = N + 1;
    /* Base cases */
    for (int j = 0; j <= N; j++)
        dp[N * stride + j] = N - j;
    for (int i = 0; i <= N; i++)
        dp[i * stride + N] = N - i;
    memset(move_i, 0, (long)(N+1) * stride);

    /* When j==N, only P elements remain, so move_i should be true */
    for (int i = 0; i < N; i++)
        move_i[i * stride + N] = 1;

    for (int i = N - 1; i >= 0; i--) {
        int pi = P[i];
        int *row = dp + i * stride;
        int *next_row = dp + (i + 1) * stride;
        unsigned char *mi_row = move_i + i * stride;
        for (int j = N - 1; j >= 0; j--) {
            if (pi == C[j]) {
                row[j] = 1 + next_row[j + 1];
            } else {
                int val_i = next_row[j];
                int val_j = row[j + 1];
                if (val_i <= val_j) {
                    row[j] = 1 + val_i;
                    if (val_i < val_j || pi < C[j])
                        mi_row[j] = 1;
                } else {
                    row[j] = 1 + val_j;
                }
            }
        }
    }
}
"""
    tmpdir = tempfile.mkdtemp()
    c_path = os.path.join(tmpdir, "dp467.c")
    so_path = os.path.join(tmpdir, "dp467.so")
    with open(c_path, "w") as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-shared", "-fPIC", "-o", so_path, c_path],
                   check=True, capture_output=True)
    lib = ctypes.CDLL(so_path)

    # Prepare arrays
    IntArr = ctypes.c_int * N
    Pa = IntArr(*P)
    Ca = IntArr(*C)

    sz = (N + 1) * (N + 1)
    DpArr = ctypes.c_int * sz
    dp = DpArr()
    MiArr = ctypes.c_ubyte * sz
    move_i = MiArr()

    lib.dp_fill(ctypes.c_int(N),
                ctypes.cast(Pa, ctypes.POINTER(ctypes.c_int)),
                ctypes.cast(Ca, ctypes.POINTER(ctypes.c_int)),
                ctypes.cast(dp, ctypes.POINTER(ctypes.c_int)),
                ctypes.cast(move_i, ctypes.POINTER(ctypes.c_ubyte)))

    # Reconstruct answer
    stride = N + 1
    ans = 0
    i, j = 0, 0
    while i < N or j < N:
        idx = i * stride + j
        if i < N and j < N and P[i] == C[j]:
            digit = P[i]
            i += 1
            j += 1
        elif move_i[idx]:
            digit = P[i]
            i += 1
        else:
            digit = C[j]
            j += 1
        ans = (B * ans + digit) % M

    # Cleanup
    os.unlink(c_path)
    os.unlink(so_path)
    os.rmdir(tmpdir)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
