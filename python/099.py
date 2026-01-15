#!/usr/bin/env python3
"""
Largest exponential (Problem 99)

Find the line number with the largest exponential value.
"""

import math


def main() -> None:
    """Find line with largest exponential."""
    from pathlib import Path
    script_dir = Path(__file__).parent
    data_file = script_dir.parent / 'data' / '0099_base_exp.txt'
    
    with open(data_file) as f:
        lines = f.readlines()
    
    max_line = 0
    max_score = float('-inf')
    
    for index, line in enumerate(lines):
        base, exponent = map(int, line.strip().split(','))
        score = exponent * math.log(base)
        if score > max_score:
            max_score = score
            max_line = index + 1
    
    print(max_line)


if __name__ == "__main__":
    main()
