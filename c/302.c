/*
 * Project Euler Problem 302: Strong Achilles Numbers
 *
 * An Achilles number is powerful (p|n => p^2|n) but not a perfect power.
 * A Strong Achilles number has both n and phi(n) being Achilles.
 * Count Strong Achilles numbers up to 10^18.
 *
 * Recursive generation of candidates with factorizations of n and phi(n).
 * Direct translation of the Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;

#define MAX_PRIMES 80000
#define MAX_DICT 64
#define MAX_SPF 1000100

static int primes[MAX_PRIMES];
static int nprimes;
static int spf[MAX_SPF];

/* Dictionary: small array of (prime, exponent) pairs */
typedef struct {
    int p[MAX_DICT];
    int e[MAX_DICT];
    int n;
} Dict;

static int dict_get(Dict *d, int key) {
    for (int i = 0; i < d->n; i++)
        if (d->p[i] == key) return d->e[i];
    return 0;
}

static int dict_has(Dict *d, int key) {
    for (int i = 0; i < d->n; i++)
        if (d->p[i] == key) return 1;
    return 0;
}

static void dict_set(Dict *d, int key, int val) {
    for (int i = 0; i < d->n; i++) {
        if (d->p[i] == key) { d->e[i] = val; return; }
    }
    if (d->n < MAX_DICT) {
        d->p[d->n] = key;
        d->e[d->n] = val;
        d->n++;
    }
}

static void dict_inc(Dict *d, int key, int val) {
    for (int i = 0; i < d->n; i++) {
        if (d->p[i] == key) { d->e[i] += val; return; }
    }
    if (d->n < MAX_DICT) {
        d->p[d->n] = key;
        d->e[d->n] = val;
        d->n++;
    }
}

static void dict_del(Dict *d, int key) {
    for (int i = 0; i < d->n; i++) {
        if (d->p[i] == key) {
            d->n--;
            d->p[i] = d->p[d->n];
            d->e[i] = d->e[d->n];
            return;
        }
    }
}

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

static int gcd_of_vals(Dict *d) {
    if (d->n == 0) return 0;
    int g = 0;
    for (int i = 0; i < d->n; i++)
        g = gcd(g, d->e[i]);
    return g;
}

static int is_prime_check(int n) {
    if (n < 2) return 0;
    if (n == 2) return 1;
    if (n % 2 == 0) return 0;
    for (int i = 3; (ll)i * i <= n; i += 2)
        if (n % i == 0) return 0;
    return 1;
}

static ll N;

/* Hash set for deduplication */
#define HASH_SIZE (1 << 21)  /* 2M entries */
#define HASH_MASK (HASH_SIZE - 1)
typedef struct hnode { ll val; struct hnode *next; } hnode;
static hnode *hash_table[HASH_SIZE];
static int achilles_count = 0;

static void achilles_add(ll val) {
    unsigned int h = (unsigned int)((val ^ (val >> 17)) * 2654435761ULL) & HASH_MASK;
    for (hnode *p = hash_table[h]; p; p = p->next)
        if (p->val == val) return; /* duplicate */
    hnode *nd = malloc(sizeof(hnode));
    nd->val = val;
    nd->next = hash_table[h];
    hash_table[h] = nd;
    achilles_count++;
}

static void helper(ll n, Dict *factors, Dict *phi, int max_p);
static void add_prime_fn(ll n, int p, int min_e, Dict *factors, Dict *phi, int max_p);

