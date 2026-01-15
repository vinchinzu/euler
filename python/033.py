#!/usr/bin/env python3
"""
Project Euler Problem 33: Digit canceling fractions

The fraction 49/98 is a curious fraction, as an inexperienced mathematician
in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which
is correct, is obtained by cancelling the 9s.

We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

There are exactly four non-trivial examples of this type of fraction,
less than one in value, and containing two digits in the numerator and denominator.

If the product of these four fractions is given in its lowest common terms,
find the value of the denominator.
"""

from math import gcd


def main():
    numerator_product = 1
    denominator_product = 1
    
    for n in range(10, 99):
        for d in range(n + 1, 100):
            # Skip trivial examples
            if n % 10 == 0 and d % 10 == 0:
                continue
            
            n_str = str(n)
            d_str = str(d)
            
            # Check for canceling patterns
            fraction_value = n / d
            canceled = False
            
            if n_str[0] == d_str[1] and n_str[1] < d_str[0]:
                if n / d == int(n_str[1]) / int(d_str[0]):
                    canceled = True
            elif n_str[1] == d_str[0] and n_str[0] < d_str[1]:
                if n / d == int(n_str[0]) / int(d_str[1]):
                    canceled = True
            
            if canceled:
                numerator_product *= n
                denominator_product *= d
    
    common_divisor = gcd(numerator_product, denominator_product)
    result = denominator_product // common_divisor
    print(result)


if __name__ == "__main__":
    main()
