# Project Euler Problem 968
# Define P(X_{a,b}, X_{a,c}, X_{a,d}, X_{a,e}, X_{b,c}, X_{b,d}, X_{b,e}, X_{c,d}, X_{c,e}, X_{d,e})
# as the sum of 2^a * 3^b * 5^c * 7^d * 11^e over all quintuples of non-negative integers (a, b, c, d, e)
# such that the sum of each two of the five variables is restricted by a given value.
# In other words, a + b ≤ X_{a,b}, a + d ≤ X_{a,d}, b + e ≤ X_{b,e} etc.
#
# For example, P(2,2,2,2,2,2,2,2,2,2) = 7120
# and P(1, 2, 3, 4, 5, 6, 7, 8, 9, 10) ≡ 799809376 (mod 10^9 + 7).
#
# Define a sequence A as follows:
# - A_0 = 1, A_1 = 7;
# - A_n = (7 * A_{n-1} + A_{n-2}^2) mod (10^9 + 7) for n ≥ 2.
#
# Also define Q(n) = P(A_{10n}, A_{10n+1}, A_{10n+2}, ..., A_{10n+9}).
#
# Find sum_{0 ≤ n < 100} Q(n). Give your answer modulo 10^9 + 7.

from typing import List
import itertools

M = 10**9 + 7

def geo(r: int, n: int, mod: int) -> int:
    """Compute sum of r^i for i from 0 to n, modulo mod."""
    if n < 0:
        return 0
    if r % mod == 1:
        return (n + 1) % mod
    p = pow(r, n + 1, mod)
    inv = pow(1 - r, mod - 2, mod)
    return ((1 - p) * inv) % mod

def geo2(base: int, n: int, k: int, offset: int, mod: int) -> int:
    """
    Compute sum of base^(k*i + offset) for i from 0 to n.
    = base^offset * sum of (base^k)^i for i from 0 to n
    = base^offset * geo(base^k, n, mod)
    """
    if n < 0:
        return 0
    base_offset = pow(base, offset, mod)
    base_k = pow(base, k, mod)
    return (base_offset * geo(base_k, n, mod)) % mod
def geo3(base: int, L: int, R: int, k: int, offset: int, mod: int) -> int:
    """
    Compute sum of base^(k*i + offset) for i from L to R.
    """
    if L > R or R < 0:
        return 0
    if L < 0:
        L = 0
    # Sum from L to R = sum from 0 to R - sum from 0 to L-1
    if L == 0:
        return geo2(base, R, k, offset, mod)
    else:
        total = geo2(base, R, k, offset, mod)
        subtract = geo2(base, L - 1, k, offset, mod)
        return (total - subtract + mod) % mod
def sum_with_linear_max(base_outer: int, base_inner: int, L: int, R: int,
                        A: int, B: int, mod: int) -> int:
    """
    Compute sum of base_outer^i * geo(base_inner, A - B*i, mod) for i from L to R
    where A - B*i is the upper limit for the inner geometric sum.
    Assumes A - B*i >= 0 for i in [L, R].
    """
    if L > R:
        return 0
    # When B = 0, max is constant A
    if B == 0:
        geo_inner = geo(base_inner, A, mod)
        return (geo_inner * geo3(base_outer, L, R, 1, 0, mod)) % mod
    # For B > 0: sum of base_outer^i * geo(base_inner, A - B*i, mod)
    # Fall back to small loop (bounded by piece splitting above).
    result = 0
    for i in range(L, R + 1):
        max_val = A - B * i
        if max_val < 0:
            break
        contrib = (pow(base_outer, i, mod) * geo(base_inner, max_val, mod)) % mod
        result = (result + contrib) % mod
    return result
