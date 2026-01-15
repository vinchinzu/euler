"""Project Euler Problem 105: Special Sum Sets.

Let S(A) represent the sum of elements in set A of size n. We shall call it
a special sum set if for any two non-empty disjoint subsets, B and C, the
following properties are true:
1. S(B) ≠ S(C); that is, sums of subsets cannot be equal.
2. If B contains more elements than C then S(B) > S(C).
Using sets.txt (a 4K text file with one-hundred sets containing seven to twelve
elements), identify all the special sum sets, A1, A2, …, Ak, and find the value
of S(A1) + S(A2) + ⋯ + S(Ak).
"""

import os
from pathlib import Path
from typing import List, Tuple


def parse_sets_from_file(filename: str) -> List[List[int]]:
    """Parse sets from file."""
    if not os.path.exists(filename):
        return []

    sets = []
    try:
        with open(filename, 'r') as f:
            for line in f:
                values = line.strip()
                if not values:
                    continue

                numbers = []
                for token in values.split(','):
                    try:
                        numbers.append(int(token))
                    except ValueError:
                        break
                else:
                    sets.append(sorted(numbers))
    except FileNotFoundError:
        return []

    return sets


def generate_subset_info(set_list: List[int]) -> List[Tuple[int, int, int]]:
    """Generate all non-empty subsets using bitmasks (efficient for small n ≤ 12).
    
    Returns array of [mask, sum, size] tuples.
    """
    n = len(set_list)
    subsets = []

    for mask in range(1, 1 << n):  # Skip empty subset (mask 0)
        sum_val = 0
        size = 0

        # Calculate sum and size for this mask
        for i in range(n):
            if (mask & (1 << i)) != 0:
                sum_val += set_list[i]
                size += 1

        subsets.append((mask, sum_val, size))

    return subsets


def is_special_sum_set(set_list: List[int]) -> bool:
    """Check if a set is a special sum set."""
    if not set_list:
        return False

    subsets = generate_subset_info(set_list)

    # Use pairwise comparison to enforce both special sum conditions
    for idx in range(len(subsets)):
        mask1, sum1, size1 = subsets[idx]

        for j in range(idx + 1, len(subsets)):
            mask2, sum2, size2 = subsets[j]
            if (mask1 & mask2) != 0:
                continue

            if sum1 == sum2:
                return False

            if size1 > size2:
                if sum1 <= sum2:
                    return False
            elif size2 > size1:
                if sum2 <= sum1:
                    return False

    return True


def main(filename: str = None) -> int:
    """Main function."""
    if filename is None:
        # Try to find sets.txt in common locations
        script_dir = Path(__file__).parent
        possible_paths = [
            script_dir.parent / 'data' / 'sets.txt',
            script_dir.parent / 'solutions' / 'sets.txt',
            script_dir / 'sets.txt',
        ]
        filename = None
        for path in possible_paths:
            if path.exists():
                filename = str(path)
                break

        if filename is None:
            return 0

    sets = parse_sets_from_file(filename)
    return sum(sum(set_list) if is_special_sum_set(set_list) else 0 for set_list in sets)


if __name__ == "__main__":
    print(main())
