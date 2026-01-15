"""
Project Euler Problem 939: Nim Variant

Two players A and B play a variant of Nim.
E(N) = number of initial settings with at most N stones where A always wins.

Given: E(4) = 9
Find: E(5000) mod 1234567891

Time Complexity: O(N^2)
Space Complexity: O(N)
"""

MOD = 1234567891


def compute_partitions(n):
    """Compute partition function p(k) for k = 0 to n using pentagonal numbers."""
    p = [0] * (n + 1)
    p[0] = 1

    for i in range(1, n + 1):
        k = 1
        while True:
            # Pentagonal numbers: k*(3*k-1)/2 and k*(3*k+1)/2
            pent1 = k * (3 * k - 1) // 2
            pent2 = k * (3 * k + 1) // 2

            sign = 1 if k % 2 == 1 else -1

            if pent1 <= i:
                p[i] = (p[i] + sign * p[i - pent1]) % MOD
            if pent2 <= i:
                p[i] = (p[i] + sign * p[i - pent2]) % MOD

            if pent1 > i and pent2 > i:
                break

            k += 1

        # Ensure non-negative
        p[i] = (p[i] % MOD + MOD) % MOD

    return p


def compute_e(n, p):
    """Compute E(n) using partition function."""
    result = 0

    for k in range(1, n + 1):
        s_start = (k + 1) // 2
        for s in range(s_start, k + 1):
            t = k - s
            ways = (p[s] * p[t]) % MOD
            result = (result + ways) % MOD

    return result


def main():
    n = 5000

    print(f"Computing E({n}) mod {MOD}...")

    partitions = compute_partitions(n)

    # Verify with small example
    e4 = compute_e(4, partitions)
    print(f"E(4) = {e4} (expected: 9)")

    result = compute_e(n, partitions)
    print(result)
    return result


if __name__ == "__main__":
    main()
