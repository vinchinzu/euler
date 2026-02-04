"""Project Euler Problem 154: Exploring Pascal's pyramid.

Find the number of coefficients in (x+y+z)^200000 divisible by 10^12.

Approach: Vectorized using numpy. For each a, compute v5 and v2 deficits
for all valid b values simultaneously and count where both conditions met.
Uses reversed-index arrays to avoid per-iteration reversal.
"""

import numpy as np

def main():
    N = 200_000
    K = 12

    # Precompute v5(i) and v2(i) for each i
    v5 = np.zeros(N + 1, dtype=np.int32)
    v2 = np.zeros(N + 1, dtype=np.int32)
    for i in range(1, N + 1):
        n = i
        while n % 5 == 0:
            v5[i] += 1
            n //= 5
        n = i
        while n % 2 == 0:
            v2[i] += 1
            n //= 2

    # F[i] = v5(i!), T[i] = v2(i!)
    F = np.cumsum(v5, dtype=np.int32)
    T = np.cumsum(v2, dtype=np.int32)

    # Pre-compute reversed arrays: FR[i] = F[N - i], TR[i] = T[N - i]
    # Then F[N-a-b] = FR[a+b], and for b in [b_lo, b_hi]:
    #   FR[a + b_lo : a + b_hi + 1]
    FR = F[N::-1].copy()  # FR[k] = F[N-k]
    TR = T[N::-1].copy()

    FN = int(F[N])
    TN = int(T[N])

    ans = 0

    max_inner = N // 2
    d5_buf = np.empty(max_inner, dtype=np.int32)
    d2_buf = np.empty(max_inner, dtype=np.int32)

    for a in range(N // 3 + 1):
        if 3 * a >= N:
            break
        tempF = int(F[a]) + K - FN
        tempT = int(T[a]) + K - TN

        b_lo = a + 1
        b_hi = (N - a - 1) // 2
        if b_lo > b_hi:
            continue

        length = b_hi - b_lo + 1

        # F[b] for b in [b_lo, b_hi]
        Fb = F[b_lo:b_hi + 1]
        Tb = T[b_lo:b_hi + 1]

        # F[N-a-b] = FR[a+b] for b in [b_lo, b_hi]
        # a+b ranges from a+b_lo to a+b_hi
        ab_lo = a + b_lo
        ab_hi = a + b_hi
        Fc = FR[ab_lo:ab_hi + 1]
        Tc = TR[ab_lo:ab_hi + 1]

        np.add(Fb, Fc, out=d5_buf[:length])
        d5_buf[:length] += tempF
        np.add(Tb, Tc, out=d2_buf[:length])
        d2_buf[:length] += tempT

        mask_count = int(np.count_nonzero((d5_buf[:length] <= 0) & (d2_buf[:length] <= 0)))
        ans += mask_count * 6

        # Case a == b
        c = N - 2 * a
        if c > a:
            if int(F[a]) + int(F[c]) + tempF <= 0 and int(T[a]) + int(T[c]) + tempT <= 0:
                ans += 3

        # Case b == c
        if (N - a) % 2 == 0:
            half = (N - a) // 2
            if half > a:
                if 2 * int(F[half]) + tempF <= 0 and 2 * int(T[half]) + tempT <= 0:
                    ans += 3

    return ans

if __name__ == "__main__":
    print(main())
