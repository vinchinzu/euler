/*
 * Project Euler 699 - Triffle Numbers
 *
 * Find sum of all n <= N where denominator of sigma(n)/n in lowest terms
 * is a power of 3.
 *
 * Start with 3-smooth numbers (2^a * 3^b), then recursively multiply by
 * primes that appear in the numerator of sigma(n)/n.
 *
 * We track the factorization and sigma(n)/n as a rational number.
 * Since N = 10^14, we use DFS with pruning.
 */
#include <stdio.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define N_LIMIT 100000000000000LL  /* 10^14 */
#define MAX_FACTORS 50

static ll answer = 0;

/* Compute sigma_1(p^e) = 1 + p + p^2 + ... + p^e */
ll sigma_pe(ll p, int e) {
    ll result = 1, pw = 1;
    for (int i = 0; i < e; i++) {
        pw *= p;
        result += pw;
    }
    return result;
}

/* GCD */
ll gcd(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

/* Check if n is a power of 3 */
int is_pow3(ll n) {
    if (n <= 0) return 0;
    while (n % 3 == 0) n /= 3;
    return n == 1;
}

/* Factor n, return list of (prime, exp) pairs.
   Only used for smallish numbers (numerators of sigma(n)/n). */
typedef struct { ll p; int e; } Factor;
static Factor factors[MAX_FACTORS];
static int nfactors;

void factorize(ll n, Factor *f, int *nf) {
    *nf = 0;
    if (n <= 1) return;
    for (ll d = 2; d * d <= n; d++) {
        if (n % d == 0) {
            f[*nf].p = d; f[*nf].e = 0;
            while (n % d == 0) { f[*nf].e++; n /= d; }
            (*nf)++;
        }
    }
    if (n > 1) { f[*nf].p = n; f[*nf].e = 1; (*nf)++; }
}

/* DFS: n is the current number, num/den = sigma(n)/n in lowest terms.
   primes[] and exps[] describe the factorization of n.
   np = number of distinct prime factors of n. */
void dfs(ll n, ll num, ll den, ll *primes, int *exps, int np) {
    /* Check if den is a power of 3 (and > 1) */
    if (den > 1 && is_pow3(den)) {
        answer += n;
    }

    /* Find prime factors of num that don't divide n */
    Factor nf[MAX_FACTORS];
    int nnf;
    factorize(num, nf, &nnf);

    for (int fi = 0; fi < nnf; fi++) {
        ll p = nf[fi].p;
        /* Check if p already divides n */
        int already = 0;
        for (int j = 0; j < np; j++)
            if (primes[j] == p) { already = 1; break; }
        if (already) continue;

        /* Try multiplying n by p^e for e = 1, 2, 3, ... */
        ll pw = 1;
        for (int e = 1; ; e++) {
            if (pw > N_LIMIT / p) break;
            pw *= p;
            if (n > N_LIMIT / pw) break;
            ll new_n = n * pw;
            ll sp = sigma_pe(p, e);
            ll new_num = num * sp;
            ll new_den = den * pw;
            ll g = gcd(new_num < 0 ? -new_num : new_num,
                       new_den < 0 ? -new_den : new_den);
            new_num /= g;
            new_den /= g;

            ll new_primes[MAX_FACTORS];
            int new_exps[MAX_FACTORS];
            int new_np = np;
            for (int j = 0; j < np; j++) {
                new_primes[j] = primes[j];
                new_exps[j] = exps[j];
            }
            new_primes[new_np] = p;
            new_exps[new_np] = e;
            new_np++;

            dfs(new_n, new_num, new_den, new_primes, new_exps, new_np);
        }
    }
}

int main(void) {
    /* Start with 3-smooth numbers: n = 2^a * 3^b, b >= 1 */
    ll pw2 = 1;
    for (int a = 0; pw2 <= N_LIMIT; a++) {
        ll pw3 = 3;
        for (int b = 1; pw2 <= N_LIMIT / pw3; b++) {
            ll n = pw2 * pw3;
            ll s2 = sigma_pe(2, a);
            ll s3 = sigma_pe(3, b);
            ll num = s2 * s3;
            ll den = n;
            ll g = gcd(num, den);
            num /= g;
            den /= g;

            ll primes[MAX_FACTORS];
            int exps[MAX_FACTORS];
            int np = 0;
            if (a > 0) { primes[np] = 2; exps[np] = a; np++; }
            primes[np] = 3; exps[np] = b; np++;

            dfs(n, num, den, primes, exps, np);

            if (pw3 > N_LIMIT / 3) break;
            pw3 *= 3;
        }
        if (pw2 > N_LIMIT / 2) break;
        pw2 *= 2;
    }

    printf("%lld\n", answer);
    return 0;
}
