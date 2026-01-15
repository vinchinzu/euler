"""Project Euler Problem 284: Steady Squares.

Find the sum of the digits of all steady squares in base 14 with up to N
digits, where a steady square is a number with no leading zeros whose square
ends in the number itself.

A steady square k with N digits must satisfy k² ≡ k (mod 14^N). This means
that k² ≡ k (mod 2^N) and k² ≡ k (mod 7^N), so k ≡ 0 or 1 in both mod 2^N
and 7^N. The trivial cases are 0 and 1.

For the case where k ≡ 0 (mod 2^N) and k ≡ 1 (mod 7^N), we have by the
Chinese Remainder Theorem that k ≡ 2^N (2^N (mod 7^N)) (mod 14^N).
Similarly, if k ≡ 1 (mod 2^N) and k ≡ 0 (mod 7^N), then k ≡ 7^N (7^N (mod
2^N)). Once we have these steady squares with N digits, it is easy to find
the smaller steady squares by taking suffixes. This means the first digit is
counted once, the second digit is counted twice, and so on. There is one catch:
since steady squares can't have leading zeros, we do not increment the
multiplier if a digit is zero.
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 284."""
    N = 10000
    B = 14

    pow2 = 2**N
    pow7 = (B // 2) ** N

    # Compute modular inverses
    def mod_inverse(a: int, m: int) -> int:
        """Modular inverse using extended Euclidean algorithm."""
        if m == 1:
            return 0
        t, new_t = 0, 1
        r, new_r = m, a % m
        while new_r != 0:
            quotient = r // new_r
            t, new_t = new_t, t - quotient * new_t
            r, new_r = new_r, r - quotient * new_r
        if r > 1:
            raise ValueError("Modular inverse does not exist")
        return (t % m + m) % m

    # Two steady squares
    steady1 = pow2 * mod_inverse(pow2, pow7) % (pow2 * pow7)
    steady2 = pow7 * mod_inverse(pow7, pow2) % (pow2 * pow7)

    ans = 1  # Count the digit '1' from trivial case

    for steady in [steady1, steady2]:
        mult = 1
        s = ""
        temp = steady
        while temp > 0:
            s = str(temp % B) + s
            temp //= B

        for c in s:
            digit = int(c, B)
            ans += mult * digit
            if c != "0":
                mult += 1

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    # Convert to base 14
    print(format(result, "x"))


if __name__ == "__main__":
    main()
