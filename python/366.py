"""Project Euler Problem 366 - Stone game with restricted moves.

A game with n stones where first player removes some (but not all), and
subsequent players cannot remove more than twice the previous move.
M(n) = largest first move guaranteeing a win, or 0 if losing position.
Find sum of M(n) for n=1 to 10^18, modulo 10^8.

Uses Fibonacci-based recursive formula from Java implementation.
"""

def solve():
    N = 10**18
    M = 10**8

    # Generate Fibonacci numbers up to N
    fibs = [1, 2]
    while fibs[-1] <= N:
        fibs.append(fibs[-1] + fibs[-2])

    def tr(n, mod):
        """Triangular sum: 0+1+2+...+n mod mod."""
        if n < 0:
            return 0
        n = n % mod
        if n % 2 == 0:
            return (n // 2) * (n + 1) % mod
        else:
            return n * ((n + 1) // 2) % mod

    def sum_range(start, end):
        """Compute sum of M(n) for n in [start, end]."""
        if start > end:
            return 0

        # Find largest Fibonacci <= start
        fibonacci = 1
        for f in fibs:
            if f <= start:
                fibonacci = f
            else:
                break

        # max_identity is the largest k such that M(fibonacci + k) = k
        # This happens when fibonacci + k <= fibonacci + (fibonacci - 1) / 2
        # So k <= (fibonacci - 1) / 2
        max_identity = min((fibonacci - 1) // 2, end - fibonacci)

        # Sum for identity range: M(fibonacci + k) = k for k in [start-fibonacci, max_identity]
        # This is tr(max_identity) - tr(start - fibonacci - 1)
        result = tr(max_identity, M) - tr(start - fibonacci - 1, M)

        # Recursive range: M(n) = M(n - fibonacci) for n > fibonacci + max_identity
        result += sum_range(max_identity + 1, end - fibonacci)

        return result % M

    ans = 0
    for i in range(len(fibs) - 1):
        if fibs[i] > N:
            break
        start = fibs[i]
        end = min(fibs[i + 1] - 1, N)
        if end >= start:
            ans = (ans + sum_range(start, end)) % M

    return ans


if __name__ == "__main__":
    print(solve())
