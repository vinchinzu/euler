"""Project Euler Problem 650: Divisors of Binomial Product.

B(n) = product_{k=0}^{n} C(n,k). D(n) = sigma(B(n)) = sum of divisors of B(n).
Find S(N) = sum_{n=1}^{N} D(n) mod 10^9+7 for N=20000.

B(n) = (n!)^{n+1} / (product_{k=0}^n k!)^2.
Exponent of p in B(n): (n+1)*v_p(n!) - 2*sum_{k=0}^n v_p(k!).
Incremental: exp_p(B(n)) - exp_p(B(n-1)) = (n-1)*v_p(n) - v_p((n-1)!).
"""

import subprocess, tempfile, os

def solve():
    N = 20000
    MOD = 10**9 + 7

    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
#define MAXN 20001

ll power(ll base, ll exp, ll mod) {
    ll r = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) r = r * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return r;
}

int spf[MAXN]; /* smallest prime factor */
int primes[MAXN];
int nprimes;
int pidx[MAXN]; /* prime index: pidx[p] = index in primes[] */

/* B_exp[i] = exponent of primes[i] in B(n) */
/* fact_exp[i] = exponent of primes[i] in n! */
int B_exp[MAXN];
int fact_exp[MAXN];

int main() {
    int N;
    ll MOD;
    scanf("%d %lld", &N, &MOD);

    /* Sieve SPF */
    for (int i = 2; i <= N; i++) spf[i] = 0;
    nprimes = 0;
    for (int i = 2; i <= N; i++) {
        if (spf[i] == 0) {
            spf[i] = i;
            primes[nprimes] = i;
            pidx[i] = nprimes;
            nprimes++;
        }
        for (int j = 0; j < nprimes && primes[j] <= spf[i] && (ll)i * primes[j] <= N; j++) {
            spf[i * primes[j]] = primes[j];
        }
    }

    /* Precompute modular inverses of (p-1) for each prime p */
    ll *inv_pm1 = (ll *)malloc(nprimes * sizeof(ll));
    for (int i = 0; i < nprimes; i++) {
        inv_pm1[i] = power(primes[i] - 1, MOD - 2, MOD);
    }

    memset(B_exp, 0, sizeof(B_exp));
    memset(fact_exp, 0, sizeof(fact_exp));

    /* D_product[i] = contribution of primes[i] to D(n) = (p^(e+1)-1)/(p-1) */
    /* We maintain this incrementally */
    ll *D_factor = (ll *)malloc(nprimes * sizeof(ll));
    for (int i = 0; i < nprimes; i++) D_factor[i] = 1; /* p^0 => (p-1)/(p-1)=1 */

    ll answer = 0;

    for (int n = 1; n <= N; n++) {
        /* Update fact_exp: v_p((n-1)!) is fact_exp before update,
           then fact_exp becomes v_p(n!) = v_p((n-1)!) + v_p(n) */

        /* Step 1: update B_exp by subtracting v_p((n-1)!) for all primes */
        /* But fact_exp currently IS v_p((n-1)!) */
        /* B_exp[i] -= fact_exp[i] for all i with fact_exp[i] > 0 */
        for (int i = 0; i < nprimes && primes[i] < n; i++) {
            if (fact_exp[i] > 0) {
                B_exp[i] -= fact_exp[i];
                /* Update D_factor for this prime */
                if (B_exp[i] > 0) {
                    D_factor[i] = (power(primes[i], B_exp[i] + 1, MOD) - 1 + MOD) % MOD * inv_pm1[i] % MOD;
                } else {
                    D_factor[i] = 1;
                }
            }
        }

        /* Step 2: add (n-1)*v_p(n) for each prime p dividing n */
        {
            int m = n;
            while (m > 1) {
                int p = spf[m];
                int e = 0;
                while (m % p == 0) { m /= p; e++; }
                int pi = pidx[p];
                B_exp[pi] += (n - 1) * e;
                /* Also update fact_exp: v_p(n!) = v_p((n-1)!) + e */
                fact_exp[pi] += e;
                /* Update D_factor */
                if (B_exp[pi] > 0) {
                    D_factor[pi] = (power(p, B_exp[pi] + 1, MOD) - 1 + MOD) % MOD * inv_pm1[pi] % MOD;
                } else {
                    D_factor[pi] = 1;
                }
            }
        }

        /* Compute D(n) = product of D_factor[i] for all primes */
        ll D = 1;
        for (int i = 0; i < nprimes && primes[i] <= n; i++) {
            D = D * D_factor[i] % MOD;
        }

        answer = (answer + D) % MOD;
    }

    printf("%lld\n", answer);

    free(inv_pm1);
    free(D_factor);
    return 0;
}
"""

    input_data = f"{N} {MOD}\n"

    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "sol.c")
        exe = os.path.join(tmpdir, "sol")
        with open(src, "w") as f:
            f.write(c_code)
        r = subprocess.run(["gcc", "-O2", "-o", exe, src], capture_output=True, text=True)
        if r.returncode != 0:
            import sys; sys.stderr.write("Compile: " + r.stderr + "\n"); return -1
        result = subprocess.run([exe], input=input_data, capture_output=True, text=True, timeout=28)
        if result.returncode != 0:
            import sys; sys.stderr.write("Runtime: " + result.stderr + "\n"); return -1
        return int(result.stdout.strip())


if __name__ == "__main__":
    print(solve())
