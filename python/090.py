#!/usr/bin/env python3
"""
Cube digit pairs (Problem 90)

How many distinct arrangements of two cubes allow for all square numbers
to be displayed?
"""

import itertools

# Target two-digit squares
TARGET_SQUARES = [
    [0, 1], [0, 4], [0, 9],
    [1, 6],
    [2, 5],
    [3, 6],
    [4, 9],
    [6, 4],
    [8, 1]
]


def can_display_digit(cube_digits: tuple[int, ...], digit_needed: int) -> bool:
    """Check if cube can display the needed digit (6/9 interchangeable)."""
    if digit_needed == 6 or digit_needed == 9:
        return 6 in cube_digits or 9 in cube_digits
    else:
        return digit_needed in cube_digits


def main() -> None:
    """Count valid cube pairs."""
    # Generate all possible cube configurations (6 digits from 0-9)
    all_cube_configurations = list(itertools.combinations(range(10), 6))
    
    valid_pair_count = 0
    
    for i, cube_a_digits in enumerate(all_cube_configurations):
        for j in range(i, len(all_cube_configurations)):
            cube_b_digits = all_cube_configurations[j]
            
            current_pair_is_valid = True
            
            for d1, d2 in TARGET_SQUARES:
                can_form_square = (
                    (can_display_digit(cube_a_digits, d1) and
                     can_display_digit(cube_b_digits, d2)) or
                    (can_display_digit(cube_a_digits, d2) and
                     can_display_digit(cube_b_digits, d1))
                )
                
                if not can_form_square:
                    current_pair_is_valid = False
                    break
            
            if current_pair_is_valid:
                valid_pair_count += 1
    
    print(valid_pair_count)


if __name__ == "__main__":
    main()
