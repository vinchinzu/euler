"""Project Euler Problem 419: Look and say sequence.

Define the look and say sequence such that the first term is 1, and each
subsequent term is the description of the previous term. For example, the
second term is 11, because the first term is "one 1". The third term is 21,
because the second term is "two 1s". Find the number of 1s, 2s, and 3s, in
the Nth term.

By the Cosmological Theorem, every sequence decays into about 100 distinct
independent components. We first find these components and what each component
transitions into; a term can be split into two components if the first digit
of the second component never matches the last digit of the first component
(which will always be the same).

Finally, we use matrix exponentiation to compute how many of each component
the Nth term has. Then, we multiply by the number of 1s, 2s, and 3s in each
component to get the final values.
"""

from __future__ import annotations

from collections import defaultdict
from typing import DefaultDict, Dict, List


def matrix_multiply(
    a: List[List[int]], b: List[List[int]], mod: int
) -> List[List[int]]:
    """Multiply two matrices modulo mod."""
    n = len(a)
    result = [[0] * n for _ in range(n)]
    for i in range(n):
        for k in range(n):
            if a[i][k]:
                for j in range(n):
                    result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % mod
    return result


def matrix_power(matrix: List[List[int]], exp: int, mod: int) -> List[List[int]]:
    """Raise matrix to power exp modulo mod."""
    n = len(matrix)
    # Identity matrix
    result = [[0] * n for _ in range(n)]
    for i in range(n):
        result[i][i] = 1

    base = [row[:] for row in matrix]
    e = exp

    while e > 0:
        if e & 1:
            result = matrix_multiply(result, base, mod)
        base = matrix_multiply(base, base, mod)
        e >>= 1

    return result


def look_and_say(s: str) -> str:
    """Apply look-and-say transformation to string s."""
    if not s:
        return ""
    
    result = []
    last_index = -1
    for i in range(len(s)):
        if i == len(s) - 1 or s[i] != s[i + 1]:
            count = i - last_index
            result.append(str(count))
            result.append(s[i])
            last_index = i
    
    return "".join(result)


def can_split_at(s: str, i: int, L: int) -> bool:
    """Check if we can split string s at position i."""
    if i >= len(s):
        return True
    
    c = s[i - 1]
    next_str = s[i:]
    
    for _ in range(L):
        if len(next_str) > L:
            next_str = next_str[:L]
        if not next_str:
            return True
        if next_str[0] == c:
            return False
        next_str = look_and_say(next_str)
    
    return True


def find_component_transitions(
    s: str,
    index_map: Dict[str, int],
    transitions: DefaultDict[str, List[str]],
    L: int,
) -> None:
    """Recursively find component transitions."""
    if s in index_map:
        return
    
    index_map[s] = len(index_map)
    next_str = look_and_say(s)
    
    last_index = 0
    for i in range(1, len(next_str) + 1):
        if i == len(next_str) or can_split_at(next_str, i, L):
            part = next_str[last_index:i]
            if part not in index_map:
                find_component_transitions(part, index_map, transitions, L)
            transitions[s].append(part)
            last_index = i


def solve() -> str:
    """Solve Problem 419."""
    N = 10**12
    L = 10
    M = 2**30
    
    index_map: Dict[str, int] = {}
    transitions: DefaultDict[str, List[str]] = defaultdict(list)
    
    start = "1"
    find_component_transitions(start, index_map, transitions, L)
    
    size = len(index_map)
    A = [[0] * size for _ in range(size)]
    
    for from_str, to_list in transitions.items():
        from_idx = index_map[from_str]
        for to_str in to_list:
            to_idx = index_map[to_str]
            A[to_idx][from_idx] += 1
    
    An = matrix_power(A, N - 1, M)
    
    num_digits: DefaultDict[str, int] = defaultdict(int)
    for s, idx in index_map.items():
        count = An[idx][0]
        for c in s:
            if c in "123":
                num_digits[c] = (num_digits[c] + count) % M
    
    ans = f"{num_digits['1']},{num_digits['2']},{num_digits['3']}"
    return ans


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
