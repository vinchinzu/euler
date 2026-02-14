/* Project Euler 342 - Sum of n where phi(n^2) is a cube.
 *
 * Find sum of all n with 1 < n < 10^10 such that phi(n^2) is a perfect cube.
 * Closely follows the Python recursive backtracking approach.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 10000000000LL
#define SQRT_MAXN 100001

static char sieve_arr[SQRT_MAXN];
static int primes[10000];
static int nprimes = 0;

void build_sieve(void) {
    memset(sieve_arr, 0, sizeof(sieve_arr));
    for (int i = 2; i < SQRT_MAXN; i++) {
        if (!sieve_arr[i]) {
            primes[nprimes++] = i;
            for (long long j = (long long)i * i; j < SQRT_MAXN; j += i)
                sieve_arr[j] = 1;
        }
    }
}

#define MAX_FACTORS 15
typedef struct { int p; int e; } Factor;

int factorize(int n, Factor *f) {
    int cnt = 0;
    for (int i = 0; i < nprimes && (long long)primes[i] * primes[i] <= n; i++) {
        if (n % primes[i] == 0) {
            f[cnt].p = primes[i];
            f[cnt].e = 0;
            while (n % primes[i] == 0) { n /= primes[i]; f[cnt].e++; }
            cnt++;
        }
    }
    if (n > 1) { f[cnt].p = n; f[cnt].e = 1; cnt++; }
    return cnt;
}

/* Phi state: map from prime -> exponent (only entries where exp % 3 != 0) */
#define MAX_PHI 30
typedef struct {
    int p[MAX_PHI];
    int e[MAX_PHI];
    int cnt;
} PhiState;

int phi_find(const PhiState *s, int p) {
    for (int i = 0; i < s->cnt; i++)
        if (s->p[i] == p) return i;
    return -1;
}

void phi_remove(PhiState *s, int idx) {
    s->cnt--;
    if (idx < s->cnt) {
        s->p[idx] = s->p[s->cnt];
        s->e[idx] = s->e[s->cnt];
    }
}

int phi_max_prime(const PhiState *s) {
    int mx = 0;
    for (int i = 0; i < s->cnt; i++)
        if (s->p[i] > mx) mx = s->p[i];
    return mx;
}

static long long ans = 0;

void helper(long long n, PhiState phi, int max_prime);
void add_prime(long long n, PhiState phi, int p, int start_e, int max_prime_for_recurse);

void helper(long long n, PhiState phi, int max_prime) {
    if (phi.cnt == 0) {
        if (n > 1) ans += n;
    } else {
        int mx = phi_max_prime(&phi);
        int idx = phi_find(&phi, mx);
        int e_mod = phi.e[idx] % 3;
        int start_e = (e_mod == 1) ? 3 : 1;
        add_prime(n, phi, mx, start_e, mx);
    }

    /* Try adding new primes */
    int mx_p = (phi.cnt > 0) ? phi_max_prime(&phi) : 0;
    for (int i = 0; i < nprimes; i++) {
        int p = primes[i];
        if (p >= max_prime) break;
        if (n * (long long)p * p >= MAXN) break;
        if (n % p == 0) continue;
        if (phi.cnt > 0 && p < mx_p) continue;
        if (phi_find(&phi, p) >= 0) continue;
        add_prime(n, phi, p, 2, max_prime);
    }
}

void add_prime(long long n, PhiState phi, int p, int start_e, int max_prime_for_recurse) {
    /* Add p^e to n for e = start_e, start_e+3, start_e+6, ... */
    long long pe = 1;
    for (int j = 0; j < start_e; j++) {
        if (pe > MAXN / p) return;
        pe *= p;
    }

    for (int e = start_e; ; e += 3) {
        if (n > (MAXN - 1) / pe) break;

        PhiState new_phi = phi;

        /* Remove p from phi if present */
        int idx = phi_find(&new_phi, p);
        if (idx >= 0) phi_remove(&new_phi, idx);

        /* Add factors of (p-1) */
        Factor factors[MAX_FACTORS];
        int nf = factorize(p - 1, factors);

        int good = 1;
        for (int fi = 0; fi < nf && good; fi++) {
            int q = factors[fi].p;
            int f = factors[fi].e;
            int qi = phi_find(&new_phi, q);
            int old_val = (qi >= 0) ? new_phi.e[qi] : 0;
            int new_val = old_val + f;
            if (new_val % 3 != 0) {
                if (qi >= 0) {
                    new_phi.e[qi] = new_val;
                } else {
                    if (new_phi.cnt >= MAX_PHI) { good = 0; break; }
                    new_phi.p[new_phi.cnt] = q;
                    new_phi.e[new_phi.cnt] = new_val;
                    new_phi.cnt++;
                }
                if (n % q == 0) { good = 0; }
            } else {
                if (qi >= 0) phi_remove(&new_phi, qi);
            }
        }

        if (good)
            helper(n * pe, new_phi, p);

        /* Next: e += 3 */
        for (int j = 0; j < 3; j++) {
            if (pe > MAXN / p) { pe = MAXN + 1; break; }
            pe *= p;
        }
    }
}

int main(void) {
    build_sieve();

    PhiState phi;
    phi.cnt = 0;
    /* Python: max_prime = 2**31, use INT_MAX */
    helper(1, phi, 2147483647);

    printf("%lld\n", ans);
    return 0;
}
