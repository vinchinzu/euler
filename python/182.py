"""Project Euler Problem 182: RSA encryption."""

import math

P = 1009
Q = 3643
PHI = (P - 1) * (Q - 1)


def main() -> int:
    """Main function."""
    min_unconcealed: int | None = None
    sum_e = 0

    for e in range(2, PHI):
        if math.gcd(e, PHI) != 1:
            continue

        unconcealed = (1 + math.gcd(e - 1, P - 1)) * (1 + math.gcd(e - 1, Q - 1))

        if min_unconcealed is None or unconcealed < min_unconcealed:
            min_unconcealed = unconcealed
            sum_e = e
        elif unconcealed == min_unconcealed:
            sum_e += e

    return sum_e


if __name__ == "__main__":
    print(main())
