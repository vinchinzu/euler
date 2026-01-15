"""Project Euler Problem 198: Ambiguous Numbers."""

import math

LIMIT_Q = 100_000_000
HALF_Q = LIMIT_Q // 2
SQRT_HALF_Q = int(math.sqrt(HALF_Q))
UPPER_FR = 1
LOWER_FR = 100


def count_between(hl: int, kl: int, hr: int, kr: int) -> int:
    """Depth-first enumeration."""
    stack = [hl, kl, hr, kr]
    total = 0

    while stack:
        kr = stack.pop()
        hr = stack.pop()
        kl = stack.pop()
        hl = stack.pop()

        initial_branch = hl == 0 and kl == 1

        while True:
            hm = hl + hr
            km = kl + kr  # mediant fraction h_m/k_m

            # Stop when 2*kl*km already exceeds the limit
            if km > HALF_Q:
                break

            # Keep only fractions strictly below 1/100
            if 100 * hm >= km:
                break

            max_partner = HALF_Q // km
            km_over_sqrt = km > SQRT_HALF_Q

            if not (hl == 0 and kl == 1):
                if kl <= max_partner:
                    total += 1

            if kr <= max_partner:
                total += 1

            left_blocked = kl > max_partner
            right_blocked = kr > max_partner

            if not right_blocked:
                stack.extend([hm, km, hr, kr])

            # Once the (0/1, 1/n) corridor can no longer yield valid right midpoints
            if right_blocked and initial_branch:
                break

            if left_blocked:
                break

            hr = hm
            kr = km

    return total


def main() -> int:
    """Main function."""
    case1 = HALF_Q - 50  # k_1 = 51 ... 50_000_000
    case2 = count_between(0, 1, 1, LOWER_FR)  # fractions in (0, 1/100)
    return case1 + case2


if __name__ == "__main__":
    print(main())
