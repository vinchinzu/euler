"""Project Euler Problem 813: XOR Power.

Let the XOR product x ⊗ y be the bitwise XOR of x and y, where the columns
in long multiplication are XORed instead of added with carry. Find the XOR
power K^N.

We compute it directly with repeated squaring, but instead of tracking the
full binary result, we only track the placements of the 1s because the 1s
in the binary are sparse.
"""

from __future__ import annotations

from typing import Set


def xor_multiply(ones1: Set[int], ones2: Set[int]) -> Set[int]:
    """Multiply two sets of bit positions using XOR multiplication."""
    result: Set[int] = set()
    for one1 in ones1:
        for one2 in ones2:
            pos = one1 + one2
            if pos in result:
                result.remove(pos)
            else:
                result.add(pos)
    return result


def solve() -> int:
    """Solve Problem 813."""
    N = 8**12 * 12**8
    K = 11
    M = 10**9 + 7

    # Convert K to set of bit positions
    k_bits: Set[int] = set()
    temp_k = K
    while temp_k > 0:
        bit_pos = (temp_k & -temp_k).bit_length() - 1
        k_bits.add(bit_pos)
        temp_k -= temp_k & -temp_k

    # Compute K^N using repeated squaring
    ones: Set[int] = {0}
    n = N
    while n > 0:
        if n & 1:
            ones = xor_multiply(ones, k_bits)
        k_bits = xor_multiply(k_bits, k_bits)
        n >>= 1

    # Sum 2^pos mod M for each position
    ans = 0
    for pos in ones:
        ans = (ans + pow(2, pos, M)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
