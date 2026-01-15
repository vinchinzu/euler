"""Project Euler Problem 424: Kakuro puzzles.

Solve cryptic Kakuro logic puzzles where each digit is replaced with one of
the letters from A to J. We use constraint programming with backtracking.

Constraints:
- The values of letters A to J must be distinct integers from 0 to 9.
- The value of each open box is a distinct integer from 1 to 9.
- Any open boxes that contain a letter must have a value equal to the letter.
- Every row and column must contain distinct integers.
- The first letter in an encrypted sum cannot be zero.
- The sum of values in a row or column equals the encrypted sum.
"""

from __future__ import annotations

import re
from dataclasses import dataclass
from typing import Dict, List, Optional, Set, Tuple


@dataclass(frozen=True)
class Dir:
    """Direction vector."""

    di: int
    dj: int

    def in_bounds(self, i: int, j: int, mult: int, size: int) -> bool:
        """Check if position is in bounds."""
        new_i = i + mult * self.di
        new_j = j + mult * self.dj
        return 0 <= new_i < size and 0 <= new_j < size


@dataclass(frozen=True)
class IPoint:
    """Integer point."""

    x: int
    y: int


class ConstraintSolver:
    """Simple backtracking constraint solver."""

    def __init__(self) -> None:
        """Initialize solver."""
        self.variables: Dict[str, Set[int]] = {}
        self.constraints: List[Tuple[List[str], List[int], int]] = []
        self.alldiff_groups: List[List[str]] = []
        self.equalities: List[Tuple[str, str]] = []
        self.inequalities: List[Tuple[str, int]] = []

    def add_variable(self, name: str, domain: Set[int]) -> None:
        """Add a variable with its domain."""
        self.variables[name] = domain.copy()

    def add_alldiff(self, var_names: List[str]) -> None:
        """Add all-different constraint."""
        self.alldiff_groups.append(var_names)

    def add_equality(self, var1: str, var2: str) -> None:
        """Add equality constraint."""
        self.equalities.append((var1, var2))

    def add_inequality(self, var: str, value: int) -> None:
        """Add inequality constraint."""
        self.inequalities.append((var, value))

    def add_weighted_sum(
        self, var_names: List[str], coefficients: List[int], target: int
    ) -> None:
        """Add weighted sum constraint."""
        self.constraints.append((var_names, coefficients, target))

    def solve(self) -> Optional[Dict[str, int]]:
        """Solve using backtracking."""
        assignment: Dict[str, int] = {}
        return self._backtrack(assignment)

    def _backtrack(self, assignment: Dict[str, int]) -> Optional[Dict[str, int]]:
        """Backtracking search."""
        if len(assignment) == len(self.variables):
            if self._check_all_constraints(assignment):
                return assignment
            return None

        # Select unassigned variable
        unassigned = [
            name for name in self.variables if name not in assignment
        ]
        if not unassigned:
            return None

        var = unassigned[0]
        domain = self.variables[var].copy()

        # Apply forward checking
        domain = self._forward_check(var, domain, assignment)

        for value in sorted(domain):
            assignment[var] = value
            if self._check_partial_constraints(assignment):
                result = self._backtrack(assignment)
                if result is not None:
                    return result
            del assignment[var]

        return None

    def _forward_check(
        self, var: str, domain: Set[int], assignment: Dict[str, int]
    ) -> Set[int]:
        """Forward checking to prune domain."""
        pruned = set()
        for value in domain:
            test_assignment = assignment.copy()
            test_assignment[var] = value
            if self._check_partial_constraints(test_assignment):
                pruned.add(value)
        return pruned

    def _check_partial_constraints(
        self, assignment: Dict[str, int]
    ) -> bool:
        """Check constraints that can be checked with partial assignment."""
        # Check equalities
        for var1, var2 in self.equalities:
            if var1 in assignment and var2 in assignment:
                if assignment[var1] != assignment[var2]:
                    return False

        # Check inequalities
        for var, value in self.inequalities:
            if var in assignment:
                if assignment[var] == value:
                    return False

        # Check alldiff groups
        for group in self.alldiff_groups:
            assigned_in_group = [
                assignment[var] for var in group if var in assignment
            ]
            if len(assigned_in_group) != len(set(assigned_in_group)):
                return False

        return True

    def _check_all_constraints(self, assignment: Dict[str, int]) -> bool:
        """Check all constraints."""
        if not self._check_partial_constraints(assignment):
            return False

        # Check weighted sum constraints
        for var_names, coefficients, target in self.constraints:
            total = 0
            for var, coeff in zip(var_names, coefficients):
                if var not in assignment:
                    return False
                total += assignment[var] * coeff
            if total != target:
                return False

        return True


