#!/usr/bin/env python3
"""
Project Euler Problem 39: Integer right triangles

If p is the perimeter of a right angle triangle with integral length sides,
there are exactly three solutions for p = 120: {20,48,52}, {24,45,51}, {30,40,50}.

For which value of p ≤ 1000, is the number of solutions maximised?
"""

from math import isqrt


def main():
    P = 1000
    counts = {}
    
    for a in range(1, P // 2 + 1):
        for b in range(a, P // 2 + 1):
            if a + b > P // 2:
                break
            c = isqrt(a * a + b * b)
            if c * c == a * a + b * b:
                perimeter = a + b + c
                if perimeter <= P:
                    counts[perimeter] = counts.get(perimeter, 0) + 1
    
    max_perimeter = max(counts.items(), key=lambda x: x[1])[0]
    print(max_perimeter)


if __name__ == "__main__":
    main()
