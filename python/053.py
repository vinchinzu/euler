#!/usr/bin/env python3
from math import comb

LIMIT = 1000000
count = 0

for n in range(1, 101):
    for r in range(1, n + 1):
        if comb(n, r) > LIMIT:
            count += 1

print(count)
