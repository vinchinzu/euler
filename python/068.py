#!/usr/bin/env python3
"""
Project Euler Problem 68: Magic 5-gon ring

Find the maximum 16-digit string for a "magic" 5-gon ring formed with numbers 1 to 10.
"""

import itertools


def max_magic_5gon_ring():
    """Find the maximum 16-digit string for a magic 5-gon ring."""
    max_found_string = ""

    # Iterate through all permutations of numbers 1 to 10
    # Permutation: [outer_nodes[0], outer_nodes[1], outer_nodes[2], outer_nodes[3], outer_nodes[4],
    #            inner_nodes[0], inner_nodes[1], inner_nodes[2], inner_nodes[3], inner_nodes[4]]
    for p in itertools.permutations(range(1, 11)):
        # p[0]..p[4] are outer nodes, p[5]..p[9] are inner nodes
        outer_nodes = list(p[:5])
        inner_nodes = list(p[5:])

        # Optimization: 10 must be an outer node for 16-digit string
        if 10 not in outer_nodes:
            continue

        # Define the 5 lines of the 5-gon ring:
        # Each line: outer_node, inner_node[i], inner_node[(i+1)%5]
        # Line 0: outer_nodes[0], inner_nodes[0], inner_nodes[1]
        # Line 1: outer_nodes[1], inner_nodes[1], inner_nodes[2]
        # Line 2: outer_nodes[2], inner_nodes[2], inner_nodes[3]
        # Line 3: outer_nodes[3], inner_nodes[3], inner_nodes[4]
        # Line 4: outer_nodes[4], inner_nodes[4], inner_nodes[0]

        line_triplets = [
            [outer_nodes[0], inner_nodes[0], inner_nodes[1]],
            [outer_nodes[1], inner_nodes[1], inner_nodes[2]],
            [outer_nodes[2], inner_nodes[2], inner_nodes[3]],
            [outer_nodes[3], inner_nodes[3], inner_nodes[4]],
            [outer_nodes[4], inner_nodes[4], inner_nodes[0]]
        ]

        # Calculate sum of first line (this will be our target sum)
        target_sum = outer_nodes[0] + inner_nodes[0] + inner_nodes[1]

        # Check if all other lines have same sum
        is_magic = (
            (outer_nodes[1] + inner_nodes[1] + inner_nodes[2] == target_sum) and
            (outer_nodes[2] + inner_nodes[2] + inner_nodes[3] == target_sum) and
            (outer_nodes[3] + inner_nodes[3] + inner_nodes[4] == target_sum) and
            (outer_nodes[4] + inner_nodes[4] + inner_nodes[0] == target_sum)
        )

        if not is_magic:
            continue

        # Find the starting line: one with the numerically lowest external node value
        min_outer_node_value = min(outer_nodes)
        start_node_index = outer_nodes.index(min_outer_node_value)

        # Construct candidate string by concatenating numbers from triplets,
        # starting from the determined start_node_index and proceeding clockwise
        candidate_string = ""
        for i in range(5):
            current_line_index = (start_node_index + i) % 5
            triplet = line_triplets[current_line_index]
            candidate_string += str(triplet[0]) + str(triplet[1]) + str(triplet[2])

        # We are looking for a 16-digit string
        if len(candidate_string) == 16:
            if max_found_string == "" or candidate_string > max_found_string:
                max_found_string = candidate_string

    return max_found_string


def main():
    result = max_magic_5gon_ring()
    print(result)


if __name__ == "__main__":
    main()
