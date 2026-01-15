#!/usr/bin/env python3
"""
Amicable chains (Problem 95)

Find the smallest member of the longest amicable chain with no element
exceeding one million.
"""

LIMIT = 1_000_000


def main() -> None:
    """Find longest amicable chain."""
    # Precompute sum of proper divisors
    sps = [0] * (LIMIT + 1)
    
    for i in range(1, LIMIT + 1):
        for j in range(2 * i, LIMIT + 1, i):
            sps[j] += i
    
    max_chain_len = 0
    smallest_member_of_longest_chain = None
    processed_info = {}
    
    for start_node in range(1, LIMIT + 1):
        if start_node in processed_info:
            continue
        
        current_path = []
        path_set = {}
        
        current_num = start_node
        
        while True:
            # Termination: out of bounds
            if current_num == 0 or current_num > LIMIT:
                for num_in_path in current_path:
                    if num_in_path not in processed_info:
                        processed_info[num_in_path] = {'len': 0}
                break
            
            # Termination: leads to already analyzed segment
            if current_num in processed_info:
                for num_in_path in current_path:
                    if num_in_path not in processed_info:
                        processed_info[num_in_path] = {'len': 0}
                break
            
            # Cycle detection
            if current_num in path_set:
                cycle_start_index = path_set[current_num]
                cycle_nodes = current_path[cycle_start_index:]
                cycle_len = len(cycle_nodes)
                
                current_cycle_min_member = min(cycle_nodes)
                
                if cycle_len > max_chain_len:
                    max_chain_len = cycle_len
                    smallest_member_of_longest_chain = current_cycle_min_member
                elif cycle_len == max_chain_len:
                    if (smallest_member_of_longest_chain is None or
                        current_cycle_min_member < smallest_member_of_longest_chain):
                        smallest_member_of_longest_chain = current_cycle_min_member
                
                # Mark cycle members
                for node_in_cycle in cycle_nodes:
                    processed_info[node_in_cycle] = {
                        'len': cycle_len,
                        'min_val': current_cycle_min_member
                    }
                
                # Mark nodes before cycle
                for node_before_cycle in current_path[:cycle_start_index]:
                    if node_before_cycle not in processed_info:
                        processed_info[node_before_cycle] = {'len': 0}
                break
            
            # Continue building path
            current_path.append(current_num)
            path_set[current_num] = len(current_path) - 1
            
            next_num = sps[current_num]
            current_num = next_num
    
    print(smallest_member_of_longest_chain)


if __name__ == "__main__":
    main()
