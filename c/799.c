/*
 * Project Euler 799 - Pentagonal Number Representations
 *
 * Find smallest pentagonal number P_c = c*(3c-1)/2 that can be expressed as
 * P_a + P_b in over 100 different ways (a <= b).
 *
 * Key insight: P_a + P_b = P_c reduces to (6a-1)^2 + (6b-1)^2 = 36c^2 - 12c + 2.
 * So we need to count representations of m(c) = 18c^2 - 6c + 1 as x^2 + y^2 with
 * x, y > 0, x <= y, x = 5 mod 6, y = 5 mod 6.
 *
 * Two-pass sieve approach using Gaussian integer factorization.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

#define CLIMIT 28000000
#define PLIMIT 500000
#define THRESHOLD 1000

static unsigned short R_arr[CLIMIT];

#define MAX_CANDS 150000
#define MAX_FACTORS_PER 20

typedef struct {
    ll c;
    int nf;
    int pidx[MAX_FACTORS_PER];
    int pexp[MAX_FACTORS_PER];
    ll remaining;
} CandInfo;

static CandInfo cand_info[MAX_CANDS];
static int ncands;

static int primes1[50000];
static int nprimes1;
static int root1v[50000], root2v[50000];
static int gauss_a[50000], gauss_b[50000];
static char is_prime_sieve[PLIMIT+1];

void compute_primes() {
    memset(is_prime_sieve, 1, sizeof(is_prime_sieve));
    is_prime_sieve[0] = is_prime_sieve[1] = 0;
    for (int i = 2; (ll)i * i <= PLIMIT; i++)
        if (is_prime_sieve[i])
            for (int j = i*i; j <= PLIMIT; j += i)
                is_prime_sieve[j] = 0;
    nprimes1 = 0;
    for (int i = 5; i <= PLIMIT; i++)
        if (is_prime_sieve[i] && i % 4 == 1)
            primes1[nprimes1++] = i;
}

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod; if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

ll sqrt_mod_func(ll n, ll p) {
    n %= p; if (n < 0) n += p;
    if (n == 0) return 0;
    if (mod_pow(n, (p-1)/2, p) != 1) return -1;
    ll Q = p - 1; int S = 0;
    while (Q % 2 == 0) { Q /= 2; S++; }
    if (S == 1) return mod_pow(n, (p+1)/4, p);
    ll z = 2;
    while (mod_pow(z, (p-1)/2, p) != p-1) z++;
    int MM = S;
    ll c = mod_pow(z, Q, p);
    ll t = mod_pow(n, Q, p);
    ll R = mod_pow(n, (Q+1)/2, p);
    while (t != 1) {
        int i = 1;
        ll temp = (lll)t * t % p;
        while (temp != 1) { temp = (lll)temp * temp % p; i++; }
        ll b = mod_pow(c, 1LL << (MM-i-1), p);
        MM = i;
        c = (lll)b * b % p;
        t = (lll)t * c % p;
        R = (lll)R * b % p;
    }
    return R;
}

void compute_roots() {
    for (int idx = 0; idx < nprimes1; idx++) {
        ll p = primes1[idx];
        ll sq = sqrt_mod_func(p - 1, p);
        if (sq == -1) { root1v[idx] = root2v[idx] = -1; continue; }
        ll inv6 = mod_pow(6, p - 2, p);
        root1v[idx] = (int)((ll)((1 + sq) % p) * inv6 % p);
        root2v[idx] = (int)((ll)((1 - sq + p) % p) * inv6 % p);
    }
}

void decompose_prime(ll p, int *a_out, int *b_out) {
    ll r = sqrt_mod_func(p - 1, p);
    if (r > p/2) r = p - r;
    ll x = p, y = r;
    while (y * y >= p) {
        ll t = x % y;
        x = y;
        y = t;
    }
    ll rem = p - y * y;
    ll s = (ll)sqrtl((double)rem);
    while (s > 0 && s*s > rem) s--;
    while ((s+1)*(s+1) <= rem) s++;
    if (s*s == rem && y*y + s*s == p) {
        *a_out = (int)(y < s ? y : s);
        *b_out = (int)(y < s ? s : y);
        return;
    }
    for (ll a = 1; a * a < p; a++) {
        ll bsq = p - a*a;
        ll b = (ll)sqrtl((double)bsq);
        if (b*b == bsq) { *a_out = (int)a; *b_out = (int)b; return; }
        b++;
        if (b*b == bsq) { *a_out = (int)a; *b_out = (int)b; return; }
    }
}

void compute_gauss_decompositions() {
    for (int idx = 0; idx < nprimes1; idx++)
        decompose_prime(primes1[idx], &gauss_a[idx], &gauss_b[idx]);
}

typedef struct { int a, b, exp; } PFactor;
static PFactor pfactors[25];
static int npf_g;
static int g_pair_count;

void gauss_pow_mul(lll *re, lll *im, int a, int b, int k) {
    lll ga = a, gb = b;
    for (int i = 0; i < k; i++) {
        lll nr = *re * ga - *im * gb;
        lll ni = *re * gb + *im * ga;
        *re = nr; *im = ni;
    }
}

void enumerate(int idx, lll re, lll im) {
    if (idx == npf_g) {
        lll coords[4][2] = {
            {re, im}, {-im, re}, {-re, -im}, {im, -re}
        };
        for (int u = 0; u < 4; u++) {
            lll x = coords[u][0], y = coords[u][1];
            if (x > 0 && y > 0 && x <= y) {
                int xm = (int)(((ll)(x % 6) + 6) % 6);
                int ym = (int)(((ll)(y % 6) + 6) % 6);
                if (xm == 5 && ym == 5) g_pair_count++;
            }
        }
        return;
    }
    int a = pfactors[idx].a, b = pfactors[idx].b, e = pfactors[idx].exp;
    for (int j = 0; j <= e; j++) {
        lll fre = 1, fim = 0;
        gauss_pow_mul(&fre, &fim, a, b, j);
        gauss_pow_mul(&fre, &fim, a, -b, e - j);
        lll nre = re * fre - im * fim;
        lll nim = re * fim + im * fre;
        enumerate(idx + 1, nre, nim);
    }
}

int find_cand(ll c) {
    int lo = 0, hi = ncands - 1;
    while (lo <= hi) {
        int mid = (lo + hi) / 2;
        if (cand_info[mid].c == c) return mid;
        if (cand_info[mid].c < c) lo = mid + 1;
        else hi = mid - 1;
    }
    return -1;
}

int count_reps_from_info(int ci) {
    CandInfo *info = &cand_info[ci];
    npf_g = 0;
    for (int i = 0; i < info->nf; i++) {
        pfactors[npf_g].a = gauss_a[info->pidx[i]];
        pfactors[npf_g].b = gauss_b[info->pidx[i]];
        pfactors[npf_g].exp = info->pexp[i];
        npf_g++;
    }
    if (info->remaining > 1) {
        int a, b;
        decompose_prime(info->remaining, &a, &b);
        pfactors[npf_g].a = a;
        pfactors[npf_g].b = b;
        pfactors[npf_g].exp = 1;
        npf_g++;
    }
    g_pair_count = 0;
    enumerate(0, 1, 1);
    return g_pair_count;
}

int main() {
    compute_primes();
    compute_roots();
    compute_gauss_decompositions();

    /* Pass 1: sieve R values */
    for (int i = 0; i < CLIMIT; i++) R_arr[i] = 1;
    for (int idx = 0; idx < nprimes1; idx++) {
        int p = primes1[idx];
        int r1 = root1v[idx], r2 = root2v[idx];
        if (r1 == -1) continue;
        for (ll c = r1; c < CLIMIT; c += p) {
            if (c < 2) continue;
            ll m = 18*c*c - 6*c + 1;
            int e = 0;
            while (m % p == 0) { m /= p; e++; }
            ll new_R = (ll)R_arr[c] * (2*e + 1);
            if (new_R > 65535) new_R = 65535;
            R_arr[c] = (unsigned short)new_R;
        }
        if (r2 != r1) {
            for (ll c = r2; c < CLIMIT; c += p) {
                if (c < 2) continue;
                ll m = 18*c*c - 6*c + 1;
                int e = 0;
                while (m % p == 0) { m /= p; e++; }
                ll new_R = (ll)R_arr[c] * (2*e + 1);
                if (new_R > 65535) new_R = 65535;
                R_arr[c] = (unsigned short)new_R;
            }
        }
    }

    /* Collect candidates */
    ncands = 0;
    for (ll c = 2; c < CLIMIT; c++) {
        if (R_arr[c] >= THRESHOLD && ncands < MAX_CANDS) {
            cand_info[ncands].c = c;
            cand_info[ncands].nf = 0;
            cand_info[ncands].remaining = 18*c*c - 6*c + 1;
            ncands++;
        }
    }

    /* Pass 2: build factorizations for candidates */
    for (int idx = 0; idx < nprimes1; idx++) {
        int p = primes1[idx];
        int r1 = root1v[idx], r2 = root2v[idx];
        if (r1 == -1) continue;
        for (ll c = r1; c < CLIMIT; c += p) {
            if (c < 2 || R_arr[c] < THRESHOLD) continue;
            int ci = find_cand(c);
            if (ci < 0) continue;
            CandInfo *info = &cand_info[ci];
            ll m = info->remaining;
            if (m % p != 0) continue;
            int e = 0;
            while (m % p == 0) { m /= p; e++; }
            info->remaining = m;
            info->pidx[info->nf] = idx;
            info->pexp[info->nf] = e;
            info->nf++;
        }
        if (r2 != r1) {
            for (ll c = r2; c < CLIMIT; c += p) {
                if (c < 2 || R_arr[c] < THRESHOLD) continue;
                int ci = find_cand(c);
                if (ci < 0) continue;
                CandInfo *info = &cand_info[ci];
                ll m = info->remaining;
                if (m % p != 0) continue;
                int e = 0;
                while (m % p == 0) { m /= p; e++; }
                info->remaining = m;
                info->pidx[info->nf] = idx;
                info->pexp[info->nf] = e;
                info->nf++;
            }
        }
    }

    /* Check candidates */
    int target = 100;
    for (int i = 0; i < ncands; i++) {
        int count = count_reps_from_info(i);
        if (count > target) {
            ll c = cand_info[i].c;
            printf("%lld\n", c * (3*c - 1) / 2);
            return 0;
        }
    }

    printf("Not found\n");
    return 1;
}
