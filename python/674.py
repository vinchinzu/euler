"""Project Euler Problem 674: Solving I-equations.

Define I(x,y) = (1+x+y)²+y-x. Given a list of I-expressions, which are either
variables or expressions of the form I(x,y) where x and y are also I-expressions,
find the least simultaneous value of all pairs of I-expressions (e1,e2), defined
as the minimum value that can be attained where e1=e2 and all variables are
non-negative integers.

Given an equality of I-expressions, we can simplify by repeatedly doing the
following:

- If I(a,b)=I(c,d), we can replace with a=c and b=d (I(x,y) is always distinct
  on the non-negative integers)
- Otherwise, all equations are of the form a=I(b,c). Find an equation such that
  the variable on the left hand side does not appear on the right hand side of
  any equation. (If there are none, the equations are unsolvable because I(a,b)
  is strictly larger than both a and b.)
- If the variable only appears once, remove it: we can evaluate it once all
  remaining variables are solved for.
- Otherwise, we add equations for the right hand sides being equal to one
  another, and repeat.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Set


M = 10**9


@dataclass(frozen=True)
class IAndVar:
    """I-expression and variable pair."""

    i: "I"
    var: str


@dataclass
class I:
    """I-expression."""

    var: str | None
    left: "I | None"
    right: "I | None"
    vars: Set[str]

    def __init__(
        self,
        var: str | None = None,
        left: "I | None" = None,
        right: "I | None" = None,
    ) -> None:
        """Initialize I-expression."""
        self.var = var
        self.left = left
        self.right = right
        if var is not None:
            self.vars = {var}
        elif left is not None and right is not None:
            self.vars = left.vars | right.vars
        else:
            self.vars = set()

    def evaluate(self, values: dict[str, int]) -> int:
        """Evaluate I-expression with given variable values."""
        if self.var is not None:
            return values.get(self.var, 0)
        if self.left is None or self.right is None:
            return 0
        x = self.left.evaluate(values)
        y = self.right.evaluate(values)
        return ((1 + x + y) ** 2 + y - x) % M

    @staticmethod
    def parse(equation: str) -> "I":
        """Parse I-expression from string."""
        return IAndIndex.parse(equation, 0).i


@dataclass
class IAndIndex:
    """I-expression and parse index."""

    i: I
    index: int

    @staticmethod
    def parse(equation: str, index: int) -> "IAndIndex":
        """Parse I-expression starting at index."""
        if index < len(equation) and equation[index] == "I":
            left_result = IAndIndex.parse(equation, index + 2)
            right_result = IAndIndex.parse(equation, left_result.index + 1)
            return IAndIndex(
                I(left=left_result.i, right=right_result.i),
                right_result.index + 1,
            )
        new_index = index
        while new_index < len(equation) and equation[new_index].islower():
            new_index += 1
        var = equation[index:new_index]
        return IAndIndex(I(var=var), new_index)


def least_simultaneous_value(e1: I, e2: I) -> int:
    """Find least simultaneous value of two I-expressions."""
    equalities: Set[IAndVar] = set()
    helper(e1, e2, equalities)

    evaluations: list[IAndVar] = []
    while equalities:
        # Find an equality where the variable doesn't appear on RHS of others
        good_equality = None
        for equality in equalities:
            if not any(
                other.i.vars.__contains__(equality.var) for other in equalities
            ):
                good_equality = equality
                break
        
        if good_equality is None:
            return 0
        
        # Collect all equalities with this variable
        good_equalities = [
            other for other in equalities if other.var == good_equality.var
        ]
        evaluations.append(good_equalities[0])
        equalities -= set(good_equalities)
        
        # Add new equalities for remaining pairs
        for i in range(1, len(good_equalities)):
            helper(good_equalities[0].i, good_equalities[i].i, equalities)

    evaluations.reverse()

    values: dict[str, int] = {}
    all_vars = e1.vars | e2.vars
    for var in all_vars:
        if not any(eval_item.var == var for eval_item in evaluations):
            values[var] = 0
    
    for evaluation in evaluations:
        values[evaluation.var] = evaluation.i.evaluate(values)
    
    return e1.evaluate(values)


def helper(e1: I, e2: I, equalities: Set[IAndVar]) -> None:
    """Helper function to build equalities."""
    if e1.var is not None and e2.var is not None:
        if e1.var < e2.var:
            equalities.add(IAndVar(e1, e2.var))
        if e1.var > e2.var:
            equalities.add(IAndVar(e2, e1.var))
    elif e1.var is not None:
        equalities.add(IAndVar(e2, e1.var))
    elif e2.var is not None:
        equalities.add(IAndVar(e1, e2.var))
    else:
        if e1.left is not None and e1.right is not None:
            if e2.left is not None and e2.right is not None:
                helper(e1.left, e2.left, equalities)
                helper(e1.right, e2.right, equalities)


def solve() -> int:
    """Solve Problem 674."""
    script_dir = Path(__file__).parent
    file_path = script_dir.parent / "kevinychen-project-euler" / "files" / "p674.txt"
    
    equations: list[I] = []
    if file_path.exists():
        with open(file_path) as f:
            for line in f:
                line = line.strip()
                if line:
                    equations.append(I.parse(line))
    else:
        # If file doesn't exist, return 0 (or handle appropriately)
        return 0

    ans = 0
    for i in range(len(equations)):
        for j in range(i + 1, len(equations)):
            ans = (ans + least_simultaneous_value(equations[i], equations[j])) % M
    
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
