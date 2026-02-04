"""Project Euler Problem 654: Neighbourly Constraints.

Find T(N, K), the number of sequences of N positive integers such that the
sum of any two consecutive elements is at most K.

Uses Berlekamp-Massey to find the recurrence, and Kitamasa's method with
FFT-based polynomial arithmetic to evaluate at N = 10^12.
"""

from __future__ import annotations
import numpy as np
from scipy.signal import fftconvolve


MOD = 10**9 + 7
SPLIT = 1 << 15
SPLIT_MOD = SPLIT % MOD
SPLIT2_MOD = (SPLIT * SPLIT) % MOD


def mod_conv(a: np.ndarray, b: np.ndarray) -> np.ndarray:
    """Polynomial multiplication mod MOD using split + FFT convolution."""
    a_lo = (a % SPLIT).astype(np.float64)
    a_hi = (a // SPLIT).astype(np.float64)
    b_lo = (b % SPLIT).astype(np.float64)
    b_hi = (b // SPLIT).astype(np.float64)

    ll = np.round(fftconvolve(a_lo, b_lo)).astype(np.int64)
    lh = np.round(fftconvolve(a_lo, b_hi)).astype(np.int64)
    hl = np.round(fftconvolve(a_hi, b_lo)).astype(np.int64)
    hh = np.round(fftconvolve(a_hi, b_hi)).astype(np.int64)

    return (ll % MOD + (lh % MOD + hl % MOD) % MOD * SPLIT_MOD % MOD
            + hh % MOD * SPLIT2_MOD % MOD) % MOD


def poly_mod_precomp(p: np.ndarray, d: int, q_rev_inv: np.ndarray,
                     cp_trunc: np.ndarray) -> np.ndarray:
    """Compute p mod char_poly using Newton's method for polynomial division.

    p: polynomial of degree <= 2*(d-1)
    d: degree of char_poly
    q_rev_inv: precomputed inverse of rev(char_poly) mod x^(d-1)
    cp_trunc: char_poly without the leading 1 (coefficients 0..d-1)
    """
    if len(p) <= d:
        return p[:d].copy() if len(p) == d else np.pad(p, (0, d - len(p)))

    n = len(p) - 1  # degree of p
    # rev(p) = p reversed
    p_rev = p[::-1].copy()
    # Compute quotient: rev(q) = rev(p) * q_rev_inv mod x^(n-d+1)
    t = n - d + 1  # degree of quotient
    # Truncate p_rev and q_rev_inv to length t
    pr_trunc = p_rev[:t]
    qi_trunc = q_rev_inv[:t]
    quot_rev = mod_conv(pr_trunc, qi_trunc)[:t]
    # Reverse to get quotient
    quot = quot_rev[::-1].copy()

    # remainder = p - quot * char_poly
    # char_poly = [cp_trunc..., 1] (monic)
    # quot * char_poly = quot * cp_trunc + quot shifted by d
    qc = mod_conv(quot, cp_trunc)
    # The remainder is p[:d] - qc[:d]
    rem = np.zeros(d, dtype=np.int64)
    rem[:d] = (p[:d] - qc[:d]) % MOD
    return rem


def precompute_q_rev_inv(char_poly: np.ndarray, d: int, max_len: int) -> np.ndarray:
    """Precompute inverse of rev(char_poly) mod x^max_len using Newton's iteration."""
    # rev(char_poly) starts with 1 (since char_poly is monic)
    q_rev = char_poly[::-1].copy()

    # Newton's iteration: given f, find g such that f*g = 1 mod x^n
    # Start with g_0 = 1
    # g_{i+1} = g_i * (2 - f * g_i) mod x^{2^{i+1}}
    g = np.array([1], dtype=np.int64)
    cur_len = 1
    target = max_len

    while cur_len < target:
        next_len = min(cur_len * 2, target)
        fg = mod_conv(q_rev[:next_len], g)[:next_len]
        # 2 - fg
        two_minus_fg = (-fg) % MOD
        two_minus_fg[0] = (2 - fg[0]) % MOD
        g = mod_conv(g, two_minus_fg)[:next_len]
        cur_len = next_len

    return g[:max_len]


def berlekamp_massey(s: list[int]) -> list[int]:
    """Find the minimal linear recurrence for s modulo MOD."""
    s_arr = np.array(s, dtype=np.int64)
    C = np.array([1], dtype=np.int64)
    B = np.array([1], dtype=np.int64)
    L = 0
    m = 1
    b = 1
    for n in range(len(s)):
        if L > 0:
            seg = s_arr[n - L:n][::-1]
            c_slice = C[1:L + 1]
            d = int(s_arr[n])
            CHUNK = 500
            for start in range(0, L, CHUNK):
                end = min(start + CHUNK, L)
                partial = np.sum((c_slice[start:end] * seg[start:end]) % MOD)
                d = (d + int(partial)) % MOD
        else:
            d = int(s_arr[n]) % MOD

        if d == 0:
            m += 1
        elif 2 * L <= n:
            T = C.copy()
            coef = d * pow(b, MOD - 2, MOD) % MOD
            if len(C) < len(B) + m:
                C = np.pad(C, (0, len(B) + m - len(C)))
            B_shifted = np.zeros(len(C), dtype=np.int64)
            B_shifted[m:m + len(B)] = B
            C = (C - coef * B_shifted) % MOD
            L = n + 1 - L
            B = T
            b = d
            m = 1
        else:
            coef = d * pow(b, MOD - 2, MOD) % MOD
            if len(C) < len(B) + m:
                C = np.pad(C, (0, len(B) + m - len(C)))
            B_shifted = np.zeros(len(C), dtype=np.int64)
            B_shifted[m:m + len(B)] = B
            C = (C - coef * B_shifted) % MOD
            m += 1
    return list(C)


def solve() -> int:
    """Solve Problem 654."""
    N = 10**12
    K = 5000

    # Compute T[1..2K-1] using numpy for speed
    dp = np.zeros(K, dtype=np.int64)
    dp[1:] = 1

    seq = []
    for i in range(1, 2 * K):
        ti = int(np.sum(dp) % MOD)
        seq.append(ti)
        new_dp = np.zeros(K, dtype=np.int64)
        new_dp[1] = ti
        offsets = np.cumsum(dp[K - 1:0:-1])
        new_dp[2:K] = (ti - offsets[:K - 2]) % MOD
        dp = new_dp

    # Find minimal linear recurrence
    C = berlekamp_massey(seq)
    d = len(C) - 1
    recurrence = [(-C[i]) % MOD for i in range(1, d + 1)]
    initial = seq[:d]

    # Build characteristic polynomial (monic)
    char_poly = np.zeros(d + 1, dtype=np.int64)
    char_poly[d] = 1
    for i in range(d):
        char_poly[d - 1 - i] = (-recurrence[i]) % MOD
    cp_trunc = char_poly[:d].copy()

    # Precompute inverse of rev(char_poly) for fast polynomial mod
    max_inv_len = d  # We need inverse up to degree d-1
    q_rev_inv = precompute_q_rev_inv(char_poly, d, max_inv_len)

    # Kitamasa: compute x^(N-1) mod char_poly
    result = np.zeros(d, dtype=np.int64)
    result[0] = 1
    base = np.zeros(d, dtype=np.int64)
    base[1] = 1

    def poly_mul_and_mod(a, b):
        p = mod_conv(a, b)
        return poly_mod_precomp(p, d, q_rev_inv, cp_trunc)

    exp = N - 1
    while exp > 0:
        if exp & 1:
            result = poly_mul_and_mod(result, base)
        base = poly_mul_and_mod(base, base)
        exp >>= 1

    ans = 0
    for i in range(d):
        ans = (ans + int(result[i]) * initial[i]) % MOD
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
