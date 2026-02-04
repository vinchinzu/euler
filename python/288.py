"""Project Euler Problem 288: An enormous factorial."""
from __future__ import annotations


def solve() -> int:
    P = 61
    Q = 10 ** 7
    E = 10
    M = P ** E  # 61^10
    BBS_MOD = 50515093

    # Generate T array: S_0=290797, S_{n+1}=S_n^2 mod 50515093, T_n=S_n mod P
    # We need T[0] through T[Q]
    # N(P,Q) = sum T_n * P^n for n=0..Q
    # NF(P,Q) = number of factors P in N(P,Q)!
    # By Legendre's formula: NF = sum_{i=1}^{inf} floor(N / P^i)
    # We need NF mod P^E
    #
    # N = T_0 + T_1*P + T_2*P^2 + ... + T_Q*P^Q
    # floor(N / P^i) = T_i + T_{i+1}*P + T_{i+2}*P^2 + ... + T_Q*P^{Q-i}
    #
    # NF = sum_{i=1}^{Q} floor(N/P^i) = sum_{i=1}^{Q} (T_i + T_{i+1}*P + ... + T_Q*P^{Q-i})
    #
    # Since we need NF mod P^E, for floor(N/P^i) we only need the first E digits:
    # floor(N/P^i) mod P^E = (T_i + T_{i+1}*P + ... + T_{i+E-1}*P^{E-1}) mod P^E
    # (for i+E-1 <= Q, otherwise fewer terms)
    #
    # So NF mod M = sum_{i=1}^{Q} sum_{j=0}^{min(E-1, Q-i)} T_{i+j} * P^j (mod M)
    #
    # Rearrange by k = i+j: T_k contributes P^j for each (i,j) with i+j=k, i>=1, 0<=j<E
    # So j ranges from max(0, k-Q) to min(E-1, k-1)
    # For k >= E and k <= Q: j ranges from 0 to E-1, coefficient = sum_{j=0}^{E-1} P^j
    # For k < E: j ranges from 0 to k-1, coefficient = sum_{j=0}^{k-1} P^j
    # For k > Q-E+1 and k <= Q: some terms missing at the high end (but Q >> E so negligible for k <= Q)
    # Actually for i >= 1 and i+j <= Q: j <= Q-i = Q-(k-j) => always true when j < E and k <= Q

    # Precompute P powers
    pows = [0] * (E + 1)
    pows[0] = 1
    for i in range(1, E + 1):
        pows[i] = pows[i - 1] * P

    # For k >= E: coeff = (P^E - 1) / (P - 1) mod M
    inv_pm1 = pow(P - 1, -1, M)
    coeff_full = ((M - 1) * inv_pm1) % M

    # For k < E: coeff_k = (P^k - 1) / (P - 1) mod M
    coeff_small = [0] * E
    for k in range(1, E):
        coeff_small[k] = ((pows[k] - 1) * inv_pm1) % M

    # Generate BBS sequence and accumulate
    ans = 0
    s = 290797
    # T[0] is not used (sum starts at i=1 so k >= 1)
    # But we need T[0] through T[Q]
    # T[0] = 290797 % P, but T[0] only appears with k=0 which has no contribution (i>=1 means k>=1)

    # We need to generate T values and accumulate on the fly
    # For k from 1 to min(E-1, Q): use coeff_small[k]
    # For k from E to Q: use coeff_full
    # But we also need T[k] for k > Q - E + 1 where some j values are missing
    # Actually let me re-derive. For a given k (= i+j):
    #   i = k - j, need i >= 1 => j <= k-1
    #   need j >= 0
    #   need j < E
    #   need i <= Q => k - j <= Q => j >= k - Q (only matters if k > Q)
    # Since k <= Q, the constraint is j from 0 to min(E-1, k-1)
    # So for k >= E: coeff = sum P^j for j=0..E-1 = coeff_full
    # For 1 <= k < E: coeff = sum P^j for j=0..k-1 = coeff_small[k]

    # Generate all T values first (10M values, ~10s in Python)
    # Actually let's try accumulating on the fly
    T_buf = [0] * (Q + 1)
    s = 290797
    for n in range(Q + 1):
        T_buf[n] = s % P
        s = (s * s) % BBS_MOD

    # Now accumulate
    # k=1 to E-1: small coefficients
    for k in range(1, E):
        ans = (ans + T_buf[k] * coeff_small[k]) % M

    # k=E to Q: full coefficient
    # Sum T_buf[E..Q] * coeff_full mod M
    # Do in chunks to keep numbers manageable
    chunk = 0
    for k in range(E, Q + 1):
        chunk += T_buf[k]
        if k % 1000000 == 0 or k == Q:
            ans = (ans + coeff_full * chunk) % M
            chunk = 0

    return ans


def main() -> None:
    print(solve())


if __name__ == "__main__":
    main()
