"""Project Euler Problem 673: Beds and Desks.

Among N students, some pairs share a bed and some pairs share a desk. Find the
number of permutations of students such that the same pairs of students share a
bed and share a desk.

Consider a graph of the N students connected by both bed-edges and desk-edges.
The graph can be split into connected components, and each component must be one
of three types:
- A cycle of k students, for even k
- A chain of k students, for even k
- A chain of k students, for odd k

Each component of the first type can be permuted k different ways, because a
student can be permuted anywhere in the cycle, and the remaining students are
then fixed. A component of the second type can be permuted in 2 different ways:
either the original order or its reverse. A component of the last type cannot
be permuted in any other way.

Finally, if there are r components of the same type, then those components can
be permuted with each other in r! different ways. Multiplying all these
possibilities gives the answer.
"""

from __future__ import annotations

from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Component:
    """Component of the graph."""

    num_students: int
    num_bed_pairings: int
    num_desk_pairings: int


def read_graph(suffix: str) -> dict[int, int]:
    """Read graph from file."""
    script_dir = Path(__file__).parent
    file_path = script_dir.parent / "kevinychen-project-euler" / "files" / f"p673_{suffix}.txt"
    
    graph: dict[int, int] = {}
    if not file_path.exists():
        return graph
    
    with open(file_path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            parts = line.split(",")
            if len(parts) >= 2:
                v1 = int(parts[0]) - 1
                v2 = int(parts[1]) - 1
                graph[v1] = v2
                graph[v2] = v1
    
    return graph


def mod_pow(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def factorial(n: int, mod: int) -> int:
    """Compute n! mod mod."""
    result = 1
    for i in range(1, n + 1):
        result = (result * i) % mod
    return result


def solve() -> int:
    """Solve Problem 673."""
    N = 500
    M = 999999937

    beds = read_graph("beds")
    desks = read_graph("desks")

    visited = [False] * N
    components: list[Component] = []
    
    for i in range(N):
        if not visited[i]:
            num_students = 0
            num_bed_pairings = 0
            num_desk_pairings = 0
            dfs = [i]
            
            while dfs:
                v = dfs.pop()
                if not visited[v]:
                    visited[v] = True
                    num_students += 1
                    if v in beds:
                        dfs.append(beds[v])
                        num_bed_pairings += 1
                    if v in desks:
                        dfs.append(desks[v])
                        num_desk_pairings += 1
            
            components.append(
                Component(
                    num_students,
                    num_bed_pairings // 2,
                    num_desk_pairings // 2,
                )
            )

    component_counts = Counter(components)
    ans = 1
    
    for component, count in component_counts.items():
        if component.num_bed_pairings + component.num_desk_pairings == component.num_students:
            ans = (ans * mod_pow(component.num_students, count, M)) % M
        elif component.num_students % 2 == 0:
            ans = (ans * mod_pow(2, count, M)) % M
        ans = (ans * factorial(count, M)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
