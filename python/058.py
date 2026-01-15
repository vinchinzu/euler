#!/usr/bin/env python3
import sympy as sp

def solve():
    prime_count = 0
    total_diagonals = 1
    side_length = 1

    while True:
        side_length += 2
        corners = [
            side_length ** 2 - 0 * (side_length - 1),
            side_length ** 2 - 1 * (side_length - 1),
            side_length ** 2 - 2 * (side_length - 1),
            side_length ** 2 - 3 * (side_length - 1)
        ]
        corners.pop(0)
        prime_count += sum(1 for n in corners if sp.isprime(n))
        total_diagonals += 4
        ratio = prime_count / total_diagonals
        if ratio < 0.10:
            return side_length

print(solve())
