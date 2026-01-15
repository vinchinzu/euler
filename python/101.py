"""Project Euler Problem 101: Optimum polynomial."""

from typing import List


def u(n: int) -> int:
    """Generate terms of the sequence u_n.
    
    u_n = 1 - n + n^2 - n^3 + n^4 - n^5 + n^6 - n^7 + n^8 - n^9 + n^10
    """
    return sum((-n) ** i for i in range(11))


def op(k_val: int, n_val: int, sequence_terms: List[int]) -> int:
    """Calculate OP(k, n) using Lagrange Interpolation.
    
    Args:
        k_val: Number of terms used to generate OP (e.g., k_val=1 means OP is degree 0, using u_1)
        n_val: The value at which to evaluate the polynomial OP(k, n_val)
        sequence_terms: An array containing the first k_val terms of the sequence [u_1, u_2, ..., u_k_val]
    
    Returns:
        The interpolated value rounded to nearest integer.
    """
    if len(sequence_terms) < k_val:
        raise ValueError(f"Not enough sequence terms provided for k_val={k_val}")

    # The known points are (1, u_1), (2, u_2), ..., (k_val, u_k_val)
    # We want to evaluate the polynomial at n_val.

    total = 0.0

    for j in range(k_val):  # j from 0 to k_val-1
        y_j = sequence_terms[j]
        x_j = j + 1  # x_j goes from 1 to k_val

        numerator = 1.0
        denominator = 1.0

        for i in range(k_val):  # i from 0 to k_val-1
            if i == j:
                continue
            x_i = i + 1  # x_i goes from 1 to k_val

            numerator *= (n_val - x_i)
            denominator *= (x_j - x_i)

        # It's possible for denominator to be zero if x_j values are not distinct,
        # but in our case, x_j are 1, 2, ..., k_val, so they are distinct.
        if denominator == 0.0:
            raise ValueError("Denominator is zero in Lagrange basis polynomial calculation")

        total += y_j * (numerator / denominator)

    # The problem expects integer FITs.
    # Lagrange polynomials with integer points (x_i, y_i) evaluated at an integer x
    # should result in a rational number. If the problem implies integer FITs,
    # we should be careful about precision.
    # Let's round to nearest integer, assuming FITs are integers.
    # Project Euler problems usually have integer answers.
    return round(total)


def main() -> int:
    """Calculate sum of FITs."""
    sum_of_fits = 0
    sequence_u_values: List[int] = []  # Stores u_1, u_2, ...

    # The generating polynomial is degree 10.
    # We expect OP(k,n) to be a BOP for k = 1, 2, ..., 10.
    # For k=11, OP(11,n) should be the original polynomial, and OP(11, 12) == u_12.
    for k in range(1, 11):
        # Generate sequence terms up to u_k
        # The op function needs the first k terms: u_1, ..., u_k.
        # If sequence_u_values is [u_1, ..., u_{k-1}], we need to add u_k.
        # The u(n) function calculates u_n, so for u_k, n=k.
        if len(sequence_u_values) < k:
            sequence_u_values.append(u(k))  # Add u_k to the list

        # These are the terms [u_1, ..., u_k]
        current_sequence_terms = sequence_u_values

        # Calculate the predicted next term: OP(k, k+1)
        # This is the First Incorrect Term (FIT) if OP(k,n) is a BOP
        predicted_next_term = op(k, k + 1, current_sequence_terms)

        # Calculate the actual next term in the sequence: u_{k+1}
        actual_next_term = u(k + 1)

        # Check if OP(k,n) is a BOP
        if predicted_next_term != actual_next_term:
            sum_of_fits += predicted_next_term

    return sum_of_fits


if __name__ == "__main__":
    print(main())
