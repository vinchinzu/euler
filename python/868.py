"""
Project Euler Problem 868

PROBLEM DESCRIPTION:
There is a method that is used by Bell ringers to generate all variations of the order that bells are rung.

The same method can be used to create all permutations of a set of letters. Consider the letters to be permuted initially in order from smallest to largest. At each step swap the largest letter with the letter on its left or right whichever generates a permutation that has not yet been seen. If neither gives a new permutation then try the next largest letter and so on. This procedure continues until all permutations have been generated.

For example, 3 swaps are required to reach the permutation CBA when starting with ABC.
The swaps are ABC -> ACB -> CAB -> CBA.
Also 59 swaps are required to reach BELFRY when starting with these letters in alphabetical order.

Find the number of swaps that are required to reach NOWPICKBELFRYMATHS when starting with these letters in alphabetical order.
"""
from __future__ import annotations
import sys

def sjt_rank(perm: list[int]) -> int:
    """
    Computes the rank (0-based index) of a permutation in the Steinhaus-Johnson-Trotter sequence.
    The permutation must be a list of integers from 0 to n-1.
    """
    n = len(perm)
    if n <= 1:
        return 0
    
    # The largest element in a permutation of 0..n-1 is n-1
    largest = n - 1
    try:
        pos = perm.index(largest)
    except ValueError:
        raise ValueError(f"Permutation must contain {largest}")

    # Create sub-permutation without largest
    # The remaining elements are 0..n-2, preserving relative order
    sub_perm = [x for x in perm if x != largest]
    
    # Recursively get rank of sub-permutation
    r_sub = sjt_rank(sub_perm)
    
    # Calculate local index
    if r_sub % 2 == 0:
        # Even rank: Largest element moves Right-to-Left (indices n-1 down to 0)
        # positions: n-1, n-2, ..., 0
        # rank contribution: (n-1) - pos
        local_index = (n - 1) - pos
    else:
        # Odd rank: Largest element moves Left-to-Right (indices 0 up to n-1)
        # positions: 0, 1, ..., n-1
        # rank contribution: pos
        local_index = pos
        
    return r_sub * n + local_index

def solve() -> int:
    target_str = "NOWPICKBELFRYMATHS"
    
    sorted_chars = sorted(list(target_str))
    
    # Map characters to 0..n-1 based on sorted order
    char_map = {char: i for i, char in enumerate(sorted_chars)}
    
    # Convert target string to list of ranks
    perm = [char_map[c] for c in target_str]
    
    return sjt_rank(perm)

if __name__ == "__main__":
    print(solve())
