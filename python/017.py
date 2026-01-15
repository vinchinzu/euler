#!/usr/bin/env python3
"""
Project Euler Problem 17: Number letter counts

If the numbers 1 to 5 are written out in words: one, two, three, four, five,
then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words,
how many letters would be used?

NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two)
contains 23 letters and 115 (one hundred and fifteen) contains 20 letters.
The use of "and" when writing out numbers is in compliance with British usage.
"""

ONES = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
    "eighteen", "nineteen"
]

TENS = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
]


def number_to_words(n: int) -> str:
    """Convert a number to its English word representation (British usage with 'and')."""
    if n < 20:
        return ONES[n]
    if n < 100:
        return TENS[n // 10] + (ONES[n % 10] if n % 10 else "")
    if n < 1000:
        rem = n % 100
        return ONES[n // 100] + "hundred" + ("" if rem == 0 else "and" + number_to_words(rem))
    if n == 1000:
        return "onethousand"
    return ""


def main():
    total = sum(len(number_to_words(i)) for i in range(1, 1001))
    print(total)


if __name__ == "__main__":
    main()
