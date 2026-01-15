"""Project Euler Problem 183: Maximum product of parts."""

import math

START_N = 5
END_N = 10_000


def main() -> int:
    """Main function."""
    sum_val = 0

    for n in range(START_N, END_N + 1):
        k0 = int(n / math.e)
        candidates = [1, n]
        for delta in range(-1, 3):
            k = k0 + delta
            if 1 <= k <= n:
                candidates.append(k)
        best_k = max(set(candidates), key=lambda k: k * math.log(n / k))

        gcd_val = math.gcd(n, best_k)
        reduced_k = best_k // gcd_val
        while reduced_k % 2 == 0:
            reduced_k //= 2
        while reduced_k % 5 == 0:
            reduced_k //= 5

        sum_val += -n if reduced_k == 1 else n

    return sum_val


if __name__ == "__main__":
    print(main())
