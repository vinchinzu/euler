"""Project Euler Problem 952: Order Modulo Factorial.

R(p, n) = multiplicative order of p mod n!.
Find R(10^9+7, 10^7) mod (10^9+7).
Embedded C for speed.
"""
import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned __int128 u128;

#define MAXN 10000001

static int spf[MAXN];
static int exponents[MAXN];

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (u128)result * base % mod;
        base = (u128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

void compute_spf(void) {
    for (int i = 0; i < MAXN; i++) spf[i] = i;
    spf[0] = spf[1] = 0;
    for (int i = 2; (ll)i * i < MAXN; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j < MAXN; j += i)
                if (spf[j] == j) spf[j] = i;
        }
    }
}

int get_ord(ll p, int q) {
    if (q == 2) return 1;
    int t = q - 1;
    int curr_ord = t;
    int temp = t;
    while (temp > 1) {
        int f = spf[temp];
        while (curr_ord % f == 0 && mod_pow(p, curr_ord / f, q) == 1)
            curr_ord /= f;
        while (temp % f == 0) temp /= f;
    }
    return curr_ord;
}

int get_legendre(int n, int p) {
    int count = 0;
    ll pp = p;
    while (pp <= n) { count += n / (int)pp; pp *= p; }
    return count;
}

int main(void) {
    int n = 10000000;
    ll p = 1000000007LL;
    ll mod_ans = 1000000007LL;

    compute_spf();
    memset(exponents, 0, sizeof(exponents));

    /* Handle q = 2 */
    int K2 = get_legendre(n, 2);
    if (K2 >= 4) exponents[2] = K2 - 3;
    else if (K2 >= 2) exponents[2] = 1;

    /* Iterate odd primes */
    for (int q = 3; q <= n; q++) {
        if (spf[q] != q) continue;

        int d_q = get_ord(p, q);

        /* Update exponents from d_q's factorization */
        int temp = d_q;
        while (temp > 1) {
            int ell = spf[temp];
            int cnt = 0;
            while (temp % ell == 0) { cnt++; temp /= ell; }
            if (exponents[ell] < cnt) exponents[ell] = cnt;
        }

        /* Compute v_q = q-adic val of p^{d_q} - 1 */
        int K_q = get_legendre(n, q);
        if (K_q == 0) continue;

        int v_q = 1;
        ll curr_mod = (ll)q * q;
        ll rem = mod_pow(p, d_q, curr_mod);
        while (rem == 1) {
            v_q++;
            if (v_q >= K_q) break;
            if (curr_mod > 2000000000000000000LL / q) break; /* overflow guard */
            curr_mod *= q;
            rem = mod_pow(p, d_q, curr_mod);
        }

        int A_q = K_q - v_q;
        if (A_q < 0) A_q = 0;
        if (exponents[q] < A_q) exponents[q] = A_q;
    }

    /* Final answer */
    ll ans = 1;
    for (int ell = 2; ell <= n; ell++) {
        if (exponents[ell] > 0) {
            ll term = mod_pow(ell, exponents[ell], mod_ans);
            ans = (u128)ans * term % mod_ans;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name
    exe_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', exe_path, c_path],
                       check=True, capture_output=True)
        result = subprocess.run([exe_path], capture_output=True, text=True, timeout=280)
        return result.stdout.strip()
    finally:
        for p in [c_path, exe_path]:
            if os.path.exists(p):
                os.unlink(p)

if __name__ == "__main__":
    print(solve())
