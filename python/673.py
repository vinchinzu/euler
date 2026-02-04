"""Project Euler Problem 673: Beds and Desks."""

from __future__ import annotations

from collections import Counter
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Component:
    """Component of the graph."""
    num_students: int
    num_bed_pairings: int
    num_desk_pairings: int


def read_graph(file_path: Path) -> dict[int, int]:
    """Read graph from file."""
    graph: dict[int, int] = {}
    with open(file_path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            parts = line.split(",")
            v1 = int(parts[0]) - 1
            v2 = int(parts[1]) - 1
            graph[v1] = v2
            graph[v2] = v1
    return graph


def mod_pow(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    return pow(base, exp, mod)


def factorial_mod(n: int, mod: int) -> int:
    """Compute n! mod mod."""
    result = 1
    for i in range(1, n + 1):
        result = (result * i) % mod
    return result


def solve() -> int:
    """Solve Problem 673."""
    N = 500
    M = 999999937

    script_dir = Path(__file__).parent
    beds = read_graph(script_dir / "0673_beds.txt")
    desks = read_graph(script_dir / "0673_desks.txt")

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
        ans = (ans * factorial_mod(count, M)) % M

    return ans


if __name__ == "__main__":
    print(solve())
