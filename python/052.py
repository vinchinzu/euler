#!/usr/bin/env python3
def same_digits(x, y):
    return sorted(str(x)) == sorted(str(y))

x = 1
while True:
    if all(same_digits(x, x * mult) for mult in range(2, 7)):
        print(x)
        break
    x += 1
