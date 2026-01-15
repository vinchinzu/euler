#!/usr/bin/env python3
LIMIT_D = 12_000
count = 0

for d in range(1, LIMIT_D + 1):
    n_min = d // 3 + 1
    n_max = (d - 1) // 2

    if n_min <= n_max:
        for n in range(n_min, n_max + 1):
            if __import__('math').gcd(n, d) == 1:
                count += 1

print(count)
