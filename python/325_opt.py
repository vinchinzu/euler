"""Project Euler Problem 325 - Stone Game optimized.

Key insight: Instead of iterating over all y, iterate over distinct values of floor(y/phi).
Since floor(y/phi) <= N/phi ≈ 0.618*N, we have at most N/phi distinct values.
"""
from decimal import Decimal, getcontext
getcontext().prec = 100

def solve():
    N = 10**16
    M = 7**10

    def mod_inv(a, m):
        return pow(a, m - 2, m)

    # High precision phi
    sqrt5 = Decimal(5).sqrt()
    phi = (1 + sqrt5) / 2

    # Part 1: 3*(sum y^2 - sum y)/2
    n_mod = N % M
    np1_mod = (N + 1) % M
    tn1_mod = (2 * N + 1) % M

    sum_y = n_mod * np1_mod % M * mod_inv(2, M) % M
    sum_y2 = n_mod * np1_mod % M * tn1_mod % M * mod_inv(6, M) % M
    part1 = 3 * (sum_y2 - sum_y + M) % M * mod_inv(2, M) % M

    # Parts 2 and 3: Iterate over distinct values of f = floor(y/phi)
    # For each f, the y values are in range [floor(f*phi) + epsilon, floor((f+1)*phi)]
    # More precisely: f = floor(y/phi) means f*phi <= y < (f+1)*phi
    # Since y is an integer: ceil(f*phi) <= y <= floor((f+1)*phi) - 1

    sum_f = 0    # sum_{y=1}^N floor(y/phi)
    sum_f2 = 0   # sum_{y=1}^N floor(y/phi)^2
    sum_yf = 0   # sum_{y=1}^N y * floor(y/phi)

    # Maximum f value
    max_f = int(Decimal(N) / phi)

    f = 0
    while f <= max_f:
        # Find y range for this f
        if f == 0:
            # f=0: y < phi, so y = 1
            y_min = 1
            y_max = int(phi) - 1  # floor(phi) - 1 = 1 - 1 = 0, but phi > 1
            # Actually, floor(y/phi) = 0 means y < phi ≈ 1.618, so y = 1
            y_max = min(int(phi), N)
        else:
            # f >= 1: f*phi <= y < (f+1)*phi
            y_min = int(Decimal(f) * phi) + 1
            if y_min <= int(Decimal(f) * phi):
                y_min = int(Decimal(f) * phi)
            # More carefully: y_min = ceil(f * phi)
            y_min_decimal = Decimal(f) * phi
            y_min = int(y_min_decimal)
            if Decimal(y_min) < y_min_decimal:
                y_min += 1

            y_max = int(Decimal(f + 1) * phi)
            # y < (f+1)*phi, so y <= floor((f+1)*phi) - 1 if (f+1)*phi is not integer
            # But (f+1)*phi is irrational, so y <= floor((f+1)*phi)
            # Actually: floor(y/phi) < f+1 means y/phi < f+1, so y < (f+1)*phi
            # Integer y: y <= floor((f+1)*phi - epsilon) = floor((f+1)*phi) if (f+1)*phi not integer
            # Since phi is irrational, y_max = floor((f+1)*phi)
            y_max = min(y_max, N)

        if y_max < y_min or y_max < 1:
            f += 1
            continue

        count = y_max - y_min + 1
        # sum of y in [y_min, y_max]
        sum_y_range_mod = ((y_min % M) + (y_max % M)) % M * ((count % M) * mod_inv(2, M) % M) % M

        sum_f = (sum_f + (f % M) * (count % M)) % M
        sum_f2 = (sum_f2 + (f % M) * (f % M) % M * (count % M)) % M
        sum_yf = (sum_yf + (f % M) * sum_y_range_mod) % M

        f += 1

    part2 = (sum_f2 + sum_f) % M * mod_inv(2, M) % M
    part3 = sum_yf % M

    ans = (part1 - part2 - part3 + 2 * M) % M
    return ans

if __name__ == "__main__":
    print(solve())
