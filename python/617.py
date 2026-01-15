"""Project Euler Problem 617: Mirror Power Sequence.

Find the number of (n, e)-MPS sequences defined by a_{i+1} = min(a_i^e, n - a_i^e),
a_i > 1, and n ≤ N.

Given a "tower power" sequence (b, b^e, b^{e²}, ... b^{e^k}), there are two
types of MPS sequences:
- A cyclic permutation of the tower powers of b (k+1 of these)
- A cyclic permutation of a suffix of the sequence (k of these).

We iterate over all such tower power sequences, and check for each of the above
types of MPS sequences whether n ≤ N.
"""

from __future__ import annotations

from math import isqrt


def pow_int(base: int, exp: int) -> int:
    """Integer power."""
    result = 1
    for _ in range(exp):
        result *= base
    return result


def solve() -> int:
    """Solve Problem 617."""
    N = 10**18

    ans = isqrt(N) - 2
    a0 = 2
    while pow_int(a0, 3) + a0 <= N:
        e = 2
        while pow_int(a0, e) + a0 <= N:
            as_list = []
            a = a0
            while pow_int(a, e) <= N:
                as_list.append(a)
                a = pow_int(a, e)

            for start in range(len(as_list)):
                for end in range(start, len(as_list)):
                    if (e > 2 or end > 0) and pow_int(as_list[end], e) + as_list[start] <= N:
                        ans += end + 1 if start == 0 else 1
            e += 1
        a0 += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
