"""Project Euler Problem 803: Pseudorandom Sequence.

Let a_n = (25214903917a_{n-1} + 11) (mod 2^48), let b_n = ⌊a_n/2^16⌋ (mod 52),
and let c_n be defined by b_n by 0->a, 1->b, ... 25->z, 26->A, 27->B, ...
51 ->Z. If a_0 is defined such that the string c starts with S, find the
index of the first occurrence of the substring T.

Suppose a_n ≡ r (mod 2^16) is the first character of S, so
a_n ≡ c('P') 2^16 + r (mod 52*2^16), and a_{n+1} = c('u') 2^16 + r'
(mod 52*2^16). Then

c('u') 2^16 + r' ≡ 25214903917 (c('P') 2^16 + r) + 11  (mod 4*2^16)
=> c('u') ≡ 25214903917 c('P') + (25214903917r+11-r')/2^16  (mod 4)
=> ⌊(25214903917r+11)/2^16⌋ - c('u') + c('P') ≡ 0   (mod 4),

and similarly for each pair of adjacent characters of S. This provides
constraints on r, and in fact we can determine only one valid value of r
where 0≤r<2^16.

We can then start at c('P') 2^16 + r and repeatedly add 52 2^16 until we
find the substring S.

Next, we find the corresponding value of r for T. We repeatedly compute
the next a_{n+1} until we find one with the right remainder r. Then, we
repeatedly compute a_{n+2^16} until we find one with the right substring T.
To compute a_{n+2^16} efficiently, we note that it is still a linear
combination of a_{n} and 1, so we can find those two coefficients by
computing a_{2^16} starting from a_0 = 0 and a_0 = 1.
"""

from __future__ import annotations

from typing import List


def char_to_code(c: str) -> int:
    """Convert character to code: a-z -> 0-25, A-Z -> 26-51."""
    if c >= "a":
        return ord(c) - ord("a")
    return ord(c) - ord("A") + 26


def codes(s: str) -> List[int]:
    """Convert string to list of character codes."""
    return [char_to_code(c) for c in s]


def next_val(a: int) -> int:
    """Compute next value in LCG sequence."""
    return (25214903917 * a + 11) % (1 << 48)


def find_r(codes_list: List[int]) -> int:
    """Find the remainder r that satisfies constraints from codes."""
    L = 1 << 16
    for r in range(L):
        a = r
        good = True
        for i in range(1, len(codes_list)):
            a = next_val(a % L)
            if ((a // L + codes_list[i - 1] - codes_list[i]) % 4) != 0:
                good = False
                break
        if good:
            return r
    raise ValueError("No valid r found")


def is_substring(a: int, codes_list: List[int]) -> bool:
    """Check if sequence starting at a matches codes_list."""
    L = 1 << 16
    for code in codes_list:
        if (a // L) % 52 != code:
            return False
        a = next_val(a)
    return True


def solve() -> int:
    """Solve Problem 803."""
    S = codes("PuzzleOne")
    T = codes("LuckyText")
    L = 1 << 16

    # Find starting value a such that sequence starts with S
    a = S[0] * L + find_r(S)
    while not is_substring(a, S):
        a += 52 * L

    # Find remainder for T
    r = find_r(T)
    ans = 0
    while a % L != r:
        a = next_val(a)
        ans += 1

    # Compute coefficients for a_{n+2^16}
    c0 = 0
    c1 = 1
    for _ in range(L):
        c0 = next_val(c0)
    for _ in range(L):
        c1 = next_val(c1)

    # Find substring T
    while not is_substring(a, T):
        a = ((c1 - c0) * a + c0) % (1 << 48)
        ans += L

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
