#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N_VAL 100000000000LL  /* 10^11 */
#define MOD   100000000LL     /* 10^8  */

typedef long long ll;
typedef int       i32;

static i32 *phi_arr;   /* phi sieve, then reused as pref0 */
static i32 *pref1;
static i32 *pref2;

/* small[e][q] and large[e][g] for Lucy DP, e=0,1,2 */
static i32 *sm[3];
static i32 *lg[3];

static ll mod(ll x) {
    return ((x % MOD) + MOD) % MOD;
}

static ll powmod(ll base, ll exp, ll m) {
    ll r = 1;
    base %= m;
    if (base < 0) base += m;
    while (exp > 0) {
        if (exp & 1) r = r * base % m;
        base = base * base % m;
        exp >>= 1;
    }
    return r;
}

/* Sum_{i=1}^{n} i^k mod MOD */
static ll S_k(int k, ll n) {
    if (n <= 0) return 0;
    ll nm = mod(n);
    if (k == 0) return nm;
    if (k == 1) {
        /* n*(n+1)/2 mod M. Since one of n,n+1 is even, do division first. */
        if (n % 2 == 0)
            return mod((n/2) % MOD * (mod(n+1))) % MOD;
        else
            return mod(nm * (mod((n+1)/2))) % MOD;
    }
    if (k == 2) {
        /* n*(n+1)*(2n+1)/6 mod M */
        ll a = n, b = n + 1, c = 2*n + 1;
        if (a % 2 == 0) a /= 2; else b /= 2;
        if (a % 3 == 0) a /= 3;
        else if (b % 3 == 0) b /= 3;
        else c /= 3;
        return mod(mod(a) * mod(b) % MOD * mod(c)) % MOD;
    }
    if (k == 3) {
        ll t = S_k(1, n);
        return t * t % MOD;
    }
    return 0;
}

static ll sq(ll n) {
    ll v = mod(n);
    return v * v % MOD;
}

/* sum_{g=1}^{L} g^e * 2^g mod M */
static ll sum_ag(ll L, int e) {
    if (L <= 0) return 0;
    ll p2 = powmod(2, L + 1, MOD);
    if (e == 0) {
        return mod(p2 - 2);
    }
    if (e == 1) {
        return mod(mod(L - 1) * p2 % MOD + 2);
    }
    if (e == 2) {
        ll Lm = mod(L);
        ll t = mod(Lm * Lm % MOD - 2 * Lm % MOD + 3);
        return mod(t * p2 % MOD - 6);
    }
    return 0;
}

static ll isqrt_ll(ll n) {
    ll r = (ll)sqrt((double)n);
    while (r * r > n) r--;
    while ((r+1)*(r+1) <= n) r++;
    return r;
}

