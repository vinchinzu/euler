#!/usr/bin/env python3
"""
Project Euler Problem 61: Cyclical figurate numbers

Find the sum of the only ordered set of six cyclic 4-digit numbers for which each
polygonal type: triangle, square, pentagonal, hexagonal, heptagonal, and octagonal,
is represented by a different number in the set.
"""

from typing import List


class CyclicalFigurateNumbersSolver:
    def __init__(self):
        # Formulas for P(k,n)
        # Index 0: Triangle (P3), 1: Square (P4), ..., 5: Octagonal (P8)
        self.polygonal_formulas = [
            lambda n: n * (n + 1) // 2,    # P3
            lambda n: n * n,                # P4
            lambda n: n * (3 * n - 1) // 2,  # P5
            lambda n: n * (2 * n - 1),      # P6
            lambda n: n * (5 * n - 3) // 2,  # P7
            lambda n: n * (3 * n - 2)       # P8
        ]

        # Ranges for n to generate 4-digit numbers (1000-9999)
        self.n_ranges = [
            (45, 140),  # P3: n=45 (1035) to n=140 (9870)
            (32, 99),   # P4: n=32 (1024) to n=99 (9801)
            (26, 81),   # P5: n=26 (1001) to n=81 (9821)
            (23, 70),   # P6: n=23 (1035) to n=70 (9730)
            (21, 63),   # P7: n=21 (1071) to n=63 (9633)
            (19, 58)    # P8: n=19 (988) to n=58 (9976)
        ]

        # @numbers_by_type_and_prefix[type_idx][prefix_val] = [list of full numbers]
        # Prefix values are 10-99 (first two digits of 4-digit numbers)
        self.numbers_by_type_and_prefix = [[[] for _ in range(100)] for _ in range(6)]
        # @all_numbers_of_type[type_idx] = [list of full numbers]
        self.all_numbers_of_type = [[] for _ in range(6)]

        self._generate_polygonal_numbers()
        self.solution_chain = None

    def _generate_polygonal_numbers(self):
        """Generates all 4-digit polygonal numbers and stores them for quick lookup."""
        # Generate all 4-digit polygonal numbers
        for type_idx in range(6):
            formula = self.polygonal_formulas[type_idx]
            start_n, end_n = self.n_ranges[type_idx]

            for n in range(start_n, end_n + 1):
                val = formula(n)
                # We are only interested in 4-digit numbers
                if val >= 1000 and val <= 9999:
                    self.all_numbers_of_type[type_idx].append(val)
                    prefix = val // 100  # First two digits
                    self.numbers_by_type_and_prefix[type_idx][prefix].append(val)

    def _dfs_find_cycle(self, chain, used_types_mask):
        """Recursive DFS to find a cycle of 6 numbers."""
        # If a solution is already found, stop searching
        if self.solution_chain is not None:
            return True

        current_length = len(chain)
        last_number_in_chain = chain[-1]

        if current_length == 6:
            # Chain of 6 numbers formed. Check if it's cyclic with first number.
            first_number_in_chain = chain[0]
            # Last two digits of last number == First two digits of first number
            if (last_number_in_chain % 100) == (first_number_in_chain // 100):
                self.solution_chain = list(chain)  # Found solution
                return True
            else:
                return False  # Not cyclic

        # Determine required prefix for next number in chain
        required_prefix = last_number_in_chain % 100

        # Try to add a number from each unused polygonal type
        for next_type_idx in range(6):
            # Check if this type_idx is NOT already used
            if (used_types_mask & (1 << next_type_idx)) == 0:
                # Get candidate numbers of this type that match required prefix
                candidate_numbers = self.numbers_by_type_and_prefix[next_type_idx][required_prefix]

                for candidate_num in candidate_numbers:
                    chain.append(candidate_num)
                    # Recursively search with new number and updated mask
                    if self._dfs_find_cycle(chain, used_types_mask | (1 << next_type_idx)):
                        return True  # Solution found and propagated up

                    chain.pop()  # Backtrack: remove number if this path didn't lead to a solution

        return False  # No solution found from this path

    def solve(self):
        """Solves problem and returns sum of numbers in unique cyclic set."""
        # Iterate through each polygonal type to be type of first number in chain
        for start_type_idx in range(6):
            # Iterate through each number of that starting type
            for start_num in self.all_numbers_of_type[start_type_idx]:
                # Start DFS with first number and its type marked as used
                if self._dfs_find_cycle([start_num], (1 << start_type_idx)):
                    # Solution is found and stored in @solution_chain
                    return sum(self.solution_chain) if self.solution_chain else None

        return None  # Should not happen based on problem statement


def main():
    solver = CyclicalFigurateNumbersSolver()
    result = solver.solve()
    print(result if result else "No solution found")


if __name__ == "__main__":
    main()
