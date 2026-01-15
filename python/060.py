#!/usr/bin/env python3
"""
Project Euler Problem 60: Prime pair sets

Find the lowest sum for a set of five primes for which any two primes concatenate
to form another prime.
"""

import math
from sympy import primerange

TARGET_SIZE = 5
DEFAULT_LIMIT = 8400


class PrimePairSetsSolver:
    def __init__(self, prime_limit=DEFAULT_LIMIT):
        # Get primes excluding 2 and 5 (they can never form valid pairs with 5)
        self.primes = [p for p in primerange(2, prime_limit) if p != 2 and p != 5]
        self.pair_cache = {}
        self.prime_cache = {}
        self._build_helpers()
        self._build_graph()
        self._prune_candidates()
        self._build_forward_lists()
        self.best_sum = float('inf')

    def find_lowest_sum_set(self):
        """Find lowest sum set of TARGET_SIZE primes."""
        self._search([], self.candidates, 0)
        return self.best_sum if self.best_sum != float('inf') else None

    def _build_helpers(self):
        """Build helper dictionaries for digit lengths, mod 3, and powers of 10."""
        self.digit_lengths = {}
        self.mod_three = {}
        for p in self.primes:
            self.digit_lengths[p] = len(str(p))
            self.mod_three[p] = p % 3

        max_digits = max(self.digit_lengths.values())
        self.ten_powers = [10 ** i for i in range(max_digits + 1)]

        max_prime = self.primes[-1]
        max_concat = max_prime * self.ten_powers[self.digit_lengths[max_prime]] + max_prime
        limit = int(math.sqrt(max_concat)) + 1
        self.trial_primes = list(primerange(2, limit))

    def _build_graph(self):
        """Build adjacency graph of primes that can concatenate to form primes."""
        self.adjacency = {p: [] for p in self.primes}
        primes_len = len(self.primes)

        for i, p in enumerate(self.primes):
            for j in range(i + 1, primes_len):
                q = self.primes[j]
                if self._concatenated_pair(p, q):
                    self.adjacency[p].append(q)
                    self.adjacency[q].append(p)

    def _prune_candidates(self):
        """Prune candidates to only those with sufficient connections."""
        self.candidates = [p for p in self.primes if p in self.adjacency]

        # Iteratively prune candidates with insufficient connections
        while True:
            lookup = {p: True for p in self.candidates}
            filtered = []
            for p in self.candidates:
                neighbors = 0
                for q in self.adjacency[p]:
                    if q in lookup:
                        neighbors += 1
                if neighbors >= TARGET_SIZE - 1:
                    filtered.append(p)

            if len(filtered) == len(self.candidates):
                break
            self.candidates = filtered

        # Filter adjacency to only include remaining candidates
        lookup = {p: True for p in self.candidates}
        self.adjacency = {p: [q for q in self.adjacency[p] if q in lookup] for p in self.candidates}
        self.candidates.sort()

    def _build_forward_lists(self):
        """Build forward lists (only neighbors greater than current prime)."""
        self.forward_map = {}
        for p in self.candidates:
            self.forward_map[p] = sorted([q for q in self.adjacency[p] if q > p])

    def _concatenated_pair(self, a, b):
        """Check if concatenating a and b in either order forms primes."""
        if a < b:
            key = (a << 16) | b
            small, large = a, b
        else:
            key = (b << 16) | a
            small, large = b, a

        if key in self.pair_cache:
            return self.pair_cache[key]

        # Quick rejection if sum of mod 3 is 0, both concatenated numbers are divisible by 3
        if (self.mod_three[small] + self.mod_three[large]) % 3 == 0:
            self.pair_cache[key] = False
            return False

        ab = self._concat_numbers(small, large)
        ba = self._concat_numbers(large, small)

        result = self._is_prime(ab) and self._is_prime(ba)
        self.pair_cache[key] = result
        return result

    def _concat_numbers(self, a, b):
        """Concatenate two numbers: a * 10^digits(b) + b"""
        return a * self.ten_powers[self.digit_lengths[b]] + b

    def _is_prime(self, n):
        """Check if n is prime using trial division."""
        if n < 2:
            return False
        if n == 2 or n == 5:
            return False

        if n in self.prime_cache:
            return self.prime_cache[n]

        limit = int(math.sqrt(n))
        for p in self.trial_primes:
            if p > limit:
                break
            if n % p == 0:
                self.prime_cache[n] = False
                return False

        self.prime_cache[n] = True
        return True

    def _search(self, clique, candidates, current_sum):
        """Depth-first search with pruning."""
        if len(clique) == TARGET_SIZE:
            if current_sum < self.best_sum:
                self.best_sum = current_sum
            return

        needed = TARGET_SIZE - len(clique)
        for idx, prime in enumerate(candidates):
            # Prune if adding this prime makes sum too large
            if current_sum + prime * needed >= self.best_sum:
                continue

            forward = self.forward_map[prime]
            remaining = TARGET_SIZE - len(clique) - 1
            if remaining > 0 and len(forward) < remaining:
                continue

            # Find intersection of candidates (from idx+1) and forward neighbors
            next_candidates = self._intersect_sorted(candidates, idx + 1, forward)
            if remaining > 0 and len(next_candidates) < remaining:
                continue

            # Prune: calculate minimum possible sum
            min_sum = current_sum + prime
            if remaining > 0:
                limit = min(remaining, len(next_candidates))
                for i in range(limit):
                    min_sum += next_candidates[i]
                if min_sum >= self.best_sum:
                    continue

            self._search(clique + [prime], next_candidates, current_sum + prime)

    def _intersect_sorted(self, candidates, start_idx, forward):
        """Find intersection of two sorted lists."""
        if not forward or start_idx >= len(candidates):
            return []

        result = []
        i, j = start_idx, 0

        while i < len(candidates) and j < len(forward):
            cand = candidates[i]
            comp = forward[j]
            if cand == comp:
                result.append(cand)
                i += 1
                j += 1
            elif cand < comp:
                i += 1
            else:
                j += 1

        return result


def main():
    solver = PrimePairSetsSolver()
    answer = solver.find_lowest_sum_set()
    print(answer if answer else "No solution found")


if __name__ == "__main__":
    main()
