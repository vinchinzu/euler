#!/usr/bin/env python3
num_terms = 100

a = [0] * num_terms
a[0] = 2

for k in range(1, num_terms):
    if (k + 1) % 3 == 0:
        a[k] = 2 * (k + 1) // 3
    else:
        a[k] = 1

p_km2 = 0
p_km1 = 1
current_p = 0

for k in range(num_terms):
    current_p = a[k] * p_km1 + p_km2
    p_km2 = p_km1
    p_km1 = current_p

numerator_h99 = current_p
sum_of_digits = sum(int(d) for d in str(numerator_h99))

print(sum_of_digits)
