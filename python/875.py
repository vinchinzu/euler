"""Project Euler Problem 875 - Quadruple Congruence. Embedded C port for speed."""

import subprocess
import tempfile
import os

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
#define M 1001961001LL

ll power(ll a, ll b, ll m) {
    ll res = 1;
    a %= m;
    if (a < 0) a += m;
    while (b > 0) {
        if (b & 1) res = res * a % m;
        a = a * a % m;
        b >>= 1;
    }
    return res;
}

int main(void) {
    int N = 12345678;

    /* Allocate arrays: spf, pe, q as int arrays, q values as ll */
    int *spf = (int *)calloc((size_t)(N + 1), sizeof(int));
    int *pe  = (int *)calloc((size_t)(N + 1), sizeof(int));
    ll  *q   = (ll *)calloc((size_t)(N + 1), sizeof(ll));

    /* Primes list - upper bound by prime counting function */
    int *primes = (int *)malloc((size_t)(N / 2) * sizeof(int));
    int num_primes = 0;

    if (!spf || !pe || !q || !primes) {
        fprintf(stderr, "malloc failed\n");
        return 1;
    }

    q[1] = 1;

    for (int i = 2; i <= N; i++) {
        if (spf[i] == 0) {
            spf[i] = i;
            pe[i] = i;
            primes[num_primes++] = i;

            /* Compute q(p) for prime p = i */
            ll p = (ll)i;
            if (p == 2) {
                q[i] = 128;
            } else {
                ll p3 = power(p, 3, M);
                ll p7 = power(p, 7, M);
                ll term2 = (p - 1) % M * p3 % M;
                q[i] = (p7 + term2) % M;
            }
        }

        for (int j = 0; j < num_primes; j++) {
            int p = primes[j];
            if (p > spf[i] || (ll)i * p > N) break;

            int next_val = i * p;
            spf[next_val] = p;

            if (p == spf[i]) {
                /* p divides i */
                int prev_pk = pe[i];
                int next_pk = prev_pk * p;
                pe[next_val] = next_pk;

                ll q_next_pk;
                if (p == 2) {
                    /* q(2^{k+1}) = 128 * q(2^k) + 2^{4(k+1)+3}
                       = 128 * q(prev_pk) + prev_pk^4 * 128 */
                    ll term = power((ll)prev_pk, 4, M);
                    term = term * 128 % M;
                    ll val = 128 * q[prev_pk] % M;
                    q_next_pk = (val + term) % M;
                } else {
                    /* q(p^{k+1}) = p^7 * q(p^k) + (p-1) * p^{4(k+1)-1} */
                    ll pp = (ll)p;
                    ll p3 = power(pp, 3, M);
                    ll p7 = power(pp, 7, M);
                    ll term = power((ll)prev_pk, 4, M);
                    term = term * p3 % M;
                    term = term % M * ((pp - 1) % M) % M;
                    ll val = p7 * q[prev_pk] % M;
                    q_next_pk = (val + term) % M;
                }

                if (next_val == next_pk) {
                    q[next_val] = q_next_pk;
                } else {
                    int rest = next_val / next_pk;
                    q[next_val] = q_next_pk * q[rest] % M;
                }
            } else {
                /* p does not divide i */
                pe[next_val] = p;
                q[next_val] = q[i] * q[p] % M;
            }
        }
    }

    ll total = 0;
    for (int i = 1; i <= N; i++) {
        total = (total + q[i]) % M;
    }

    printf("%lld\n", total);

    free(spf);
    free(pe);
    free(q);
    free(primes);
    return 0;
}
"""

def main():
    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "p875.c")
        exe = os.path.join(tmpdir, "p875")
        with open(src, "w") as f:
            f.write(C_CODE)
        subprocess.run(["gcc", "-O2", "-o", exe, src], check=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())

if __name__ == "__main__":
    main()
