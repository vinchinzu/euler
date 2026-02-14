/*
 * Project Euler Problem 465: Polar polygons (visible boundaries)
 *
 * Uses Euler totient summatory function (Lucy DP) and modular arithmetic.
 * N = 7^13, M = 10^9 + 7.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

static ll pow_mod(ll base, ll exp, ll m) {
    ll result = 1;
    base = ((base % m) + m) % m;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % m;
        base = (lll)base * base % m;
        exp >>= 1;
    }
    return result;
}

static int *phi_arr;
static ll *prefix_phi;
static int sieve_lim;

static void sieve_phi(int limit) {
    sieve_lim = limit;
    phi_arr = (int *)malloc((limit + 1) * sizeof(int));
    for (int i = 0; i <= limit; i++) phi_arr[i] = i;
    for (int i = 2; i <= limit; i++)
        if (phi_arr[i] == i)
            for (int j = i; j <= limit; j += i)
                phi_arr[j] = phi_arr[j] / i * (i - 1);
}

typedef struct {
    ll *small;
    ll *large;
    ll N;
    int L;
} SumPhi;

static SumPhi compute_sum_phi(ll N, ll mod_val) {
    SumPhi sp;
    sp.N = N;
    sp.L = (int)sqrt((double)N);
    while ((ll)sp.L * sp.L > N) sp.L--;
    while ((ll)(sp.L + 1) * (sp.L + 1) <= N) sp.L++;

    int L = sp.L;
    sp.small = (ll *)calloc(L + 2, sizeof(ll));
    sp.large = (ll *)calloc(L + 2, sizeof(ll));

    for (int v = 0; v <= L && v <= sieve_lim; v++)
        sp.small[v] = prefix_phi[v] % mod_val;

    for (int x = L; x >= 1; x--) {
        ll n = N / x;
        if (n <= L) {
            sp.large[x] = sp.small[(int)n];
            continue;
        }
        if (n <= sieve_lim) {
            sp.large[x] = prefix_phi[(int)n] % mod_val;
            continue;
        }

        ll half_n, other;
        if (n % 2 == 0) { half_n = n / 2; other = n + 1; }
        else { half_n = (n + 1) / 2; other = n; }
        ll val = (half_n % mod_val) * (other % mod_val) % mod_val;

        ll d = 2;
        while (d <= n) {
            ll q = n / d;
            ll d_max = n / q;

            ll s;
            if (q <= L)
                s = sp.small[(int)q];
            else {
                ll xd = (ll)x * d;
                if (xd <= L)
                    s = sp.large[(int)xd];
                else if (q <= sieve_lim)
                    s = prefix_phi[(int)q] % mod_val;
                else
                    s = sp.small[(int)(N / xd)];
            }

            ll count = (d_max - d + 1) % mod_val;
            val = ((val - count * s) % mod_val + mod_val) % mod_val;

            d = d_max + 1;
        }

        sp.large[x] = val;
    }

    return sp;
}

static ll get_sp(SumPhi *sp, ll v) {
    if (v <= sp->L) return sp->small[(int)v];
    return sp->large[(int)(sp->N / v)];
}

int main(void) {
    ll N = 1;
    for (int i = 0; i < 13; i++) N *= 7; /* 7^13 = 96889010407 */
    ll M = 1000000007LL;

    int L = (int)sqrt((double)N);
    while ((ll)L * L > N) L--;
    while ((ll)(L + 1) * (L + 1) <= N) L++;

    double cbrt_N = cbrt((double)N);
    int slimit = (int)(cbrt_N * cbrt_N) + 100;
    int ndl = (int)(N / L + 2);
    if (slimit < ndl) slimit = ndl;
    if (slimit < L + 2) slimit = L + 2;

    sieve_phi(slimit);
    prefix_phi = (ll *)calloc(slimit + 1, sizeof(ll));
    for (int i = 1; i <= slimit; i++)
        prefix_phi[i] = prefix_phi[i-1] + phi_arr[i];

    ll M1 = M - 1;
    SumPhi sp1 = compute_sum_phi(N, M1);
    SumPhi sp2 = compute_sum_phi(N, M);

    ll T = 1;
    int NdivL = (int)(N / L);
    for (int x = 1; x <= NdivL; x++) {
        T = (lll)T * pow_mod(N / x + 1, phi_arr[x], M) % M;
    }
    for (int q = 1; q < L; q++) {
        ll spq = get_sp(&sp1, N / q);
        ll spq1 = get_sp(&sp1, N / (q + 1));
        ll diff = ((spq - spq1) % M1 + M1) % M1;
        T = (lll)T * pow_mod(q + 1, diff, M) % M;
    }

    ll sq_2N1 = (2 * (N % M) + 1) % M;
    sq_2N1 = (lll)sq_2N1 * sq_2N1 % M;

    ll T8 = pow_mod(T, 8, M);
    ll T4 = pow_mod(T, 4, M);

    ll ans = (T8 - 1 + M) % M;
    ll sub = (ll)((lll)(sq_2N1 - 1 + M) % M * T4 % M);
    ans = (ans - sub + M) % M;

    for (int x = 1; x <= NdivL; x++) {
        ll sq = (N / x) % M;
        sq = (lll)sq * sq % M;
        ll term = (lll)4 * (phi_arr[x] % M) % M * sq % M;
        ans = (ans + term) % M;
    }
    for (int q = 1; q < L; q++) {
        ll spq = get_sp(&sp2, N / q);
        ll spq1 = get_sp(&sp2, N / (q + 1));
        ll diff = ((spq - spq1) % M + M) % M;
        ll sq = (ll)((lll)q * q % M);
        ll term = (ll)((lll)4 * diff % M * sq % M);
        ans = (ans + term) % M;
    }

    ans = (ans % M + M) % M;

    free(phi_arr);
    free(prefix_phi);
    free(sp1.small); free(sp1.large);
    free(sp2.small); free(sp2.large);

    printf("%lld\n", ans);
    return 0;
}
