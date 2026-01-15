"""Project Euler Problem 131.

There are some prime values, p, for which there exists a positive integer, n,
such that the expression n³ + n²p is a perfect cube.
For example, when p = 19, 8³ + 8² × 19 = 12³.
What is perhaps most surprising is that for each prime with this property
the value of n is unique, and there are only four such primes below one-hundred.
How many primes below one million have this remarkable property?

Mathematical parametrization: All solutions are of the form
n = a³, p = 3a² + 3a + 1, k = a²(a+1), where a ≥ 1 is a positive integer
and p must be prime. This comes from solving n²(n + p) = k³ with gcd(n, n+p) = 1.
"""

from sympy import isprime


def main() -> int:
    """Main function."""
    count = 0
    a = 1
    while True:
        p = 3 * a * a + 3 * a + 1
        if p >= 1_000_000:
            break
        if isprime(p):
            count += 1
        a += 1
    return count


if __name__ == "__main__":
    print(main())
