"""Project Euler Problem 833: Triangular Square.

Find the sum of c for all integer triples (a,b,c) such that c<=N, a<b, and
the product of the triangular numbers T_a and T_b is a perfect square, c^2.
"""

from math import gcd
from functools import lru_cache

def solve():
    N = 10**35
    M = 136101521

    # Precompute Bernoulli numbers for sum of powers
    # B_0=1, B_1=-1/2, B_2=1/6, B_3=0, B_4=-1/30, etc.
    # We'll compute modular Bernoulli numbers

    def mod_inv(a, m):
        return pow(a, -1, m)

    # Sum of k-th powers from 1 to n mod M using Faulhaber's formula
    # Sum_{i=1}^n i^k = (1/(k+1)) * sum_{j=0}^k binom(k+1,j) * B_j * n^{k+1-j}
    # where B_j are Bernoulli numbers

    # Precompute Bernoulli numbers mod M
    MAX_K = 100  # Maximum power we might need

    # Bernoulli numbers: B[k] stored as numerator mod M (already divided properly)
    # Using the recurrence: sum_{j=0}^{n} binom(n+1, j) B_j = 0 for n >= 1
    # So B_n = -1/(n+1) * sum_{j=0}^{n-1} binom(n+1, j) B_j

    # Compute binomial coefficients mod M
    fact = [1] * (MAX_K + 2)
    for i in range(1, MAX_K + 2):
        fact[i] = fact[i-1] * i % M

    inv_fact = [1] * (MAX_K + 2)
    inv_fact[MAX_K + 1] = mod_inv(fact[MAX_K + 1], M)
    for i in range(MAX_K, -1, -1):
        inv_fact[i] = inv_fact[i+1] * (i+1) % M

    def binom(n, k):
        if k < 0 or k > n:
            return 0
        return fact[n] * inv_fact[k] % M * inv_fact[n-k] % M

    # Compute Bernoulli numbers mod M
    B = [0] * (MAX_K + 1)
    B[0] = 1
    for n in range(1, MAX_K + 1):
        s = 0
        for j in range(n):
            s = (s + binom(n+1, j) * B[j]) % M
        B[n] = (-s * mod_inv(n+1, M)) % M

    # B[1] should be -1/2 mod M
    # Actually Bernoulli B_1 = -1/2, but in some conventions B_1 = 1/2
    # Let's verify: sum_{i=1}^n i = n(n+1)/2
    # Faulhaber: (1/2)(B_0*n^2 + 2*B_1*n^1) = (1/2)(n^2 + 2*(-1/2)*n) = (n^2-n)/2 = n(n-1)/2 - wrong!
    # The issue is the formula. Let me use a different approach.

    # Better: use polynomial interpolation for sum of powers
    # sum_{i=1}^n i^k is a polynomial of degree k+1 in n
    # We can compute it by evaluating at k+2 points and interpolating

    @lru_cache(maxsize=None)
    def sum_powers(n, k):
        """Sum of i^k for i=1 to n, mod M."""
        if n <= 0:
            return 0
        if k == 0:
            return n % M
        if k == 1:
            return n * (n + 1) // 2 % M
        if k == 2:
            return n * (n + 1) * (2*n + 1) // 6 % M
        if k == 3:
            s = n * (n + 1) // 2
            return s * s % M

        # For higher k, use Lagrange interpolation
        # sum_{i=1}^n i^k is a polynomial P(n) of degree k+1
        # Evaluate P at points 0, 1, 2, ..., k+1 and interpolate

        # P(0) = 0
        # P(j) = sum_{i=1}^j i^k for j >= 1

        y = [0] * (k + 2)
        y[0] = 0
        pw = 0
        for j in range(1, k + 2):
            pw = (pw + pow(j, k, M)) % M
            y[j] = pw

        # Lagrange interpolation at n
        # P(n) = sum_{j=0}^{k+1} y[j] * prod_{i != j} (n - i) / (j - i)

        n_mod = n % M

        # Compute prod_{i=0}^{k+1} (n - i) mod M
        num_prod = 1
        for i in range(k + 2):
            num_prod = num_prod * ((n_mod - i) % M) % M

        result = 0
        for j in range(k + 2):
            # term = y[j] * prod_{i != j} (n - i) / (j - i)
            # = y[j] * num_prod / (n - j) * 1 / prod_{i != j} (j - i)

            if (n_mod - j) % M == 0:
                # n = j (mod M), special case - shouldn't happen for large n
                # Compute directly
                term = y[j]
                for i in range(k + 2):
                    if i != j:
                        term = term * ((n_mod - i) % M) % M * mod_inv((j - i) % M, M) % M
            else:
                # prod_{i != j} (j - i) = (-1)^{k+1-j} * j! * (k+1-j)!
                denom = ((-1) ** (k + 1 - j)) * fact[j] * fact[k + 1 - j]
                denom = denom % M

                term = y[j] * num_prod % M * mod_inv((n_mod - j) % M, M) % M * mod_inv(denom, M) % M

            result = (result + term) % M

        return result

    # Polynomial class for symbolic Pell equation solutions
    class Poly:
        def __init__(self, coeffs):
            self.c = list(coeffs)
            while len(self.c) > 1 and self.c[-1] == 0:
                self.c.pop()

        def eval(self, x):
            r = 0
            p = 1
            for c in self.c:
                r += c * p
                p *= x
            return r

        def __add__(self, other):
            l = max(len(self.c), len(other.c))
            r = [0] * l
            for i, v in enumerate(self.c):
                r[i] += v
            for i, v in enumerate(other.c):
                r[i] += v
            return Poly(r)

        def __mul__(self, other):
            if not self.c or not other.c:
                return Poly([0])
            r = [0] * (len(self.c) + len(other.c) - 1)
            for i, a in enumerate(self.c):
                for j, b in enumerate(other.c):
                    r[i+j] += a * b
            return Poly(r)

    # Pell equation: x^2 - D*y^2 = 1 where D = a(a+1)
    # Base solution: (x, y) = (2a+1, 2)
    # Recurrence: (x', y') = (x * x0 + D * y * y0, x * y0 + y * x0)

    D = Poly([0, 1, 1])  # a^2 + a = a(a+1)
    base_x = Poly([1, 2])  # 2a + 1
    base_y = Poly([2])  # 2

    x = base_x
    y = base_y
    ys = []

    # Generate solutions until c > N for a = 1
    # c = D * y1 * y2 / 8, and for a=1, D=2
    while y.eval(1) < N:
        ys.append(y)
        new_x = x * base_x + D * y * base_y
        new_y = x * base_y + y * base_x
        x, y = new_x, new_y

    ans = 0
    for i in range(len(ys)):
        for j in range(i + 1, len(ys)):
            if gcd(i + 1, j + 1) == 1:
                # prod = D * y_i * y_j
                prod = D * ys[i] * ys[j]

                # Binary search for max a such that prod(a) / 8 <= N
                lo, hi = 0, 2**60
                while lo + 1 < hi:
                    mid = (lo + hi) // 2
                    if prod.eval(mid) // 8 <= N:
                        lo = mid
                    else:
                        hi = mid

                # Sum prod(a) for a = 1 to lo
                # prod is a polynomial, so sum each term
                for e, coeff in enumerate(prod.c):
                    if coeff != 0:
                        ans = (ans + sum_powers(lo, e) * (coeff % M)) % M

    ans = ans * mod_inv(8, M) % M
    return ans


if __name__ == "__main__":
    print(solve())
