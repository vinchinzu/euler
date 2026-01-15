#!/usr/bin/env python3
import math

LIMIT = 10000

def period_of_sqrt_cf(n):
    a0 = int(math.isqrt(n))
    if a0 * a0 == n:
        return 0

    period = 0
    m = 0
    d = 1
    a = a0

    while True:
        m = d * a - m
        d = (n - m * m) // d
        a = (a0 + m) // d
        period += 1
        if a == 2 * a0:
            break

    return period

count = 0
for n in range(2, LIMIT + 1):
    if period_of_sqrt_cf(n) % 2 == 1:
        count += 1

print(count)
