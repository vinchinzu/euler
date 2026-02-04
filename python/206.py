"""Project Euler Problem 206: Concealed Square.

Find the unique positive integer whose square is of the form
1_2_3_4_5_6_7_8_9_0.

Since the square ends in 0, the number must end in 0, so n = 10k.
Then n^2 ends in 00, matching _0. Since n ends in 0, and the second-to-last
digit must be 9 (pattern ...9_0), we need (n/10)^2 to end in 9, meaning
n/10 must end in 3 or 7. So n ends in 30 or 70.

We iterate n from sqrt(1929394959697989990) down to sqrt(1020304050607080900),
checking only values ending in 30 or 70.
"""

from math import isqrt


def matches_pattern(n_squared):
    """Check if n_squared matches 1_2_3_4_5_6_7_8_9_0."""
    s = str(n_squared)
    if len(s) != 19:
        return False
    # Fixed positions (0-indexed): 0->1, 2->2, 4->3, 6->4, 8->5, 10->6, 12->7, 14->8, 16->9, 18->0
    return (s[0] == '1' and s[2] == '2' and s[4] == '3' and s[6] == '4'
            and s[8] == '5' and s[10] == '6' and s[12] == '7' and s[14] == '8'
            and s[16] == '9' and s[18] == '0')


def solve():
    lo = 1020304050607080900
    hi = 1929394959697989990

    # Start from the top and work down
    n = isqrt(hi)
    # Align n to end in 30 or 70
    r = n % 100
    if r > 70:
        n -= (r - 70)
    elif r > 30:
        n -= (r - 30)
    else:
        n -= (r + 30)  # go to previous number ending in 70

    lo_n = isqrt(lo)

    while n >= lo_n:
        sq = n * n
        if matches_pattern(sq):
            return n
        # Alternate between 30 and 70 endings
        if n % 100 == 70:
            n -= 40  # 70 -> 30
        else:
            n -= 60  # 30 -> 70 of previous hundred

    return 0


if __name__ == "__main__":
    print(solve())
