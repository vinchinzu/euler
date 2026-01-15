#!/usr/bin/env python3
"""
Anagramic squares (Problem 98)

Find the largest square number formed by any member of a square anagram word pair.
"""

import math
from typing import Dict, List, Set


def main() -> None:
    """Find largest square anagram pair."""
    from pathlib import Path
    script_dir = Path(__file__).parent
    data_file = script_dir.parent / 'data' / 'words.txt'
    
    # Read words
    with open(data_file) as f:
        words = [w.strip().strip('"') for w in f.read().split(',')]
    
    # Determine maximum word length
    max_len = max(len(w) for w in words)
    
    # Precompute squares grouped by length
    squares_by_length: Dict[int, Set[int]] = {}
    n = 1
    while True:
        square = n * n
        s_str = str(square)
        if len(s_str) > max_len:
            break
        length = len(s_str)
        if length not in squares_by_length:
            squares_by_length[length] = set()
        squares_by_length[length].add(square)
        n += 1
    
    # Group words by sorted letters (signature) to find anagrams
    anagram_groups: Dict[str, List[str]] = {}
    for word in words:
        signature = ''.join(sorted(word))
        if signature not in anagram_groups:
            anagram_groups[signature] = []
        anagram_groups[signature].append(word)
    
    # Filter to groups with pairs
    anagram_groups = {k: v for k, v in anagram_groups.items() if len(v) > 1}
    
    max_found_square = 0
    
    for group in anagram_groups.values():
        word_len = len(group[0])
        candidate_squares = squares_by_length.get(word_len, set())
        if not candidate_squares:
            continue
        
        # Check all pairs in the group
        for i in range(len(group)):
            for j in range(i + 1, len(group)):
                word1, word2 = group[i], group[j]
                
                for square1_val in candidate_squares:
                    s1_chars = list(str(square1_val))
                    
                    # Build mapping from word1 to square1
                    mapping: Dict[str, str] = {}
                    possible_map = True
                    
                    for char_idx, char in enumerate(word1):
                        digit = s1_chars[char_idx]
                        if char in mapping:
                            if mapping[char] != digit:
                                possible_map = False
                                break
                        else:
                            mapping[char] = digit
                    
                    if not possible_map:
                        continue
                    
                    # Check: different letters must map to different digits
                    if len(set(mapping.values())) != len(mapping):
                        continue
                    
                    # Form number for word2 using mapping
                    s2_chars = [mapping[char] for char in word2]
                    s2_str = ''.join(s2_chars)
                    
                    if len(s2_str) != word_len:
                        continue
                    
                    if word_len > 1 and s2_str[0] == '0':
                        continue
                    
                    square2_val = int(s2_str)
                    
                    if square2_val in candidate_squares:
                        max_found_square = max(max_found_square, square1_val, square2_val)
    
    print(max_found_square)


if __name__ == "__main__":
    main()
