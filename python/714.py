"""Project Euler Problem 714: Duodigits.

A duodigit is a number whose decimal representation uses no more than two
different digits. Find Σ_{k=1}^N d(k), where d(k) is the smallest positive
multiple of k that is a duodigit.

We use brute force. For each k, we compute d(k) by looking at numbers with
increasing numbers of digits. For a given number of digits, we find all
"templates" of where the first digit should go (the second digit has to be
in the remaining places) and all combinations of the two digits d1 and d2.
"""

from __future__ import annotations

from math import isqrt


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Fast exponentiation modulo mod."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def d(k: int) -> float:
    """Find smallest positive multiple of k that is a duodigit."""
    b = 10
    num_digits = 1

    while True:
        # Precompute powers of B
        pows = [b**i for i in range(num_digits)]
        mod_pows = [pow_mod(b, i, k) for i in range(num_digits)]

        n = 1 << num_digits
        nums = [[0.0] * b for _ in range(n)]
        mods = [[0] * b for _ in range(n)]

        # Build all subsets using bitmasks
        for bitset in range(1, n):
            # Find the rightmost set bit
            i = (bitset & -bitset).bit_length() - 1
            prev_bitset = bitset - (bitset & -bitset)
            num = nums[prev_bitset][1] + pows[i]
            mod_val = (mods[prev_bitset][1] + mod_pows[i]) % k
            for d in range(1, b):
                nums[bitset][d] = d * num
                mods[bitset][d] = (d * mod_val) % k

        min_d = float("inf")
        # Try all combinations of two digits
        for bitset in range(n // 2):
            for d1 in range(b):
                for d2 in range(1, b):
                    num = nums[bitset][d1] + nums[n - 1 - bitset][d2]
                    mod_val = (
                        mods[bitset][d1] + mods[n - 1 - bitset][d2]
                    ) % k
                    if num < min_d and mod_val == 0:
                        min_d = num

        if min_d < float("inf"):
            return min_d

        num_digits += 1


def solve() -> float:
    """Solve Problem 714."""
    n = 50000
    ans = 0.0
    for k in range(1, n + 1):
        ans += d(k)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(f"{result:.12e}".replace("+", ""))
    return int(result)


if __name__ == "__main__":
    main()
