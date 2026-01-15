#!/usr/bin/env python3
"""
Project Euler Problem 22: Name scores

Using data/names.txt, a 46K text file containing over five-thousand first names,
begin by sorting it into alphabetical order. Then working out the alphabetical value
for each name, multiply this value by its alphabetical position in the list to obtain a name score.

For example, when the list is sorted into alphabetical order, COLIN, which is worth
3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a
score of 938 × 53 = 49714.

What is the total of all the name scores in the file?
"""

def name_value(name: str) -> int:
    """Calculate the alphabetical value of a name."""
    return sum(ord(c) - ord('A') + 1 for c in name)


def main():
    from pathlib import Path
    script_dir = Path(__file__).parent
    data_file = script_dir.parent / 'data' / 'names.txt'
    
    with open(data_file, 'r') as f:
        content = f.read()
    
    names = content.replace('"', '').split(',')
    names.sort()
    
    total = sum(name_value(name) * (i + 1) for i, name in enumerate(names))
    print(total)


if __name__ == "__main__":
    main()
