/* Project Euler Problem 915 - Recursive Modulo Sequence
 * Uses summatory Euler totient (Lucy DP) and periodic sequence H.
 * sum_{g=1}^N H(g) * (2*S_Phi(floor(N/g)) - 1) mod 123456789
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

#define MOD 123456789LL
#define PRECOMPUTE_LIMIT 1000000

static int phi_arr[PRECOMPUTE_LIMIT + 1];
static ll s_phi[PRECOMPUTE_LIMIT + 1];

void compute_s_phi_table(void) {
    for (int i = 0; i <= PRECOMPUTE_LIMIT; i++) phi_arr[i] = i;
    for (int i = 2; i <= PRECOMPUTE_LIMIT; i++) {
        if (phi_arr[i] == i) { /* prime */
            for (int j = i; j <= PRECOMPUTE_LIMIT; j += i)
                phi_arr[j] -= phi_arr[j] / i;
        }
    }
    ll current = 0;
    s_phi[0] = 0;
    for (int i = 1; i <= PRECOMPUTE_LIMIT; i++) {
        current = (current + phi_arr[i]) % MOD;
        s_phi[i] = current;
    }
}

/* Hash table for memoized S_Phi values */
#define HTSIZE (1 << 18)
#define HTMASK (HTSIZE - 1)
typedef struct HEntry { ll key; ll val; struct HEntry *next; } HEntry;
static HEntry *htable[HTSIZE];
#define HPOOL_SIZE (1 << 18)
static HEntry hpool[HPOOL_SIZE];
static int hpool_idx = 0;

static HEntry *halloc(void) { return &hpool[hpool_idx++]; }

static ll *hlookup(ll key) {
    unsigned int h = (unsigned int)((ull)key * 2654435761ULL) & HTMASK;
    HEntry *e = htable[h];
    while (e) { if (e->key == key) return &e->val; e = e->next; }
    return NULL;
}

static void hinsert(ll key, ll val) {
    unsigned int h = (unsigned int)((ull)key * 2654435761ULL) & HTMASK;
    HEntry *e = halloc();
    e->key = key; e->val = val; e->next = htable[h]; htable[h] = e;
}

typedef unsigned long long ull;

ll INV2;

ll S_Phi(ll n) {
    if (n <= PRECOMPUTE_LIMIT) return s_phi[n];
    ll *p = hlookup(n);
    if (p) return *p;

    ll nm = n % MOD;
    ll term1 = nm * ((n + 1) % MOD) % MOD * INV2 % MOD;

    ll sub_sum = 0;
    ll l = 2;
    while (l <= n) {
        ll val = n / l;
        ll r = (val == 0) ? n : n / val;
        ll count = (r - l + 1) % MOD;
        ll term = count * S_Phi(val) % MOD;
        sub_sum = (sub_sum + term) % MOD;
        l = r + 1;
    }

    ll res = (term1 - sub_sum % MOD + MOD) % MOD;
    hinsert(n, res);
    return res;
}

/* H(g) = s(s(g)) where s is the sequence x -> ((x-1)^3 + 2) */
/* s mod MOD has preperiod 53, period 33705 */
/* s mod 33705 has preperiod 2, period 420 */

static ll S_mod_M[53 + 33705 + 200];
static ll S_mod_P1[2 + 420 + 200];
static ll H_vals[1001]; /* 1-based */
static ll H_prefix[1001]; /* 1-based prefix sums */

ll s_step_mod(ll x, ll m) {
    ll xm1 = ((x - 1) % m + m) % m;
    ll res = ((lll)xm1 * xm1 % m * xm1 % m + 2) % m;
    return res;
}

void compute_H_period(void) {
    int limit_M = 53 + 33705 + 100;
    S_mod_M[1] = 1;
    for (int i = 2; i <= limit_M; i++)
        S_mod_M[i] = s_step_mod(S_mod_M[i - 1], MOD);

    int limit_P1 = 2 + 420 + 100;
    S_mod_P1[1] = 1;
    for (int i = 2; i <= limit_P1; i++)
        S_mod_P1[i] = s_step_mod(S_mod_P1[i - 1], 33705);

    for (int g = 1; g <= 1000; g++) {
        if (g <= 4) {
            ll sg = S_mod_M[g];
            H_vals[g] = S_mod_M[sg];
        } else {
            int eff_g = 3 + ((g - 3) % 420);
            if (eff_g < 1) eff_g += 420;
            ll s_g_mod_P1 = S_mod_P1[eff_g];
            ll K = s_g_mod_P1;
            while (K <= 53) K += 33705;
            H_vals[g] = S_mod_M[K];
        }
    }

    ll curr = 0;
    H_prefix[0] = 0;
    for (int i = 1; i <= 1000; i++) {
        curr = (curr + H_vals[i]) % MOD;
        H_prefix[i] = curr;
    }
}

ll get_sum_H(ll n) {
    if (n <= 0) return 0;
    if (n <= 1000) return H_prefix[n];

    ll sum_pre = H_prefix[4];
    ll count = n - 4;
    ll P = 420;
    ll num_full = count / P;
    ll rem = count % P;

    ll sum_period = (H_prefix[4 + P] - H_prefix[4] + MOD) % MOD;

    ll total = sum_pre;
    total = (total + (num_full % MOD) * sum_period) % MOD;
    ll term_rem = (H_prefix[4 + rem] - H_prefix[4] + MOD) % MOD;
    total = (total + term_rem) % MOD;
    return total;
}

int main(void) {
    ll N = 100000000LL; /* 10^8 */

    INV2 = (MOD + 1) / 2;

    compute_s_phi_table();
    memset(htable, 0, sizeof(htable));
    compute_H_period();

    ll total_sum = 0;
    ll l = 1;
    while (l <= N) {
        ll val = N / l;
        ll r = (val == 0) ? N : N / val;

        ll sum_H_range = (get_sum_H(r) - get_sum_H(l - 1) + MOD) % MOD;
        ll phi_val = S_Phi(val);
        ll weight = (2 * phi_val - 1 + MOD) % MOD;
        ll term = sum_H_range * weight % MOD;
        total_sum = (total_sum + term) % MOD;

        l = r + 1;
    }

    printf("%lld\n", total_sum);
    return 0;
}
