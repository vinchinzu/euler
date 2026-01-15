#!/usr/bin/env python3
"""
Project Euler Problem 79: Passcode Derivation

A common security method used for online banking is to ask the user
for three random characters from a passcode. Analyse the keylog file to find the
shortest possible secret passcode of unknown length N.
"""

def main():
    """Read keylog data and find shortest passcode."""
    from pathlib import Path
    script_dir = Path(__file__).parent
    data_file = script_dir.parent / 'data' / 'keylog.txt'
    
    # Read keylog data
    with open(data_file) as f:
        lines = f.read().split('\n')

    # Build graph representation
    nodes = set()
    adj = {}
    in_degree = {}

    for attempt in lines:
        if not attempt:
            continue

        d1, d2, d3 = attempt[0], attempt[1], attempt[2]

        # Add characters to set of nodes
        nodes.add(d1)
        nodes.add(d2)
        nodes.add(d3)

        # Add edges and update in-degrees
        # Edge d1 -> d2
        if d1 not in adj:
            adj[d1] = set()
        if d2 not in adj[d1]:
            adj[d1].add(d2)
            in_degree[d2] = in_degree.get(d2, 0) + 1

        # Edge d2 -> d3
        if d2 not in adj:
            adj[d2] = set()
        if d3 not in adj[d2]:
            adj[d2].add(d3)
            in_degree[d3] = in_degree.get(d3, 0) + 1

    # Initialize in-degree for all nodes
    for node in nodes:
        in_degree.setdefault(node, 0)

    # Topological Sort (Kahn's Algorithm)
    queue = [node for node in nodes if in_degree[node] == 0]
    queue.sort()

    passcode_chars = []
    while queue:
        u = queue.pop(0)
        passcode_chars.append(u)

        # Sort neighbors to process them in a deterministic order
        for v in sorted(adj.get(u, set())):
            in_degree[v] -= 1
            if in_degree[v] == 0:
                queue.append(v)
        queue.sort()

    if len(passcode_chars) == len(nodes):
        passcode = ''.join(passcode_chars)
        print(passcode)
    else:
        print("Error: Could not determine passcode (possible cycle in graph or missing data).")


if __name__ == "__main__":
    main()
