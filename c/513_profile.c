#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define N 100000

int8_t *mobius;

void compute_mobius(int limit) {
    mobius = (int8_t *)calloc(limit + 1, sizeof(int8_t));
    int *spf = (int *)malloc((limit + 1) * sizeof(int));
    mobius[1] = 1;
    for (int i = 0; i <= limit; i++) spf[i] = i;
    for (int i = 2; i <= limit; i++) {
        if (spf[i] == i) {
            mobius[i] = -1;
            for (ll j = (ll)i * i; j <= limit; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        } else {
            int p = spf[i];
            int q = i / p;
            if (q % p == 0) mobius[i] = 0;
            else mobius[i] = -mobius[q];
        }
    }
    free(spf);
}

int main() {
    compute_mobius(N);
    int nonzero = 0;
    ll total_L = 0;
    ll total_pairs = 0;
    for (int g = 1; g <= N; g++) {
        if (mobius[g] != 0) {
            nonzero++;
            ll n = N / g;
            ll L = (ll)sqrtl(1.5 * n);
            total_L += L;
            total_pairs += L * (L-1) / 2;  // approximate
        }
    }
    printf("nonzero mobius: %d\n", nonzero);
    printf("total L sum: %lld\n", total_L);
    printf("total pairs estimate: %lld\n", total_pairs);
    free(mobius);
    return 0;
}
