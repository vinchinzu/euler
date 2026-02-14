"""Project Euler Problem 319 - Bounded Sequences.

t(n) = sum_{k=1}^n (3^k - 2^k) - sum_{k=2}^n t(floor(n/k))

Use sqrt decomposition for O(n^{3/4}).
Embedded C for speed.
"""
import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (unsigned __int128)result * base % mod;
        base = (unsigned __int128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main() {
    ll N = 10000000000LL;
    ll M = 1000000000LL;
    ll M2 = 2 * M;

    /* Enumerate distinct floor(N/k) values */
    int cap = 300000;
    ll *values = (ll *)malloc(cap * sizeof(ll));
    int nv = 0;
    ll k = 1;
    while (k <= N) {
        ll v = N / k;
        values[nv++] = v;
        k = N / v + 1;
    }

    /* Reverse to ascending order */
    for (int i = 0; i < nv / 2; i++) {
        ll tmp = values[i];
        values[i] = values[nv - 1 - i];
        values[nv - 1 - i] = tmp;
    }

    ll sqN = (ll)sqrtl((long double)N);
    while ((sqN + 1) * (sqN + 1) <= N) sqN++;
    while (sqN * sqN > N) sqN--;

    ll *t_arr = (ll *)calloc(nv, sizeof(ll));

    for (int idx = 0; idx < nv; idx++) {
        ll n = values[idx];
        ll l = (ll)sqrtl((long double)n);
        while ((l + 1) * (l + 1) <= n) l++;
        while (l * l > n) l--;

        /* S(n) = (3^{n+1} - 1)/2 - (2^{n+1} - 1) mod M */
        ll p3 = mod_pow(3, n + 1, M2);
        ll val3 = (p3 - 1) / 2;  /* 3^k is odd, so p3-1 is even */
        ll val2 = (mod_pow(2, n + 1, M) - 1 + M) % M;
        ll result = (val3 - val2 + M) % M;

        /* Subtract sum_{k=2}^{l} t(floor(n/k)) */
        for (ll kk = 2; kk <= l; kk++) {
            ll nk = n / kk;
            int idx2;
            if (nk <= sqN)
                idx2 = (int)(nk - 1);
            else
                idx2 = nv - (int)(N / nk);
            result = (result - t_arr[idx2] + M) % M;
        }

        /* Subtract sum_{q=1}^{n/l - 1} (n/q - n/(q+1)) * t(q) */
        ll upper = (l > 0) ? n / l : 1;
        for (ll q = 1; q < upper; q++) {
            ll count = n / q - n / (q + 1);
            ll sub = (count % M) * t_arr[(int)(q - 1)] % M;
            result = (result - sub + M) % M;
        }

        t_arr[idx] = result;
    }

    printf("%lld\n", t_arr[nv - 1]);

    free(values);
    free(t_arr);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name
    exe_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', exe_path, c_path, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([exe_path], capture_output=True, text=True, timeout=280)
        return result.stdout.strip()
    finally:
        for p in [c_path, exe_path]:
            if os.path.exists(p):
                os.unlink(p)

if __name__ == "__main__":
    print(solve())
