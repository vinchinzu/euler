"""Project Euler Problem 319 - Bounded Sequences.

t(n) = sum_{k=1}^n (3^k - 2^k) - sum_{k=2}^n t(floor(n/k))

Use sqrt decomposition for O(n^{3/4}).
Compute bottom-up, optimized for Python.
"""
import math

def solve():
    N = 10**10
    M = 10**9

    # Enumerate all distinct values of floor(N/k)
    values = []
    k = 1
    while k <= N:
        v = N // k
        values.append(v)
        k = N // v + 1
    values.sort()

    nv = len(values)

    # Map value -> index. Small values map directly, large values via N//v
    # values[0..sqrt(N)-1] are 1, 2, ..., sqrt(N)
    # values[sqrt(N)..nv-1] are N//sqrt(N), ..., N//2, N//1 = N
    sqN = math.isqrt(N)

    # Build index lookup: for v <= sqN, idx = v - 1 (since values start at 1)
    # For v > sqN, v = N // k for some k <= sqN, idx = nv - k
    def get_idx(v):
        if v <= sqN:
            return v - 1
        else:
            return nv - (N // v)

    t_arr = [0] * nv
    M2 = 2 * M  # for 3^k computation

    for idx in range(nv):
        n = values[idx]
        l = math.isqrt(n)

        # S(n) = (3^{n+1} - 1)/2 - (2^{n+1} - 1) mod M
        val3 = (pow(3, n + 1, M2) - 1) // 2
        val2 = pow(2, n + 1, M) - 1
        result = (val3 - val2) % M

        # Subtract sum_{k=2}^{l} t(n // k)
        for k in range(2, l + 1):
            nk = n // k
            result -= t_arr[get_idx(nk)]

        # Subtract sum_{q=1}^{n//l - 1} (n//q - n//(q+1)) * t(q)
        upper = n // l if l > 0 else 1
        for q in range(1, upper):
            count = n // q - n // (q + 1)
            result -= count * t_arr[q - 1]  # t(q) = t_arr[q-1] since values[q-1] = q for small q

        t_arr[idx] = result % M

    return t_arr[nv - 1]

if __name__ == "__main__":
    print(solve())