def ipow(base: int, exp: int) -> int:
    """Integer power."""
    result = 1
    for _ in range(exp):
        result *= base
    return result


def parse_puzzle(line: str) -> Tuple[int, List[List[str]]]:
    """Parse puzzle line."""
    # Split by comma, but not inside parentheses
    parts = re.split(r",(?![^(]*\))", line)
    size = int(parts[0])
    grid: List[List[str]] = []
    for i in range(size):
        row: List[str] = []
        for j in range(size):
            row.append(parts[size * i + j + 1])
        grid.append(row)
    return size, grid


def solve_puzzle(puzzle_line: str) -> int:
    """Solve a single puzzle and return the letter values as integer."""
    size, grid = parse_puzzle(puzzle_line)
    B = 10

    solver = ConstraintSolver()

    # Create letter variables A-J (0-9)
    letter_vars: Dict[str, str] = {}
    for c in range(ord("A"), ord("A") + B):
        letter_name = f"letter_{chr(c)}"
        letter_vars[chr(c)] = letter_name
        solver.add_variable(letter_name, set(range(B)))

    # All letters must be distinct
    solver.add_alldiff(list(letter_vars.values()))

    # Create grid variables
    grid_vars: Dict[IPoint, str] = {}
    for i in range(size):
        for j in range(size):
            cell = grid[i][j]
            if len(cell) > 0 and cell[0] >= "A" and cell[0] <= "O":
                var_name = f"grid_{i}_{j}"
                grid_vars[IPoint(i, j)] = var_name
                solver.add_variable(var_name, set(range(1, B)))
                # If it's a letter A-J, add equality constraint
                if cell[0] in letter_vars:
                    solver.add_equality(var_name, letter_vars[cell[0]])

    # Process directed sums
    for i in range(size):
        for j in range(size):
            cell = grid[i][j]
            # Split by non-word characters but keep parentheses
            parts = re.split(r"\W+", cell)
            for part in parts:
                if not part:
                    continue
                if part.startswith("h"):
                    dir_obj = Dir(0, 1)
                elif part.startswith("v"):
                    dir_obj = Dir(1, 0)
                else:
                    continue

                # Collect variables in this direction
                variables: List[str] = []
                coefficients: List[int] = []
                mult = 1
                while dir_obj.in_bounds(i, j, mult, size):
                    point = IPoint(i + mult * dir_obj.di, j + mult * dir_obj.dj)
                    if point not in grid_vars:
                        break
                    variables.append(grid_vars[point])
                    coefficients.append(-1)
                    mult += 1

                if not variables:
                    continue

                # All variables in this sum must be distinct
                solver.add_alldiff(variables)

                # Process the encrypted sum
                sum_str = part[1:]
                for k, char in enumerate(sum_str):
                    if char in letter_vars:
                        variables.append(letter_vars[char])
                        power = ipow(B, len(sum_str) - 1 - k)
                        coefficients.append(power)

                # First letter cannot be zero
                if sum_str and sum_str[0] in letter_vars:
                    solver.add_inequality(letter_vars[sum_str[0]], 0)

                # Add weighted sum constraint (sum equals zero)
                solver.add_weighted_sum(variables, coefficients, 0)

    # Solve
    solution = solver.solve()
    if solution is None:
        raise ValueError("No solution found")

    # Extract letter values
    result_str = ""
    for c in range(ord("A"), ord("A") + B):
        letter_name = letter_vars[chr(c)]
        result_str += str(solution[letter_name])

    return int(result_str)


def solve() -> int:
    """Solve all puzzles and return sum."""
    total = 0
    with open("kevinychen-project-euler/files/p424.txt") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            total += solve_puzzle(line)
    return total


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
