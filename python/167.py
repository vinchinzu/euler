"""Project Euler Problem 167: Investigating Ulam sequences."""

from typing import Dict, List, Set, Tuple


def next_ulam(terms: List[int]) -> int:
    """Generate the next Ulam number using the textbook definition."""
    candidate = terms[-1] + 1
    while True:
        ways = 0
        left = 0
        right = len(terms) - 1

        while left < right:
            sum_val = terms[left] + terms[right]
            if sum_val == candidate:
                ways += 1
                if ways > 1:
                    break
                left += 1
                right -= 1
            elif sum_val < candidate:
                left += 1
            else:
                right -= 1

        if ways == 1:
            terms.append(candidate)
            return candidate

        candidate += 1


def build_period(v: int) -> Dict[str, int]:
    """Build the odd membership pattern once the two even Ulam numbers are known."""
    terms = [2, v]
    odd_flags: Dict[int, int] = {}
    odd_flags[(v - 1) // 2] = 1
    even_terms = [2]

    while len(even_terms) < 2:
        value = next_ulam(terms)
        if value % 2 == 1:
            odd_flags[(value - 1) // 2] = 1
        if value % 2 == 0:
            even_terms.append(value)

    e2 = even_terms[-1]
    t = e2 // 2

    max_index = (terms[-1] - 1) // 2
    for idx in range(max_index + 1):
        if idx not in odd_flags:
            odd_flags[idx] = 0

    state_mask = (1 << t) - 1
    i = max_index + 1
    state = 0
    for offset in range(t):
        idx = i - t + offset
        bit = odd_flags.get(idx, 0) if idx >= 0 else 0
        state = (state << 1) | bit

    seen: Dict[int, int] = {state: i}

    while True:
        prev = odd_flags.get(i - 1, 0)
        shifted = odd_flags.get(i - t, 0) if i - t >= 0 else 0
        new_flag = prev ^ shifted
        odd_flags[i] = new_flag

        state = ((state << 1) & state_mask) | new_flag
        i += 1

        if state in seen:
            period_start = seen[state]
            period_length = i - period_start
            return {
                "even_terms": even_terms,
                "odd_flags": odd_flags,
                "period_start": period_start,
                "period_length": period_length,
            }

        seen[state] = i


def get_ulam_k(v: int, k: int) -> int:
    """Get the k-th Ulam number for sequence starting with [2, v]."""
    info = build_period(v)
    even_terms = info["even_terms"]
    odd_flags = info["odd_flags"]
    period_start = info["period_start"]
    period_length = info["period_length"]

    prefix_odds: List[int] = []
    for idx in range(period_start):
        if odd_flags.get(idx, 0) == 1:
            prefix_odds.append(2 * idx + 1)

    prefix_terms = sorted(prefix_odds + even_terms)

    if k <= len(prefix_terms):
        return prefix_terms[k - 1]

    remaining = k - len(prefix_terms)

    period_flags = [odd_flags.get(period_start + offset, 0) for offset in range(period_length)]
    period_indices: List[int] = []
    for offset, flag in enumerate(period_flags):
        if flag == 1:
            period_indices.append(offset)

    if not period_indices:
        raise ValueError("Unexpected empty period")

    per_period = len(period_indices)
    full_periods, rem = divmod(remaining - 1, per_period)

    base_index = period_start + full_periods * period_length
    chosen_offset = period_indices[rem]
    odd_index = base_index + chosen_offset

    return 2 * odd_index + 1


def main() -> int:
    """Main function."""
    total = 0
    for n in range(2, 11):
        v = 2 * n + 1
        total += get_ulam_k(v, 10 ** 11)
    return total


if __name__ == "__main__":
    print(main())
