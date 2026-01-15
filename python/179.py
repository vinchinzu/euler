"""Project Euler Problem 179: Consecutive positive divisors."""

from typing import List

LIMIT = 10_000_000


def main() -> int:
    """Main function."""
    spf: List[int] = [0] * (LIMIT + 1)
    tau: List[int] = [1] * (LIMIT + 1)
    exponent: List[int] = [0] * (LIMIT + 1)
    primes: List[int] = []

    tau[1] = 1

    for i in range(2, LIMIT + 1):
        if spf[i] == 0:
            spf[i] = i
            primes.append(i)
            tau[i] = 2
            exponent[i] = 1

        for p in primes:
            if p * i > LIMIT:
                break
            spf[p * i] = p

            if p == spf[i]:
                exponent[p * i] = exponent[i] + 1
                tau[p * i] = tau[i] // (exponent[i] + 1) * (exponent[p * i] + 1)
                break
            else:
                exponent[p * i] = 1
                tau[p * i] = tau[i] * 2

    count = 0
    for n in range(2, LIMIT):
        if tau[n] == tau[n + 1]:
            count += 1

    return count


if __name__ == "__main__":
    print(main())
