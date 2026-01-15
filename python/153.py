"""Project Euler Problem 153: Investigating Gaussian Integers."""

from typing import Dict
import math


class GaussianDivisorSum:
    """Calculate sum of Gaussian integer divisors."""

    def __init__(self, limit: int) -> None:
        """Initialize with limit."""
        self.limit = limit
        self.g_memo: Dict[int, int] = {}

    def g_function(self, n: int) -> int:
        """
        Calculate G(n) = sum_{i=1 to n} sigma_1(i).

        Uses the standard divisor sum formula with hyperbola method.
        """
        if n in self.g_memo:
            return self.g_memo[n]
        if n == 0:
            return 0

        result = 0
        k = 1

        while k <= n:
            q = n // k
            # Find the largest k' such that floor(n/k') = q
            next_k = n // q + 1

            # Sum k * q for k in [k, min(next_k-1, n)]
            last_k = min(next_k - 1, n)
            count = last_k - k + 1
            sum_k = count * (k + last_k) // 2
            result += sum_k * q

            k = next_k

        self.g_memo[n] = result
        return result

    def solve(self) -> int:
        """Solve the main problem by combining the components."""
        sqrt_limit = int(math.sqrt(self.limit))

        # Calculate S1 = G(limit)
        total_sum = self.g_function(self.limit)

        # This part calculates:
        # S2_prime = sum_{u>v>=1, gcd=1} (u+v)*G(floor(N/d))
        # + sum_{u=v=1} u*G(floor(N/d))
        s2_prime = 0

        # Case u = v = 1
        s2_prime += self.g_function(self.limit // 2)

        # Case u > v >= 1
        for u in range(2, sqrt_limit + 1):
            for v in range(1, u):
                if math.gcd(u, v) != 1:
                    continue

                d = u * u + v * v
                if d > self.limit:
                    break

                s2_prime += (u + v) * self.g_function(self.limit // d)

        return total_sum + 2 * s2_prime


def main() -> int:
    """Main function."""
    solver = GaussianDivisorSum(10 ** 8)
    return solver.solve()


if __name__ == "__main__":
    print(main())