int main(void) {
    ll N = N_VAL;
    ll L = isqrt_ll(N);    /* ~316227 */

    /* U ~ N^{2/3}, at least L+1 */
    ll U = (ll)round(pow((double)N, 2.0/3.0));
    if (U < L + 1) U = L + 1;

    /* Allocate phi sieve as int32: U+1 elements */
    phi_arr = (i32 *)malloc((size_t)(U + 1) * sizeof(i32));
    if (!phi_arr) { fprintf(stderr, "alloc phi fail\n"); return 1; }

    /* Initialize phi[i] = i */
    for (ll i = 0; i <= U; i++) phi_arr[i] = (i32)i;

    /* Euler totient sieve */
    for (ll i = 2; i <= U; i++) {
        if (phi_arr[i] == (i32)i) {  /* i is prime */
            for (ll j = i; j <= U; j += i) {
                phi_arr[j] -= phi_arr[j] / (i32)i;
            }
        }
    }

    /* Allocate prefix arrays */
    pref1 = (i32 *)malloc((size_t)(U + 1) * sizeof(i32));
    pref2 = (i32 *)malloc((size_t)(U + 1) * sizeof(i32));
    if (!pref1 || !pref2) { fprintf(stderr, "alloc pref fail\n"); return 1; }

    /* Compute pref1[x] = sum_{i=1}^x i * phi(i) mod M */
    {
        ll running = 0;
        pref1[0] = 0;
        for (ll x = 1; x <= U; x++) {
            running = (running + (x % MOD) * (ll)phi_arr[x]) % MOD;
            pref1[x] = (i32)running;
        }
    }

    /* Compute pref2[x] = sum_{i=1}^x i^2 * phi(i) mod M */
    {
        ll running = 0;
        pref2[0] = 0;
        for (ll x = 1; x <= U; x++) {
            ll xm = x % MOD;
            running = (running + xm * xm % MOD * (ll)phi_arr[x]) % MOD;
            pref2[x] = (i32)running;
        }
    }

    /* Now compute pref0 in-place over phi_arr:
       pref0[x] = sum_{i=1}^x phi(i) mod M */
    {
        ll running = 0;
        phi_arr[0] = 0;
        for (ll x = 1; x <= U; x++) {
            running = (running + (ll)phi_arr[x]) % MOD;
            phi_arr[x] = (i32)running;
        }
    }
    /* Now phi_arr is pref0 */

    /* Allocate small and large arrays for Lucy DP */
    for (int e = 0; e < 3; e++) {
        sm[e] = (i32 *)calloc((size_t)(L + 2), sizeof(i32));
        lg[e] = (i32 *)calloc((size_t)(L + 2), sizeof(i32));
        if (!sm[e] || !lg[e]) { fprintf(stderr, "alloc sm/lg fail\n"); return 1; }
    }

    /* Lucy DP for each e = 0, 1, 2 */
    for (int e = 0; e < 3; e++) {
        i32 *pref;
        if (e == 0) pref = phi_arr;
        else if (e == 1) pref = pref1;
        else pref = pref2;

        /* Initialize small[q] from prefix array */
        for (ll q = 1; q <= L; q++) {
            sm[e][q] = pref[q];
        }

        /* Compute large[g] for g = L..1 */
        for (ll g = L; g >= 1; g--) {
            ll n = N / g;

            if (n <= U) {
                lg[e][g] = pref[n];
                continue;
            }

            ll result = S_k(e + 1, n);
            ll d = 2;
            while (d <= n) {
                ll q = n / d;
                ll d_max = n / q;
                ll coeff = mod(S_k(e, d_max) - S_k(e, d - 1));

                ll Te_q;
                if (q <= U) {
                    Te_q = (ll)pref[q];
                } else if (q <= L) {
                    Te_q = (ll)sm[e][q];
                } else {
                    Te_q = (ll)lg[e][N / q];
                }

                result = mod(result - coeff * (Te_q % MOD) % MOD);
                d = d_max + 1;
            }

            lg[e][g] = (i32)(result % MOD);
        }
    }

    /* Helper macros for sp_div and sp_get */
    /* sp_div(e, g): T_e(N/g) */
    #define sp_div(ee, gg) ({ \
        ll _v = N / (gg); \
        ll _r; \
        if (_v <= L) _r = (ll)sm[ee][_v]; \
        else _r = (ll)lg[ee][gg]; \
        _r; \
    })

    /* sp_get(e, q): T_e(q), looked up either in small or large */
    #define sp_get(ee, qq) ({ \
        ll _q = (qq); \
        ll _r; \
        if (_q <= L) _r = (ll)sm[ee][_q]; \
        else _r = (ll)lg[ee][N / _q]; \
        _r; \
    })

    /* Main formula */
    ll p2N1 = powmod(2, N + 1, MOD);  /* 2^(N+1) mod M */
    ll ans = powmod(p2N1, N + 1, MOD); /* (2^(N+1))^(N+1) mod M */
    ans = mod(ans - 1);
    ans = mod(ans - sq(N + 1));

    ll n1 = mod(N + 1);
    ll n1sq = sq(N + 1);

    ll term = mod(p2N1 - 1 - n1 - S_k(1, N));
    ans = mod(ans - 2 * n1 % MOD * term % MOD);

    /* First loop: g = 1..L */
    for (ll g = 1; g <= L; g++) {
        ll gm = mod(g);
        ll T = mod(
            gm * gm % MOD * mod(sp_div(2, g)) % MOD
            - 3 * n1 % MOD * gm % MOD * mod(sp_div(1, g)) % MOD
            + 2 * n1sq % MOD * mod(sp_div(0, g)) % MOD
            - mod(N + 1 - g) * n1 % MOD
        );
        ll p2g = powmod(2, g, MOD);
        ans = mod(ans - mod(p2g - 2) * T % MOD);
    }

    /* Second loop: q = 1..N/L - 1 */
    ll q_lim = N / L;  /* N/L, exclusive upper bound */
    for (ll q = 1; q < q_lim; q++) {
        ll t2 = mod(sp_get(2, q));
        ll t1 = mod(sp_get(1, q));
        ll t0 = mod(sp_get(0, q));

        ll nq  = N / q;
        ll nq1 = N / (q + 1);

        ll sag2 = mod(sum_ag(nq, 2) - sum_ag(nq1, 2));
        ll sag1 = mod(sum_ag(nq, 1) - sum_ag(nq1, 1));
        ll sag0 = mod(sum_ag(nq, 0) - sum_ag(nq1, 0));
        ll sp2  = mod(S_k(2, nq) - S_k(2, nq1));
        ll sp1  = mod(S_k(1, nq) - S_k(1, nq1));
        ll sp0  = mod(S_k(0, nq) - S_k(0, nq1));

        ll v3t1m1 = mod(3 * t1 - 1);
        ll v2t0m1 = mod(2 * t0 - 1);

        ans = mod(ans
               - t2 * sag2 % MOD
               + n1 * v3t1m1 % MOD * sag1 % MOD
               - n1sq * v2t0m1 % MOD * sag0 % MOD
               + 2 * t2 % MOD * sp2 % MOD
               - 2 * n1 % MOD * v3t1m1 % MOD * sp1 % MOD
               + 2 * n1sq % MOD * v2t0m1 % MOD * sp0 % MOD
        );
    }

    printf("%lld\n", mod(ans));

    /* Cleanup */
    free(phi_arr);
    free(pref1);
    free(pref2);
    for (int e = 0; e < 3; e++) {
        free(sm[e]);
        free(lg[e]);
    }

    return 0;
}
