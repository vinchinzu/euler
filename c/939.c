/* Project Euler Problem 939 - Nim Variant
 * E(N) using partition function.
 * E(n) = sum_{k=1}^{n} sum_{s=ceil(k/2+1/2)}^{k} p(s)*p(k-s) mod MOD
 * where p is partition function computed via pentagonal theorem.
 */
#include <stdio.h>

typedef long long ll;
#define MOD 1234567891LL
#define N 5000

static ll p[N + 1];

void compute_partitions(void) {
    p[0] = 1;
    for (int i = 1; i <= N; i++) {
        ll val = 0;
        for (int k = 1; ; k++) {
            int pent1 = k * (3 * k - 1) / 2;
            int pent2 = k * (3 * k + 1) / 2;
            int sign = (k % 2 == 1) ? 1 : -1;

            if (pent1 <= i)
                val = (val + sign * p[i - pent1]) % MOD;
            if (pent2 <= i)
                val = (val + sign * p[i - pent2]) % MOD;

            if (pent1 > i && pent2 > i) break;
        }
        p[i] = ((val % MOD) + MOD) % MOD;
    }
}

int main(void) {
    compute_partitions();

    ll result = 0;
    for (int k = 1; k <= N; k++) {
        int s_start = (k + 1) / 2;
        for (int s = s_start; s <= k; s++) {
            int t = k - s;
            ll ways = p[s] * p[t] % MOD;
            result = (result + ways) % MOD;
        }
    }

    printf("%lld\n", result);
    return 0;
}