def compute_P_efficient(X: List[int], mod: int) -> int:
    """
    Semi-efficient computation: loops over a,b,c; handles d,e with piecewise sums.
    X = [Xab, Xac, Xad, Xae, Xbc, Xbd, Xbe, Xcd, Xce, Xde]
    """
    result = 0
    # Level a: 0 <= a <= min(X[0], X[1], X[2], X[3])
    max_a = min(X[0], X[1], X[2], X[3])
    for a in range(max_a + 1):
        # Level b: 0 <= b <= min(X[0]-a, X[4], X[5], X[6])
        max_b = min(X[0] - a, X[4], X[5], X[6])
        if max_b < 0:
            continue
        for b in range(max_b + 1):
            # Level c: 0 <= c <= min(X[1]-a, X[4]-b, X[7], X[8])
            max_c = min(X[1] - a, X[4] - b, X[7], X[8])
            if max_c < 0:
                continue
            for c in range(max_c + 1):
                # Level d: 0 <= d <= min(X[2]-a, X[5]-b, X[7]-c, X[9])
                max_d = min(X[2] - a, X[5] - b, X[7] - c, X[9])
                if max_d < 0:
                    continue
                # For each d in [0, max_d], we compute:
                # 7^d * geo(11, min(X[3]-a, X[6]-b, X[8]-c, X[9]-d), mod)
                # Split into pieces where the binding constraint for e changes.
                e_limits = [X[3] - a, X[6] - b, X[8] - c]
                critical_points = [0, max_d + 1]
                for limit in e_limits:
                    cp = X[9] - limit
                    if 0 <= cp <= max_d + 1:
                        critical_points.append(cp)
                critical_points = sorted(set(critical_points))
                # Process each piece [critical_points[i], critical_points[i+1})
                for i in range(len(critical_points) - 1):
                    d_lo = critical_points[i]
                    d_hi = critical_points[i + 1] - 1
                    if d_lo > d_hi or d_lo > max_d:
                        continue
                    d_hi = min(d_hi, max_d)
                    # Determine which constraint is binding using a test point
                    d_test = d_lo
                    max_e_test = min(X[3] - a, X[6] - b, X[8] - c, X[9] - d_test)
                    if max_e_test < 0:
                        continue
                    # If X[9] - d binds (linear), use linear-max sum. Else constant.
                    if max_e_test == X[9] - d_test:
                        contrib_de = sum_with_linear_max(7, 11, d_lo, d_hi, X[9], 1, mod)
                    else:
                        geo_e = geo(11, max_e_test, mod)
                        geo_d = geo3(7, d_lo, d_hi, 1, 0, mod)
                        contrib_de = (geo_d * geo_e) % mod
                    contrib = (
                        pow(2, a, mod)
                        * pow(3, b, mod)
                        * pow(5, c, mod)
                        * contrib_de
                    ) % mod
                    result = (result + contrib) % mod
    return result
def _pair_index() -> dict[tuple[int, int], int]:
    """Map (i,j) with i<j to the index in X."""
    return {
        (0, 1): 0,
        (0, 2): 1,
        (0, 3): 2,
        (0, 4): 3,
        (1, 2): 4,
        (1, 3): 5,
        (1, 4): 6,
        (2, 3): 7,
        (2, 4): 8,
        (3, 4): 9,
    }

def _get_upper(level: int, fixed: List[int], X: List[int]) -> int:
    """Upper bound for variable at `level`, given earlier fixed values."""
    pair_to_x = _pair_index()
    min_val = 10**18
    for k in range(level):
        min_val = min(min_val, X[pair_to_x[(k, level)]] - fixed[k])
    for m in range(level + 1, 5):
        min_val = min(min_val, X[pair_to_x[(level, m)]])
    return min_val

