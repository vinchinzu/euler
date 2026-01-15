"""Project Euler Problem 151: Paper sheets of standard sizes."""

from typing import Dict, Tuple


def solve_151(
    counts: Tuple[int, int, int, int, int], memo: Dict[Tuple[int, ...], float]
) -> float:
    """Solve the paper sheet problem recursively with memoization."""
    state_tuple = tuple(counts)
    if state_tuple in memo:
        return memo[state_tuple]

    num_sheets = sum(counts)
    if num_sheets == 0:
        return 0.0

    if num_sheets == 1 and counts[4] == 1:
        return 0.0

    current_event_contribution = 0.0
    if num_sheets == 1:
        current_event_contribution = 1.0

    future_expected_value = 0.0

    for idx_ax, count_of_ax in enumerate(counts):
        if count_of_ax > 0:
            prob_pick_ax = count_of_ax / num_sheets

            new_counts = list(counts)
            new_counts[idx_ax] -= 1

            if idx_ax < 4:
                for k in range(idx_ax + 1, 5):
                    new_counts[k] += 1

            future_expected_value += prob_pick_ax * solve_151(
                tuple(new_counts), memo
            )

    result = current_event_contribution + future_expected_value
    memo[state_tuple] = result
    return result


def main() -> None:
    """Main function."""
    initial_counts = (1, 0, 0, 0, 0)
    memo: Dict[Tuple[int, ...], float] = {}
    raw_expected_value = solve_151(initial_counts, memo)
    final_expected_value = raw_expected_value - 1.0
    print(f"{final_expected_value:.6f}")


if __name__ == "__main__":
    main()
