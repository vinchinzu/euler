"""Project Euler Problem 621: Sum of three triangular numbers.

G(n) = number of ordered triples of triangular numbers summing to n.
Find G(17526 * 10^9).

Since t_k = k(k+1)/2, we have G(n) = number of (a,b,c) with a,b,c >= 0
and a(a+1)/2 + b(b+1)/2 + c(c+1)/2 = n.

This equals the number of (x,y,z) with x,y,z odd positive and
x^2 + y^2 + z^2 = 8n + 3 (where x=2a+1, etc).

For each k (first triangular number), the remainder is m = n - k(k+1)/2.
We need the number of ordered pairs of triangular numbers summing to m.
This equals the number of (x,y) with x,y odd positive and x^2+y^2 = 8m+2.
Each such pair corresponds to r_2(8m+2)/4 where r_2 counts signed representations.

r_2(N) = 4 * sum_{d|N} chi(d) where chi(d) = (-1)^((d-1)/2) for odd d (Jacobi symbol).
Actually r_2(N) = 4 * (d_1(N) - d_3(N)) where d_1 = #divisors ≡ 1 mod 4,
d_3 = #divisors ≡ 3 mod 4.

For our case N = 8m+2 = 2*(4m+1). Since 4m+1 is odd:
r_2(2*(4m+1)) = r_2(2) * r_2(4m+1)... no, r_2 is not multiplicative in general,
but for the divisor sum it works differently.

Actually: r_2(n) = 4*(d_1(n) - d_3(n)).
For n = 2*q with q odd: divisors of n are d and 2d where d|q.
d_1(n) - d_3(n) = sum_{d|q} [d mod 4 == 1] - [d mod 4 == 3] + [2d mod 4 == 1] - [2d mod 4 == 3]
Since q is odd, d is odd, 2d ≡ 2 mod 4, so [2d mod 4 == 1] = [2d mod 4 == 3] = 0.
So d_1(n) - d_3(n) = d_1(q) - d_3(q). Thus r_2(2q) = r_2(q) for odd q, but through divisors.

Actually wait, r_2(2q) where q is odd: the formula r_2(n) = 4(d_1(n) - d_3(n)) counts
all integer representations including negative and zero. For n = 2q with q odd:
the divisors of 2q are: {d, 2d : d | q}. Since d is odd, d is either 1 or 3 mod 4.
2d is 2 mod 4, which is neither 1 nor 3 mod 4. So d_1(2q) - d_3(2q) = d_1(q) - d_3(q).

So r_2(8m+2) = 4*(d_1(4m+1) - d_3(4m+1)).

The number of ordered pairs of non-negative (a,b) with a^2+b^2 = N:
From r_2(N) = 4*(d_1-d_3), we have r_2 counts (a,b) with a,b in Z. For N>0,
pairs with a=0 or b=0 contribute r_1(N) for each, but for N=8m+2 which is 2 mod 4,
the only way a^2 + 0^2 = 8m+2 is a^2 = 8m+2, which requires 8m+2 to be a perfect square.
8m+2 = 2 mod 8, and squares are 0 or 1 mod 4, so 8m+2 is 2 mod 4 which can't be a square.
So there are no representations with a=0 or b=0.
Similarly all representations have both a,b nonzero.
Each unordered pair {|a|, |b|} with a != b gives 8 signed representations (4 sign choices * 2 orderings).
Each pair with |a|=|b| gives 4 (since (a,a),(a,-a),(-a,a),(-a,-a)).
But for N = 8m+2: a^2+b^2 = 8m+2 -> 2(a^2+b^2)/2 -> a,b must have the same parity.
Actually a^2+b^2 = 8m+2 = 2 mod 8. If a,b both odd: a^2+b^2 ≡ 1+1 = 2 mod 8. Good.
If a,b both even: a^2+b^2 ≡ 0 mod 4 ≡ 0 or 4 mod 8. Not 2. Bad.
If one odd one even: a^2+b^2 ≡ 1 mod 2, odd. Bad.
So a,b must both be odd.

For a=b: 2a^2 = 8m+2 -> a^2 = 4m+1. a odd, a^2 ≡ 1 mod 8, 4m+1 mod 8 depends on m.
If 4m+1 = a^2 then a^2 ≡ 1 mod 4. OK. But a^2 mod 8 = 1. 4m+1 mod 8: m even -> 1 mod 8 -> possible.
m odd -> 5 mod 8 -> 5 is not 1 mod 8 -> impossible.

Anyway, the number of ways to write 8m+2 as x^2+y^2 with x,y > 0, odd:
= r_2(8m+2) / 4 (since each such (x,y) with x,y > 0 odd gives 4 sign representations
due to the 4 sign choices on the pair, and ordering is preserved).

Wait no: r_2(N) counts ordered (a,b) with a,b in Z. For each pair (x,y) with x>0, y>0,
x != y: there are 8 = 4 signs * 2 orderings. For x=y: there are 4 = 4 signs.
So number of (x,y) with x,y > 0 = (r_2(N) - 4*[is_square(N/2)]) / 8 + [is_square(N/2)]
... this is getting complicated. Let me just directly compute.

G(n) = sum_{k=0}^{L} f(n - k(k+1)/2)
where f(m) = #{(a,b) >= 0 : a(a+1)/2 + b(b+1)/2 = m}
         = #{(x,y) odd, >= 1 : x^2+y^2 = 8m+2}

And #{(x,y) odd, >= 1 : x^2+y^2 = N} with N = 8m+2:
= r_2(N) / 4 since all representations have both x,y odd nonzero,
  and for each (x,y) with x,y>0 there are 4 sign combos: (+-x, +-y).

So f(m) = r_2(8m+2)/4 = (d_1(4m+1) - d_3(4m+1)).

G(n) = sum_{k=0}^{L} (d_1(Q_k) - d_3(Q_k)) where Q_k = 4*(n - k(k+1)/2) + 1 = 4n+1 - 2k(k+1).

We need to compute d_1(Q_k) - d_3(Q_k) for each k efficiently.

For a number q, d_1(q) - d_3(q) = product over p^e || q of:
  if p = 2: 1 (since all powers of 2 are even, doesn't contribute to 1 or 3 mod 4)
  Actually wait, 2 contributes: 2^0=1≡1, 2^1=2≡2, etc. Divisors involving 2 are even, not 1 or 3 mod 4.
  So for p=2: it doesn't change the count since powers of 2 times odd ≡ 0 mod 2.

Actually the formula is: for n = 2^a * prod p_i^{e_i} * prod q_j^{f_j} where p_i ≡ 1 mod 4
and q_j ≡ 3 mod 4:
d_1(n) - d_3(n) = 0 if any f_j is odd, otherwise = prod (e_i + 1).

This is because the Dirichlet character chi_4(d) = (-1)^((d-1)/2) for odd d, 0 for even d,
is completely multiplicative. And sum_{d|n} chi_4(d) = prod_{p|n} sum_{k=0}^{v_p(n)} chi_4(p)^k.
For p=2: chi_4(2) = 0, so the product factor is chi_4(1) = 1. No wait, sum chi_4(2^k) for k=0..a
= chi_4(1) = 1 (since chi_4(2^k) = 0 for k >= 1).
For p ≡ 1 mod 4: chi_4(p) = 1, sum = e+1.
For p ≡ 3 mod 4: chi_4(p) = -1, sum = 1 if e even, 0 if e odd.

So d_1(n) - d_3(n) = prod_{p≡1(4)} (e_p+1) * prod_{p≡3(4)} [e_p even ? 1 : 0].
(Ignoring powers of 2 which just contribute 1.)

Since Q_k = 4n+1 - 2k(k+1) is always odd (4n+1 is odd, 2k(k+1) is even), no factor of 2.

So I need to factorize Q_k for each k and compute the product.
Q_k = 4*17526*10^9 + 1 - 2k(k+1) = 70104000000001 - 2k(k+1).
L = floor((-1+sqrt(1+8n))/2) where n = 17526*10^9.
sqrt(8*17526*10^9+1) ≈ sqrt(1.40208*10^14) ≈ 11841372. So L ≈ 5920685.

That's ~6M values of k. For each, we need to factorize Q_k. Direct factorization is too slow.
Instead, use a sieve approach: for each small prime p, find which Q_k are divisible by p.

Q_k ≡ 0 mod p iff 2k(k+1) ≡ Q_0 mod p iff 2k^2+2k ≡ Q_0 mod p.
This is a quadratic in k mod p, giving 0, 1, or 2 solutions.
Then every p-th value of k from each solution.
"""

