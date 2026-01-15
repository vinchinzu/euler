#!/usr/bin/env python3
LIMIT = 1_000_000

phi_values = list(range(LIMIT + 1))

for i in range(2, LIMIT + 1):
    if phi_values[i] == i:
        j = i
        while j <= LIMIT:
            phi_values[j] -= phi_values[j] // i
            j += i

total_fractions = 0
for d in range(2, LIMIT + 1):
    total_fractions += phi_values[d]

print(total_fractions)
