#!/usr/bin/env python3
"""
Arithmetic expressions (Problem 93)

Find the set of four distinct digits that produces the longest consecutive
sequence starting from 1.
"""

import itertools

EPSILON = 1e-7


def apply_op(a: float, op: str, b: float) -> float | None:
    """Apply operation to two numbers."""
    if op == '/' and b == 0.0:
        return None
    
    if op == '+':
        return a + b
    elif op == '-':
        return a - b
    elif op == '*':
        return a * b
    elif op == '/':
        return a / b
    return None


def main() -> None:
    """Find best digit set."""
    max_n_found = 0
    best_digits_string = ""
    operators = ['+', '-', '*', '/']
    
    # Iterate through all combinations of 4 distinct digits
    for digits in itertools.combinations(range(10), 4):
        achieved_targets = set()
        
        # Iterate through all permutations
        for p in itertools.permutations(digits):
            # Iterate through all operator combinations
            for ops_list in itertools.product(operators, repeat=3):
                # Scheme 1: ((p[0] op0 p[1]) op1 p[2]) op2 p[3]
                val_s1_1 = apply_op(float(p[0]), ops_list[0], float(p[1]))
                if val_s1_1 is not None:
                    val_s1_2 = apply_op(val_s1_1, ops_list[1], float(p[2]))
                    if val_s1_2 is not None:
                        final_res_s1 = apply_op(val_s1_2, ops_list[2], float(p[3]))
                        if (final_res_s1 is not None and
                            abs(final_res_s1) != float('inf') and
                            final_res_s1 > 0 and
                            abs(final_res_s1 - round(final_res_s1)) < EPSILON):
                            achieved_targets.add(round(final_res_s1))
                
                # Scheme 2: (p[0] op0 (p[1] op1 p[2])) op2 p[3]
                val_s2_1 = apply_op(float(p[1]), ops_list[1], float(p[2]))
                if val_s2_1 is not None:
                    val_s2_2 = apply_op(float(p[0]), ops_list[0], val_s2_1)
                    if val_s2_2 is not None:
                        final_res_s2 = apply_op(val_s2_2, ops_list[2], float(p[3]))
                        if (final_res_s2 is not None and
                            abs(final_res_s2) != float('inf') and
                            final_res_s2 > 0 and
                            abs(final_res_s2 - round(final_res_s2)) < EPSILON):
                            achieved_targets.add(round(final_res_s2))
                
                # Scheme 3: p[0] op0 ((p[1] op1 p[2]) op2 p[3])
                val_s3_1 = apply_op(float(p[1]), ops_list[1], float(p[2]))
                if val_s3_1 is not None:
                    val_s3_2 = apply_op(val_s3_1, ops_list[2], float(p[3]))
                    if val_s3_2 is not None:
                        final_res_s3 = apply_op(float(p[0]), ops_list[0], val_s3_2)
                        if (final_res_s3 is not None and
                            abs(final_res_s3) != float('inf') and
                            final_res_s3 > 0 and
                            abs(final_res_s3 - round(final_res_s3)) < EPSILON):
                            achieved_targets.add(round(final_res_s3))
                
                # Scheme 4: p[0] op0 (p[1] op1 (p[2] op2 p[3]))
                val_s4_1 = apply_op(float(p[2]), ops_list[2], float(p[3]))
                if val_s4_1 is not None:
                    val_s4_2 = apply_op(float(p[1]), ops_list[1], val_s4_1)
                    if val_s4_2 is not None:
                        final_res_s4 = apply_op(float(p[0]), ops_list[0], val_s4_2)
                        if (final_res_s4 is not None and
                            abs(final_res_s4) != float('inf') and
                            final_res_s4 > 0 and
                            abs(final_res_s4 - round(final_res_s4)) < EPSILON):
                            achieved_targets.add(round(final_res_s4))
                
                # Scheme 5: (p[0] op0 p[1]) op1 (p[2] op2 p[3])
                val_s5_1 = apply_op(float(p[0]), ops_list[0], float(p[1]))
                if val_s5_1 is not None:
                    val_s5_2 = apply_op(float(p[2]), ops_list[2], float(p[3]))
                    if val_s5_2 is not None:
                        final_res_s5 = apply_op(val_s5_1, ops_list[1], val_s5_2)
                        if (final_res_s5 is not None and
                            abs(final_res_s5) != float('inf') and
                            final_res_s5 > 0 and
                            abs(final_res_s5 - round(final_res_s5)) < EPSILON):
                            achieved_targets.add(round(final_res_s5))
        
        # Check for longest consecutive sequence
        current_n = 0
        while (current_n + 1) in achieved_targets:
            current_n += 1
        
        if current_n > max_n_found:
            max_n_found = current_n
            best_digits_string = ''.join(str(d) for d in digits)
    
    print(best_digits_string)


if __name__ == "__main__":
    main()
