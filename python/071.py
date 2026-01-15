#!/usr/bin/env python3
limit_d = 1_000_000
target_d = 0

for d_candidate in range(limit_d, 0, -1):
    if d_candidate % 7 == 5:
        target_d = d_candidate
        break

numerator_n = (3 * target_d - 1) // 7

print(numerator_n)
