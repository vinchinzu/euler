#!/usr/bin/env python3
"""Check what happens at n=8, k=23 and k=41."""

import itertools

def cnk_2type(n):
    best = {}
    all_vecs = list(itertools.product([0, 1], repeat=n))
    nv = len(all_vecs)
    for ip in range(nv):
        P = all_vecs[ip]
        wp = sum(P)
        for iq in range(ip, nv):
            Q = all_vecs[iq]
            wq = sum(Q)
            for sigma in all_vecs:
                s = sum(sigma)
                k = s * wp + (n - s) * wq
                patterns = set()
                if s > 0:
                    patterns.add(P)
                if s < n:
                    patterns.add(Q)
                for j in range(n):
                    if P[j] == Q[j]:
                        col = (P[j],) * n
                    elif P[j] == 1:
                        col = sigma
                    else:
                        col = tuple(1 - sigma[i] for i in range(n))
                    patterns.add(col)
                comp = len(patterns)
                if k not in best or comp < best[k]:
                    best[k] = comp
    return best

n = 8
best = cnk_2type(n)
print(f"n=8, k=23: 2-type comp = {best.get(23, 'N/A')}")
print(f"n=8, k=41: 2-type comp = {best.get(41, 'N/A')}")

# Also check full distribution
from collections import Counter
dist = Counter(best.get(k, 999) for k in range(n*n+1))
print(f"Distribution: {dict(sorted(dist.items()))}")

# What is C(8) if k=23 and k=41 have comp=4?
S2 = set()
for c in range(1, n):
    S2.add(c * c)
    S2.add(n * n - c * c)
for x in range(1, n):
    y = n - x
    S2.add(x * x + y * y)
    S2.add(2 * x * y)
S2.discard(0)
S2.discard(n * n)

N2 = len(S2)
print(f"N2 = {N2}")

# Now let me try to verify k=23 with 3-type more thoroughly
# Maybe I need non-block, non-left-aligned 3-type
# Try ALL (P,Q,R) triples (not just left-aligned) and block σ
print(f"\nSearching for comp<=3 at k=23 with 3-type block...")
all_vecs = list(itertools.product([0, 1], repeat=n))
nv = len(all_vecs)
found = False

for ip in range(nv):
    if found: break
    P = all_vecs[ip]
    wp = sum(P)
    for iq in range(ip, nv):
        if found: break
        Q = all_vecs[iq]
        wq = sum(Q)
        for ir in range(iq, nv):
            if found: break
            R = all_vecs[ir]
            wr = sum(R)
            types = [P, Q, R]
            # Block arrangements: a rows of P, b of Q, c of R
            for a in range(n+1):
                if found: break
                for b in range(n+1-a):
                    c = n - a - b
                    k = a*wp + b*wq + c*wr
                    if k != 23:
                        continue
                    # Check complexity
                    patterns = set()
                    if a > 0: patterns.add(P)
                    if b > 0: patterns.add(Q)
                    if c > 0: patterns.add(R)
                    rows = [P]*a + [Q]*b + [R]*c
                    for j in range(n):
                        col = tuple(rows[i][j] for i in range(n))
                        patterns.add(col)
                    if len(patterns) <= 3:
                        print(f"  Found! P={P}, Q={Q}, R={R}, a={a}, b={b}, c={c}, comp={len(patterns)}")
                        found = True
                        break

if not found:
    print("  Not found with block arrangement.")

# Try non-block: 3 row types with general σ for k=23
print(f"\nSearching with general σ for k=23...")
found2 = False
for ip in range(nv):
    if found2: break
    P = all_vecs[ip]
    wp = sum(P)
    for iq in range(ip, nv):
        if found2: break
        Q = all_vecs[iq]
        wq = sum(Q)
        for ir in range(iq, nv):
            if found2: break
            R = all_vecs[ir]
            wr = sum(R)
            types = [P, Q, R]
            for sigma in itertools.product(range(3), repeat=n):
                s0 = sigma.count(0)
                s1 = sigma.count(1)
                s2 = sigma.count(2)
                k = s0*wp + s1*wq + s2*wr
                if k != 23:
                    continue
                patterns = set()
                used = set(sigma)
                for u in used:
                    patterns.add(types[u])
                for j in range(n):
                    col = tuple(types[sigma[i]][j] for i in range(n))
                    patterns.add(col)
                if len(patterns) <= 3:
                    print(f"  Found! P={P}, Q={Q}, R={R}, σ={sigma}, comp={len(patterns)}")
                    found2 = True
                    break

if not found2:
    print("  Not found with general σ either.")
    print("  c(8,23) >= 4 confirmed!")
