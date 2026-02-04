"""Project Euler Problem 644: Squares on the Line."""

import math
from heapq import heappop, heappush


def feq(a, b, eps=1e-10):
    return abs(a - b) < eps


def solve():
    A = 200
    B = 500
    SQRT2 = math.sqrt(2)

    # Phase 1: Compute nimbers using event-based system
    # nimbers is a sorted list of (pos, nimber_value) pairs
    nimber_positions = [0.0]
    nimber_values = [0]

    # currentNimbers multiset
    cn_counts = {}

    def ms_add(val):
        cn_counts[val] = cn_counts.get(val, 0) + 1

    def ms_remove(val):
        c = cn_counts.get(val, 0)
        if c <= 1:
            cn_counts.pop(val, None)
        else:
            cn_counts[val] = c - 1

    def ms_contains(val):
        return cn_counts.get(val, 0) > 0

    # Events: (pos, is_add_int, value) - use int for is_add to avoid comparison issues
    # is_add_int: 0 = add (True), 1 = remove (False) - so adds are processed first at same pos
    events = []
    counter = 0  # tiebreaker

    def push_event(pos, is_add, value):
        nonlocal counter
        heappush(events, (pos, 0 if is_add else 1, counter, value))
        counter += 1

    push_event(1.0, True, 0)

    while events:
        event_pos, is_add_int, _, event_value = heappop(events)
        if event_pos > B:
            break

        if is_add_int == 0:
            ms_add(event_value)
        else:
            ms_remove(event_value)

        if events and feq(events[0][0], event_pos):
            continue

        nimber = 1
        while ms_contains(nimber):
            nimber += 1

        if nimber == nimber_values[-1]:
            continue

        nimber_positions.append(event_pos)
        nimber_values.append(nimber)

        n_entries = len(nimber_positions)
        for i in range(n_entries):
            pos = nimber_positions[i]
            xor_val = nimber ^ nimber_values[i]
            push_event(event_pos + pos + 1, True, xor_val)
            push_event(event_pos + pos + SQRT2, True, xor_val)
            if pos != 0:
                # lowerEntry(lastKey) = second to last entry
                prev_last_nimber = nimber_values[-2]
                # lowerEntry(pos) = entry with key strictly less than pos
                # Since nimber_positions is sorted, find the one before index i
                prev_pos_nimber = nimber_values[i - 1] if i > 0 else 0
                new_nimber = prev_last_nimber ^ prev_pos_nimber
                push_event(event_pos + pos + 1, False, new_nimber)
                push_event(event_pos + pos + SQRT2, False, new_nimber)

    # Phase 2: Compute sizes
    sizes = []
    for a in range(1, B + 1):
        b = 1
        while True:
            size = a + b * SQRT2
            if size > B:
                break
            if size >= A:
                sizes.append(size)
            b += 1
    sizes.sort()

    # Phase 3: Build ranges grouped by nimber value
    from collections import defaultdict
    ranges_map = defaultdict(list)
    for i in range(len(nimber_positions) - 1):
        nim_val = nimber_values[i]
        ranges_map[nim_val].append((nimber_positions[i], nimber_positions[i + 1]))

    # Phase 4: Build RangePair events
    events2 = []
    counter2 = 0
    pair_id = 0
    pair_by_id = {}

    for nim_val, rng_list in ranges_map.items():
        for r1 in rng_list:
            for r2 in rng_list:
                pid = pair_id
                pair_id += 1
                pair_by_id[pid] = (r1, r2)
                heappush(events2, (r1[0] + r2[0], 0, pid))  # 0 = add
                heappush(events2, (r1[1] + r2[1], 1, pid))  # 1 = remove

    # Phase 5: For each size, compute probability
    active_pairs = set()
    probs = {}

    for size in sizes:
        while events2 and events2[0][0] < size:
            _, is_remove, pid = heappop(events2)
            if is_remove == 0:
                active_pairs.add(pid)
            else:
                active_pairs.discard(pid)

        prob = 0.0
        for pid in active_pairs:
            r1, r2 = pair_by_id[pid]
            intersection = min(r1[1], size - r2[0]) - max(r1[0], size - r2[1])
            if intersection > 0:
                prob += intersection / size

        probs[size] = prob

    # Phase 6: Find maximum L * f(L)
    # Build a lookup that maps size -> prob, using approximate matching
    # Java uses probs.getOrDefault(size - 1, 0.) and probs.getOrDefault(size - sqrt(2), 0.)
    # These are exact lookups by the double key. Since our sizes are a+b*sqrt(2),
    # size-1 = (a-1)+b*sqrt(2) and size-sqrt(2) = a+(b-1)*sqrt(2), both of which are in the sizes set.

    ans = 0.0
    # Build index for fast lookup
    prob_index = {}
    for s in sizes:
        # Round to avoid floating point mismatch
        prob_index[round(s * 1e8)] = probs[s]

    for size in sizes:
        s1 = size - 1
        s2 = size - SQRT2
        k1 = round(s1 * 1e8)
        k2 = round(s2 * 1e8)
        p1 = prob_index.get(k1, 0.0)
        p2 = prob_index.get(k2, 0.0)
        L_val = size * (p1 + p2) / 2
        if L_val > ans:
            ans = L_val

    return f"{ans:.8f}"


if __name__ == "__main__":
    print(solve())
