"""Project Euler Problem 272: Modular Cubes, part 2."""
from __future__ import annotations
from math import isqrt
import bisect
import numpy as np


def solve() -> int:
    N = 10 ** 11
    K = 5

    def sieve(limit):
        if limit < 2:
            return []
        is_prime = bytearray(b'\x01') * (limit + 1)
        is_prime[0] = is_prime[1] = 0
        for i in range(2, isqrt(limit) + 1):
            if is_prime[i]:
                is_prime[i*i::i] = bytearray(len(is_prime[i*i::i]))
        return [i for i in range(2, limit + 1) if is_prime[i]]

    def primes_mod(n, a, m):
        return [p for p in sieve(n) if p % m == a]

    small_ps = primes_mod(100, 1, 3)
    max_p = N // 9
    max_q = N // 9
    for i in range(K - 2):
        max_p //= small_ps[i]
    for i in range(K - 1):
        max_q //= small_ps[i]

    ps = primes_mod(int(max_p), 1, 3)
    max_q_int = int(max_q)
    len_ps = len(ps)
    ps_arr = np.array(ps, dtype=np.int64)

    # Build prod_qs sieve
    prod_qs = bytearray(b'\x01') * (max_q_int + 1)
    prod_qs[0] = 0
    for p in primes_mod(max_q_int, 1, 3):
        for i in range(p, max_q_int + 1, p):
            prod_qs[i] = 0

    def build_prefix_sum():
        psum = np.zeros(max_q_int + 2, dtype=np.int64)
        for i in range(max_q_int + 1):
            psum[i + 1] = psum[i] + (i if prod_qs[i] else 0)
        return psum

    prefix_sum = build_prefix_sum()

    ans = 0

    def accumulate_last_level(index, prod):
        """Handle remaining=1 case efficiently using numpy vectorization."""
        nonlocal ans
        max_prime = N // prod
        hi = bisect.bisect_right(ps, max_prime, index, len_ps)
        if hi <= index:
            return

        # For primes where prod * p^2 > N, only single power contributes
        # prod * p^2 > N => p > sqrt(N/prod)
        sqrt_limit = isqrt(N // prod)
        mid = bisect.bisect_right(ps, sqrt_limit, index, hi)

        # Batch process single-power primes (from mid to hi) using numpy
        if mid < hi:
            chunk = ps_arr[mid:hi]
            # For each p: new_prod = prod * p, limit = min(N // new_prod, max_q_int)
            new_prods = prod * chunk
            limits = np.minimum(N // new_prods, max_q_int).astype(np.int64)
            # ans += sum(new_prod * prefix_sum[limit + 1])
            psum_vals = prefix_sum[limits + 1]
            ans += int(np.sum(new_prods * psum_vals))

        # Handle multi-power primes (from index to mid) with Python loop
        for ni in range(index, mid):
            p = ps[ni]
            new_prod = prod * p
            while new_prod <= N:
                limit = min(N // new_prod, max_q_int)
                ans += int(new_prod * prefix_sum[limit + 1])
                new_prod *= p

    def run_helper(start_prod, start_num_ps):
        nonlocal ans
        stack = [(0, start_prod, start_num_ps)]
        while stack:
            index, prod, num_ps = stack.pop()
            if num_ps >= K:
                limit = min(N // prod, max_q_int)
                ans += int(prod * prefix_sum[limit + 1])
                continue

            remaining = K - num_ps
            if remaining == 1:
                accumulate_last_level(index, prod)
                continue

            max_start = len_ps - remaining
            for new_index in range(index, max_start + 1):
                p = ps[new_index]
                min_final_prod = prod
                ok = True
                for i in range(remaining):
                    min_final_prod *= ps[new_index + i]
                    if min_final_prod > N:
                        ok = False
                        break
                if not ok:
                    break
                new_prod = prod * p
                while new_prod <= N:
                    stack.append((new_index + 1, new_prod, num_ps + 1))
                    new_prod *= p

    # Case 1: with factor of 3^2 (9)
    run_helper(9, 1)

    # Exclude multiples of 9 from prod_qs for case 2
    for i in range(9, max_q_int + 1, 9):
        prod_qs[i] = 0
    prefix_sum = build_prefix_sum()

    # Case 2: without factor of 3
    run_helper(1, 0)

    return ans


def main() -> None:
    print(solve())


if __name__ == "__main__":
    main()