static void helper(ll n, Dict *factors, Dict *phi, int max_p) {
    /* Find largest prime in phi with exponent 1 */
    int bad_p = 0;
    for (int i = 0; i < phi->n; i++) {
        if (phi->e[i] == 1 && phi->p[i] > bad_p)
            bad_p = phi->p[i];
    }

    if (bad_p == 0) {
        /* Check if both n and phi(n) are Achilles */
        if (factors->n > 0) {
            int ge = gcd_of_vals(factors);
            int gp = gcd_of_vals(phi);
            if (ge == 1 && gp == 1)
                achilles_add(n);
        }

        /* Snapshot phi keys before iterating (phi may be modified) */
        int phi_keys[MAX_DICT];
        int phi_n = phi->n;
        for (int i = 0; i < phi_n; i++)
            phi_keys[i] = phi->p[i];

        /* Try adding primes already in phi */
        for (int i = 0; i < phi_n; i++) {
            int p = phi_keys[i];
            if (p < max_p)
                add_prime_fn(n, p, 2, factors, phi, p);
        }

        /* Try adding new primes with exponent >= 3 */
        for (int i = 0; i < nprimes; i++) {
            int p = primes[i];
            if (p >= max_p) break;
            if ((double)n * (double)p * (double)p * (double)p >= (double)N) break;
            if (!dict_has(phi, p))
                add_prime_fn(n, p, 3, factors, phi, p);
        }
    } else if ((double)n * (double)bad_p * (double)bad_p < (double)N) {
        /* Try to fix bad_p by adding it */
        add_prime_fn(n, bad_p, 2, factors, phi, max_p);

        /* Add primes q where bad_p | (q-1) */
        int p = bad_p + 1;
        while (p < max_p && (double)n * (double)p * (double)p < (double)N) {
            if (p % bad_p == 1 && is_prime_check(p))
                add_prime_fn(n, p, 2, factors, phi, max_p);
            p += bad_p;
        }
    }
}

static void add_prime_fn(ll n, int p, int min_e, Dict *factors, Dict *phi, int max_p) {
    if (dict_has(factors, p)) return;

    /* Save previous phi[p] */
    int prev_e = dict_get(phi, p);
    int had_prev = dict_has(phi, p);

    /* Update phi for adding p^(min_e-1) */
    dict_inc(phi, p, min_e - 1);

    /* Factor p-1 and add to phi */
    int phi_p_factors[40];
    int npf = 0;
    int temp = p - 1;
    while (temp > 1) {
        int pf;
        if (temp < MAX_SPF) {
            pf = spf[temp];
        } else {
            pf = temp;
            for (int i = 0; i < nprimes; i++) {
                if ((ll)primes[i] * primes[i] > temp) break;
                if (temp % primes[i] == 0) { pf = primes[i]; break; }
            }
        }
        phi_p_factors[npf++] = pf;
        dict_inc(phi, pf, 1);
        temp /= pf;
    }

    /* Try different exponents */
    int e = min_e;
    ll power_p = 1;
    for (int i = 0; i < min_e; i++) {
        if (power_p > N / p) { power_p = N + 1; break; }
        power_p *= p;
    }

    while (power_p <= N && n <= N / power_p) {
        dict_set(factors, p, e);
        helper(n * power_p, factors, phi, max_p);

        dict_inc(phi, p, 1);
        e++;
        if (power_p > N / p) break;
        power_p *= p;
    }

    /* Restore factors */
    dict_del(factors, p);

    /* Restore phi[p] */
    if (had_prev) {
        dict_set(phi, p, prev_e);
    } else {
        dict_del(phi, p);
    }

    /* Restore phi for factors of p-1 */
    for (int i = 0; i < npf; i++) {
        int pf = phi_p_factors[i];
        int cur = dict_get(phi, pf);
        if (cur <= 1)
            dict_del(phi, pf);
        else
            dict_set(phi, pf, cur - 1);
    }
}

static void sieve_spf(void) {
    for (int i = 0; i < MAX_SPF; i++) spf[i] = i;
    for (int i = 2; (ll)i * i < MAX_SPF; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j < MAX_SPF; j += i)
                if (spf[j] == j) spf[j] = i;
        }
    }
}

static void sieve_primes_list(int L) {
    char *is_p = calloc(L + 1, 1);
    for (int i = 2; i <= L; i++) is_p[i] = 1;
    for (int i = 2; (ll)i * i <= L; i++) {
        if (is_p[i]) {
            for (int j = i * i; j <= L; j += i)
                is_p[j] = 0;
        }
    }
    nprimes = 0;
    for (int i = 2; i <= L; i++)
        if (is_p[i]) primes[nprimes++] = i;
    free(is_p);
}

int main(void) {
    N = 1000000000000000000LL; /* 10^18 */
    int L = (int)(pow((double)N, 1.0/3.0)) + 2;

    sieve_spf();
    sieve_primes_list(L);

    Dict factors = {.n = 0};
    Dict phi = {.n = 0};

    helper(1, &factors, &phi, 1000000000);

    printf("%d\n", achilles_count);
    return 0;
}
