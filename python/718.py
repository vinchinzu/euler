"""Project Euler Problem 718: Unreachable Numbers.

Find the sum of all positive integers that cannot be expressed as A*a + B*b +
C*c for positive integers a,b,c.

Greedily compute all integers that can be expressed as A + B*b + C*c. For each
such integer k, all of k+A, k+2A, ... can all be expressed, so if k = Aq+r,
then r,A+r,...A(q-1)+r are the only integers ≡ A (mod r) that cannot be
expressed.

As an optimization, instead of using a single priority queue to compute these
integers, we use two normal queues, one for adding B and the other for adding
C, because each of those will already be in the correct order.
"""

from __future__ import annotations

from collections import deque


def ncr(n: int, r: int) -> int:
    """Binomial coefficient C(n, r)."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    result = 1
    for i in range(min(r, n - r)):
        result = result * (n - i) // (i + 1)
    return result


def imod(a: int, m: int) -> int:
    """Modulo operation: a mod m."""
    return a % m


def solve() -> int:
    """Solve Problem 718."""
    n = 6
    a = 17**n
    b = 19**n
    c = 23**n
    m = 10**9 + 7

    visited = [False] * a
    pq1: deque[int] = deque()
    pq2: deque[int] = deque()
    val = a + b + c
    ans = 0

    while True:
        visited[imod(val, a)] = True
        q = val // a
        r = val % a
        ans = (ans + ncr(q, 2) * a + r * q) % m

        next_val1 = val + b
        next_val2 = val + c

        if not visited[imod(next_val1, a)]:
            pq1.append(next_val1)
        if not visited[imod(next_val2, a)]:
            pq2.append(next_val2)

        if not pq1 and not pq2:
            break

        val1 = pq1[0] if pq1 else None
        val2 = pq2[0] if pq2 else None
        if val1 is None:
            val = val2
        elif val2 is None:
            val = val1
        else:
            val = min(val1, val2)

        if pq1 and val == pq1[0]:
            pq1.popleft()
        if pq2 and val == pq2[0]:
            pq2.popleft()

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
