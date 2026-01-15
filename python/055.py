#!/usr/bin/env python3
def is_lychrel(n, max_iter=50):
    for _ in range(max_iter):
        rev_n = int(str(n)[::-1])
        n = n + rev_n
        if str(n) == str(n)[::-1]:
            return False
    return True

count = sum(1 for i in range(1, 10000) if is_lychrel(i))
print(count)
