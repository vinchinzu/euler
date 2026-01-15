#!/usr/bin/env python3
def digit_sum(n):
    return sum(int(d) for d in str(n))

max_sum = 0
for a in range(1, 100):
    for b in range(1, 100):
        s = digit_sum(a ** b)
        max_sum = max(max_sum, s)

print(max_sum)
