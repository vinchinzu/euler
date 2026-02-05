"""Project Euler Problem 715: Sextuplet Norms."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define M 1000000007LL

ll imod(ll a, ll m) {
    return ((a % m) + m) % m;
}

ll sum_cubes(ll n, ll mod) {
    // (n*(n+1)/2)^2 mod m
    ll s = n % mod * ((n + 1) % mod) % mod;
    // s is even, so s/2 is safe
    if (s % 2 == 0) s /= 2;
    else s = s * ((mod + 1) / 2) % mod;  // modular inverse of 2
    return s * s % mod;
}

int main() {
    ll N = 1000000000000LL;
    int L1 = (int)cbrt((double)N);
    int L2 = (int)(N / L1);

    // Sieve smallest prime factors
    int* ff = (int*)malloc((L2 + 1) * sizeof(int));
    for (int i = 0; i <= L2; i++) ff[i] = i;
    for (int i = 2; (ll)i * i <= L2; i++) {
        if (ff[i] == i) {
            for (int j = i * i; j <= L2; j += i) {
                if (ff[j] == j) ff[j] = i;
            }
        }
    }

    // Compute mu'
    ll* mu_prime = (ll*)calloc(L2 + 1, sizeof(ll));
    mu_prime[1] = 1;
    for (int i = 2; i <= L2; i++) {
        int d = ff[i];
        if (d != 2 && (i / d) % d != 0) {
            mu_prime[i] = mu_prime[i / d] * (d % 4 == 1 ? -1 : 1);
        }
    }

    // Compute small = cumulative sum of mu'
    ll* small = (ll*)calloc(L2 + 1, sizeof(ll));
    for (int i = 1; i <= L2; i++) {
        small[i] = (small[i - 1] + mu_prime[i]) % M;
    }

    ll isMod4Sq[4] = {0, 1, 0, -1};
    ll cumIsMod4Sq[4] = {0, 1, 1, 0};

    // Compute big
    ll* big = (ll*)calloc(L1 + 2, sizeof(ll));
    for (int i = L1; i >= 1; i--) {
        big[i] = 1;
        ll ni = N / i;
        int sqrtni = (int)sqrt((double)ni);
        // Adjust for floating point error
        while ((ll)(sqrtni + 1) * (sqrtni + 1) <= ni) sqrtni++;
        while ((ll)sqrtni * sqrtni > ni) sqrtni--;

        for (int k = 2; k < sqrtni; k++) {
            if ((ll)i * k < L1) {
                big[i] = (big[i] - isMod4Sq[k % 4] * big[i * k]) % M;
            } else {
                int idx = (int)(ni / k);
                big[i] = (big[i] - isMod4Sq[k % 4] * small[idx]) % M;
            }
        }

        int max_t = (int)(ni / sqrtni);
        for (int t = 1; t <= max_t; t++) {
            ll nit = ni / t;
            ll nit1 = ni / (t + 1);
            ll diff = (cumIsMod4Sq[imod(nit, 4)] - cumIsMod4Sq[imod(nit1, 4)]) % M;
            big[i] = (big[i] - diff * small[t]) % M;
        }
        big[i] = imod(big[i], M);
    }

    ll ans = 0;

    // Sum over i <= L2
    for (int i = 1; i <= L2; i++) {
        ll sc = sum_cubes(N / i, M);
        ans = (ans + sc * mu_prime[i]) % M;
    }

    // Sum over t < L1
    for (int t = 1; t < L1; t++) {
        ll sc = sum_cubes(t, M);
        ll diff = (big[t] - big[t + 1]) % M;
        ans = (ans + sc * diff) % M;
    }

    ans = imod(ans, M);
    printf("%lld\n", ans);

    free(ff);
    free(mu_prime);
    free(small);
    free(big);
    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name

    exe = c_file[:-2]
    try:
        subprocess.run(['gcc', '-O3', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
        result = subprocess.check_output([exe], timeout=300).decode().strip()
        return int(result)
    finally:
        if os.path.exists(c_file):
            os.unlink(c_file)
        if os.path.exists(exe):
            os.unlink(exe)


if __name__ == "__main__":
    print(solve())
