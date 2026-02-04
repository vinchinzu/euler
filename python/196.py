"""Project Euler Problem 196 - Prime triplets.

Find sum of primes in rows 5678027 and 7208785 that are part of prime triplets.
Uses segmented sieve with odd-only optimization.
"""
from math import isqrt

def solve():
    TARGET_ROWS = [5678027, 7208785]

    def tr(n):
        return n * (n + 1) // 2

    def sum_good_primes(row):
        start = tr(row - 3) + 1
        end = tr(row + 2)

        # Odd-only segmented sieve
        base = start + 1 if start % 2 == 0 else start
        sieve_len = (end - base) // 2 + 1

        sieve_limit = isqrt(end)

        # Fast small primes generation using slice assignment
        small_sieve = bytearray(sieve_limit + 1)
        small_sieve[0] = small_sieve[1] = 1
        for i in range(2, isqrt(sieve_limit) + 1):
            if not small_sieve[i]:
                n = len(range(i * i, sieve_limit + 1, i))
                small_sieve[i * i::i] = b'\x01' * n

        # Main sieve - mark composites
        is_composite = bytearray(sieve_len)
        for p in range(3, sieve_limit + 1, 2):
            if small_sieve[p]:
                continue
            # Find first odd multiple of p >= max(p*p, base)
            first = p * p
            if first < base:
                k = (base + p - 1) // p
                if k % 2 == 0:
                    k += 1
                first = p * k
            if first > end:
                continue
            idx = (first - base) // 2
            # Use slice assignment for speed
            n = len(range(idx, sieve_len, p))
            is_composite[idx::p] = b'\x01' * n

        # Helper to check primality
        def is_prime_num(num):
            if num < 2:
                return False
            if num == 2:
                return True
            if num % 2 == 0:
                return False
            idx = (num - base) // 2
            if idx < 0 or idx >= sieve_len:
                return False
            return not is_composite[idx]

        # Build row info (5 rows: row-2..row+2, indexed 0-4)
        row_starts = [tr(row - 3 + i) + 1 for i in range(5)]
        row_lens = [row - 2 + i for i in range(5)]

        dirs = [(-1, -1), (-1, 0), (-1, 1), (1, -1), (1, 0), (1, 1)]

        # Pre-build primality arrays for each row to avoid repeated function calls
        is_p = []
        for ri in range(5):
            rlen = row_lens[ri]
            rs = row_starts[ri]
            arr = bytearray(rlen)
            for j in range(rlen):
                if is_prime_num(rs + j):
                    arr[j] = 1
            is_p.append(arr)

        # Compute isCentralPrime for rows 1, 2, 3
        central = [None] * 5
        for ri in range(1, 4):
            c = bytearray(row_lens[ri])
            rlen = row_lens[ri]
            for j in range(rlen):
                if not is_p[ri][j]:
                    continue
                count = 0
                for di, dj in dirs:
                    ni, nj = ri + di, j + dj
                    if 0 <= ni < 5 and 0 <= nj < row_lens[ni]:
                        if is_p[ni][nj]:
                            count += 1
                            if count >= 2:
                                break
                if count >= 2:
                    c[j] = 1
            central[ri] = c

        # Sum good primes in target row (ri=2)
        total = 0
        target_ri = 2
        target_start = row_starts[target_ri]
        target_len = row_lens[target_ri]

        for j in range(target_len):
            if not is_p[target_ri][j]:
                continue
            good = central[target_ri][j]
            if not good:
                for di, dj in dirs:
                    ni, nj = target_ri + di, j + dj
                    if 1 <= ni <= 3 and 0 <= nj < row_lens[ni]:
                        if central[ni][nj]:
                            good = True
                            break
            if good:
                total += target_start + j

        return total

    return sum_good_primes(TARGET_ROWS[0]) + sum_good_primes(TARGET_ROWS[1])

if __name__ == "__main__":
    print(solve())
