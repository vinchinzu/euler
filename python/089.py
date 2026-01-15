#!/usr/bin/env python3
"""
Roman numerals (Problem 89)

Find the number of characters saved by writing Roman numerals in minimal form.
"""

ROMAN_VALUES = {
    'I': 1, 'V': 5, 'X': 10, 'L': 50,
    'C': 100, 'D': 500, 'M': 1000
}

MINIMAL_ROMAN_RULES = [
    (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), (100, "C"),
    (90, "XC"), (50, "L"), (40, "XL"), (10, "X"), (9, "IX"), (5, "V"),
    (4, "IV"), (1, "I")
]


def roman_to_int(roman_str: str) -> int:
    """Convert Roman numeral string to integer."""
    total = 0
    i = 0
    while i < len(roman_str):
        val1 = ROMAN_VALUES[roman_str[i]]
        
        if i + 1 < len(roman_str):
            val2 = ROMAN_VALUES[roman_str[i+1]]
            if val1 < val2:
                total += (val2 - val1)
                i += 2
            else:
                total += val1
                i += 1
        else:
            total += val1
            i += 1
    return total


def int_to_minimal_roman(number: int) -> str:
    """Convert integer to minimal Roman numeral."""
    if number <= 0:
        return ""
    
    result_roman = ""
    for value, numeral_string in MINIMAL_ROMAN_RULES:
        while number >= value:
            result_roman += numeral_string
            number -= value
    return result_roman


def main() -> None:
    """Process Roman numerals and count saved characters."""
    from pathlib import Path
    script_dir = Path(__file__).parent
    data_file = script_dir.parent / 'data' / '0089_roman.txt'
    
    with open(data_file) as f:
        original_numerals_list = [line.strip() for line in f if line.strip()]
    
    total_characters_saved = 0
    
    for original_roman in original_numerals_list:
        original_length = len(original_roman)
        integer_value = roman_to_int(original_roman)
        minimal_roman = int_to_minimal_roman(integer_value)
        minimal_length = len(minimal_roman)
        total_characters_saved += (original_length - minimal_length)
    
    print(total_characters_saved)


if __name__ == "__main__":
    main()
