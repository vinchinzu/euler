/*
 * Project Euler Problem 510: Tangent Circles.
 * Sum r_A + r_B + r_C for all valid circle configurations with r_A <= r_B <= N.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;

int isqrt_ll(ll n) {
    ll r = (ll)sqrt((double)n);
    while (r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return (int)r;
}

int main() {
    ll N = 1000000000LL;
    int L = isqrt_ll(N);

    /* Sieve primes up to L */
    int *primes = NULL;
    int num_primes = 0;
    {
        char *is_prime = (char*)calloc(L + 1, 1);
        memset(is_prime, 1, L + 1);
        is_prime[0] = is_prime[1] = 0;
        for (int i = 2; (ll)i * i <= L; i++)
            if (is_prime[i])
                for (int j = i * i; j <= L; j += i)
                    is_prime[j] = 0;
        for (int i = 2; i <= L; i++)
            if (is_prime[i]) num_primes++;
        primes = (int*)malloc(num_primes * sizeof(int));
        int idx = 0;
        for (int i = 2; i <= L; i++)
            if (is_prime[i]) primes[idx++] = i;
        free(is_prime);
    }

    /* Mobius function up to L */
    int *mu = (int*)calloc(L + 1, sizeof(int));
    {
        char *is_square_free = (char*)calloc(L + 1, 1);
        memset(is_square_free, 1, L + 1);
        for (int i = 0; i <= L; i++) mu[i] = 1;
        for (int pi = 0; pi < num_primes; pi++) {
            int p = primes[pi];
            if (p > L) break;
            for (int j = p; j <= L; j += p) {
                mu[j] *= -1;
                if (j % ((ll)p * p) == 0)
                    is_square_free[j] = 0;
            }
        }
        for (int i = 0; i <= L; i++)
            if (!is_square_free[i]) mu[i] = 0;
        free(is_square_free);
    }

    ll ans = 0;

    for (int g = 1; (ll)g * g <= N; g++) {
        if (mu[g] == 0) continue;
        ll n = N / ((ll)g * g);

        for (int b = 1; (ll)b * b <= n; b++) {
            ll b2 = (ll)b * b;

            /* Find all divisors of b^2 using prime factorization of b */
            int divs[2000];
            int ndivs = 0;
            {
                /* factorize b */
                int bb = b;
                int pf[20], pe[20];
                int npf = 0;
                for (int pi = 0; pi < num_primes && (ll)primes[pi] * primes[pi] <= bb; pi++) {
                    if (bb % primes[pi] == 0) {
                        pf[npf] = primes[pi];
                        pe[npf] = 0;
                        while (bb % primes[pi] == 0) {
                            pe[npf]++;
                            bb /= primes[pi];
                        }
                        npf++;
                    }
                }
                if (bb > 1) {
                    pf[npf] = bb;
                    pe[npf] = 1;
                    npf++;
                }
                /* Generate divisors of b^2 (exponents doubled) */
                divs[0] = 1;
                ndivs = 1;
                for (int i = 0; i < npf; i++) {
                    int old_count = ndivs;
                    ll pp = 1;
                    for (int e = 0; e < 2 * pe[i]; e++) {
                        pp *= pf[i];
                        for (int j = 0; j < old_count; j++) {
                            divs[ndivs++] = (int)(divs[j] * pp);
                        }
                    }
                }
            }

            for (int di = 0; di < ndivs; di++) {
                int d = divs[di];
                int a = d - b;
                if (a > 0 && a <= b) {
                    int c = b - (int)(b2 / d);
                    ll tri = (ll)b * (b + 1) / 2;  /* Wait, need n / b^2 */
                    ll nb2 = n / b2;
                    ll tr_nb2 = nb2 * (nb2 + 1) / 2;
                    ans += (ll)mu[g] * ((ll)g * g) *
                           ((ll)a * a + (ll)b * b + (ll)c * c) * tr_nb2;
                }
            }
        }
    }

    printf("%lld\n", ans);

    free(primes);
    free(mu);
    return 0;
}
