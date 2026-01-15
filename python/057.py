#!/usr/bin/env python3
def solve():
    limit = 1000
    count = 0

    n = 1
    d = 1

    for _ in range(limit):
        current_expansion_n = n + 2 * d
        current_expansion_d = n + d

        if len(str(current_expansion_n)) > len(str(current_expansion_d)):
            count += 1

        n = current_expansion_n
        d = current_expansion_d

    return count

print(solve())
