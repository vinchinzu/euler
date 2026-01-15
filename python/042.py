#!/usr/bin/env python3
"""
Project Euler Problem 42: Coded triangle numbers

Using words.txt, a 16K text file containing nearly two-thousand common English words,
how many are triangle words?
"""

from math import isqrt


def word_value(word: str) -> int:
    """Calculate the alphabetical value of a word."""
    return sum(ord(c) - ord('A') + 1 for c in word.upper() if 'A' <= c <= 'Z')


def is_triangle(n: int) -> bool:
    """Check if n is a triangular number."""
    d = 1 + 8 * n
    root = isqrt(d)
    return root * root == d


def main():
    from pathlib import Path
    script_dir = Path(__file__).parent
    data_file = script_dir.parent / 'data' / 'words.txt'
    
    with open(data_file, 'r') as f:
        content = f.read()
    
    # Parse CSV format: "WORD1","WORD2",...
    words = content.replace('"', '').split(',')
    
    count = sum(1 for word in words if is_triangle(word_value(word)))
    print(count)


if __name__ == "__main__":
    main()
