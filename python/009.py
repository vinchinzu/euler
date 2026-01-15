#!/usr/bin/env python3
"""
Project Euler Problem 9: Special Pythagorean triplet

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
a² + b² = c².

For example, 3² + 4² = 9 + 16 = 25 = 5².

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
"""


def find_pythagorean_product(perimeter: int) -> int:
    """Find the Pythagorean triplet product for given perimeter."""
    if perimeter < 1 or not isinstance(perimeter, int):
        raise ValueError("Perimeter must be a positive integer")
    
    # Optimization: a must be less than perimeter/3 since a < b < c
    max_a = perimeter // 3
    
    for a in range(1, max_a + 1):
        # For each a, b must be > a and < (perimeter - a)/2 to ensure b < c
        max_b = (perimeter - a - 1) // 2
        
        for b in range(a + 1, max_b + 1):
            c = perimeter - a - b
            
            # Ensure c > b and c > 0
            if c <= b or c <= 0:
                continue
            
            # Check Pythagorean theorem
            if a * a + b * b == c * c:
                return a * b * c
    
    raise ValueError(f"No Pythagorean triplet found for perimeter {perimeter}")


def main():
    result = find_pythagorean_product(1000)
    print(result)


if __name__ == "__main__":
    main()
