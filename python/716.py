"""Project Euler Problem 716: Grid Graphs.

Let a grid graph G be a grid of HxW nodes such that each vertical line is up
or down, and each horizontal line is left or right, and let S(G) be the number
of strongly connected components in G. Find the sum of S(G) over all possible
grid graphs of HxW nodes.

C(h, w) satisfies a linear recurrence in both h and w. We compute C for small
values by brute force, use BM to find the recurrence, and Kitamasa to
extrapolate to large H, W.
"""

from __future__ import annotations


def berlekamp_massey(seq, mod):
    """Find shortest linear recurrence mod prime."""
    n = len(seq)
    C = [1]
    B = [1]
    L = 0
    m = 1
    b = 1
    for i in range(n):
        d = seq[i]
        for j in range(1, L + 1):
            d = (d + C[j] * seq[i - j]) % mod
        d %= mod
        if d == 0:
            m += 1
        elif 2 * L <= i:
            T = C[:]
            coeff = d * pow(b, mod - 2, mod) % mod
            while len(C) < len(B) + m:
                C.append(0)
            for j in range(len(B)):
                C[j + m] = (C[j + m] - coeff * B[j]) % mod
            L = i + 1 - L
            B = T
            b = d
            m = 1
        else:
            coeff = d * pow(b, mod - 2, mod) % mod
            while len(C) < len(B) + m:
                C.append(0)
            for j in range(len(B)):
                C[j + m] = (C[j + m] - coeff * B[j]) % mod
            m += 1
    return L, [(-C[i]) % mod for i in range(1, L + 1)]


def poly_mult_mod(a, b, rec, mod):
    """Multiply polynomials a, b modulo characteristic polynomial and mod."""
    L = len(rec)
    raw = [0] * (len(a) + len(b) - 1)
    for i in range(len(a)):
        if a[i] == 0:
            continue
        for j in range(len(b)):
            raw[i + j] = (raw[i + j] + a[i] * b[j]) % mod
    rep = [rec[L - 1 - i] % mod for i in range(L)]
    for i in range(len(raw) - 1, L - 1, -1):
        if raw[i] == 0:
            continue
        c = raw[i]
        raw[i] = 0
        for j in range(L):
            raw[i - L + j] = (raw[i - L + j] + c * rep[j]) % mod
    return raw[:L]


def eval_recurrence(rec, init, n, mod):
    """Evaluate linear recurrence at position n using Kitamasa method."""
    L = len(rec)
    if n < L:
        return init[n] % mod
    result = [0] * L
    result[0] = 1
    base = [0] * L
    if L > 1:
        base[1] = 1
    else:
        base[0] = rec[0] % mod
    exp = n
    while exp > 0:
        if exp & 1:
            result = poly_mult_mod(result, base, rec, mod)
        base = poly_mult_mod(base, base, rec, mod)
        exp >>= 1
    ans = 0
    for i in range(L):
        ans = (ans + result[i] * init[i]) % mod
    return ans


def precompute_list_props(n):
    """Precompute properties of all binary lists of length n.

    Returns a dict: bitmask -> (first0, first1, last0, last1, bit0, bitlast)
    where first0 = index of first 0, first1 = index of first 1, etc.
    -1 if not found.
    """
    props = {}
    for mask in range(1 << n):
        bits = [(mask >> j) & 1 for j in range(n)]
        b0 = bits[0]
        blast = bits[-1]
        first0 = first1 = last0 = last1 = -1
        for j in range(n):
            if bits[j] == 0:
                if first0 == -1:
                    first0 = j
                last0 = j
            else:
                if first1 == -1:
                    first1 = j
                last1 = j
        props[mask] = (first0, first1, last0, last1, b0, blast)
    return props


def C(h, w):
    """Compute sum of S(G) over all grid graphs of h x w nodes."""
    vprops = precompute_list_props(h)
    hprops = precompute_list_props(w)

    total = 0
    for vi in range(1 << h):
        vf0, vf1, vl0, vl1, v0, vlast = vprops[vi]
        for hi_val in range(1 << w):
            hf0, hf1, hl0, hl1, h0, hlast = hprops[hi_val]

            # x1 = indexOf(horiz, 1-v0) = first occurrence of (1-v0) in horiz
            x1 = hf1 if v0 == 0 else hf0
            # y1 = indexOf(vert, 1-h0) = first occurrence of (1-h0) in vert
            y1 = vf1 if h0 == 0 else vf0
            # x2 = lastIndexOf(horiz, v0)
            x2 = hl0 if v0 == 0 else hl1
            # y2 = indexOf(vert, hlast)
            y2 = vf0 if hlast == 0 else vf1
            # x3 = indexOf(horiz, vlast)
            x3 = hf0 if vlast == 0 else hf1
            # y3 = lastIndexOf(vert, h0)
            y3 = vl0 if h0 == 0 else vl1
            # x4 = lastIndexOf(horiz, 1-vlast)
            x4 = hl1 if vlast == 0 else hl0
            # y4 = lastIndexOf(vert, 1-hlast)
            y4 = vl1 if hlast == 0 else vl0

            if x1 == -1 or x2 == -1 or y1 == -1 or y3 == -1:
                total += w * h
            else:
                area = (
                    x1 * y1
                    + (w - 1 - max(x1 - 1, x2)) * y2
                    + x3 * (h - 1 - max(y1 - 1, y3))
                    + (w - 1 - max(x3 - 1, x4)) * (h - 1 - max(y2 - 1, y4))
                )
                if area < w * h:
                    area += 1
                total += area
    return total


def solve():
    """Solve Problem 716."""
    H = 10000
    W = 20000
    M = 10**9 + 7

    max_h = 10
    max_w = 10

    # Step 1: For each h, compute C(h, w) for small w, find recurrence, extrapolate to W
    vals_at_W = []
    for h in range(1, max_h + 1):
        row = [C(h, w) % M for w in range(1, max_w + 1)]
        L, rec = berlekamp_massey(row, M)
        val = eval_recurrence(rec, row[:L], W - 1, M)
        vals_at_W.append(val)

    # Step 2: Find recurrence in h and extrapolate to H
    L_h, rec_h = berlekamp_massey(vals_at_W, M)
    result = eval_recurrence(rec_h, vals_at_W[:L_h], H - 1, M)

    return result % M


def main():
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
