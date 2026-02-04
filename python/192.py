"""Project Euler Problem 192 - Best Approximations.

Find the sum of the denominators of the best approximations to sqrt(n)
for non-squares 2 <= n <= 100000 with denominator bound 10^12.

Uses continued fraction expansion and semiconvergent comparison.
"""
import math

def solve():
    N = 100000
    K = 10**12
    total = 0

    for n in range(2, N + 1):
        a0 = math.isqrt(n)
        if a0 * a0 == n:
            continue

        # Generate convergents of sqrt(n) using standard CF algorithm
        m, d, a = 0, 1, a0
        # Previous and current convergent
        A_prev, B_prev = 1, 0
        A_curr, B_curr = a0, 1

        # Store steps for the Pell-like algorithm
        steps_A = [A_prev, A_curr]
        steps_B = [B_prev, B_curr]

        while True:
            m = d * a - m
            d = (n - m * m) // d
            a = (a0 + m) // d

            A_next = a * A_curr + A_prev
            B_next = a * B_curr + B_prev

            if B_next > K:
                # The last convergent with B <= K is (A_curr, B_curr)
                # Now check semiconvergents
                h = (K - B_prev) // B_curr
                den1 = B_curr
                den2 = B_prev + h * B_curr

                if h > a // 2:
                    total += den2
                elif h < (a + 1) // 2:
                    total += den1
                else:
                    # h == a/2 (a even) or h == (a-1)/2+1 = (a+1)/2
                    # Need to compare |A_curr/B_curr - sqrt(n)| vs |num2/den2 - sqrt(n)|
                    num1 = A_curr
                    num2 = A_prev + h * A_curr

                    # Compare cross-multiplied: which is closer to sqrt(n)?
                    # |num1/den1 - sqrt(n)| vs |num2/den2 - sqrt(n)|
                    # Equivalent: |num1*den2 - num2*den1| cross check
                    # Using the identity: convergents alternate being above/below sqrt(n)
                    # So: (num1*den2 - num2*den1) and their distances from sqrt(n)*den
                    # Better: compare (num1^2 - n*den1^2)^2 * den2^2 vs (num2^2 - n*den2^2)^2 * den1^2
                    # Since |x/d - sqrt(n)| ~ |x^2 - n*d^2| / (d*(x + d*sqrt(n)))
                    # ~ |x^2 - n*d^2| / (2*d^2*sqrt(n)) for good approximations
                    # So compare |num1^2 - n*den1^2| * den2^2 vs |num2^2 - n*den2^2| * den1^2

                    cross1 = num1 * den2
                    cross2 = num2 * den1
                    bot = den1 * den2

                    # Compare |cross1 - cross2| using the formula from Java:
                    # if (cross1^2 - cross2^2)^2 > 4n * (cross1 - cross2)^2 * bot^2
                    # XOR cross1 > cross2 => use den1, else den2
                    diff_sq = (cross1**2 - cross2**2)**2
                    rhs = 4 * n * ((cross1 - cross2) * bot)**2

                    if (diff_sq > rhs) ^ (cross1 > cross2):
                        total += den1
                    else:
                        total += den2
                break

            A_prev, A_curr = A_curr, A_next
            B_prev, B_curr = B_curr, B_next

    return total

if __name__ == "__main__":
    print(solve())
