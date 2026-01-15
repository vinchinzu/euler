#!/usr/bin/env python3
import math

FACTORIALS = {
    0: 1, 1: 1, 2: 2, 3: 6, 4: 24,
    5: 120, 6: 720, 7: 5040, 8: 40320, 9: 362880
}

def sum_digit_factorials(n, factorials_map):
    if n == 0:
        return factorials_map[0]
    
    sum_val = 0
    temp_n = n
    while temp_n > 0:
        sum_val += factorials_map[temp_n % 10]
        temp_n //= 10
    return sum_val

def get_chain_length(start_node, factorials_map, chain_length_cache):
    if start_node in chain_length_cache:
        return chain_length_cache[start_node]
    
    current_path = []
    current_num = start_node
    
    while True:
        if current_num in chain_length_cache:
            length_from_current_num = chain_length_cache[current_num]
            for j, path_node in enumerate(current_path):
                chain_length_cache[path_node] = len(current_path) - j + length_from_current_num
            break
        elif current_num in current_path:
            loop_start_index = current_path.index(current_num)
            # Chain length = number of non-repeating terms (all terms before repeat)
            chain_len = len(current_path)
            # All nodes in this path have chain_len non-repeating terms
            for j in range(len(current_path)):
                chain_length_cache[current_path[j]] = chain_len
            break
        else:
            current_path.append(current_num)
            current_num = sum_digit_factorials(current_num, factorials_map)
    
    return chain_length_cache[start_node]

LIMIT = 1_000_000
TARGET_LENGTH = 60

def main():
    count_of_sixty_length_chains = 0
    chain_length_cache = {}

    chain_length_cache[169] = 3
    chain_length_cache[363601] = 3
    chain_length_cache[1454] = 3
    chain_length_cache[871] = 2
    chain_length_cache[45361] = 2
    chain_length_cache[872] = 2
    chain_length_cache[45362] = 2
    chain_length_cache[145] = 1
    chain_length_cache[1] = 1
    chain_length_cache[2] = 1
    chain_length_cache[40585] = 1

    for i in range(1, LIMIT):
        chain_len = get_chain_length(i, FACTORIALS, chain_length_cache)
        if chain_len == TARGET_LENGTH:
            count_of_sixty_length_chains += 1

    print(count_of_sixty_length_chains)

if __name__ == "__main__":
    main()
