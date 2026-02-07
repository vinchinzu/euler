#!/usr/bin/env python3
"""
Verify n=8. With 256*256*256/2 = 8M combinations, should be feasible.
But let me optimize: once we fix P,Q, the column constraint determines what σ must look like.
"""

import itertools

def find_comp2_direct(n):
    """Find k values achievable with complexity ≤ 2 by direct construction."""
    achievable = set()
    all_vecs = list(itertools.product([0,1], repeat=n))

    for P in all_vecs:
        for Q in all_vecs:
            if P >= Q:
                continue

            # For each column j, the column depends on whether P[j]=Q[j] or not.
            # If P[j]=Q[j]: column j is constant, all entries = P[j].
            #   Column j is (P[j], P[j], ..., P[j]) which must equal P or Q.
            # If P[j]≠Q[j]: column j depends on σ.
            #   If P[j]=1, Q[j]=0: col j = σ (must be P or Q)
            #   If P[j]=0, Q[j]=1: col j = complement(σ) (must be P or Q)

            # First check constant columns:
            valid_PQ = True
            for j in range(n):
                if P[j] == Q[j]:
                    const_col = tuple([P[j]] * n)
                    if const_col != P and const_col != Q:
                        valid_PQ = False
                        break
            if not valid_PQ:
                continue

            # Now determine what σ must be:
            # σ must be in {P, Q} (if any j has P[j]=1, Q[j]=0)
            # complement(σ) must be in {P, Q} (if any j has P[j]=0, Q[j]=1)
            has_10 = any(P[j]==1 and Q[j]==0 for j in range(n))
            has_01 = any(P[j]==0 and Q[j]==1 for j in range(n))

            possible_sigmas = set()

            if not has_10 and not has_01:
                # P = Q (since P[j]=Q[j] for all j), but we required P < Q. Skip.
                continue

            if has_10 and has_01:
                # σ ∈ {P, Q} AND complement(σ) ∈ {P, Q}
                # So {σ, complement(σ)} ⊆ {P, Q}
                # Either σ=P and comp(σ)=Q, or σ=Q and comp(σ)=P
                comp_P = tuple(1-x for x in P)
                comp_Q = tuple(1-x for x in Q)
                if comp_P == Q:
                    possible_sigmas.add(P)
                    possible_sigmas.add(Q)
                # σ=Q, comp(σ)=P?
                if comp_Q == P:
                    possible_sigmas.add(Q)
                    possible_sigmas.add(P)
            elif has_10:
                # σ ∈ {P, Q}
                possible_sigmas.add(P)
                possible_sigmas.add(Q)
            elif has_01:
                # complement(σ) ∈ {P, Q}
                comp_P = tuple(1-x for x in P)
                comp_Q = tuple(1-x for x in Q)
                possible_sigmas.add(comp_P)
                possible_sigmas.add(comp_Q)

            for sigma in possible_sigmas:
                s = sum(sigma)
                k = 0
                for j in range(n):
                    # column j sum = s*P[j] + (n-s)*Q[j] if σ is uniform weight
                    # Actually, k = sum over i,j of M[i][j]
                    # M[i][j] = P[j] if sigma[i]=1 else Q[j]
                    # sum over i: sum(sigma)*P[j] + (n-sum(sigma))*Q[j]
                    k += s * P[j] + (n - s) * Q[j]
                if 0 < k < n*n:
                    achievable.add(k)

    return achievable

def get_comp2_k_values_v1(n):
    achievable = set()
    for a in range(n+1):
        for b in range(n+1-a):
            for c in range(n+1-a-b):
                d = n - a - b - c
                if c == 0 and d == 0:
                    continue
                zero_is_P = (b + c == 0)
                zero_is_Q = (b + d == 0)
                ones_is_P = (a + d == 0)
                ones_is_Q = (a + c == 0)
                if a > 0 and not (zero_is_P or zero_is_Q):
                    continue
                if b > 0 and not (ones_is_P or ones_is_Q):
                    continue
                possible_s = set()
                if c > 0 and d > 0:
                    for sigma_choice in ['P', 'Q']:
                        for comp_choice in ['P', 'Q']:
                            s1 = (b + c) if sigma_choice == 'P' else (b + d)
                            s2 = (a + d) if comp_choice == 'P' else (a + c)
                            if s1 == s2:
                                possible_s.add(s1)
                elif c > 0:
                    possible_s.add(b + c)
                    possible_s.add(b + d)
                elif d > 0:
                    possible_s.add(a + d)
                    possible_s.add(a + c)
                for s in possible_s:
                    if 0 <= s <= n:
                        k = s * (b + c) + (n - s) * (b + d)
                        if 0 < k < n * n:
                            achievable.add(k)
                        k_comp = n * n - k
                        if 0 < k_comp < n * n:
                            achievable.add(k_comp)
    return achievable

for n in [7, 8, 9, 10]:
    print(f"\nComputing n={n}...")
    direct = find_comp2_direct(n)
    analytic = get_comp2_k_values_v1(n)
    print(f"n={n}: direct count={len(direct)}, analytic count={len(analytic)}")
    if direct != analytic:
        print(f"  DIFF: in direct not analytic: {sorted(direct - analytic)}")
        print(f"       in analytic not direct: {sorted(analytic - direct)}")
    else:
        print("  MATCH")

    N2 = len(direct)
    N3 = (n*n - 1) - N2
    Cn = 2*n*n + N3
    print(f"  C({n}) = {Cn}")
