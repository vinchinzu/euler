/* Project Euler 489 - Common factors between two sequences
 * Translated from python/489.py
 *
 * For given (a, b), find GCD of (n^3+b, (n+a)^3+b) using prime factorization + CRT.
 * Sum G(a,b) for a=1..18, b=1..1900.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

/* Factor storage */
#define MAX_FACTORS 64
typedef struct {
    ll primes[MAX_FACTORS];
    int exps[MAX_FACTORS];
    int count;
} Factors;

ll gcd_ll(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

void factorize(ll n, Factors *f) {
    f->count = 0;
    if (n < 0) n = -n;
    if (n <= 1) return;
    for (ll d = 2; d * d <= n; d++) {
        if (n % d == 0) {
            f->primes[f->count] = d;
            f->exps[f->count] = 0;
            while (n % d == 0) {
                f->exps[f->count]++;
                n /= d;
            }
            f->count++;
        }
    }
    if (n > 1) {
        f->primes[f->count] = n;
        f->exps[f->count] = 1;
        f->count++;
    }
}

/* Merge two factor lists (union of prime sets) into a unique sorted prime list */
void merge_primes(Factors *f1, Factors *f2, ll *out, int *out_count) {
    *out_count = 0;
    for (int i = 0; i < f1->count; i++) {
        int found = 0;
        for (int j = 0; j < *out_count; j++)
            if (out[j] == f1->primes[i]) { found = 1; break; }
        if (!found) out[(*out_count)++] = f1->primes[i];
    }
    for (int i = 0; i < f2->count; i++) {
        int found = 0;
        for (int j = 0; j < *out_count; j++)
            if (out[j] == f2->primes[i]) { found = 1; break; }
        if (!found) out[(*out_count)++] = f2->primes[i];
    }
}

/* Extended GCD */
void ext_gcd(ll a, ll b, ll *g, ll *x, ll *y) {
    if (a == 0) { *g = b; *x = 0; *y = 1; return; }
    ll g1, x1, y1;
    ext_gcd(b % a, a, &g1, &x1, &y1);
    *g = g1;
    *x = y1 - (b / a) * x1;
    *y = x1;
}

/* Modular inverse; returns -1 if not invertible */
ll mod_inv(ll a, ll m) {
    if (m == 1) return 0;
    a = ((a % m) + m) % m;
    ll g, x, y;
    ext_gcd(a, m, &g, &x, &y);
    if (g != 1) return -1;
    return ((x % m) + m) % m;
}

/* CRT for two congruences */
/* Returns -1 if no solution */
ll crt2(ll r1, ll m1, ll r2, ll m2, ll *out_m) {
    ll g = gcd_ll(m1, m2);
    if ((r2 - r1) % g != 0) return -1;
    ll lcm = m1 / g * m2;
    ll inv = mod_inv(m1 / g, m2 / g);
    if (inv < 0) return -1;
    ll r = r1 + (lll)m1 % lcm * ((lll)((r2 - r1) / g) % lcm * inv % lcm) % lcm;
    r = ((r % lcm) + lcm) % lcm;
    *out_m = lcm;
    return r;
}

ll cb_mod(ll n, ll m) {
    n = ((n % m) + m) % m;
    return (lll)n * n % m * n % m;
}

/* For a given prime p, find the highest power p^e dividing GCD for some n,
 * and all valid n mod p^e */
#define MAX_NS 1000
typedef struct {
    ll ns[MAX_NS];
    int count;
    ll modulus;
} PrimeSolution;

void find_prime_solutions(ll a, ll b, ll p, PrimeSolution *sol) {
    sol->count = 0;
    sol->modulus = 1;

    ll prev_m = 1;
    ll prev_ns[MAX_NS];
    int prev_count = 0;

    ll m = 1;
    while (1) {
        m = prev_m * p;
        /* Overflow check */
        if (m / p != prev_m || m > 1000000000000LL) break;

        ll ns_buf[MAX_NS];
        int ns_count = 0;

        if (gcd_ll(p, 6 * a) == 1) {
            /* n = -(2a^2)^{-1} * (3b + a^3) mod m */
            ll inv_val = mod_inv((lll)2 * a % m * a % m, m);
            if (inv_val >= 0) {
                ll n = (-(lll)inv_val % m * ((3 * b + (lll)a * a * a) % m) % m + 2 * m) % m;
                if (cb_mod(n, m) == ((-b) % m + m) % m &&
                    cb_mod(n + a, m) == ((-b) % m + m) % m) {
                    if (ns_count < MAX_NS) ns_buf[ns_count++] = n;
                }
            }
        } else {
            for (ll n = 0; n < m && ns_count < MAX_NS; n++) {
                ll v1 = (cb_mod(n, m) + (b % m + m) % m) % m;
                ll v2 = (cb_mod(n + a, m) + (b % m + m) % m) % m;
                if (v1 == 0 && v2 == 0) {
                    ns_buf[ns_count++] = n;
                }
            }
        }

        if (ns_count == 0) break;

        prev_count = ns_count;
        memcpy(prev_ns, ns_buf, ns_count * sizeof(ll));
        prev_m = m;
    }

    if (prev_count > 0) {
        sol->count = prev_count;
        memcpy(sol->ns, prev_ns, prev_count * sizeof(ll));
        sol->modulus = prev_m;
    }
}

ll G(int a, int b) {
    Factors f1, f2;
    factorize(6LL * a, &f1);
    ll expr = (lll)a * a * a * a * a * a + 27LL * b * b;
    factorize(expr, &f2);

    ll all_primes[128];
    int num_primes;
    merge_primes(&f1, &f2, all_primes, &num_primes);

    if (num_primes == 0) return 0;

    PrimeSolution sols[128];
    int num_sols = 0;

    for (int i = 0; i < num_primes; i++) {
        PrimeSolution sol;
        find_prime_solutions(a, b, all_primes[i], &sol);
        if (sol.count > 0) {
            sols[num_sols++] = sol;
        }
    }

    if (num_sols == 0) return 0;

    /* CRT combination - iterate through all combinations */
    /* Use iterative approach */
    ll *cur_rs = malloc(MAX_NS * sizeof(ll));
    ll *cur_ms = malloc(MAX_NS * sizeof(ll));
    int cur_count = 0;

    /* Start with first prime */
    for (int i = 0; i < sols[0].count; i++) {
        cur_rs[cur_count] = sols[0].ns[i];
        cur_ms[cur_count] = sols[0].modulus;
        cur_count++;
    }

    for (int si = 1; si < num_sols; si++) {
        ll *new_rs = malloc(MAX_NS * sizeof(ll));
        ll *new_ms = malloc(MAX_NS * sizeof(ll));
        int new_count = 0;

        for (int ci = 0; ci < cur_count; ci++) {
            for (int ni = 0; ni < sols[si].count; ni++) {
                ll out_m;
                ll r = crt2(cur_rs[ci], cur_ms[ci], sols[si].ns[ni], sols[si].modulus, &out_m);
                if (r >= 0 && new_count < MAX_NS) {
                    new_rs[new_count] = r;
                    new_ms[new_count] = out_m;
                    new_count++;
                }
            }
        }

        free(cur_rs); free(cur_ms);
        cur_rs = new_rs; cur_ms = new_ms;
        cur_count = new_count;
    }

    ll best = -1;
    for (int i = 0; i < cur_count; i++) {
        ll v = cur_rs[i];
        if (v < 0) v += cur_ms[i];
        if (best < 0 || v < best) best = v;
    }

    free(cur_rs); free(cur_ms);
    return best >= 0 ? best : 0;
}

int main() {
    int MA = 18;
    int N = 1900;
    ll ans = 0;

    for (int a = 1; a <= MA; a++) {
        for (int b = 1; b <= N; b++) {
            ans += G(a, b);
        }
    }

    printf("%lld\n", ans);
    return 0;
}
