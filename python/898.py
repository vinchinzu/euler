
import math
import bisect
import os

# Project Euler Problem 898
#
# PROBLEM DESCRIPTION:
# <p>
# Claire Voyant is a teacher playing a game with a class of students.
# A fair coin is tossed on the table. All the students can see the outcome of the toss, but Claire cannot.
# Each student then tells Claire whether the outcome is head or tail. The students may lie, but Claire knows the probability that each individual student lies. Moreover, the students lie independently.
# After that, Claire attempts to guess the outcome using an optimal strategy.
# </p>
# <p>
# For example, for a class of four students with lying probabilities $20\%,40\%,60\%,80\%$, Claire guesses correctly with probability 0.832.
# </p>
# <p>
# Find the probability that Claire guesses correctly for a class of 51 students each lying with a probability of $25\%, 26\%, \dots, 75\%$
#  respectively.
# </p>
# <p>
# Give your answer rounded to 10 digits after the decimal point.
# </p>

def get_pair_dist(p: float) -> list[tuple[float, float]]:
    """
    Returns list of (value, prob) for a pair with lie probability p.
    w = log((1-p)/p)
    Values: 2w, 0, -2w
    Probs: (1-p)^2, 2p(1-p), p^2
    """
    w = math.log((1 - p) / p)
    return [
        (2 * w, (1 - p) ** 2),
        (0.0, 2 * p * (1 - p)),
        (-2 * w, p ** 2)
    ]

def get_subset_sums(pairs: list[float]) -> list[tuple[float, float]]:
    """
    Generates all possible sums for a list of pairs.
    Returns list of (sum_val, prob).
    """
    # Start with sum=0, prob=1
    current_states = [(0.0, 1.0)]

    for p in pairs:
        dist = get_pair_dist(p)
        next_states = []
        for val_acc, prob_acc in current_states:
            for val, prob in dist:
                next_states.append((val_acc + val, prob_acc * prob))
        current_states = next_states

    return current_states

def solve() -> float:
    # Generate pairs p values: 0.25, 0.26, ..., 0.49
    # The pairs are (0.25, 0.75), (0.26, 0.74), ..., (0.49, 0.51).
    # Student 50 (p=0.50) is ignored as their weight is 0.

    pairs = [k / 100.0 for k in range(25, 50)]

    n = len(pairs)
    mid = n // 2

    pairs_A = pairs[:mid]
    pairs_B = pairs[mid:]

    sums_A = get_subset_sums(pairs_A)
    sums_B = get_subset_sums(pairs_B)

    # Sort sums_B by value
    sums_B.sort(key=lambda x: x[0])

    # Compute suffix sums of probabilities for B
    n_B = len(sums_B)
    suffix_probs = [0.0] * (n_B + 1)
    current_sum = 0.0
    for i in range(n_B - 1, -1, -1):
        current_sum += sums_B[i][1]
        suffix_probs[i] = current_sum

    vals_B = [x[0] for x in sums_B]

    total_prob = 0.0

    # Epsilon for equality check
    EPS = 1e-9

    for val_A, prob_A in sums_A:
        target = -val_A

        # We need sum > 0 => val_B > target
        # And sum == 0 => val_B == target (add 0.5 * prob)

        idx_start = bisect.bisect_left(vals_B, target - EPS)
        idx_end = bisect.bisect_right(vals_B, target + EPS)

        # Strictly greater: from idx_end onwards
        prob_strict = suffix_probs[idx_end]
        total_prob += prob_A * prob_strict

        # Equal
        if idx_end > idx_start:
            prob_equal = suffix_probs[idx_start] - suffix_probs[idx_end]
            total_prob += prob_A * prob_equal * 0.5

    return total_prob

if __name__ == "__main__":
    result = solve()
    formatted_result = f"{result:.10f}"
    print(formatted_result)

    # Write answer to file in the same directory as the script
    script_dir = os.path.dirname(os.path.abspath(__file__))
    with open(os.path.join(script_dir, "answer.txt"), "w") as f:
        f.write(formatted_result)
