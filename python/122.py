"""Project Euler Problem 122: Efficient Exponentiation.

Find the sum of m(k) for k=1 to 200, where m(k) is the minimum number of 
multiplications to compute n^k.
"""

from typing import List, Set, Tuple

LIMIT = 200
# MAX_COST_TO_PROCESS is the maximum cost of a state whose children will be explored.
# Based on known values, m(191) = 11 is the max m(k) for k <= 200.
# Children of states with cost=11 (i.e., new_cost=12) are generated, and their m(k) values updated.
# Children of states with cost=12 (i.e., new_cost=13) are not generated.
MAX_COST_TO_PROCESS = 11


def main() -> int:
    """Main function using BFS to find shortest addition chains."""
    # m[k] stores the minimum number of multiplications for n^k
    # m[0] is unused. m[1] to m[LIMIT] are used.
    m: List[float] = [float('inf')] * (LIMIT + 1)
    m[1] = 0  # Base case: n^1 requires 0 multiplications

    # Initial state for BFS:
    # The chain {1} is achieved with 0 multiplications.
    initial_chain_set: Set[int] = {1}

    # For the visited_states set, we need a hashable representation of the chain set.
    # A frozen sorted tuple of elements serves this purpose.
    initial_chain_repr = tuple(sorted(initial_chain_set))  # Store sorted tuple

    # Queue stores [Set_object_for_chain, cost]
    queue: List[Tuple[Set[int], int]] = [(initial_chain_set, 0)]
    head = 0  # Simulating queue behavior with an array for BFS

    # visited_states stores [frozen_tuple_representation_of_chain, cost]
    # This set prevents redundant exploration of identical states (same chain elements and cost).
    visited_states: Set[Tuple[Tuple[int, ...], int]] = {(initial_chain_repr, 0)}

    while head < len(queue):
        current_chain_set, cost = queue[head]
        head += 1

        # If the current state's cost is too high, don't explore its children
        if cost > MAX_COST_TO_PROCESS:
            continue

        # Iterate over all pairs (x,y) from the current chain to form new exponents
        for x in current_chain_set:
            for y in current_chain_set:
                new_val = x + y
                if new_val > LIMIT:  # We only care about exponents up to LIMIT
                    continue

                new_cost = cost + 1

                # If this path offers a better way to new_val, OR an equally good way
                # (which might lead to different subsequent optimal paths for other numbers)
                if new_cost <= m[new_val]:
                    # Update m[new_val] if this path is strictly shorter
                    if new_cost < m[new_val]:
                        m[new_val] = new_cost

                    # new_elements_for_path is the set of numbers available after this multiplication step
                    new_elements_for_path = current_chain_set.copy()
                    new_elements_for_path.add(new_val)

                    # Create a hashable representation for the visited_states set
                    new_chain_repr = tuple(sorted(new_elements_for_path))
                    state_key_for_visited = (new_chain_repr, new_cost)

                    # If this specific state (chain elements and cost) hasn't been visited,
                    # add it to the queue and mark as visited.
                    if state_key_for_visited not in visited_states:
                        visited_states.add(state_key_for_visited)
                        queue.append((new_elements_for_path, new_cost))

    # Sum m[k] for k=1 to LIMIT
    return sum(int(m[k]) for k in range(1, LIMIT + 1))


if __name__ == "__main__":
    print(main())
