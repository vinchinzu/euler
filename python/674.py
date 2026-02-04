"""Project Euler Problem 674: Solving I-equations."""

from __future__ import annotations

from pathlib import Path


M = 10**9


class I:
    """I-expression: either a variable or I(left, right)."""

    __slots__ = ('var', 'left', 'right', 'vars', '_hash')

    def __init__(self, var=None, left=None, right=None):
        self.var = var
        self.left = left
        self.right = right
        if var is not None:
            self.vars = frozenset([var])
        elif left is not None and right is not None:
            self.vars = left.vars | right.vars
        else:
            self.vars = frozenset()
        self._hash = hash((self.var, id(self.left), id(self.right)))

    def __eq__(self, other):
        if not isinstance(other, I):
            return NotImplemented
        if self.var is not None and other.var is not None:
            return self.var == other.var
        if self.var is not None or other.var is not None:
            return False
        return self.left is other.left and self.right is other.right

    def __hash__(self):
        return self._hash

    def evaluate(self, values):
        if self.var is not None:
            return values[self.var]
        x = self.left.evaluate(values)
        y = self.right.evaluate(values)
        return ((1 + x + y) ** 2 + y - x) % M

    @staticmethod
    def parse(equation):
        result = _parse(equation, 0)
        return result[0]


def _parse(equation, index):
    """Parse I-expression starting at index. Returns (I, next_index)."""
    if index < len(equation) and equation[index] == 'I':
        left_i, left_idx = _parse(equation, index + 2)
        right_i, right_idx = _parse(equation, left_idx + 1)
        return I(left=left_i, right=right_i), right_idx + 1
    new_index = index
    while new_index < len(equation) and equation[new_index].islower():
        new_index += 1
    var = equation[index:new_index]
    return I(var=var), new_index


class IAndVar:
    """Pair of I-expression and variable name."""

    __slots__ = ('i', 'var', '_hash')

    def __init__(self, i, var):
        self.i = i
        self.var = var
        self._hash = hash((id(i), var))

    def __eq__(self, other):
        if not isinstance(other, IAndVar):
            return NotImplemented
        return self.i is other.i and self.var == other.var

    def __hash__(self):
        return self._hash


def least_simultaneous_value(e1, e2):
    """Find least simultaneous value of two I-expressions."""
    equalities = set()
    helper(e1, e2, equalities)

    evaluations = []
    while equalities:
        # Find an equality where the variable doesn't appear on RHS of any other
        good_equality = None
        for equality in equalities:
            found = False
            for other in equalities:
                if equality.var in other.i.vars:
                    found = True
                    break
            if not found:
                good_equality = equality
                break

        if good_equality is None:
            return 0

        # Collect all equalities with this variable
        good_equalities = [other for other in equalities if other.var == good_equality.var]
        evaluations.append(good_equalities[0])
        for ge in good_equalities:
            equalities.discard(ge)

        # Add new equalities for remaining pairs
        for k in range(1, len(good_equalities)):
            helper(good_equalities[0].i, good_equalities[k].i, equalities)

    evaluations.reverse()

    values = {}
    all_vars = e1.vars | e2.vars
    eval_vars = {ev.var for ev in evaluations}
    for var in all_vars:
        if var not in eval_vars:
            values[var] = 0

    for evaluation in evaluations:
        values[evaluation.var] = evaluation.i.evaluate(values)

    return e1.evaluate(values)


def helper(e1, e2, equalities):
    """Build equalities from two I-expressions."""
    if e1.var is not None and e2.var is not None:
        if e1.var < e2.var:
            equalities.add(IAndVar(e1, e2.var))
        elif e1.var > e2.var:
            equalities.add(IAndVar(e2, e1.var))
    elif e1.var is not None:
        equalities.add(IAndVar(e2, e1.var))
    elif e2.var is not None:
        equalities.add(IAndVar(e1, e2.var))
    else:
        helper(e1.left, e2.left, equalities)
        helper(e1.right, e2.right, equalities)


def solve():
    """Solve Problem 674."""
    script_dir = Path(__file__).parent
    file_path = script_dir / "0674_i_expressions.txt"

    equations = []
    with open(file_path) as f:
        for line in f:
            line = line.strip()
            if line:
                equations.append(I.parse(line))

    ans = 0
    for i in range(len(equations)):
        for j in range(i + 1, len(equations)):
            ans += least_simultaneous_value(equations[i], equations[j])
    ans %= M

    return ans


if __name__ == "__main__":
    print(solve())
