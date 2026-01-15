#!/usr/bin/env python3
"""
Product-sum numbers (Problem 88)

Find the sum of all minimal product-sum numbers for 2 <= k <= 12000.
"""

K_MAX = 12000
N_LIMIT = 2 * K_MAX + 4000


def find_product_sums(
    product: int,
    sum_of_factors: int,
    num_non_one_factors: int,
    min_factor_to_use: int,
    limit_k: int,
    limit_n: int,
    solutions_array: list[float]
) -> None:
    """Recursive function to find product-sums."""
    f = min_factor_to_use
    while True:
        current_P = product * f
        if current_P > limit_n:
            break
        
        current_S_f = sum_of_factors + f
        current_j = num_non_one_factors + 1
        
        # Number of ones needed
        num_ones = current_P - current_S_f
        
        if num_ones >= 0:
            k = current_j + num_ones
            
            if k <= limit_k:
                if current_P < solutions_array[k]:
                    solutions_array[k] = current_P
        
        # Recursive call
        if current_j < 2 * K_MAX.bit_length():
            find_product_sums(
                current_P, current_S_f, current_j, f,
                limit_k, limit_n, solutions_array
            )
        
        f += 1


def main() -> None:
    """Find minimal product-sum numbers."""
    min_N_for_k = [float('inf')] * (K_MAX + 1)
    
    find_product_sums(1, 0, 0, 2, K_MAX, N_LIMIT, min_N_for_k)
    
    # Collect unique values
    unique_minimal_Ns = set()
    for k_val in range(2, K_MAX + 1):
        if min_N_for_k[k_val] != float('inf'):
            unique_minimal_Ns.add(int(min_N_for_k[k_val]))
    
    print(sum(unique_minimal_Ns))


if __name__ == "__main__":
    main()
