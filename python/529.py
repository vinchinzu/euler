"""Project Euler Problem 529: 10-substring-friendly Numbers.

A number is 10-substring-friendly if every digit belongs to a substring whose
digit sum is 10. Find T(10^18) mod 10^9+7 where T(n) counts 10-substring-friendly
numbers from 1 to 10^n.

Approach:
1. Build a sparse transition matrix over reachable DP states (mask, s).
2. Use sparse matrix-vector multiplication to compute enough terms of the sequence.
3. Use Berlekamp-Massey to find the minimal linear recurrence.
4. Evaluate at n=10^18 using polynomial exponentiation modulo the characteristic polynomial.
"""

from __future__ import annotations
import numpy as np
from scipy.sparse import csr_matrix


def berlekamp_massey(s: list[int], mod: int) -> list[int]:
    """Find shortest linear recurrence using Berlekamp-Massey algorithm."""
    n = len(s)
    C = [1]
    B = [1]
    L = 0
    m = 1
    b = 1
    for i in range(n):
        d = s[i]
        for j in range(1, L + 1):
            d = (d + C[j] * s[i - j]) % mod
        if d == 0:
            m += 1
        elif 2 * L <= i:
            T = list(C)
            coeff = d * pow(b, -1, mod) % mod
            while len(C) < len(B) + m:
                C.append(0)
            for j in range(len(B)):
                C[j + m] = (C[j + m] - coeff * B[j]) % mod
            L = i + 1 - L
            B = T
            b = d
            m = 1
        else:
            coeff = d * pow(b, -1, mod) % mod
            while len(C) < len(B) + m:
                C.append(0)
            for j in range(len(B)):
                C[j + m] = (C[j + m] - coeff * B[j]) % mod
            m += 1
    return [(-c) % mod for c in C[1:L + 1]]


def solve() -> int:
    """Solve Problem 529."""
    N = 10**18
    MOD = 10**9 + 7
    B = 10
    HALF = 1 << 15

    # Phase 1: Find all reachable DP states.
    # State = (suffix_sums_bitmask, partial_sum_s)
    # bit i of mask means there is a suffix whose digit sum equals i.
    # s tracks the uncovered digit sum (s=0 means all digits are covered).
    dp = {}
    dp[(1, 0)] = 1
    for _ in range(15):
        new_dp = {}
        for (mask, s) in dp:
            for d in range(B):
                if d > B - s:
                    break
                new_suf = ((mask << d) & ((1 << B) - 1)) | 1
                new_s = 0 if (mask & (1 << (B - d))) > 0 else d + s
                new_dp[(new_suf, new_s)] = 1
        dp = new_dp

    states = sorted(dp.keys())
    state_idx = {s: i for i, s in enumerate(states)}
    NS = len(states)

    # Phase 2: Build sparse transition matrix.
    rows, cols, vals = [], [], []
    for i, (mask, s) in enumerate(states):
        for d in range(B):
            if d > B - s:
                break
            new_suf = ((mask << d) & ((1 << B) - 1)) | 1
            new_s = 0 if (mask & (1 << (B - d))) > 0 else d + s
            if (new_suf, new_s) in state_idx:
                j = state_idx[(new_suf, new_s)]
                rows.append(j)
                cols.append(i)
                vals.append(1)

    sp_trans = csr_matrix((vals, (rows, cols)), shape=(NS, NS), dtype=np.int64)

    # Initial and target vectors.
    v = np.zeros(NS, dtype=np.int64)
    v[state_idx[(1, 0)]] = 1
    target = np.zeros(NS, dtype=np.int64)
    for i, (mask, s) in enumerate(states):
        if s == 0:
            target[i] = 1

    # Phase 3: Compute sequence values using sparse matrix-vector multiplication.
    # seq[k] = target . M^k . v = count of k-digit zero-padded friendly strings.
    # T(n) = seq[n] - 1 (subtract the all-zeros string which is never friendly).
    num_terms = 5600
    seq = []
    cur = v.copy()
    for _ in range(num_terms):
        val = int(np.sum(target * cur) % MOD)
        seq.append(val)
        cur = sp_trans.dot(cur) % MOD

    # Phase 4: Find linear recurrence via Berlekamp-Massey.
    rec = berlekamp_massey(seq, MOD)
    L = len(rec)

    # Phase 5: Polynomial exponentiation to evaluate at n = 10^18.
    rec_arr = np.array(rec, dtype=np.int64)
    rec_rev = rec_arr[::-1].copy()

    def poly_mult_mod_np(a: np.ndarray, b: np.ndarray) -> np.ndarray:
        """Multiply polynomials mod characteristic polynomial, using splitting to avoid overflow."""
        a_lo = a % HALF
        a_hi = a // HALF
        b_lo = b % HALF
        b_hi = b // HALF
        conv_ll = np.convolve(a_lo, b_lo)
        conv_lh = np.convolve(a_lo, b_hi)
        conv_hl = np.convolve(a_hi, b_lo)
        conv_hh = np.convolve(a_hi, b_hi)
        full_len = 2 * L - 1
        result = np.zeros(2 * L, dtype=np.int64)
        result[:full_len] = (
            conv_ll[:full_len] % MOD
            + ((conv_lh[:full_len] + conv_hl[:full_len]) % MOD) * HALF % MOD
            + (conv_hh[:full_len] % MOD) * (HALF * HALF % MOD) % MOD
        ) % MOD
        for i in range(2 * L - 2, L - 1, -1):
            if result[i] == 0:
                continue
            c = int(result[i])
            result[i] = 0
            result[i - L:i] = (result[i - L:i] + c * rec_rev) % MOD
        return result[:L]

    base = np.zeros(L, dtype=np.int64)
    base[1] = 1  # polynomial x
    result_poly = np.zeros(L, dtype=np.int64)
    result_poly[0] = 1  # polynomial 1

    exp = N
    while exp > 0:
        if exp & 1:
            result_poly = poly_mult_mod_np(result_poly, base)
        base = poly_mult_mod_np(base, base)
        exp >>= 1

    # Evaluate: answer = sum(result_poly[i] * seq[i]) mod MOD
    ans = 0
    for i in range(L):
        ans = (ans + int(result_poly[i]) * seq[i]) % MOD

    # T(N) = seq[N] - 1
    return (ans - 1) % MOD


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
