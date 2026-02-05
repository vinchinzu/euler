"""Project Euler Problem 361 - Thue-Morse based sequence A_n.

Find sum_{k=1}^18 A(10^k) mod 10^9, where A(n) is the nth number (sorted)
whose binary representation appears as a contiguous subsequence of the
Thue-Morse sequence.

Algorithm (from Java reference):
- Use recurrences to compute firstIndexWithLen(l) - the first index with l bits
- Binary search to find the length of A(n)
- Recursively determine the position in Thue-Morse sequence
- Compute the prefix of Thue-Morse at that position

Key recurrences:
- l(n) = n for n <= 3
- l(2n) = l(n) + l(n+1)
- l(2n+1) = 2*l(n+1)

firstIndexWithLen recurrences:
- For len <= 3: 2^(len-1)
- For even len: firstIndex(len/2) + 3*firstIndex(len/2+1) - 7
- For odd len: 3*firstIndex(len/2+1) + firstIndex(len/2+2) - 7
"""

from functools import lru_cache


def solve():
    N = 18
    M = 10**9
    L = 10**10

    @lru_cache(maxsize=None)
    def firstIndexWithLen(length):
        """Return the first index n such that A(n) has 'length' bits."""
        if length <= 3:
            return 1 << (length - 1)
        elif length % 2 == 0:
            return firstIndexWithLen(length // 2) + 3 * firstIndexWithLen(length // 2 + 1) - 7
        else:
            return 3 * firstIndexWithLen(length // 2 + 1) + firstIndexWithLen(length // 2 + 2) - 7

    def numValuesWithLen(length):
        """Return the count of values A(n) that have exactly 'length' bits."""
        return firstIndexWithLen(length + 1) - firstIndexWithLen(length)

    def positionInT(length, index):
        """Find the position in Thue-Morse sequence for the index-th value of given length."""
        if length <= 1:
            return length
        nv = numValuesWithLen((length + 1) // 2)
        if index < nv:
            position = positionInT((length + 1) // 2, index)
            return position * 2
        else:
            position = positionInT(length // 2 + 1, numValuesWithLen(length) - index - 1)
            # highestOneBit equivalent: find the highest power of 2 <= (position + length - 1)
            val = position + length - 1
            if val <= 0:
                highest = 0
            else:
                highest = 1 << (val.bit_length() - 1)
            return (position + highest * 2) * 2 + 1

    @lru_cache(maxsize=None)
    def prefixOfT(length):
        """Compute the prefix of Thue-Morse of given length mod M."""
        if length == 0:
            return 0
        if length == 1:
            return 0
        # half = highest power of 2 <= (length - 1)
        half = 1 << ((length - 1).bit_length() - 1)
        return ((prefixOfT(half) + 1) * pow(2, length - half, M) - prefixOfT(length - half) - 1) % M

    def A(n):
        """Compute A(n) mod M."""
        # Binary search for the length of A(n)
        low, high = 0, L
        while low + 1 < high:
            mid = (low + high) // 2
            if firstIndexWithLen(mid) > n:
                high = mid
            else:
                low = mid
        length = low
        position = positionInT(length, n - firstIndexWithLen(length))
        return (prefixOfT(position + length) - prefixOfT(position) * pow(2, length, M)) % M

    ans = 0
    for k in range(1, N + 1):
        ans += A(10**k)
    ans = ans % M

    return ans


if __name__ == "__main__":
    print(solve())
