"""Project Euler Problem 325 - Stone Game.

The sum S(N) = sum_{y=1}^N sum_{x=ceil(y/phi)}^{y-1} (x+y) can be computed using
floor sum identities for the golden ratio.

We use O(log N) recursive computation based on the continued fraction of 1/phi.
"""

from math import isqrt

def solve():
    N = 10**16
    M = 7**10

    # M = 7^10 is not prime, so use Euler's theorem: a^(-1) ≡ a^(φ(M)-1) (mod M)
    # φ(7^10) = 7^10 - 7^9 = 7^9 * 6
    phi_M = 7**9 * 6
    inv2 = pow(2, phi_M - 1, M)
    inv6 = pow(6, phi_M - 1, M)

    def sum_k(n):
        """sum_{k=1}^n k mod M"""
        n = n % M
        return n * (n + 1) % M * inv2 % M

    def sum_k2(n):
        """sum_{k=1}^n k^2 mod M"""
        n = n % M
        return n * (n + 1) % M * (2*n + 1) % M * inv6 % M

    def floor_phi_inv(n):
        """floor(n * (sqrt(5)-1)/2) exactly."""
        if n <= 0:
            return 0
        s = isqrt(5 * n * n)
        return (s - n) // 2

    def floor_phi(n):
        """floor(n * (sqrt(5)+1)/2) exactly."""
        if n <= 0:
            return 0
        s = isqrt(5 * n * n)
        return (s + n) // 2

    # We compute sum_{y=1}^n y^a * floor(y*phi_inv)^b for (a,b) in {(0,1), (0,2), (1,1)}
    # using a recursive algorithm based on the continued fraction of phi_inv.
    #
    # Key insight: phi_inv = 1/(1 + phi_inv), so the continued fraction is [0;1,1,1,...]
    #
    # The algorithm uses the following recursion:
    # For alpha = phi_inv with integer part 0 and fractional part phi_inv:
    # The sum over k=1..n of k^a * floor(k*alpha)^b can be expressed in terms of
    # sums over j=1..m of j^a' * floor(j*alpha')^b' where m = floor(n*alpha) and alpha' = 1/alpha = phi.
    #
    # Since floor(k*phi) = k + floor(k*phi_inv), this creates a recursive structure.
    #
    # We use a vector of sums: (S_00, S_01, S_02, S_10, S_11, S_20)
    # where S_ab = sum_{k=1}^n k^a * floor(k*alpha)^b
    #
    # The recursion comes from the Beatty sequence identity:
    # sum_{k=1}^n floor(k*alpha) + sum_{j=1}^m floor(j/alpha) = n*m where m = floor(n*alpha)

    # For the golden ratio specifically, we use the recursive structure:
    # Let T(n, phi_inv) = (S01, S02, S11) for sums over k=1..n with alpha = phi_inv
    # Let T(m, phi) = (S01', S02', S11') for sums over j=1..m with alpha = phi
    #
    # Since floor(j*phi) = j + floor(j*phi_inv):
    # S01'(m, phi) = sum floor(j*phi) = m(m+1)/2 + S01(m, phi_inv)
    # S02'(m, phi) = sum floor(j*phi)^2 = sum (j + floor(j*phi_inv))^2
    #             = sum j^2 + 2*sum j*floor(j*phi_inv) + sum floor(j*phi_inv)^2
    #             = m(m+1)(2m+1)/6 + 2*S11(m, phi_inv) + S02(m, phi_inv)
    # S11'(m, phi) = sum j*floor(j*phi) = sum j*(j + floor(j*phi_inv))
    #             = m(m+1)(2m+1)/6 + S11(m, phi_inv)
    #
    # For the Beatty identity:
    # sum_{k=1}^n floor(k*phi_inv) + sum_{j=1}^m floor(j*phi) = n*m
    # So S01(n) + S01'(m) = n*m
    # S01(n) = n*m - m(m+1)/2 - S01(m)
    #
    # For the higher order sums, we need more complex identities.
    # Using the lattice point counting interpretation:
    # sum_{k=1}^n sum_{j=1}^{floor(k*alpha)} 1 = #{(k,j): 1<=k<=n, 1<=j<=k*alpha}
    #                                         = #{(k,j): k>=1, j>=1, j/alpha <= k <= n}
    #                                         = sum_{j=1}^m (n - ceil(j/alpha) + 1)
    # where m = floor(n*alpha).
    #
    # This gives: S01(n) = n*m - sum_{j=1}^m ceil(j/alpha) + m
    #                    = n*m - sum_{j=1}^m floor(j/alpha) - #{j: j/alpha is not integer} + m
    #                    = n*m - S01'(m) (since no j/alpha is an integer for irrational alpha)
    #
    # Wait, that's the same identity. Let me work on the second moment.
    #
    # sum_{k=1}^n floor(k*alpha)^2 = sum_{k=1}^n (sum_{j=1}^{floor(k*alpha)} 1)^2
    # = sum_{k=1}^n sum_{j1=1}^{f_k} sum_{j2=1}^{f_k} 1 where f_k = floor(k*alpha)
    # = 2*sum_{k=1}^n sum_{1<=j1<j2<=f_k} 1 + sum_{k=1}^n f_k
    # = 2*sum_{k=1}^n f_k*(f_k-1)/2 + S01(n)
    # = sum_{k=1}^n f_k^2 - sum_{k=1}^n f_k + S01(n)
    # = S02(n) - S01(n) + S01(n) = S02(n)
    # This is circular and doesn't help.
    #
    # Let me try: sum f_k^2 = 2*sum_{j<f_k} j + sum f_k
    #                      = 2*sum_{k=1}^n sum_{j=1}^{f_k-1} j + S01(n)
    #                      = 2*sum_{k=1}^n f_k*(f_k-1)/2 + S01(n)
    #                      = sum f_k^2 - S01(n) + S01(n) = sum f_k^2 (circular again)
    #
    # Alternative: use the "diagonal" counting
    # S02(n) = sum_{d=1}^{max_f} (2d-1) * #{k: f_k >= d}
    #        = sum_{d=1}^{max_f} (2d-1) * (n - ceil(d/alpha) + 1)  (for d <= max_f)
    # But this still requires O(max_f) = O(n*alpha) terms.
    #
    # The key insight for the O(log n) algorithm:
    # We maintain a state vector and transform it based on the continued fraction coefficients.
    # For phi_inv = [0; 1, 1, 1, ...], each step is the same transformation.

    # Let me implement the recursion directly using memoization.
    # The state is (n, is_phi_inv) where is_phi_inv indicates which sequence we're summing.
    # We compute (S01, S02, S11) for that state.

    memo = {}

    def compute_sums(n, is_phi_inv):
        """
        Compute (S01, S02, S11) mod M where:
        S01 = sum_{k=1}^n floor(k*alpha)
        S02 = sum_{k=1}^n floor(k*alpha)^2
        S11 = sum_{k=1}^n k * floor(k*alpha)

        alpha = phi_inv if is_phi_inv else phi
        """
        if n <= 0:
            return (0, 0, 0)

        key = (n, is_phi_inv)
        if key in memo:
            return memo[key]

        if is_phi_inv:
            # alpha = phi_inv, 1/alpha = phi
            m = floor_phi_inv(n)
            if m == 0:
                # All floor values are 0
                memo[key] = (0, 0, 0)
                return (0, 0, 0)

            # Get sums for phi with range m
            S01_phi, S02_phi, S11_phi = compute_sums(m, False)

            # Use Beatty identity: S01(n, phi_inv) + S01(m, phi) = n * m
            S01 = ((n % M) * (m % M) - S01_phi) % M

            # For S02 and S11, we use the splitting based on floor values.
            # For k in [1, n], let f_k = floor(k*phi_inv). Then f_k in [0, m].
            # Split by the value of f_k:
            # - For f in [0, m-1]: full count c_f = floor((f+1)*phi) - floor(f*phi)
            # - For f = m: partial count = n - floor(m*phi)
            #
            # S02 = sum_{f=0}^{m-1} f^2 * c_f + m^2 * (n - floor(m*phi))
            # S11 = sum_{f=0}^{m-1} f * (sum of k with floor(k*phi_inv) = f) + m * (sum of k with f_k = m)
            #
            # The sum of k with floor(k*phi_inv) = f is:
            # sum_{k=floor(f*phi)+1}^{floor((f+1)*phi)} k = (floor((f+1)*phi) + floor(f*phi) + 1) * c_f / 2
            #
            # This can be expressed using the recursion on floor sums.

            # c_f = floor((f+1)*phi) - floor(f*phi) = (f+1) + floor((f+1)*phi_inv) - f - floor(f*phi_inv)
            #     = 1 + floor((f+1)*phi_inv) - floor(f*phi_inv)
            #
            # For f in [0, m-1], the count c_f = 1 or 2.
            # The total count sum_{f=0}^{m-1} c_f = floor(m*phi) = p
            #
            # S02_partial = sum_{f=0}^{m-1} f^2 * c_f = sum_{f=0}^{m-1} f^2 + sum_{f=0}^{m-1} f^2 * (c_f - 1)
            # Now c_f - 1 = floor((f+1)*phi_inv) - floor(f*phi_inv) = 1 if f+1 appears in phi_inv sequence, else 0
            # So sum_{f=0}^{m-1} f^2 * (c_f - 1) = sum_{j: floor(j*phi_inv) < m} floor(j*phi_inv)^2
            #                                   = S02(floor(m*phi) - 1, phi_inv) ??? Not quite right.

            # Actually: c_f - 1 = 1 iff there exists j with floor(j*phi_inv) = f AND j < p = floor(m*phi)
            # The values floor(j*phi_inv) for j = 1, ..., p-1 cover exactly those f in [0, m-2] for which c_f = 2
            # plus possibly some with c_f = 1.
            #
            # Hmm, the relationship is:
            # {floor(j*phi_inv) : j = 1, ..., p} gives values in [0, m-1] (since floor(p*phi_inv) = m-1 or m-something)
            #
            # Actually floor(floor(m*phi) * phi_inv) = floor(m*phi*phi_inv) = floor(m*(phi-1)) = floor(m*phi_inv + m*(-1+phi_inv))
            # = floor(m*phi_inv + m*(-phi_inv^2)) = ... this is getting complicated.
            #
            # Let me use a different approach: express everything via the known recursion.

            # For the full calculation, I'll use the matrix approach.
            # The transformation from (n, phi_inv) to (m, phi) can be expressed as a linear transformation
            # on the sum vector, plus some polynomial terms.

            p = floor_phi(m)  # p = floor(m * phi) = m + floor(m * phi_inv)
            q = n - p  # count of k with floor(k*phi_inv) = m

            if q < 0:
                q = 0
                p = n

            # S02 = S02_A + S02_B where:
            # S02_A = sum over k=1..p of floor(k*phi_inv)^2 (these have floor values 0..m-1)
            # S02_B = m^2 * q (k's from p+1 to n have floor value m)
            #
            # S11 = S11_A + S11_B where:
            # S11_A = sum over k=1..p of k*floor(k*phi_inv)
            # S11_B = m * sum_{k=p+1}^n k = m * (n(n+1)/2 - p(p+1)/2)

            # For S02_A and S11_A, I need the sums over k=1..p.
            # But p = floor(m*phi) ≈ m * 1.618, which is larger than m, so this doesn't give a reduction.
            #
            # The issue is that the recursion doesn't reduce the problem size for S02 and S11 directly.
            # We need to transform between phi_inv and phi sums.

            # Key identity for S02:
            # S02(n, phi_inv) + S02(m, phi) + 2*S11(m, phi) + something = ???
            #
            # Let me derive using generating functions or lattice point counting.

            # LATTICE POINT APPROACH:
            # S02(n, alpha) = #{(k, j1, j2) : 1 <= k <= n, 1 <= j1 <= floor(k*alpha), 1 <= j2 <= floor(k*alpha)}
            # This counts lattice points in a 3D region.
            #
            # Alternatively, S02 = 2*#{(k, j1, j2) : 1 <= j1 < j2 <= floor(k*alpha)} + S01
            #                   = 2*sum_{k=1}^n C(floor(k*alpha), 2) + S01
            #                   = sum_{k=1}^n floor(k*alpha)*(floor(k*alpha)-1) + S01
            #                   = S02 - S01 + S01 = S02 (circular)
            #
            # Let me try the "diagonal" approach:
            # S02(n, alpha) = sum_{d=1}^{m} (2d-1) * #{k : floor(k*alpha) >= d}
            #              = sum_{d=1}^{m} (2d-1) * (n - ceil(d/alpha) + 1)
            #              = (2m-1)*n - (2m-1)*ceil(m/alpha) + (2m-1) + sum_{d=1}^{m-1} (2d-1)*(n-ceil(d/alpha)+1)
            #
            # This doesn't simplify nicely either.

            # FINAL WORKING APPROACH: Use the Euclidean-like recursion with all 6 sums
            # Sums: (n, S01, S02, sum_k, S11, sum_k2) = (n, S01, S02, n(n+1)/2, S11, n(n+1)(2n+1)/6)
            #
            # The transformation is based on how each sum changes when we go from (n, phi_inv) to (m, phi).

            # For k in [1, n], partition based on f_k = floor(k*phi_inv):
            # - For each f in [0, m-1], k ranges over [L_f+1, L_{f+1}] where L_f = floor(f*phi)
            # - For f = m, k ranges over [L_m+1, n] where L_m = floor(m*phi) = p
            #
            # S01(n, phi_inv) = sum_{f=0}^{m-1} f * c_f + m * q where c_f = L_{f+1} - L_f, q = n - p
            # S02(n, phi_inv) = sum_{f=0}^{m-1} f^2 * c_f + m^2 * q
            # S11(n, phi_inv) = sum_{f=0}^{m-1} f * sum_{k=L_f+1}^{L_{f+1}} k + m * sum_{k=p+1}^n k
            #
            # sum_{f=0}^{m-1} f * c_f = sum_{f=0}^{m-1} f * (L_{f+1} - L_f)
            #                        = sum_{f=1}^m (f-1)*L_f - sum_{f=0}^{m-1} f*L_f
            #                        = (m-1)*L_m - sum_{f=1}^{m-1} L_f
            #                        = (m-1)*p - (S01(m-1, phi) - 0)  [since L_0 = 0]
            #                        = (m-1)*p - S01(m-1, phi)
            #
            # S01(m-1, phi) = (m-1)*m/2 + S01(m-1, phi_inv)
            # So: sum_{f=0}^{m-1} f * c_f = (m-1)*p - (m-1)*m/2 - S01(m-1, phi_inv)
            #
            # S01(n, phi_inv) = (m-1)*p - (m-1)*m/2 - S01(m-1, phi_inv) + m*q
            #                = (m-1)*p - (m-1)*m/2 - S01(m-1, phi_inv) + m*(n-p)
            #                = m*n - p - (m-1)*m/2 - S01(m-1, phi_inv)
            #
            # But we also have: S01(n, phi_inv) = n*m - S01(m, phi) = n*m - m(m+1)/2 - S01(m, phi_inv)
            # Let me verify: S01(m-1, phi_inv) vs S01(m, phi_inv)
            # S01(m, phi_inv) = S01(m-1, phi_inv) + floor(m*phi_inv)
            # So the difference is floor(m*phi_inv).
            #
            # From identity: S01(n, phi_inv) = n*m - m(m+1)/2 - S01(m, phi_inv)
            # From partition: S01(n, phi_inv) = m*n - p - (m-1)*m/2 - S01(m-1, phi_inv)
            #                                = m*n - p - (m-1)*m/2 - S01(m, phi_inv) + floor(m*phi_inv)
            #
            # Equating: n*m - m(m+1)/2 = m*n - p - (m-1)*m/2 + floor(m*phi_inv)
            #           -m(m+1)/2 = -p - (m-1)*m/2 + floor(m*phi_inv)
            #           -m(m+1)/2 + (m-1)*m/2 = -p + floor(m*phi_inv)
            #           -m = -p + floor(m*phi_inv)
            #           p = m + floor(m*phi_inv)
            # Yes! This is consistent since p = floor(m*phi) = floor(m*(1+phi_inv)) = m + floor(m*phi_inv).

            # Now let's derive S02:
            # sum_{f=0}^{m-1} f^2 * c_f = sum_{f=0}^{m-1} f^2 * (L_{f+1} - L_f)
            #                          = sum_{f=1}^m (f-1)^2 * L_f - sum_{f=0}^{m-1} f^2 * L_f
            #                          = (m-1)^2 * L_m - sum_{f=1}^{m-1} (2f-1) * L_f
            #                          = (m-1)^2 * p - 2*sum_{f=1}^{m-1} f*L_f + sum_{f=1}^{m-1} L_f
            #                          = (m-1)^2 * p - 2*S11(m-1, phi) + S01(m-1, phi)
            #
            # S11(m-1, phi) = sum_{f=1}^{m-1} f * floor(f*phi) = sum f * (f + floor(f*phi_inv))
            #              = (m-1)*m*(2m-1)/6 + S11(m-1, phi_inv)
            # S01(m-1, phi) = (m-1)*m/2 + S01(m-1, phi_inv)
            #
            # So:
            # sum f^2 * c_f = (m-1)^2*p - 2*(m-1)*m*(2m-1)/6 - 2*S11(m-1, phi_inv) + (m-1)*m/2 + S01(m-1, phi_inv)
            #
            # S02(n, phi_inv) = above + m^2 * q
            #                = (m-1)^2*p - (m-1)*m*(2m-1)/3 - 2*S11(m-1, phi_inv) + (m-1)*m/2 + S01(m-1, phi_inv) + m^2*(n-p)
            #
            # Similarly for S11:
            # sum_{f=0}^{m-1} f * sum_{k=L_f+1}^{L_{f+1}} k = sum_{f=0}^{m-1} f * (L_{f+1}^2 + L_{f+1} - L_f^2 - L_f)/2
            #                                              = (1/2) * sum f * ((L_{f+1} - L_f)*(L_{f+1} + L_f + 1))
            #                                              = (1/2) * sum f * c_f * (L_{f+1} + L_f + 1)
            #
            # This is getting very complex. Let me try a direct recursive computation.

            # SIMPLIFIED APPROACH: Recursively compute using the simpler formula
            # S01(n, phi_inv) = n*m - m*(m+1)/2 - S01(m, phi_inv)
            # For S02 and S11, compute them jointly with S01 using the same recursion depth.

            # Actually, let me just compute S01, S02, S11 by iterating through the recursion steps
            # and maintaining all three sums together.

            # The recursion: (n, phi_inv) -> (m, phi) -> (m', phi_inv) -> ...
            # where m = floor(n*phi_inv), m' = floor(m*phi_inv)
            #
            # At each step, we transform the sums.

            # Alternatively, compute S01 recursively (O(log n)), and for S02 and S11, use the
            # closed-form expressions in terms of S01 and other sums.

            # Let's compute S01 first using the standard recursion.
            # Then express S02 and S11 in terms of S01 values at different scales.

            # Actually, I realize the Java solution uses a FloorSums library that handles all these
            # complexities. Let me implement a simpler version that computes the sums via the
            # grouping approach but using the Fibonacci structure.

            pass

        else:
            # alpha = phi, 1/alpha = phi_inv
            # floor(k*phi) = k + floor(k*phi_inv)

            S01_phi_inv, S02_phi_inv, S11_phi_inv = compute_sums(n, True)

            n_mod = n % M
            sum_k_val = n_mod * (n_mod + 1) % M * inv2 % M
            sum_k2_val = n_mod * (n_mod + 1) % M * (2*n_mod + 1) % M * inv6 % M

            # S01(n, phi) = sum floor(k*phi) = sum (k + floor(k*phi_inv)) = n(n+1)/2 + S01(n, phi_inv)
            S01 = (sum_k_val + S01_phi_inv) % M

            # S02(n, phi) = sum floor(k*phi)^2 = sum (k + floor(k*phi_inv))^2
            #             = sum k^2 + 2*sum k*floor(k*phi_inv) + sum floor(k*phi_inv)^2
            #             = n(n+1)(2n+1)/6 + 2*S11(n, phi_inv) + S02(n, phi_inv)
            S02 = (sum_k2_val + 2 * S11_phi_inv + S02_phi_inv) % M

            # S11(n, phi) = sum k*floor(k*phi) = sum k*(k + floor(k*phi_inv))
            #             = n(n+1)(2n+1)/6 + S11(n, phi_inv)
            S11 = (sum_k2_val + S11_phi_inv) % M

            memo[key] = (S01, S02, S11)
            return (S01, S02, S11)

        # For phi_inv case, we need to work harder.
        # Use the splitting approach with recursion to a smaller scale.

        # Let me try a different formulation using the inverse relationship.
        # For (n, phi_inv), we can express in terms of (m, phi) and polynomial terms.

        # Actually, the fundamental identity is:
        # For alpha = p/q (rational), sum_{k=1}^n floor(k*p/q) can be computed via Euclidean recursion.
        # For irrational alpha like phi_inv, we use the continued fraction structure.
        #
        # The key is that the recursion depth is O(log n) because m = floor(n*phi_inv) ≈ 0.618n.

        # Let me implement the recursion for S01 and verify it works, then extend.

        # S01(n, phi_inv) = n*m - m*(m+1)/2 - S01(m, phi_inv)
        m = floor_phi_inv(n)
        if m == 0:
            memo[key] = (0, 0, 0)
            return (0, 0, 0)

        S01_m, S02_m, S11_m = compute_sums(m, True)

        n_mod = n % M
        m_mod = m % M

        S01 = (n_mod * m_mod - m_mod * (m_mod + 1) % M * inv2 - S01_m) % M

        # For S02 and S11, I need to derive the recursion.
        # Using the Beatty complementary property:
        # sum_{k=1}^n floor(k*phi_inv)^2 + sum_{j=1}^m floor(j*phi)^2 = ???
        #
        # There's no simple sum identity for squared floor functions.
        # Instead, use the counting/grouping approach.

        # Let p = floor(m*phi) be the transition point.
        # For k in [1, p], floor(k*phi_inv) < m
        # For k in [p+1, n], floor(k*phi_inv) = m

        p = floor_phi(m)
        q = n - p  # count of k with floor = m

        if q < 0:
            # All values in [1, n] have floor < m
            # This means n < ceil(m*phi), so n <= floor(m*phi) = p
            # In this case, we recurse on the (n, phi_inv) sums over [1, n] ⊂ [1, p]
            # which means all floor values are in [0, m-1].
            # We can express this via (m-1, phi_inv) sums.

            # Actually, let's just compute directly for small n.
            if n <= 1000:
                s01, s02, s11 = 0, 0, 0
                for k in range(1, n + 1):
                    f = floor_phi_inv(k)
                    s01 = (s01 + f) % M
                    s02 = (s02 + f * f) % M
                    s11 = (s11 + k * f) % M
                memo[key] = (s01, s02, s11)
                return (s01, s02, s11)

            # For large n with q < 0, this shouldn't happen often. Let's handle it.
            q = 0
            p = n

        # Split sums:
        # S01 = sum_{k=1}^p floor(k*phi_inv) + m * q  (already computed above)
        # S02 = sum_{k=1}^p floor(k*phi_inv)^2 + m^2 * q
        # S11 = sum_{k=1}^p k*floor(k*phi_inv) + m * sum_{k=p+1}^n k

        # For the sums over [1, p], we need another recursion or identity.
        # The values floor(k*phi_inv) for k in [1, p] are in [0, m-1].
        # These form a Sturmian sequence related to the Fibonacci word.

        # Here's the key insight: for k in [1, p] with p = floor(m*phi),
        # the floor values floor(k*phi_inv) for k=1,...,p are exactly:
        # 0, 1, 1, 2, 3, 3, 4, 5, 5, ... up to some prefix.
        #
        # The sums can be expressed using the (m, phi) and (m, phi_inv) relationships.

        # Actually, we can use:
        # sum_{k=1}^p floor(k*phi_inv)^2 = sum_{f=0}^{m-1} f^2 * count_f
        # where count_f = floor((f+1)*phi) - floor(f*phi)
        #
        # sum_{f=0}^{m-1} f^2 * (floor((f+1)*phi) - floor(f*phi))
        # Using summation by parts:
        # = (m-1)^2 * floor(m*phi) - sum_{f=1}^{m-1} ((f)^2 - (f-1)^2) * floor(f*phi)
        # = (m-1)^2 * p - sum_{f=1}^{m-1} (2f-1) * floor(f*phi)
        # = (m-1)^2 * p - 2*S11(m-1, phi) + S01(m-1, phi)

        # S11(m-1, phi) = sum_{f=1}^{m-1} f * floor(f*phi) = sum f*(f + floor(f*phi_inv))
        #              = sum_{f=1}^{m-1} f^2 + S11(m-1, phi_inv)
        #              = (m-1)*m*(2m-1)/6 + S11(m-1, phi_inv)
        # S01(m-1, phi) = sum_{f=1}^{m-1} (f + floor(f*phi_inv)) = (m-1)*m/2 + S01(m-1, phi_inv)

        # So:
        # sum_{k=1}^p floor(k*phi_inv)^2 = (m-1)^2*p - 2*(m-1)*m*(2m-1)/6 - 2*S11(m-1,phi_inv) + (m-1)*m/2 + S01(m-1,phi_inv)

        # Similarly for S11:
        # sum_{k=1}^p k * floor(k*phi_inv) = sum_{f=0}^{m-1} f * sum_{k: floor(k*phi_inv)=f} k
        # = sum_{f=0}^{m-1} f * sum_{k=floor(f*phi)+1}^{floor((f+1)*phi)} k
        # = sum_{f=0}^{m-1} f * (floor((f+1)*phi) + floor(f*phi) + 1) * (floor((f+1)*phi) - floor(f*phi)) / 2

        # This is complex. Let me denote L_f = floor(f*phi) and simplify.
        # sum = sum_{f=0}^{m-1} f * (L_{f+1} + L_f + 1) * (L_{f+1} - L_f) / 2
        #     = (1/2) * sum f * (L_{f+1}^2 - L_f^2 + L_{f+1} - L_f)
        #     = (1/2) * (sum f * (L_{f+1}^2 - L_f^2) + sum f * (L_{f+1} - L_f))
        #     = (1/2) * (sum f * (L_{f+1}^2 - L_f^2) + S01_A)  [where S01_A = sum_{k=1}^p floor(k*phi_inv)]

        # sum f * (L_{f+1}^2 - L_f^2) = sum_{f=1}^m (f-1)*L_f^2 - sum_{f=0}^{m-1} f*L_f^2
        #                            = (m-1)*L_m^2 - sum_{f=1}^{m-1} L_f^2
        #                            = (m-1)*p^2 - S02(m-1, phi)
        # S02(m-1, phi) = (m-1)*m*(2m-1)/6 + 2*S11(m-1,phi_inv) + S02(m-1,phi_inv)

        # This requires S02(m-1, phi_inv) and S11(m-1, phi_inv), which recurse.

        # The recursion is now: (n, phi_inv) depends on (m, phi_inv) where m < n*phi_inv < n
        # and also on (m-1, phi_inv) for the group A sums.
        # But m-1 < m, so the recursion still decreases.

        # Let me implement this carefully.

        # Get sums for (m-1, phi_inv)
        if m >= 2:
            S01_m1, S02_m1, S11_m1 = compute_sums(m - 1, True)
        else:
            S01_m1, S02_m1, S11_m1 = 0, 0, 0

        m1_mod = (m - 1) % M
        p_mod = p % M

        # S01_A = sum_{k=1}^p floor(k*phi_inv) = (m-1)*p - (m-1)*m/2 - S01(m-1, phi_inv)
        # But we already computed S01 = S01_A + m*q using the Beatty identity.
        # Let me verify: S01 = n*m - m*(m+1)/2 - S01(m, phi_inv)
        #                   = n*m - m*(m+1)/2 - S01(m-1, phi_inv) - floor(m*phi_inv)
        # And S01_A + m*q = S01
        # So S01_A = S01 - m*q

        S01_A = (S01 - m_mod * (q % M)) % M

        # S02_A = sum_{k=1}^p floor(k*phi_inv)^2
        #       = (m-1)^2*p - 2*S11(m-1, phi) + S01(m-1, phi)
        #       = (m-1)^2*p - 2*((m-1)*m*(2m-1)/6 + S11(m-1, phi_inv)) + (m-1)*m/2 + S01(m-1, phi_inv)

        sum_k2_m1 = m1_mod * (m1_mod + 1) % M * (2*m1_mod + 1) % M * inv6 % M  # (m-1)*m*(2m-1)/6
        sum_k_m1 = m1_mod * (m1_mod + 1) % M * inv2 % M  # (m-1)*m/2

        S02_A = (m1_mod * m1_mod % M * p_mod
                 - 2 * sum_k2_m1 - 2 * S11_m1
                 + sum_k_m1 + S01_m1) % M

        # S02 = S02_A + m^2 * q
        S02 = (S02_A + m_mod * m_mod % M * (q % M)) % M

        # For S11:
        # S11_A = (1/2) * ((m-1)*p^2 - S02(m-1, phi) + S01_A)
        # S02(m-1, phi) = sum_{f=1}^{m-1} (f + floor(f*phi_inv))^2
        #              = (m-1)*m*(2m-1)/6 + 2*S11(m-1, phi_inv) + S02(m-1, phi_inv)

        S02_phi_m1 = (sum_k2_m1 + 2 * S11_m1 + S02_m1) % M

        S11_A = (m1_mod * p_mod % M * p_mod - S02_phi_m1 + S01_A) % M * inv2 % M

        # S11 = S11_A + m * sum_{k=p+1}^n k
        # sum_{k=p+1}^n k = n*(n+1)/2 - p*(p+1)/2
        sum_k_n = n_mod * (n_mod + 1) % M * inv2 % M
        sum_k_p = p_mod * (p_mod + 1) % M * inv2 % M
        sum_tail = (sum_k_n - sum_k_p) % M

        S11 = (S11_A + m_mod * sum_tail) % M

        memo[key] = (S01, S02, S11)
        return (S01, S02, S11)

    # Compute the final answer
    S01, S02, S11 = compute_sums(N, True)

    sum_y = sum_k(N)
    sum_y2 = sum_k2(N)

    # S(N) = sum_{y=1}^N ( 3(y^2-y)/2 - (floor(y/phi)^2 + floor(y/phi))/2 - y*floor(y/phi) )
    # where floor(y/phi) = floor(y*phi_inv)

    # Part 1: 3*(sum_y2 - sum_y)/2
    part1 = 3 * (sum_y2 - sum_y + M) % M * inv2 % M

    # Part 2: -(sum floor^2 + sum floor)/2 = -(S02 + S01)/2
    part2 = (S02 + S01) % M * inv2 % M

    # Part 3: -S11
    part3 = S11

    ans = (part1 - part2 - part3 + 2 * M) % M
    return ans


if __name__ == "__main__":
    print(solve())