import subprocess, tempfile, os
from math import isqrt

def solve():
    N = 17526 * 10**9
    M = 8 * N + 3  # = 140208000000003
    # L = max k with k(k+1)/2 <= N
    L = (isqrt(8*N+1) - 1) // 2  # = 5920685

    # Q_k = 4*(N - k(k+1)/2) + 1 = 4N+1 - 2k(k+1) for k=0,...,L
    # All Q_k are positive odd integers.
    # G(N) = sum_{k=0}^{L} (d_1(Q_k) - d_3(Q_k))
    # where d_1-d_3 = prod_{p≡1(4)} (e_p+1) if all p≡3(4) exponents are even, else 0.

    # Use a sieve: maintain result[k] = product so far, and remaining[k] = Q_k / (sieved primes).
    # For each prime p up to sqrt(max Q_k), find k values where p | Q_k, and process.

    Q0 = 4*N + 1  # 70104000000001
    # Q_k = Q0 - 2k(k+1)
    # max Q_k = Q0 (at k=0), min Q_k = Q0 - 2*L*(L+1) ≈ Q0 - 8N ≈ Q0 - Q0 + 1 = 1.
    # sqrt(Q0) ≈ 8372812

    # Sieve primes up to sqrt(Q0)
    sieve_limit = isqrt(Q0) + 1

    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;

