"""Project Euler Problem 472: Lightbulb placement.

People sit in a row of N seats, the first person choosing any seat and each
subsequent person choosing a seat furthest from anyone else already seated.
Find sum_{k=1}^N f(k) where f(k) is the number of chairs where the first
person can sit such that the total number of seated people is maximized.

Uses the pattern-based approach from the Java solution.
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 472."""
    N = 10**12
    M = 10**8

    def tr(n, m):
        """Triangular number n*(n+1)/2 mod m."""
        return n * (n + 1) // 2 % m

    def sumF(n):
        F = [1, 2, 2, 4, 3, 6, 2, 6, 3]
        sumf = 0
        index = 0
        for i in range(9):
            sumf = (sumf + F[i]) % M
            index += 1
            if index >= n:
                return sumf

        while True:
            length = index // 4  # len in Java
            sumf = (sumf + 8) % M
            index += 1
            if index >= n:
                return sumf

            l = 1
            while l <= length // 2:
                # Rising range
                count = min(n - index, l)
                sumf = (sumf + 2 * tr(count, M)) % M
                index += l
                if index >= n:
                    return sumf

                # Peak
                sumf = (sumf + 2 * (2 * l + 1)) % M
                index += 1
                if index >= n:
                    return sumf

                # Falling range
                count2 = max(l - (n - index), 1)
                sumf = (sumf + 2 * (tr(l, M) - tr(count2, M))) % M
                index += l - 1
                if index >= n:
                    return sumf

                l *= 2

            # Center rising range
            count = min(n - index, length)
            sumf = (sumf + 2 * tr(count, M)) % M
            index += length
            if index >= n:
                return sumf

            # Center peak
            sumf = (sumf + 3 * (length + 1)) % M
            index += 1
            if index >= n:
                return sumf

            # Center falling range
            count2 = max(length - (n - index) + 2, 2)
            sumf = (sumf + tr(length + 2, M) - tr(count2, M)) % M
            index += length
            if index >= n:
                return sumf

    return sumF(N) % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
