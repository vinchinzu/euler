"""Project Euler Problem 322 - Binomial coefficients divisible by 10.

Find T(m, n) = count of i in [n, m) where C(i, n) is divisible by 10.
m = 10^18, n = 10^12 - 10.

By Lucas' theorem, C(i, n) is divisible by p iff there's a carry when
adding (i-n) and n in base p. Use inclusion-exclusion for p=2 and p=5.
Uses numpy for fast bitwise operations on the dual count.
"""
import numpy as np

def solve():
    N = 10**18
    K = 10**12 - 10

    def num_no_carries(n, k, p):
        """Count i in [0, n) such that adding i and k in base p has no carries."""
        if n == 0:
            return 0
        largest_pow = 1
        while largest_pow * p <= n:
            largest_pow *= p
        num_remaining = 1
        pp = 1
        while pp < largest_pow:
            num_remaining *= (p - (k // pp % p))
            pp *= p
        result = 0
        top_digit_n = n // largest_pow
        top_digit_k = k // largest_pow % p
        for i in range(p):
            if i + top_digit_k >= p:
                break
            if i == top_digit_n:
                result += num_no_carries(n % largest_pow, k, p)
            elif i < top_digit_n:
                result += num_remaining
        return result

    no_carry_2 = num_no_carries(N - K, K, 2)
    no_carry_5 = num_no_carries(N - K, K, 5)

    # Build all no-carry base-5 values (MSD first like Java)
    k5_str = ''
    tmp = K
    while tmp > 0:
        k5_str = str(tmp % 5) + k5_str
        tmp //= 5

    all_allowed = []
    for ch in k5_str:
        d = int(ch)
        all_allowed.append(list(range(5 - d)))

    no_carry_vals = [0]
    for allowed in all_allowed:
        new_vals = []
        for v in no_carry_vals:
            for d in allowed:
                new_vals.append(v * 5 + d)
        no_carry_vals = new_vals

    big_pow5 = 5 ** len(k5_str)
    limit = N - K

    # Count d = val + j*big_pow5 < limit with d & K == 0
    # Using numpy for fast vectorized bitwise operations
    K_np = np.uint64(K)
    step = np.uint64(big_pow5)

    no_carry_both = 0
    for val_int in no_carry_vals:
        if val_int >= limit:
            continue
        max_j = (limit - 1 - val_int) // big_pow5
        j_arr = np.arange(int(max_j) + 1, dtype=np.uint64)
        d_arr = np.uint64(val_int) + j_arr * step
        mask = (d_arr & K_np) == np.uint64(0)
        no_carry_both += int(np.sum(mask))

    ans = (N - K) - no_carry_2 - no_carry_5 + no_carry_both
    return ans

if __name__ == "__main__":
    print(solve())
