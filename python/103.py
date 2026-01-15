"""Project Euler Problem 103: Special sum sets."""

from typing import List


def is_special_sum_set(set_list: List[int]) -> bool:
    """Check if a set is a special sum set.
    
    A special sum set satisfies:
    1. S(B) ≠ S(C) for any two non-empty disjoint subsets B and C
    2. If B contains more elements than C then S(B) > S(C)
    """
    n = len(set_list)
    # assignments: 0 for neither, 1 for B, 2 for C
    assignments = [0] * n

    def check_subsets_recursively(k: int) -> bool:
        """Recursive helper to iterate through all 3^n assignments."""
        if k == n:
            # We have a full assignment, form subsets B and C
            subset_b = [set_list[i] for i in range(n) if assignments[i] == 1]
            subset_c = [set_list[i] for i in range(n) if assignments[i] == 2]

            # Skip if B or C is empty (disjoint non-empty subsets required)
            if not subset_b or not subset_c:
                return True

            sum_b = sum(subset_b)
            sum_c = sum(subset_c)

            # Condition 1: S(B) != S(C)
            if sum_b == sum_c:
                return False

            # Condition 2: if |B| > |C|, then S(B) > S(C)
            if len(subset_b) > len(subset_c) and sum_b <= sum_c:
                return False

            # Condition 2 (cont.): if |C| > |B|, then S(C) > S(B)
            if len(subset_c) > len(subset_b) and sum_c <= sum_b:
                return False

            return True  # This specific pair of B and C is fine

        # Recursive step: try assigning current element to neither, B, or C
        for i in range(3):
            assignments[k] = i
            if not check_subsets_recursively(k + 1):
                return False

        return True  # All assignments from this path were fine

    return check_subsets_recursively(0)


def main() -> str:
    """The candidate set for n=7."""
    A = [20, 31, 38, 39, 40, 42, 45]

    if is_special_sum_set(A):
        return ''.join(str(x) for x in A)
    else:
        raise ValueError(f"The set {A} is not a special sum set.")


if __name__ == "__main__":
    print(main())