def _compute_level(level: int, fixed: List[int], X: List[int],
                   bases: List[int], mod: int) -> int:
    """
    Closed-form summation across remaining variables starting at `level`.
    Integrates over all later variables by piecewise geometric sums.
    """
    if level == 5:
        return 1
    r = bases[level]
    u = _get_upper(level, fixed, X)
    if u < 0:
        return 0

    pair_to_x = _pair_index()
    critical = {0, u + 1}
    for m in range(level + 1, 5):
        x_lm = X[pair_to_x[(level, m)]]
        for k in range(level):
            x_km = X[pair_to_x[(k, m)]]
            v = x_lm - (x_km - fixed[k])
            if 0 <= v <= u + 1:
                critical.add(v)
        for p in range(m + 1, 5):
            x_mp = X[pair_to_x[(m, p)]]
            v = x_lm - x_mp
            if 0 <= v <= u + 1:
                critical.add(v)
    crit = sorted(critical)

    total = 0
    for i in range(len(crit) - 1):
        start = crit[i]
        end = crit[i + 1] - 1
        if start > end:
            continue
        test_v = (start + end) // 2

        binding: dict[int, tuple[str, int, int]] = {}
        min_vals: dict[int, int] = {}
        for m in range(level + 1, 5):
            terms: dict[tuple[str, int, int], int] = {}
            lm_key = ("l", level, m)
            terms[lm_key] = X[pair_to_x[(level, m)]] - test_v
            for k in range(level):
                k_key = ("k", k, m)
                terms[k_key] = X[pair_to_x[(k, m)]] - fixed[k]
            for p in range(m + 1, 5):
                p_key = ("p", m, p)
                terms[p_key] = X[pair_to_x[(m, p)]]
            min_val = min(terms.values())
            min_vals[m] = min_val
            ordered = sorted(
                terms.items(),
                key=lambda kv: (
                    kv[1],
                    0 if kv[0][0] == "l" else (1 if kv[0][0] == "k" else 2),
                ),
            )
            binding[m] = ordered[0][0]

        linear_m = [m for m in binding if binding[m][0] == "l"]
        constant_m = [m for m in binding if m not in linear_m]

        const = 1
        for m in constant_m:
            const = (const * geo(bases[m], min_vals[m], mod)) % mod

        a_list: List[tuple[int, int, int]] = []
        for m in linear_m:
            r_m = bases[m]
            a_inv = pow(r_m - 1, mod - 2, mod)  # 1 / (r_m - 1)
            x_m = X[pair_to_x[(level, m)]]
            a_list.append((a_inv, x_m, r_m))

        prod_a = 1
        for a_inv, _, _ in a_list:
            prod_a = (prod_a * a_inv) % mod
        multiplier = pow(-1, len(linear_m), mod)

        indices = list(range(len(linear_m)))
        subsets = itertools.chain.from_iterable(
            itertools.combinations(indices, k) for k in range(len(indices) + 1)
        )
        for subset in subsets:
            len_s = len(subset)
            sign = pow(-1, len_s, mod)
            const_s = 1
            base_s = 1
            for idx in subset:
                a_inv, x_m, r_m = a_list[idx]
                const_s = (const_s * pow(r_m, x_m + 1, mod)) % mod
                base_s = (base_s * pow(r_m, mod - 2, mod)) % mod
            coeff = (sign * const_s) % mod
            the_coeff = (const * prod_a * multiplier * coeff) % mod
            effective_r = (r * base_s) % mod

            seg_sum = (
                the_coeff
                * pow(effective_r, start, mod)
                * geo(effective_r, end - start, mod)
            ) % mod
            total = (total + seg_sum) % mod

    return total

def compute_P_closed_form(X: List[int], mod: int) -> int:
    """
    Compute P(X) using a closed-form piecewise summation across all 5 levels.
    X = [Xab, Xac, Xad, Xae, Xbc, Xbd, Xbe, Xcd, Xce, Xde]
    """
    bases = [2, 3, 5, 7, 11]
    return _compute_level(0, [], X, bases, mod)

def compute_P(X: List[int], mod: int) -> int:
    """
    Compute P(X) choosing brute force for small X, otherwise semi-efficient.
    """
    if all(x < 20 for x in X):
        return compute_P_bruteforce(X, mod)
    return compute_P_efficient(X, mod)

def compute_P_bruteforce(X: List[int], mod: int) -> int:
    """
    Brute force computation for small X values; used for testing examples.
    """
    s = 0
    max_a = min(X[0], X[1], X[2], X[3])
    for a in range(max_a + 1):
        max_b = min(X[0] - a, X[4], X[5], X[6])
        for b in range(max_b + 1):
            max_c = min(X[1] - a, X[4] - b, X[7], X[8])
            for c in range(max_c + 1):
                max_d = min(X[2] - a, X[5] - b, X[7] - c, X[9])
                for d in range(max_d + 1):
                    max_e = min(X[3] - a, X[6] - b, X[8] - c, X[9] - d)
                    term = (
                        pow(2, a, mod)
                        * pow(3, b, mod)
                        * pow(5, c, mod)
                        * pow(7, d, mod)
                        * geo(11, max_e, mod)
                    ) % mod
                    s = (s + term) % mod
    return s

def main():
    # Compute A up to A[1000]
    A: List[int] = [0] * 1001
    A[0] = 1
    A[1] = 7
    for n in range(2, 1001):
        sq = pow(A[n - 2], 2, M)
        A[n] = (7 * A[n - 1] + sq) % M

    total = 0
    for n in range(100):
        Xs = A[10 * n : 10 * n + 10]
        q = compute_P(Xs, M)
        total = (total + q) % M

    print(total)

if __name__ == "__main__":
    main()
