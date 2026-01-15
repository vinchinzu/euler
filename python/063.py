#!/usr/bin/env python3
count = 0

for n in range(1, 22):
    for b in range(1, 10):
        power_val = b ** n
        num_digits = len(str(power_val))

        if num_digits == n:
            count += 1
        elif num_digits > n:
            pass

print(count)
