#!/usr/bin/env python3
"""Project Euler Problem 415 - Titanic sets.

A titanic set is a set of lattice points such that some line passes through
exactly 2 points. Find the number of titanic sets such that each point (x, y)
satisfies 0 <= x,y <= N.

Solution ported from Java reference.
"""

from math import isqrt


def pow_mod(base, exp, mod):
    """Modular exponentiation."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def sq(n, mod):
    """Return n^2 mod mod."""
    return (n * n) % mod


def tr(n, mod):
    """Return n(n+1)/2 mod mod."""
    return (n * (n + 1) // 2) % mod


def sum_powers(n, exp, mod):
    """Return sum_{k=1}^n k^exp mod mod."""
    if exp == 0:
        return n % mod
    elif exp == 1:
        return tr(n, mod)
    elif exp == 2:
        return (n * (n + 1) * (2 * n + 1) // 6) % mod
    else:
        raise ValueError(f"exp={exp} not supported")


def sum_ag(L, e, mod):
    """Sum of arithmetico-geometric series: sum_{g=1}^L g^e * 2^g mod M."""
    if e == 0:
        return (pow_mod(2, L + 1, mod) - 2) % mod
    elif e == 1:
        return (((L - 1) % mod * pow_mod(2, L + 1, mod) + 2) % mod) % mod
    elif e == 2:
        return (((sq(L, mod) - 2 * L + 3) % mod * pow_mod(2, L + 1, mod) - 6) % mod) % mod
    else:
        raise ValueError(f"e={e} not supported")


class QuotientValues:
    """
    Stores values indexed by quotients floor(N/q) for q = 1, 2, ..., N.
    There are only O(sqrt(N)) distinct quotient values.

    For q <= sqrt(N): use array indexed by q
    For q > sqrt(N): floor(N/q) < sqrt(N), so use array indexed by floor(N/q)
    """
    def __init__(self, N, M):
        self.N = N
        self.M = M
        self.L = isqrt(N)
        # For small quotients (q <= L), store value at index q
        self.small = [0] * (self.L + 1)
        # For large quotients (q > L), store value at index floor(N/q)
        self.large = [0] * (self.L + 2)

    def set_div(self, q, val):
        """Set value for floor(N/q)."""
        if q <= self.L:
            self.small[q] = val % self.M
        else:
            self.large[self.N // q] = val % self.M

    def get_div(self, q):
        """Get value for index q (where q <= L)."""
        return self.small[q]

    def get(self, q):
        """Get value for large quotients (where q is the quotient value)."""
        return self.large[q]


def compute_sum_phis(N, e, M):
    """
    Compute sum_{x=1}^{floor(N/q)} x^e * phi(x) for all distinct quotients.
    Returns a QuotientValues object.

    Uses the identity:
    sum_{x=1}^n x^e * phi(x) = sum_{d=1}^n sum_{x=1}^{floor(n/d)} x^e * mu(d) * d^e * sum_{k|x, k=1}^{floor(n/d)} k

    Actually, we use Lucy_Hedgehog style DP:
    S(n) = sum_{x=1}^n x^e * phi(x)

    Use identity: sum_{d=1}^n phi(d) = n(n+1)/2
    So: phi(n) = n(n+1)/2 - sum_{d=2}^n phi(floor(n/d))

    For sum x^e*phi(x), we need a different approach.
    Let T_e(n) = sum_{x=1}^n x^e * phi(x)

    We know sum_{d|n} phi(d) = n
    So sum_{x=1}^n sum_{d|x} phi(d) = sum_{x=1}^n x = n(n+1)/2

    For our case, we use:
    sum_{d=1}^n d^e * T_0(floor(n/d)) = sum_{x=1}^n x^(e+1)

    This allows computing T_0 recursively, then use:
    T_1(n) = sum_{d=1}^n T_0(floor(n/d)) * d - n(n+1)/2
    etc.

    Actually let's use a simpler direct sieve approach for e=0,1,2.
    """
    L = isqrt(N)
    result = QuotientValues(N, M)

    # Precompute phi values for x <= L using sieve
    phi = list(range(L + 1))  # phi[0] = 0, phi[i] = i initially
    for i in range(2, L + 1):
        if phi[i] == i:  # i is prime
            for j in range(i, L + 1, i):
                phi[j] -= phi[j] // i

    # Compute prefix sums of x^e * phi(x) for x <= L
    prefix = [0] * (L + 2)
    for x in range(1, L + 1):
        prefix[x] = (prefix[x - 1] + pow_mod(x, e, M) * phi[x]) % M

    # For each distinct quotient value
    # Small quotients: q = 1, 2, ..., L
    # floor(N/q) ranges from N down to N/L ~ sqrt(N)

    # For large quotients: floor(N/q) = 1, 2, ..., L-1
    # These correspond to q in range (N/L, N]

    # Use memoization/DP for floor(N/q) values
    # T(n) = sum_{x=1}^n x^e * phi(x)

    # For n <= L, T(n) = prefix[n]
    # For n > L, we need to compute

    # The key identity for phi:
    # sum_{d=1}^n phi(d) = n*(n+1)/2
    #
    # For T_e(n) = sum_{x=1}^n x^e * phi(x), we have:
    # sum_{d=1}^n d^e * S(floor(n/d)) = sum_{x=1}^n x^e * (number of coprime pairs (x,y) with y<=x)
    # = sum_{x=1}^n x * x^e = sum_{x=1}^n x^{e+1}
    # where S(m) = sum_{x=1}^m phi(x)

    # For e=0: sum_{d=1}^n S(floor(n/d)) = n(n+1)/2
    # So S(n) = n(n+1)/2 - sum_{d=2}^n S(floor(n/d))

    # For T_e, we need to be more careful.
    # T_0(n) = sum_{x=1}^n phi(x)
    # T_1(n) = sum_{x=1}^n x * phi(x)
    # T_2(n) = sum_{x=1}^n x^2 * phi(x)

    # Identity: sum_{d=1}^n d^e * T_0(floor(n/d)) = sum_{x=1}^n x^e * x = sum x^{e+1}

    # Let's compute T_0 first using standard Lucy DP
    T0 = {}

    def compute_T0(n):
        if n in T0:
            return T0[n]
        if n <= L:
            # phi prefix sum
            s = 0
            for x in range(1, n + 1):
                s = (s + phi[x]) % M
            T0[n] = s
            return s
        # sum phi(x) for x=1..n = n(n+1)/2 - sum_{d=2}^n S(floor(n/d))
        s = (n * (n + 1) // 2) % M
        d = 2
        while d <= n:
            q = n // d
            d_max = n // q
            s = (s - (d_max - d + 1) * compute_T0(q)) % M
            d = d_max + 1
        T0[n] = s
        return s

    # Compute T_e using the identity:
    # sum_{d=1}^n T_0(floor(n/d)) * d^e = sum x^{e+1}
    # T_e can be derived but it's complex.

    # Let's use a direct formula for T_e.
    # For e=0: T_0(n) = sum phi(x) for x=1..n (computed above)
    # For e=1: sum x*phi(x) = sum_{x=1}^n x * phi(x)
    #         Using sum_{d|x} phi(d) = x:
    #         sum_{x=1}^n x * phi(x) = (sum_{x=1}^n x * sum_{d|x} phi(d) - sum_{x=1}^n x) / ...
    # This is getting complex. Let's just precompute for small n and use recursion.

    # Actually, let's use a cleaner identity.
    # T_1(n) = sum_{x=1}^n x * phi(x)
    # We know: sum_{d=1}^n T_0(floor(n/d)) = n(n+1)/2
    # And: sum_{d=1}^n d * T_0(floor(n/d)) = sum x^2
    #
    # Hmm, let's think differently.
    # sum_{gcd(a,b)=1, 1<=a<=b<=n} a = sum_{b=1}^n sum_{a=1}^{b-1, gcd(a,b)=1} a + sum_{a=1}^n 1 (for a=b=1 not counted)
    # = sum_{b=1}^n b*phi(b)/2 + ... (for b>=2) + 1
    #
    # Actually: sum_{1<=a<b<=n, gcd(a,b)=1} (a+b) = sum_{b=2}^n phi(b) * b
    # So: sum_{b=1}^n phi(b) * b = sum (coprime pairs a<b) (a+b) + 1 (for b=1)

    # Let me use the direct computation for large values.
    # T_1(n) = sum x * phi(x) for x = 1..n

    # For large n, use: T_1(n) = n * (n+1) * (2n+1) / 12 * some factor...
    # This is complex. Let me try a different approach: direct sieve + memoization.

    T1 = {}
    T2 = {}

    # For n <= L, compute directly from phi values
    for n in range(1, L + 1):
        s1 = s2 = 0
        for x in range(1, n + 1):
            s1 = (s1 + x * phi[x]) % M
            s2 = (s2 + x * x % M * phi[x]) % M
        T1[n] = s1
        T2[n] = s2

    # For larger n, we need a recursive formula.
    # Using the identity for T_e with the mobius function:
    # T_e(n) = sum_{d=1}^n mu(d) * d^e * sum_{k=1}^{floor(n/d)} k^{e+1}
    # But this requires summing over mobius which is also O(sqrt(n)) per call.

    # Let's use:
    # sum_{d=1}^n d^e * T_0(floor(n/d)) = sum_{x=1}^n x^{e+1}
    # This relates T_0 values at different points, but we need T_e not T_0.

    # Alternative: Use Lucy-style DP for T_e directly
    # T_e(n) = sum_{x=1}^n x^e * phi(x)
    # = sum_{x=1}^n x^e * (x - sum_{p|x, p prime} x/p + ...)  (Euler product)
    # = sum_{x=1}^n x^{e+1} - sum_p sum_{k=1}^{n/p} (kp)^e * something...

    # This is getting too complex. Let me use a hybrid approach:
    # Compute T_0, T_1, T_2 for all quotient values using memoization

    def compute_T_full(n, e):
        if e == 0:
            return compute_T0(n)

        cache = T1 if e == 1 else T2
        if n in cache:
            return cache[n]

        if n <= L:
            return cache[n]  # Already computed

        # For T_1: Use identity
        # sum_{d=1}^n T_0(floor(n/d)) * d = sum_{x=1}^n x^2 (almost but not quite)
        # Actually: sum_{a=1}^n sum_{b=1}^n [gcd(a,b)=1] * a = sum_b sum_a [gcd=1] a
        # = sum_b phi(b) * b / 2 (for a < b) * 2 (symmetry) + sum [a=b] = sum b * phi(b)
        # Hmm this counts differently.

        # Let me use the explicit Dirichlet hyperbola method.
        # T_e(n) = sum_{x=1}^n x^e * phi(x)
        # = sum_{x=1}^n x^e * sum_{d|x} mu(d) * (x/d)
        # = sum_{d=1}^n mu(d) * sum_{k=1}^{n/d} (dk)^e * k
        # = sum_{d=1}^n mu(d) * d^e * sum_{k=1}^{n/d} k^{e+1}

        # We need sum of mu(d) * d^e * S_{e+1}(n/d) where S_k(m) = sum_{i=1}^m i^k

        # Using hyperbola: sum over d <= sqrt(n) and sum over k <= sqrt(n)
        s = 0
        sqrt_n = isqrt(n)

        # Sum over d <= sqrt(n)
        # Need mu values and S_{e+1} values
        # Precompute mu for small values

        # This is still complex. Let me just compute T_e directly for quotient values
        # by iterating over all x up to the quotient (expensive but doable)

        # For n up to N/L ~ sqrt(N), we can compute directly in O(sqrt(N)) each
        # Total: O(sqrt(N)) quotient values * O(sqrt(N)) = O(N)

        # Let's just iterate:
        s = 0
        for x in range(1, n + 1):
            if x <= L:
                phi_x = phi[x]
            else:
                # Compute phi(x) directly
                phi_x = x
                temp = x
                d = 2
                while d * d <= temp:
                    if temp % d == 0:
                        phi_x -= phi_x // d
                        while temp % d == 0:
                            temp //= d
                    d += 1
                if temp > 1:
                    phi_x -= phi_x // temp
            s = (s + pow_mod(x, e, M) * phi_x) % M

        cache[n] = s
        return s

    # Compute all quotient values needed
    # For g from 1 to L: need floor(N/g)
    # For q from 1 to N/L-1: need q

    for g in range(1, L + 1):
        val = N // g
        if e == 0:
            result.set_div(g, compute_T0(val))
        else:
            result.set_div(g, compute_T_full(val, e))

    # For quotients q = 1, 2, ..., L (corresponding to large g values)
    for q in range(1, L + 1):
        if e == 0:
            result.large[q] = compute_T0(q)
        else:
            result.large[q] = compute_T_full(q, e)

    return result


def solve():
    N = 10**11
    L = isqrt(N)
    M = 10**8

    # Compute sum_phis for e = 0, 1, 2
    sum_phis = [compute_sum_phis(N, e, M) for e in range(3)]

    # Total subsets: 2^((N+1)^2)
    ans = pow_mod(pow_mod(2, N + 1, M), N + 1, M)

    # Subtract empty set
    ans = (ans - 1) % M

    # Subtract single points
    ans = (ans - sq(N + 1, M)) % M

    # Subtract horizontal/vertical lines (3 or more collinear points)
    # For each row/column: 2^(N+1) - 1 - (N+1) - C(N+1,2) non-titanic sets
    term = (pow_mod(2, N + 1, M) - 1 - (N + 1) % M - tr(N, M)) % M
    ans = (ans - 2 * (N + 1) % M * term) % M

    # Subtract diagonal lines for g <= L
    for g in range(1, L + 1):
        T = (sq(g, M) * sum_phis[2].get_div(g)
             - 3 * (N + 1) % M * g % M * sum_phis[1].get_div(g)
             + 2 * sq(N + 1, M) * sum_phis[0].get_div(g)
             - (N + 1 - g) % M * ((N + 1) % M)) % M
        ans = (ans - (pow_mod(2, g, M) - 2) * T) % M

    # Subtract diagonal lines for q from 1 to N/L - 1
    for q in range(1, N // L):
        ans = (ans
               - sum_phis[2].get(q) * (sum_ag(N // q, 2, M) - sum_ag(N // (q + 1), 2, M))
               + (N + 1) % M * (3 * sum_phis[1].get(q) - 1) % M * (sum_ag(N // q, 1, M) - sum_ag(N // (q + 1), 1, M))
               - sq(N + 1, M) * (2 * sum_phis[0].get(q) - 1) % M * (sum_ag(N // q, 0, M) - sum_ag(N // (q + 1), 0, M))
               + 2 * sum_phis[2].get(q) * (sum_powers(N // q, 2, M) - sum_powers(N // (q + 1), 2, M))
               - 2 * (N + 1) % M * (3 * sum_phis[1].get(q) - 1) % M * (sum_powers(N // q, 1, M) - sum_powers(N // (q + 1), 1, M))
               + 2 * sq(N + 1, M) * (2 * sum_phis[0].get(q) - 1) % M * (sum_powers(N // q, 0, M) - sum_powers(N // (q + 1), 0, M))
              ) % M

    return ans % M


if __name__ == "__main__":
    print(solve())
