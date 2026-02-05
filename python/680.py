#!/usr/bin/env python3
"""Project Euler Problem 680: Yarra Gnisrever.

Given an array A of n integers from 0 to n-1, and K operations of the form
"reverse the sub-array from index F_{2n-1} and F_{2n} inclusive (where F_n are
the Fibonacci numbers), find R = sum_{i=0}^{N-1} i A[i] for the final array.

Direct translation from Java reference implementation.
Uses a segment tree with lazy reversal.
"""


def solve():
    N = 10**18
    K = 10**6
    M = 10**9

    def sum_powers_1(n, mod):
        """Sum of 1 + 2 + ... + n mod m."""
        # = n * (n + 1) / 2
        n = n % (2 * mod)  # Ensure no overflow issues
        return (n * (n + 1) // 2) % mod

    def sum_powers_2(n, mod):
        """Sum of 1^2 + 2^2 + ... + n^2 mod m."""
        # = n * (n + 1) * (2n + 1) / 6
        n = n % (6 * mod)  # Ensure divisibility
        return (n * (n + 1) * (2 * n + 1) // 6) % mod

    class Node:
        __slots__ = ['len', 'first', 'diff', 'left', 'right']

        def __init__(self, len_or_left, first_or_right=None, diff=None):
            if first_or_right is None:
                # Node(left, right) constructor - but we pass as positional
                raise ValueError("Use create_leaf or create_internal")
            if diff is not None:
                # Leaf node: Node(len, first, diff)
                self.len = len_or_left
                self.first = first_or_right
                self.diff = diff
                self.left = None
                self.right = None
            else:
                # Internal node: create_internal handles this
                raise ValueError("Use create_internal")

        @staticmethod
        def create_leaf(length, first, diff):
            node = object.__new__(Node)
            node.len = length
            node.first = first
            node.diff = diff
            node.left = None
            node.right = None
            return node

        @staticmethod
        def create_internal(left, right):
            node = object.__new__(Node)
            node.len = left.len + right.len
            node.first = 0  # Not used for internal nodes
            node.diff = 1
            node.left = left
            node.right = right
            return node

        def canonicalize(self):
            if self.left is not None and self.diff == -1:
                left, right = self.left, self.right
                self.left = right.reverse()
                self.right = left.reverse()
                self.diff = 1

        def reverse(self):
            if self.left is None:
                self.first += self.diff * (self.len - 1)
            self.diff *= -1
            return self

        def do_reverse(self, start, end):
            self.canonicalize()
            if self.left is None:
                # Split into [0, start), [start, end) reversed, [end, len)
                self.left = Node.create_leaf(start, self.first, self.diff)
                mid = Node.create_leaf(end - start, self.first + self.diff * (end - 1), -self.diff)
                right_part = Node.create_leaf(self.len - end, self.first + self.diff * end, self.diff)
                self.right = Node.create_internal(mid, right_part)
                self.diff = 1
            elif end <= self.left.len:
                self.left.do_reverse(start, end)
            elif start >= self.left.len:
                self.right.do_reverse(start - self.left.len, end - self.left.len)
            else:
                self.left.ensure_cut_at(start)
                self.right.ensure_cut_at(end - self.left.len)
                left_right, right_left = self.left.right, self.right.left
                self.left.right = right_left.reverse()
                self.right.left = left_right.reverse()
                self.left.len = self.left.left.len + self.left.right.len
                self.right.len = self.right.left.len + self.right.right.len

        def ensure_cut_at(self, index):
            self.canonicalize()
            if self.left is None:
                self.left = Node.create_leaf(index, self.first, self.diff)
                self.right = Node.create_leaf(self.len - index, self.first + self.diff * index, self.diff)
                self.diff = 1
            elif index < self.left.len:
                self.left.ensure_cut_at(index)
                left_left, left_right = self.left.left, self.left.right
                self.left = left_left
                self.right = Node.create_internal(left_right, self.right)
            elif index > self.left.len:
                self.right.ensure_cut_at(index - self.left.len)
                right_left, right_right = self.right.left, self.right.right
                self.right = right_right
                self.left = Node.create_internal(self.left, right_left)

        def R(self, start):
            self.canonicalize()
            if self.len == 0:
                return 0
            if self.left is None:
                # Java: return (start % M * (first % M) % M * (len % M)
                #         + (start * diff + first) % M * sumPowers(len - 1, 1, M)
                #         + diff * sumPowers(len - 1, 2, M)) % M
                #
                # This computes sum_{i=0}^{len-1} (start + i) * (first + diff * i)
                # = sum_{i=0}^{len-1} (start * first + start * diff * i + first * i + diff * i^2)
                # = len * start * first + (start * diff + first) * sum(i) + diff * sum(i^2)
                # where sum(i) = 0+1+...+(len-1) = sumPowers(len-1, 1)
                # and sum(i^2) = 0+1+4+...+(len-1)^2 = sumPowers(len-1, 2)

                term1 = (start % M) * (self.first % M) % M * (self.len % M) % M
                coef = ((start * self.diff + self.first) % M + M) % M
                term2 = coef * sum_powers_1(self.len - 1, M) % M
                term3 = self.diff * sum_powers_2(self.len - 1, M) % M
                return (term1 + term2 + term3) % M
            return (self.left.R(start) + self.right.R(start + self.left.len)) % M

    # Precompute Fibonacci numbers mod N
    F = [0] * (2 * K + 1)
    F[1] = F[2] = 1
    for i in range(3, 2 * K + 1):
        F[i] = (F[i - 2] + F[i - 1]) % N

    # Create initial tree representing [0, 1, 2, ..., N-1]
    tree = Node.create_leaf(N, 0, 1)

    # Perform K reverse operations
    import sys
    sys.setrecursionlimit(5000000)

    for i in range(1, K + 1):
        s = F[2 * i - 1]
        t = F[2 * i]
        if s < t:
            tree.do_reverse(s, t + 1)
        else:
            tree.do_reverse(t, s + 1)

    ans = tree.R(0)
    return ans


def main():
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
