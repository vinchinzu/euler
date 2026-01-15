#!/usr/bin/env python3
import sympy as sp

def solve():
    limit = 200000
    primes = list(sp.primerange(2, limit))

    for p in primes:
        s_p = str(p)
        num_digits = len(s_p)
        if num_digits < 2:
            continue

        digit_counts = {}
        digit_positions = {}

        for pos, digit in enumerate(s_p):
            if digit not in digit_counts:
                digit_counts[digit] = 0
                digit_positions[digit] = []
            digit_counts[digit] += 1
            digit_positions[digit].append(pos)

        for digit, count in digit_counts.items():
            if count < 3:
                continue

            positions = digit_positions[digit]
            prime_family = []

            for replacement in '0123456789':
                if 0 in positions and replacement == '0' and num_digits > 1:
                    continue

                candidate_chars = list(s_p)
                for pos in positions:
                    candidate_chars[pos] = replacement
                candidate_str = ''.join(candidate_chars)

                candidate_num = int(candidate_str)
                if sp.isprime(candidate_num):
                    prime_family.append(candidate_num)

            if len(prime_family) == 8:
                return min(prime_family)
    return None

print(solve())
