"""Project Euler Problem 551: Sum of digits sequence.

Find a_N if a_0 = 1 and a_n = a_{n-1} + (sum of digits of a_{n-1}).

Suppose that a_i = q*m + r, where m is a power of 10. Then a_{i+1} = q*m + r +
sumDigits(q) + sumDigits(r), and more importantly, for any term a_j = q'*m + r
where sumDigits(q) = sumDigits(q'), the succeeding terms will have the same
successive differences until the terms become at least q*(m+1) or q'*(m+1). So
we can memoize the fact that given such an a_i, there is a future a_i' such that
i' = i + di and a_i' = a_i + da.

Now we can start with a_i = a_0 = 1, and repeatedly fetch a_i' as long as i' < N.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Dict, Tuple


@dataclass(frozen=True)
class Key:
    """Key for memoization cache."""

    r: int
    m: int
    sum_q: int


@dataclass(frozen=True)
class Jump:
    """Represents a jump in the sequence."""

    di: int
    da: int


def sum_digits(n: int) -> int:
    """Return the sum of digits of n."""
    return sum(int(d) for d in str(n))


def solve() -> int:
    """Solve Problem 551."""
    N = 10**15
    B = 10

    cache: Dict[Key, Jump] = {}

    def get_jump(r: int, m: int, sum_q: int) -> Jump:
        """Get jump information for given state."""
        key = Key(r, m, sum_q)
        if key in cache:
            return cache[key]

        di = 0
        da = 0
        while r + da < m:
            if m <= B**3:  # cb(B) = B^3
                di += 1
                da += sum_q + sum_digits(r + da)
            else:
                jump = get_jump(
                    (r + da) % (m // B),
                    m // B,
                    sum_q + sum_digits((r + da) // (m // B)),
                )
                di += jump.di
                da += jump.da

        result = Jump(di, da)
        cache[key] = result
        return result

    di = 0
    ans = 1
    m = N
    while m >= 1:
        while True:
            jump = get_jump(ans % m, m, sum_digits(ans // m))
            if di + jump.di >= N:
                break
            di += jump.di
            ans += jump.da
        m //= B

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