int main() {
    ll N_val, Q0_val;
    int L_val;
    scanf("%lld %lld %d", &N_val, &Q0_val, &L_val);

    int sz = L_val + 1;

    /* remaining[k] = Q_k with small prime factors removed */
    /* result[k] = product of (e_p+1) for p≡1(4), 0 if any p≡3(4) has odd exponent */
    ll *remaining = (ll *)malloc(sz * sizeof(ll));
    int *result = (int *)malloc(sz * sizeof(int));

    for (int k = 0; k < sz; k++) {
        remaining[k] = Q0_val - 2LL * k * (k + 1);
        result[k] = 1;
    }

    /* Sieve primes up to sqrt(Q0) using simple sieve */
    int sieve_limit = (int)(sqrt((double)Q0_val)) + 2;
    char *is_prime = (char *)calloc(sieve_limit + 1, 1);
    memset(is_prime, 1, sieve_limit + 1);
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (ll)i * i <= sieve_limit; i++)
        if (is_prime[i])
            for (int j = i * i; j <= sieve_limit; j += i)
                is_prime[j] = 0;

    /* For each prime p, find k values where p | Q_k */
    /* Q_k = Q0 - 2k(k+1) ≡ 0 (mod p) */
    /* 2k^2 + 2k - Q0 ≡ 0 (mod p) */
    /* k^2 + k ≡ Q0/2 (mod p) -- need inverse of 2 */
    /* (k + 1/2)^2 ≡ Q0/2 + 1/4 (mod p) */
    /* (2k+1)^2 ≡ 2*Q0 + 1 (mod p) */
    /* Note: 2*Q0 + 1 = 2*(4N+1) + 1 = 8N+3 */

    ll M_val = 2LL * Q0_val + 1; /* = 8N + 3 */

    for (int p = 3; p <= sieve_limit; p += 2) {
        if (!is_prime[p]) continue;

        ll m_mod_p = M_val % p;
        /* Need (2k+1)^2 ≡ m_mod_p (mod p) */
        /* Check if m_mod_p is a QR mod p */
        /* Use Euler criterion: m^((p-1)/2) mod p == 1 */
        ll euler = 1;
        {
            ll base = m_mod_p % p;
            ll exp = (p - 1) / 2;
            ll r = 1;
            while (exp > 0) {
                if (exp & 1) r = r * base % p;
                base = base * base % p;
                exp >>= 1;
            }
            euler = r;
        }

        if (euler != 1 && m_mod_p != 0) continue;

        /* Find sqrt of m_mod_p mod p */
        ll sq;
        if (m_mod_p == 0) {
            sq = 0;
        } else if (p % 4 == 3) {
            ll base = m_mod_p;
            ll exp = (p + 1) / 4;
            ll r = 1;
            while (exp > 0) {
                if (exp & 1) r = r * base % p;
                base = base * base % p;
                exp >>= 1;
            }
            sq = r;
        } else {
            /* Tonelli-Shanks */
            ll Q = p - 1;
            int S = 0;
            while (Q % 2 == 0) { Q /= 2; S++; }
            ll z = 2;
            while (1) {
                ll base = z % p, exp2 = (p-1)/2, r2 = 1;
                while (exp2 > 0) { if(exp2&1) r2=r2*base%p; base=base*base%p; exp2>>=1; }
                if (r2 != 1) break;
                z++;
            }
            ll MM = S;
            ll c = 1; { ll base=z%p, exp2=Q; while(exp2>0){if(exp2&1)c=c*base%p;base=base*base%p;exp2>>=1;} }
            ll t = 1; { ll base=m_mod_p%p, exp2=Q; while(exp2>0){if(exp2&1)t=t*base%p;base=base*base%p;exp2>>=1;} }
            ll R = 1; { ll base=m_mod_p%p, exp2=(Q+1)/2; while(exp2>0){if(exp2&1)R=R*base%p;base=base*base%p;exp2>>=1;} }
            while (t != 1) {
                ll tt = t;
                int ii = 0;
                while (tt != 1) { tt = tt * tt % p; ii++; }
                ll b2 = c;
                for (int jj = 0; jj < MM - ii - 1; jj++) b2 = b2 * b2 % p;
                MM = ii;
                c = b2 * b2 % p;
                t = t * c % p;
                R = R * b2 % p;
            }
            sq = R;
        }

        /* 2k+1 ≡ ±sq (mod p) -> k ≡ (±sq - 1) / 2 (mod p) */
        /* inv2 = (p+1)/2 */
        ll inv2 = (p + 1) / 2;
        ll roots[2];
        int nroots;
        if (m_mod_p == 0) {
            roots[0] = (p - 1) * inv2 % p; /* (0-1)/2 mod p = (p-1)*inv2 mod p */
            nroots = 1;
        } else {
            roots[0] = (sq - 1 + p) % p * inv2 % p;
            roots[1] = (p - sq - 1 + p) % p * inv2 % p;
            nroots = (roots[0] == roots[1]) ? 1 : 2;
        }

        int p_mod4 = p % 4;

        for (int ri = 0; ri < nroots; ri++) {
            ll k_start = roots[ri];
            for (ll k = k_start; k < sz; k += p) {
                /* Divide out p from remaining[k] */
                int e = 0;
                while (remaining[k] % p == 0) {
                    remaining[k] /= p;
                    e++;
                }
                if (e > 0) {
                    if (p_mod4 == 1) {
                        result[k] *= (e + 1);
                    } else { /* p ≡ 3 mod 4 */
                        if (e % 2 == 1) {
                            result[k] = 0;
                        }
                    }
                }
            }
        }
    }

    /* After sieving, remaining[k] is either 1 or a single large prime factor */
    /* (or possibly a product of large primes, but since we sieved up to sqrt(Q0), */
    /* remaining[k] can have at most one prime factor > sqrt(Q0)) */
    ll answer = 0;
    for (int k = 0; k < sz; k++) {
        if (result[k] == 0) continue;
        ll r = remaining[k];
        if (r == 1) {
            answer += result[k];
        } else {
            /* r is a prime > sieve_limit */
            /* r ≡ 1 mod 4: multiply result by 2 */
            /* r ≡ 3 mod 4: result becomes 0 (odd exponent 1 for a 3mod4 prime) */
            if (r % 4 == 1) {
                answer += result[k] * 2;
            }
            /* r % 4 == 3: contributes 0 */
        }
    }

    printf("%lld\n", answer);

    free(remaining);
    free(result);
    free(is_prime);
    return 0;
}
"""

    input_data = f"{N} {Q0} {L}\n"

    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "sol.c")
        exe = os.path.join(tmpdir, "sol")
        with open(src, "w") as f:
            f.write(c_code)
        r = subprocess.run(["gcc", "-O2", "-lm", "-o", exe, src], capture_output=True, text=True)
        if r.returncode != 0:
            import sys; sys.stderr.write("Compile error: " + r.stderr + "\n"); return -1
        result = subprocess.run([exe], input=input_data, capture_output=True, text=True, timeout=28)
        import sys
        sys.stderr.write(result.stderr)
        if result.returncode != 0:
            sys.stderr.write("Runtime error: " + result.stderr + "\n"); return -1
        return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
