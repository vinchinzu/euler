"""Project Euler Problem 290: Digital Signature.

Find the number of positive integers less than 10^N such that the sum of the
digits of n equals the sum of the digits of 137n.

Let counts(i, j, k) be the number of positive integers n with i digits such
that:
- The sum of the digits of 137n and the sum of the digits of n have a
  difference of j.
- If 137n has r more digits than n, then k is the number consisting of the
  first r digits, otherwise 0.

For each n with i digits, we can append any digit from 0 to 9 to the
beginning. Multiplying that digit by 137 and adding to the current k gives a
number t. The new value of j can be computed by subtracting the sum of the
digits of the current carry k, adding the digits of t, and then subtracting
the digit d. The new value of k is just t shifted over by 1 digit, or ⌊t/10⌋.

We iterate over all such numbers up to N digits, and finally the answer is
the sum of all n where i=N and j=0.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Dict


@dataclass(frozen=True)
class Key:
    """Key for memoization."""

    num_digits: int
    diff: int
    carry: int


def sum_digits(n: int) -> int:
    """Sum of digits of n."""
    return sum(int(d) for d in str(n))


def solve() -> int:
    """Solve Problem 290."""
    N = 18
    K = 137
    B = 10

    counts: Dict[Key, int] = {Key(0, 0, 0): 1}

    for i in range(N):
        new_counts: Dict[Key, int] = {}
        for j in range(-B * N, B * N + 1):
            for k in range(K):
                key = Key(i, j, k)
                count = counts.get(key)
                if count is None:
                    continue
                for d in range(B):
                    t = d * K + k
                    new_j = j - sum_digits(k) + sum_digits(t) - d
                    new_k = t // B
                    new_key = Key(i + 1, new_j, new_k)
                    new_counts[new_key] = new_counts.get(new_key, 0) + count
        counts = new_counts

    ans = 0
    for carry in range(K):
        key = Key(N, 0, carry)
        ans += counts.get(key, 0)

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
