"""Project Euler Problem 325 - Stone Game.

Efficient implementation using Beatty sequence properties for golden ratio.
"""
from decimal import Decimal, getcontext
getcontext().prec = 100

def solve():
    N = 10**16
    M = 7**10

    # For modular arithmetic with large numbers
    def mod_inv(a, m):
        return pow(a, m - 2, m)

    def sum_1_to_n(n, mod):
        """sum_{i=1}^n i mod mod = n*(n+1)/2"""
        return (n % mod) * ((n + 1) % mod) % mod * mod_inv(2, mod) % mod

    def sum_squares_1_to_n(n, mod):
        """sum_{i=1}^n i^2 mod mod = n*(n+1)*(2n+1)/6"""
        return (n % mod) * ((n + 1) % mod) % mod * ((2*n + 1) % mod) % mod * mod_inv(6, mod) % mod

    # Part 1: 3*(sum y^2 - sum y)/2
    sum_y2 = sum_squares_1_to_n(N, M)
    sum_y = sum_1_to_n(N, M)
    part1 = 3 * (sum_y2 - sum_y + M) % M * mod_inv(2, M) % M

    # For the floor sums with golden ratio, use the reciprocal Beatty sequence property
    # phi = (1 + sqrt(5))/2, phi_inv = phi - 1 = (sqrt(5) - 1)/2
    # The sequences floor(n*phi) and floor(n*phi_inv) partition the positive integers

    # Use high precision for phi
    from math import isqrt
    sqrt5 = Decimal(5).sqrt()
    phi = (1 + sqrt5) / 2
    phi_inv = (sqrt5 - 1) / 2

    def floor_phi_inv(y):
        """Compute floor(y / phi) = floor(y * phi_inv)"""
        return int(Decimal(y) * phi_inv)

    def find_y_max_for_f(f):
        """Find maximum y such that floor(y/phi) = f"""
        # floor(y/phi) = f means f <= y/phi < f+1
        # So f*phi <= y < (f+1)*phi
        return int(Decimal(f + 1) * phi) - 1

    # Compute the three sums using the hyperbola method
    # Group consecutive y values with the same floor(y/phi)

    sum_f = 0    # sum_{y=1}^N floor(y/phi)
    sum_f2 = 0   # sum_{y=1}^N floor(y/phi)^2
    sum_yf = 0   # sum_{y=1}^N y * floor(y/phi)

    # The maximum value of floor(N/phi) is approximately N*0.618
    max_f = floor_phi_inv(N)

    # For each value f from 0 to max_f, find the range of y values
    for f in range(max_f + 1):
        if f == 0:
            y_min = 1
        else:
            y_min = find_y_max_for_f(f - 1) + 1

        y_max = min(find_y_max_for_f(f), N)

        if y_max < y_min:
            continue

        count = y_max - y_min + 1
        # sum of y in range [y_min, y_max]
        sum_y_range = (y_min + y_max) * count // 2

        sum_f = (sum_f + f * count) % M
        sum_f2 = (sum_f2 + f * f % M * (count % M)) % M
        sum_yf = (sum_yf + f % M * (sum_y_range % M)) % M

    # Combine parts
    # ans = 3*(sum y^2 - sum y)/2 - (sum f^2 + sum f)/2 - sum y*f
    part2 = (sum_f2 + sum_f) % M * mod_inv(2, M) % M
    part3 = sum_yf

    ans = (part1 - part2 - part3) % M
    if ans < 0:
        ans += M

    return ans

if __name__ == "__main__":
    print(solve())
