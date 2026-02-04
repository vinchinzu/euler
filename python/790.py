"""Project Euler Problem 790: Clock Grid.

Find the sum of the values of a KxK grid of clocks all starting at 12
(o'clock), after N operations of the form "increment the hour hand for all
clocks with x_l <= x <= x_h and y_l <= y <= y_h" are performed, and clocks
cycle from 12 o'clock to 1 o'clock.

We use coordinate compression + a segment tree with lazy shifts.
"""

from __future__ import annotations

import array
from collections import defaultdict


def solve() -> int:
    """Solve Problem 790."""
    N = 100000
    K = 50515093
    T = 12

    # Generate BBS sequence: S_0=290797, S_{t+1} = S_t^2 mod K
    s = [0] * (4 * N)
    s[0] = 290797
    for i in range(1, 4 * N):
        s[i] = s[i - 1] * s[i - 1] % K

    # Build queries: N_t uses (S_{4t-4}, S_{4t-3}, S_{4t-2}, S_{4t-1})
    # Query(a, b, c, d) => x1=min(a,b), x2=max(a,b)+1, y1=min(c,d), y2=max(c,d)+1
    q_x1 = [0] * N
    q_x2 = [0] * N
    q_y1 = [0] * N
    q_y2 = [0] * N
    for t in range(N):
        base = 4 * t
        a, b, c, d = s[base], s[base + 1], s[base + 2], s[base + 3]
        if a <= b:
            q_x1[t] = a; q_x2[t] = b + 1
        else:
            q_x1[t] = b; q_x2[t] = a + 1
        if c <= d:
            q_y1[t] = c; q_y2[t] = d + 1
        else:
            q_y1[t] = d; q_y2[t] = c + 1

    # Coordinate compression for x and y
    xs_set = {0, K}
    ys_set = {0, K}
    for t in range(N):
        xs_set.add(q_x1[t])
        xs_set.add(q_x2[t])
        ys_set.add(q_y1[t])
        ys_set.add(q_y2[t])

    xs = sorted(xs_set)
    ys = sorted(ys_set)
    y_to_idx = {}
    for i, y in enumerate(ys):
        y_to_idx[y] = i

    ny = len(ys) - 1  # number of y intervals
    # Segment tree size
    seg_l = 1
    while seg_l < ny:
        seg_l *= 2
    tree_size = 2 * seg_l

    # Flatten hour_counts: tree_size * T entries
    # hour_counts[index * T + h] = count of clocks at hour h in node index
    hc = array.array('q', [0]) * (tree_size * T)
    shifts = array.array('i', [0]) * tree_size

    # Initialize leaf nodes
    for i in range(ny):
        hc[(seg_l + i) * T] = ys[i + 1] - ys[i]

    # merge function
    def merge(idx):
        left = 2 * idx
        right = 2 * idx + 1
        sl = shifts[left]
        sr = shifts[right]
        base_idx = idx * T
        base_l = left * T
        base_r = right * T
        for h in range(T):
            hc[base_idx + h] = hc[base_l + (h - sl) % T] + hc[base_r + (h - sr) % T]

    # Build tree bottom-up
    for i in range(seg_l - 1, 0, -1):
        merge(i)

    # Group queries by x
    add_queries = defaultdict(list)
    rem_queries = defaultdict(list)
    for t in range(N):
        yi1 = y_to_idx[q_y1[t]]
        yi2 = y_to_idx[q_y2[t]]
        add_queries[q_x1[t]].append((yi1, yi2))
        rem_queries[q_x2[t]].append((yi1, yi2))

    ans = 0
    prev_x = 0

    # Hour values: h=0 means 12 (value T), else h
    hval = list(range(T))
    hval[0] = T

    def update(from_idx, to_idx, diff, index, low, high):
        if from_idx >= high or to_idx <= low:
            return
        if from_idx <= low and to_idx >= high:
            shifts[index] += diff
            return
        mid = (low + high) >> 1
        update(from_idx, to_idx, diff, 2 * index, low, mid)
        update(from_idx, to_idx, diff, 2 * index + 1, mid, high)
        merge(index)

    root_base = T  # index 1, base = 1*T = T
    for x in xs:
        # Accumulate contribution from columns prev_x to x
        dx = x - prev_x
        if dx > 0:
            s1 = shifts[1]
            for h in range(T):
                ans += hval[h] * hc[root_base + (h - s1) % T] * dx

        # Process add queries at x
        if x in add_queries:
            for yi1, yi2 in add_queries[x]:
                update(yi1, yi2, 1, 1, 0, seg_l)

        # Process remove queries at x
        if x in rem_queries:
            for yi1, yi2 in rem_queries[x]:
                update(yi1, yi2, -1, 1, 0, seg_l)

        prev_x = x

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
