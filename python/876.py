from math import gcd
from itertools import product

def solve():
    ans = 0

    def get_divisors(n, prime_factors):
        """Get all divisors of n given its prime factorization."""
        divs = [1]
        for p, e in prime_factors:
            new_divs = []
            pe = 1
            for _ in range(e + 1):
                for d in divs:
                    new_divs.append(d * pe)
                pe *= p
            divs = new_divs
        return divs

    for k in range(1, 19):
        a = 6 ** k   # 2^k * 3^k
        b = 10 ** k  # 2^k * 5^k

        # Prime factorizations
        a_factors = [(2, k), (3, k)]
        b_factors = [(2, k), (5, k)]

        a_divs = get_divisors(a, a_factors)
        b_divs = get_divisors(b, b_factors)

        f = {}  # c -> min numSteps

        for y in a_divs:
            for z in b_divs:
                if gcd(y, z) != 1:
                    continue
                c = (y + z) * (a // y + b // z)
                # Compute numSteps using Euclidean-like algorithm
                ly, lz = y, z
                numSteps = 0
                i = 0  # start with l[0] = y
                while True:
                    if i == 0:
                        if ly == 0:
                            break
                        numSteps += lz // ly
                        lz %= ly
                        i = 1
                    else:
                        if lz == 0:
                            break
                        numSteps += ly // lz
                        ly %= lz
                        i = 0

                if c not in f or numSteps < f[c]:
                    f[c] = numSteps

        for c, numSteps in f.items():
            ans += numSteps
            if c < 2 * (a + b):
                ans += numSteps - 1

    print(ans)

if __name__ == "__main__":
    solve()
