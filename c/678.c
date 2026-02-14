/*
 * Project Euler 678 - Fermat-like Equations
 *
 * Count (a,b,c,e,f) with a^e + b^e = c^f, a<b, e>=2, f>=3, c^f<=N.
 * Uses Gaussian integers for sums of two squares, divisor enumeration
 * for cubes, and direct enumeration for e>=5.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define N_VAL 1000000000000000000LL  /* 10^18 */

static int ff[1000100];  /* smallest prime factor */

static int isqrt_ll(ll n) {
    if (n <= 0) return 0;
    ll r = (ll)sqrt((double)n);
    while (r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return (int)r;
}

static int is_sq(ll n) {
    if (n < 0) return 0;
    ll r = (ll)sqrt((double)n);
    /* Check r-1, r, r+1 */
    for (ll t = (r > 0 ? r - 1 : 0); t <= r + 1; t++) {
        if (t * t == n) return 1;
    }
    return 0;
}

/* Gaussian integer pair (re, im) */
typedef struct { ll re, im; } GI;

/* Find (a,b) such that a^2+b^2 = p for prime p = 1 mod 4 */
static GI find_gaussian_factor(int p) {
    /* Find quadratic non-residue */
    int x = 2;
    while (1) {
        ll pw = 1;
        ll exp = ((ll)p - 1) / 2;
        ll base = x;
        ll mod = p;
        ll r = 1;
        while (exp > 0) {
            if (exp & 1) r = r * base % mod;
            base = base * base % mod;
            exp >>= 1;
        }
        if (r == (ll)p - 1) break;
        x++;
    }

    /* r = x^((p-1)/4) mod p */
    ll rr = 1;
    {
        ll base = x, exp = ((ll)p - 1) / 4, mod = p;
        while (exp > 0) {
            if (exp & 1) rr = rr * base % mod;
            base = base * base % mod;
            exp >>= 1;
        }
    }

    /* Euclidean algorithm */
    int sqrt_p = isqrt_ll(p);
    ll m = p, n = rr;
    while (n > sqrt_p) {
        ll tmp = n;
        n = m % n;
        m = tmp;
    }

    ll a = n;
    ll b = isqrt_ll((ll)p - n * n);
    GI result;
    result.re = a < b ? a : b;
    result.im = a < b ? b : a;
    return result;
}

/* Factor counting */
typedef struct { int p; int e; } PrimePower;

static int factorize(ll n, PrimePower *factors) {
    int nf = 0;
    /* n should be small enough for ff[] */
    while (n > 1) {
        int p = ff[(int)n];
        int e = 0;
        while ((int)(n) > 0 && n % p == 0) { n /= p; e++; }
        factors[nf].p = p;
        factors[nf].e = e;
        nf++;
    }
    return nf;
}

/* Sums of two squares from prime factorization (factors of c, each * f) */
/* Returns number of pairs (x,y) with x<=y, x^2+y^2=product, x>0, x<y */
typedef struct { ll x, y; } Pair;

static Pair pairs_buf[10000];
static int num_pairs;

static void sums_of_two_squares(PrimePower *factors, int nf, int f_mult) {
    num_pairs = 0;

    /* Check feasibility: p=3 mod 4 must have even total exponent */
    for (int i = 0; i < nf; i++) {
        if (factors[i].p % 4 == 3 && (factors[i].e * f_mult) % 2 == 1) return;
    }

    /* Build using Gaussian integers */
    GI *results = (GI *)malloc(100000 * sizeof(GI));
    int nresults = 1;
    results[0].re = 1;
    results[0].im = 0;

    for (int fi = 0; fi < nf; fi++) {
        int p = factors[fi].p;
        int exp = factors[fi].e * f_mult;

        if (p == 2) {
            for (int iter = 0; iter < exp; iter++) {
                for (int i = 0; i < nresults; i++) {
                    ll re = results[i].re, im = results[i].im;
                    results[i].re = re - im;
                    results[i].im = re + im;
                }
            }
        } else if (p % 4 == 1) {
            GI gf = find_gaussian_factor(p);
            ll a = gf.re, b = gf.im;

            /* Powers of (a+bi) and (a-bi) */
            GI *pow_pos = (GI *)malloc((exp + 1) * sizeof(GI));
            GI *pow_neg = (GI *)malloc((exp + 1) * sizeof(GI));
            pow_pos[0].re = 1; pow_pos[0].im = 0;
            pow_neg[0].re = 1; pow_neg[0].im = 0;
            for (int i = 1; i <= exp; i++) {
                pow_pos[i].re = pow_pos[i-1].re * a - pow_pos[i-1].im * b;
                pow_pos[i].im = pow_pos[i-1].re * b + pow_pos[i-1].im * a;
                pow_neg[i].re = pow_neg[i-1].re * a + pow_neg[i-1].im * b;
                pow_neg[i].im = -pow_neg[i-1].re * b + pow_neg[i-1].im * a;
            }

            GI *new_results = (GI *)malloc((ll)nresults * (exp + 1) * sizeof(GI));
            int new_n = 0;
            for (int i = 0; i < nresults; i++) {
                ll re = results[i].re, im = results[i].im;
                for (int k = 0; k <= exp; k++) {
                    ll fre = pow_pos[k].re * pow_neg[exp-k].re - pow_pos[k].im * pow_neg[exp-k].im;
                    ll fim = pow_pos[k].re * pow_neg[exp-k].im + pow_pos[k].im * pow_neg[exp-k].re;
                    new_results[new_n].re = re * fre - im * fim;
                    new_results[new_n].im = re * fim + im * fre;
                    new_n++;
                }
            }
            free(results);
            results = new_results;
            nresults = new_n;

            free(pow_pos);
            free(pow_neg);
        } else if (p % 4 == 3) {
            ll scale = 1;
            for (int i = 0; i < exp / 2; i++) scale *= p;
            for (int i = 0; i < nresults; i++) {
                results[i].re *= scale;
                results[i].im *= scale;
            }
        }
    }

    /* Deduplicate pairs */
    for (int i = 0; i < nresults; i++) {
        ll x = results[i].re < 0 ? -results[i].re : results[i].re;
        ll y = results[i].im < 0 ? -results[i].im : results[i].im;
        if (x > y) { ll t = x; x = y; y = t; }
        /* Check if already in pairs_buf */
        int found = 0;
        for (int j = 0; j < num_pairs; j++) {
            if (pairs_buf[j].x == x && pairs_buf[j].y == y) { found = 1; break; }
        }
        if (!found) {
            pairs_buf[num_pairs].x = x;
            pairs_buf[num_pairs].y = y;
            num_pairs++;
        }
    }

    free(results);
}

/* Get all divisors of n from its factorization */
static ll divisors_buf[100000];
static int num_divisors;

static void get_divisors(PrimePower *factors, int nf, int f_mult) {
    num_divisors = 1;
    divisors_buf[0] = 1;
    for (int fi = 0; fi < nf; fi++) {
        int p = factors[fi].p;
        int exp = factors[fi].e * f_mult;
        int old_n = num_divisors;
        ll pp = 1;
        for (int e = 1; e <= exp; e++) {
            pp *= p;
            for (int i = 0; i < old_n; i++) {
                divisors_buf[num_divisors++] = divisors_buf[i] * pp;
            }
        }
    }
}

/* For e>=5, precomputed sums: hash map from c^f -> count */
#define EMAX 64
#define HASH_E_SIZE 65536
#define HASH_E_MASK (HASH_E_SIZE - 1)

typedef struct { ll key; int val; int occupied; } EHashEntry;

typedef struct {
    EHashEntry entries[HASH_E_SIZE];
} EHashMap;

static EHashMap *e_counts[EMAX];

static int ehm_hash(ll key) {
    unsigned long long u = (unsigned long long)key;
    u = (u ^ (u >> 32)) * 0x45d9f3bULL;
    return (int)(u & HASH_E_MASK);
}

static void ehm_add(EHashMap *h, ll key, int val) {
    int idx = ehm_hash(key);
    for (;;) {
        if (!h->entries[idx].occupied) {
            h->entries[idx].key = key;
            h->entries[idx].val = val;
            h->entries[idx].occupied = 1;
            return;
        }
        if (h->entries[idx].key == key) {
            h->entries[idx].val += val;
            return;
        }
        idx = (idx + 1) & HASH_E_MASK;
    }
}

static int ehm_get(EHashMap *h, ll key) {
    int idx = ehm_hash(key);
    for (;;) {
        if (!h->entries[idx].occupied) return 0;
        if (h->entries[idx].key == key) return h->entries[idx].val;
        idx = (idx + 1) & HASH_E_MASK;
    }
}

int main() {
    int limit = (int)(pow((double)N_VAL, 1.0/3.0)) + 100;

    /* Smallest prime factor sieve */
    for (int i = 0; i <= limit; i++) ff[i] = i;
    for (int i = 2; (ll)i * i <= limit; i++) {
        if (ff[i] == i) {
            for (int j = i * i; j <= limit; j += i) {
                if (ff[j] == j) ff[j] = i;
            }
        }
    }

    /* Precompute sums a^e + b^e for e >= 5 */
    for (int e = 5; e < EMAX; e++) {
        if ((1LL << e) >= N_VAL) break;
        e_counts[e] = (EHashMap *)calloc(1, sizeof(EHashMap));

        /* Find max a such that a^e < N */
        int max_a = 1;
        {
            double log_max = log((double)N_VAL) / e;
            max_a = (int)exp(log_max) + 2;
        }

        ll *pows = (ll *)malloc((max_a + 2) * sizeof(ll));
        int npows = 0;
        for (int a = 1; ; a++) {
            ll ae = 1;
            int overflow = 0;
            for (int i = 0; i < e; i++) {
                if (ae > N_VAL / a + 1) { overflow = 1; break; }
                ae *= a;
            }
            if (overflow || ae >= N_VAL) break;
            pows[npows++] = ae;
        }

        for (int i = 0; i < npows; i++) {
            for (int j = i + 1; j < npows; j++) {
                ll cf = pows[i] + pows[j];
                if (cf <= N_VAL) {
                    ehm_add(e_counts[e], cf, 1);
                }
            }
        }
        free(pows);
    }

    ll ans = 0;

    /* Iterate over all c^f where f >= 3 and c^f <= N */
    for (int f = 3; (1LL << f) <= N_VAL; f++) {
        for (int c = 2; ; c++) {
            /* Compute c^f */
            ll cf = 1;
            int overflow = 0;
            for (int i = 0; i < f; i++) {
                if (cf > N_VAL / c) { overflow = 1; break; }
                cf *= c;
            }
            if (overflow || cf > N_VAL) break;

            /* Get prime factorization of c */
            PrimePower factors[30];
            int nf = factorize(c, factors);

            /* e = 2: sums of two squares */
            sums_of_two_squares(factors, nf, f);
            for (int i = 0; i < num_pairs; i++) {
                if (pairs_buf[i].x > 0 && pairs_buf[i].x < pairs_buf[i].y)
                    ans++;
            }

            /* e = 3: sums of two cubes */
            get_divisors(factors, nf, f);
            for (int di = 0; di < num_divisors; di++) {
                ll d = divisors_buf[di];
                /* d^3 < 4*cf */
                if (d > 0 && (lll)d * d * d < (lll)4 * cf) {
                    /* disc = 4*cf/d - d^2 */
                    ll disc = 4 * (cf / d) - d * d;
                    /* Need 4*cf divisible by d exactly */
                    if (4 * cf % d != 0) continue;
                    disc = 4 * cf / d - d * d;
                    if (disc > 0 && disc < 3 * d * d && is_sq(3 * disc))
                        ans++;
                }
            }

            /* e = 4: sums of two fourth powers */
            /* Reuse the sums_of_two_squares result */
            for (int i = 0; i < num_pairs; i++) {
                ll x = pairs_buf[i].x, y = pairs_buf[i].y;
                if (x > 0 && x < y && is_sq(x) && is_sq(y))
                    ans++;
            }

            /* e >= 5: lookup precomputed */
            for (int e = 5; e < EMAX; e++) {
                if ((1LL << e) >= cf) break;
                if (e_counts[e]) {
                    ans += ehm_get(e_counts[e], cf);
                }
            }
        }
    }

    printf("%lld\n", ans);

    for (int e = 5; e < EMAX; e++) {
        if (e_counts[e]) free(e_counts[e]);
    }
    return 0;
}
