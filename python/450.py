"""Project Euler Problem 450: Hypocycloid lattice points.

Rewritten in C for performance.
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

static ll gcd_ll(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

static int isqrt_int(ll n) {
    ll r = (ll)sqrt((double)n);
    while (r*r > n) r--;
    while ((r+1)*(r+1) <= n) r++;
    return (int)r;
}

int main(void) {
    const int N = 1000000;

    /* Precompute Euler totient phi */
    int *phi = (int*)malloc((2*N+1) * sizeof(int));
    for (int i = 0; i <= 2*N; i++) phi[i] = i;
    for (int i = 2; i <= 2*N; i++) {
        if (phi[i] == i) { /* i is prime */
            for (int j = i; j <= 2*N; j += i)
                phi[j] = phi[j] / i * (i - 1);
        }
    }

    /* Precompute Mobius function */
    int *mu = (int*)malloc((2*N+1) * sizeof(int));
    char *is_prime = (char*)calloc(2*N+1, 1);
    for (int i = 0; i <= 2*N; i++) { mu[i] = 1; is_prime[i] = 1; }
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; i <= 2*N; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= 2*N; j += i) {
                if (j != i) is_prime[j] = 0;
                if ((ll)j % ((ll)i*i) == 0)
                    mu[j] = 0;
                else
                    mu[j] = -mu[j];
            }
        }
    }
    free(is_prime);

    ll ans = 0;

    /* Cardinal angles case */
    for (int S = 3; S <= N; S++) {
        ll tr_NS = (ll)(N/S) * ((ll)(N/S) + 1) / 2;
        ans += 2LL * phi[S] * tr_NS * S;

        if (S % 4 != 0) {
            ll res = 0;
            int sq = isqrt_int(S);
            for (int d = 1; d <= sq; d++) {
                if (S % d == 0) {
                    ll tr_val = (ll)((S-1)/2/d) * ((ll)((S-1)/2/d) + 1) / 2;
                    res += (ll)d * tr_val * mu[d];
                    if (d != S/d) {
                        int d2 = S/d;
                        ll tr_val2 = (ll)((S-1)/2/d2) * ((ll)((S-1)/2/d2) + 1) / 2;
                        res += (ll)d2 * tr_val2 * mu[d2];
                    }
                }
            }
            ans -= 2LL * (S % 2 == 0 ? 2 : 1) * tr_NS * res;
        }
    }

    /* Non-cardinal angles case - Pythagorean triples */
    /* a = m^2 - n^2, b = 2mn, c = m^2 + n^2 where m>n>0, gcd(m,n)=1, m+n odd */
    for (int m = 2; m < 1000; m++) {
        for (int nn = 1; nn < m; nn++) {
            if ((m + nn) % 2 == 0) continue;
            if (gcd_ll(m, nn) != 1) continue;

            ll tr_a = (ll)m*m - (ll)nn*nn;
            ll tr_b = 2LL*m*nn;
            ll cc = (ll)m*m + (ll)nn*nn;  /* c of the Pythagorean triple */

            /* For each (sin_val, cos_val) pair from the triple */
            ll ox_vals[2] = {tr_a, tr_b};
            ll oy_vals[2] = {tr_b, tr_a};

            for (int oi = 0; oi < 2; oi++) {
                ll order_x = ox_vals[oi];
                ll order_y = oy_vals[oi];

                for (int sx = 0; sx < 2; sx++) {
                    ll sin_val = sx == 0 ? -order_x : order_x;
                    for (int cx = 0; cx < 2; cx++) {
                        ll cos_val = cx == 0 ? -order_y : order_y;

                        for (int n = 2; n < 100; n++) {
                            /* common = c^(max(n-1,2)) */
                            ll common = 1;
                            int exp = n-1 > 2 ? n-1 : 2;
                            int overflow = 0;
                            for (int e = 0; e < exp; e++) {
                                if (common > N / cc + 1) { overflow = 1; break; }
                                common *= cc;
                            }
                            if (overflow || common > N) break;

                            for (int mp = 1; mp < n; mp++) {
                                if (gcd_ll(mp, n) != 1) continue;

                                for (ll k = 1; ; k++) {
                                    ll r_val = k * mp * common;
                                    ll R_val = k * (mp + n) * common;
                                    if (R_val > N) break;

                                    /* Compute Chebyshev polynomials T and U */
                                    /* T[i+1] = 2*cos_val*T[i] - c^2*T[i-1] */
                                    /* T[0] = 1, T[1] = cos_val */
                                    ll cc2 = cc * cc;
                                    ll *T = (ll*)malloc((n+1)*sizeof(ll));
                                    T[0] = 1;
                                    T[1] = cos_val;
                                    for (int i = 2; i <= n; i++)
                                        T[i] = 2 * cos_val * T[i-1] - cc2 * T[i-2];

                                    ll *U = (ll*)malloc(n*sizeof(ll));
                                    U[0] = 1;
                                    if (n > 1) U[1] = 2 * cos_val;
                                    for (int i = 2; i < n; i++)
                                        U[i] = 2 * cos_val * U[i-1] - cc2 * U[i-2];

                                    /* c_power_n_minus_m = c^(n-mp) */
                                    ll cpnm = 1;
                                    for (int e = 0; e < n-mp; e++) cpnm *= cc;

                                    ll x_cn = (R_val - r_val) * cpnm * T[mp] + r_val * T[n];
                                    ll y_cn = (R_val - r_val) * cpnm * sin_val * U[mp-1]
                                             - r_val * sin_val * U[n-1];

                                    /* c_power_n = c^n */
                                    ll cpn = 1;
                                    for (int e = 0; e < n; e++) cpn *= cc;

                                    if (cpn != 0 && x_cn % cpn == 0 && y_cn % cpn == 0) {
                                        ll xv = x_cn / cpn;
                                        ll yv = y_cn / cpn;
                                        ans += (xv < 0 ? -xv : xv) + (yv < 0 ? -yv : yv);
                                    }

                                    free(T);
                                    free(U);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    printf("%lld\n", ans);
    free(phi);
    free(mu);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as src:
        src.write(c_code)
        src_path = src.name
    bin_path = src_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, src_path, '-lm'], check=True,
                       capture_output=True, text=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True, timeout=30)
        print(result.stdout.strip())
    except subprocess.CalledProcessError as e:
        print(f"Error: {e.stderr}", flush=True)
        raise
    finally:
        os.unlink(src_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
