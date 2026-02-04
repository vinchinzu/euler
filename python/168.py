"""Project Euler Problem 168 - Number Rotations.

Find the last 5 digits of the sum of all positive integers with 2 to 100 digits
that are a divisor of their right rotation.

A number b is valid if mult*b (as a string) has its last digit moved to the front
to equal b. We build numbers digit by digit.
"""

def solve():
    N = 100
    K = 5
    B = 10
    MOD = B ** K

    total = 0
    for mult in range(1, B):
        for last_digit in range(1, B):
            b = str(last_digit)
            for i in range(1, N + 1):
                multb = str(mult * int(b))
                # Check: rotating multb (last char to front) gives b
                rotated = multb[1:] + multb[0]
                if rotated == b and b[0] != '0' and i > 1:
                    total = (total + int(b)) % MOD
                # Next b: take last i chars of multb, append last_digit
                b = multb[len(multb) - i:] + str(last_digit)

    return total % MOD

if __name__ == "__main__":
    print(solve())
