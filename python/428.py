"""Project Euler Problem 428: Necklace of circles.

A necklace triplet (a,b,c) with k >= 3 circles exists when the Steiner chain
condition delta = (1+s^2)/(1-s^2) matches, where s = sin(pi/k).
Only k=3,4,6 give rational delta, yielding three disjoint sets.

k=4: ac = b(a+b+c)  =>  S4 = sum_{b=1}^N tau(2*b^2)
k=3: ac = 3b(a+b+c) =>  S3 = sum_{b=1}^N tau(12*b^2)
k=6: 3ac = b(a+b+c) =>  S6 via divisor counting with mod 3 conditions

Answer = S3 + S4 + S6.

Uses:
- Hyperbola method for F(X) = sum 2^omega(d)
- Quotient grouping for T(X) = sum tau(b^2) = sum_{d} 2^omega(d)*floor(X/d)
- Recursive stripping of factors 2,3 for T_odd, T_odd_no3
- G = lambda * B decomposition for S6_chi (Dirichlet convolution)
- Lucy DP for pi_1 (prime counting in arithmetic progression)
- DFS over 1mod3-smooth numbers with large prime contribution via partial summation
"""

from math import isqrt


def solve(N):
    sqrtN = isqrt(N)

    # Sieve mu and primes up to N^{2/3}
    cbrt = int(round(N ** (1 / 3)))
    while (cbrt + 1) ** 3 <= N:
        cbrt += 1
    while cbrt ** 3 > N:
        cbrt -= 1
    sieve_limit = max(cbrt * cbrt, sqrtN + 1)

    mu = [0] * (sieve_limit + 1)
    mu[1] = 1
    is_comp = bytearray(sieve_limit + 1)
    primes_all = []
    for i in range(2, sieve_limit + 1):
        if not is_comp[i]:
            primes_all.append(i)
            mu[i] = -1
        for p in primes_all:
            if p * i > sieve_limit:
                break
            is_comp[p * i] = 1
            if i % p == 0:
                mu[p * i] = 0
                break
            mu[p * i] = -mu[i]

    mu_prefix = [0] * (sieve_limit + 1)
    for i in range(1, sieve_limit + 1):
        mu_prefix[i] = mu_prefix[i - 1] + mu[i]

    # Mertens function with caching for large values
    mertens_cache = {}

    def mertens(n):
        if n <= sieve_limit:
            return mu_prefix[n]
        if n in mertens_cache:
            return mertens_cache[n]
        s = 0
        d = 2
        while d <= n:
            q = n // d
            d_max = n // q
            s += (d_max - d + 1) * mertens(q)
            d = d_max + 1
        result = 1 - s
        mertens_cache[n] = result
        return result

    # Pre-fill mertens cache
    d = 1
    while d <= N:
        mertens(N // d)
        d = N // (N // d) + 1

    # Primes up to sqrtN (for Lucy DP)
    primes_small = [p for p in primes_all if p <= sqrtN]

    # Q(X) = #{squarefree n <= X}
    Q_cache = {}

    def Q(X):
        if X <= 0:
            return 0
        if X in Q_cache:
            return Q_cache[X]
        s = 0
        for k in range(1, isqrt(X) + 1):
            if mu[k] != 0:
                s += mu[k] * (X // (k * k))
        Q_cache[X] = s
        return s

    # F(X) = sum_{d=1}^X 2^omega(d)
    F_cache = {}

    def F_val(X):
        if X <= 0:
            return 0
        if X in F_cache:
            return F_cache[X]
        sX = isqrt(X)
        result = 0
        for d in range(1, sX + 1):
            if mu[d] != 0:
                result += X // d
        max_q = X // (sX + 1)
        for q in range(1, max_q + 1):
            result += q * (Q(X // q) - Q(X // (q + 1)))
        F_cache[X] = result
        return result

    # Precompute F for quotient values of all needed T arguments
    needed_T = set()
    for a in range(61):
        pw2 = 1 << a
        if pw2 > N:
            break
        for c in range(40):
            pw3 = 3 ** c
            pw = pw2 * pw3
            if pw > N:
                break
            needed_T.add(N // pw)

    for X in sorted(needed_T):
        dd = 1
        while dd <= X:
            qq = X // dd
            F_val(qq)
            dd = X // qq + 1

    # T(X) = sum_{b=1}^X tau(b^2) via quotient grouping
    T_cache = {}

    def T_c(X):
        if X <= 0:
            return 0
        if X in T_cache:
            return T_cache[X]
        result = 0
        d = 1
        while d <= X:
            q = X // d
            d_max = X // q
            result += q * (F_val(d_max) - F_val(d - 1))
            d = d_max + 1
        T_cache[X] = result
        return result

    # T_odd(X) = sum_{m odd, m<=X} tau(m^2)
    T_odd_cache = {}

    def T_odd(X):
        if X <= 0:
            return 0
        if X in T_odd_cache:
            return T_odd_cache[X]
        result = T_c(X)
        a = 1
        pw = 2
        while pw <= X:
            result -= (2 * a + 1) * T_odd(X // pw)
            a += 1
            pw *= 2
        T_odd_cache[X] = result
        return result

    # T_on3(X) = sum_{m odd, gcd(m,3)=1, m<=X} tau(m^2)
    T_on3_cache = {}

    def T_on3(X):
        if X <= 0:
            return 0
        if X in T_on3_cache:
            return T_on3_cache[X]
        result = T_odd(X)
        c = 1
        pw = 3
        while pw <= X:
            result -= (2 * c + 1) * T_on3(X // pw)
            c += 1
            pw *= 3
        T_on3_cache[X] = result
        return result

    # S4 = sum_{b=1}^N tau(2*b^2)
    S4 = 0
    a = 0
    pw = 1
    while pw <= N:
        S4 += (2 * a + 2) * T_odd(N // pw)
        a += 1
        pw *= 2

    # S3 = sum_{b=1}^N tau(12*b^2)
    S3 = 0
    a = 0
    pw2 = 1
    while pw2 <= N:
        c = 0
        pw3 = 1
        while pw2 * pw3 <= N:
            S3 += (2 * a + 3) * (2 * c + 2) * T_on3(N // (pw2 * pw3))
            c += 1
            pw3 *= 3
        a += 1
        pw2 *= 2

    # S6_div3: contribution from b divisible by 3
    S6_div3 = 0
    v = 1
    pw3 = 3
    while pw3 <= N:
        a = 0
        pw2 = 1
        while pw2 * pw3 <= N:
            S6_div3 += (2 * v - 1) * (2 * a + 3) * T_on3(N // (pw2 * pw3))
            a += 1
            pw2 *= 2
        v += 1
        pw3 *= 3

    # S6_tau: sum_{b coprime to 3} tau(4*b^2)
    S6_tau = 0
    a = 0
    pw = 1
    while pw <= N:
        S6_tau += (2 * a + 3) * T_on3(N // pw)
        a += 1
        pw *= 2

    # === S6_chi via G = lambda * B decomposition ===
    # G(b) = chi_3(b) * tau_chi(b), multiplicative.
    # G = lambda * B where B supported on primes ≡ 1 mod 3, B(p^k) = 4k.
    # sum G(b) for b coprime to 3 = sum_d B(d) * L3(N/d)
    # L3(X) = L(X) + L(X//3), L(X) = sum_{j=1}^{isqrt(X)} M(X//j^2)

    L_cache = {}

    def L(X):
        if X <= 0:
            return 0
        if X in L_cache:
            return L_cache[X]
        total = 0
        for j in range(1, isqrt(X) + 1):
            total += mertens(X // (j * j))
        L_cache[X] = total
        return total

    L3_cache = {}

    def L3(X):
        if X <= 0:
            return 0
        if X in L3_cache:
            return L3_cache[X]
        result = L(X) + L(X // 3)
        L3_cache[X] = result
        return result

    primes_1mod3 = [p for p in primes_all if p % 3 == 1]

    # Lucy DP for pi_1 (primes ≡ 1 mod 3)
    small_pi1 = [0] * (sqrtN + 1)
    big_pi1 = [0] * (sqrtN + 1)
    small_pi2 = [0] * (sqrtN + 1)
    big_pi2 = [0] * (sqrtN + 1)

    for v in range(1, sqrtN + 1):
        small_pi1[v] = (v + 2) // 3 - 1
        small_pi2[v] = (v + 1) // 3
    for k in range(1, sqrtN + 1):
        V = N // k
        big_pi1[k] = (V + 2) // 3 - 1
        big_pi2[k] = (V + 1) // 3

    quotients_desc = []
    d = 1
    while d <= N:
        quotients_desc.append(N // d)
        d = N // (N // d) + 1
    quotients_desc.sort(reverse=True)

    for p in primes_small:
        if p == 3:
            continue
        pp = p * p
        p1 = small_pi1[p - 1]
        p2 = small_pi2[p - 1]

        for V in quotients_desc:
            if V < pp:
                break
            Vp = V // p
            c1 = small_pi1[Vp] if Vp <= sqrtN else big_pi1[N // Vp]
            c2 = small_pi2[Vp] if Vp <= sqrtN else big_pi2[N // Vp]

            if p % 3 == 1:
                if V <= sqrtN:
                    small_pi1[V] -= c1 - p1
                    small_pi2[V] -= c2 - p2
                else:
                    k = N // V
                    big_pi1[k] -= c1 - p1
                    big_pi2[k] -= c2 - p2
            else:
                if V <= sqrtN:
                    old1 = small_pi1[V]
                    old2 = small_pi2[V]
                    small_pi1[V] = old1 - (c2 - p2)
                    small_pi2[V] = old2 - (c1 - p1)
                else:
                    k = N // V
                    old1 = big_pi1[k]
                    old2 = big_pi2[k]
                    big_pi1[k] = old1 - (c2 - p2)
                    big_pi2[k] = old2 - (c1 - p1)

    def pi1(V):
        if V < 2:
            return 0
        if V <= sqrtN:
            return small_pi1[V]
        return big_pi1[N // V]

    # DFS over d with prime factors ≡ 1 mod 3 (≤ sqrtN)
    # plus large prime contribution via partial summation with pi_1
    sum_G = 0

    def dfs_g(idx, d_val, b_val, last_prime):
        nonlocal sum_G
        sum_G += b_val * L3(N // d_val)

        # Large prime contribution
        upper_p = N // d_val
        lower_p = max(last_prime, sqrtN)

        if upper_p > lower_p:
            large_sum = 0
            p = lower_p + 1
            while p <= upper_p:
                q = N // (d_val * p)
                if q > 0:
                    p_range_hi = min(upper_p, N // (d_val * q))
                else:
                    p_range_hi = upper_p
                p_range_lo = (
                    max(lower_p + 1, N // (d_val * (q + 1)) + 1)
                    if q < upper_p
                    else lower_p + 1
                )

                cnt = pi1(p_range_hi) - pi1(p_range_lo - 1)
                if cnt > 0:
                    large_sum += cnt * L3(q)

                p = p_range_hi + 1

            sum_G += 4 * b_val * large_sum

        for i in range(idx, len(primes_1mod3)):
            p = primes_1mod3[i]
            if p > sqrtN:
                break
            if d_val * p > N:
                break
            pk = p
            k = 1
            while d_val * pk <= N:
                dfs_g(i + 1, d_val * pk, b_val * (4 * k), p)
                k += 1
                pk *= p

    dfs_g(0, 1, 1, 0)

    S6_chi = -sum_G
    S6 = S6_div3 + (S6_tau + S6_chi) // 2

    return S3 + S4 + S6


if __name__ == "__main__":
    print(solve(10 ** 9))
