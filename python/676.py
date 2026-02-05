#!/usr/bin/env python3
"""
Project Euler 676 - Matching Digit Sums

Find sum of all (k,l) pairs where d(i, 2^k) = d(i, 2^l) for i <= 10^16.
Uses digit DP with LCM-based chunking.
"""
from collections import defaultdict
from math import gcd

def M(N, k, l):
    """Compute sum of i <= N where d(i, 2^k) = d(i, 2^l)."""
    g = gcd(k, l)
    lcm = k * l // g
    max_v = 1 << lcm

    def get_diff(v):
        sk = sum((v >> (i*k)) & ((1<<k)-1) for i in range(lcm // k))
        sl = sum((v >> (i*l)) & ((1<<l)-1) for i in range(lcm // l))
        return sl - sk

    diff_lookup = [get_diff(v) for v in range(max_v)]

    # Precompute delta statistics for free transitions
    delta_count = defaultdict(int)
    delta_digit_sum = defaultdict(int)
    for d in range(max_v):
        delta = diff_lookup[d]
        delta_count[delta] += 1
        delta_digit_sum[delta] += d

    def free_transition(states, pos_val):
        new_states = defaultdict(lambda: (0, 0))
        for old_diff, (cnt, sm) in states.items():
            if cnt == 0:
                continue
            for delta, dc in delta_count.items():
                new_diff = old_diff + delta
                ds = delta_digit_sum[delta]
                ocnt, osm = new_states[new_diff]
                new_states[new_diff] = (
                    ocnt + cnt * dc,
                    osm + dc * sm + cnt * ds * pos_val
                )
        return new_states

    # Convert N to base max_v
    digits = []
    n = N
    while n > 0:
        digits.append(n % max_v)
        n //= max_v
    digits = digits[::-1]

    if not digits:
        return 0

    # Digit DP
    tight = {0: (1, 0)}
    free = {}

    pos_val = max_v ** (len(digits) - 1)

    for pos, limit in enumerate(digits):
        new_tight = defaultdict(lambda: (0, 0))
        new_free = defaultdict(lambda: (0, 0))

        # Tight transitions
        for old_diff, (cnt, sm) in tight.items():
            if cnt == 0:
                continue
            for d in range(limit + 1):
                new_diff = old_diff + diff_lookup[d]
                contrib = d * pos_val
                if d < limit:
                    ocnt, osm = new_free[new_diff]
                    new_free[new_diff] = (ocnt + cnt, osm + sm + cnt * contrib)
                else:
                    ocnt, osm = new_tight[new_diff]
                    new_tight[new_diff] = (ocnt + cnt, osm + sm + cnt * contrib)

        # Free transitions
        if free:
            free_new = free_transition(free, pos_val)
            for diff, (cnt, sm) in free_new.items():
                ocnt, osm = new_free[diff]
                new_free[diff] = (ocnt + cnt, osm + sm)

        tight = dict(new_tight)
        free = dict(new_free)
        pos_val //= max_v

    total = 0
    if 0 in tight:
        total += tight[0][1]
    if 0 in free:
        total += free[0][1]

    return total

def solve():
    total = 0
    for k in range(3, 7):
        for l_val in range(1, k - 1):
            total += M(10**16, k, l_val)
    return total % (10**16)

if __name__ == "__main__":
    print(solve())
