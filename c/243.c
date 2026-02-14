/*
 * Project Euler Problem 243: Resilience
 *
 * Find the smallest d such that phi(d)/(d-1) < 15499/94744.
 */
#include <stdio.h>
#include <string.h>

#define PLIMIT 100

static int primes[30];
static int nprimes;

void sieve(void) {
    char is_prime[PLIMIT + 1];
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; i * i <= PLIMIT; i++)
        if (is_prime[i])
            for (int j = i * i; j <= PLIMIT; j += i)
                is_prime[j] = 0;
    nprimes = 0;
    for (int i = 2; i <= PLIMIT; i++)
        if (is_prime[i])
            primes[nprimes++] = i;
}

typedef long long ll;

static ll R_num = 15499;
static ll R_den = 94744;
static ll best_ans;
static int base_index;
static ll prev_prod, prev_phi;

void search(int idx, ll m, ll phi_m) {
    ll d = prev_prod * m;
    if (d >= best_ans) return;
    ll phi_d = prev_phi * phi_m;
    /* Check: phi_d * R_den < R_num * (d - 1) */
    if (phi_d * R_den < R_num * (d - 1)) {
        best_ans = d;
        return;
    }
    for (int i = idx; i <= base_index && i < nprimes; i++) {
        int p = primes[i];
        ll new_m = m * p;
        if (prev_prod * new_m >= best_ans) break;
        /* phi factor for additional p: since p | prev_prod already, phi(m*p) = phi(m)*p */
        search(i, new_m, phi_m * p);
    }
}

int main(void) {
    sieve();

    ll prod = 1, phi = 1;
    base_index = 0;

    for (int i = 0; i < nprimes; i++) {
        int p = primes[i];
        prod *= p;
        phi *= (p - 1);
        if (phi * R_den < R_num * (prod - 1)) {
            base_index = i;
            break;
        }
    }

    best_ans = prod;

    prev_prod = prod / primes[base_index];
    prev_phi = phi / (primes[base_index] - 1);

    /* Try multiplying prev_prod by powers of primes <= primes[base_index] */
    for (int i = 0; i < base_index; i++) {
        int p = primes[i];
        ll test_prod = prev_prod * p;
        ll test_phi = prev_phi * p;
        if (test_phi * R_den < R_num * (test_prod - 1)) {
            if (test_prod < best_ans)
                best_ans = test_prod;
        }
    }

    search(0, 1, 1);

    printf("%lld\n", best_ans);
    return 0;
}
