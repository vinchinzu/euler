#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define N 100000

ll total_iters = 0;

ll count_pq_case1(ll k, ll l, ll n) {
    ll count = 0;
    for (ll p = 1; ; p++) {
        total_iters++;
        ll q_min = l;
        if (p >= q_min) q_min = p + 1;
        ll q_bc = ((lll)(3*k + l) * p + (k + l) - 1) / (k + l);
        if (q_bc > q_min) q_min = q_bc;
        ll q_max = (lll)l * p / k;
        ll q_cn = (n + (lll)k * p) / l;
        if (q_cn < q_max) q_max = q_cn;
        if (q_min > q_max) {
            if (q_cn < q_bc) break;
            continue;
        }
        count += q_max - q_min + 1;
    }
    return count;
}

ll count_pq_case2(ll k, ll l, ll n) {
    ll l_half = (l + 1) / 2;
    ll n_half = n / 2;
    ll count = 0;
    for (ll p = 1; ; p++) {
        total_iters++;
        ll q_min = l_half;
        if (p >= q_min) q_min = p + 1;
        ll q_bc = ((lll)(3*k + l) * p + (k + l) - 1) / (k + l);
        if (q_bc > q_min) q_min = q_bc;
        ll q_max = (lll)l * p / k;
        ll q_cn = (n_half + (lll)k * p) / l;
        if (q_cn < q_max) q_max = q_cn;
        if (q_min > q_max) {
            if (q_cn < q_bc) break;
            continue;
        }
        count += q_max - q_min + 1;
    }
    return count;
}

ll count_pq_case3(ll k, ll l, ll n) {
    ll count = 0;
    for (ll v = 1; ; v++) {
        total_iters++;
        ll u_min = v + 1;
        if (l - v > u_min) u_min = l - v;
        ll u_from_y = ((lll)(k + l) * v + (l - k) - 1) / (l - k);
        if (u_from_y > u_min) u_min = u_from_y;
        ll u_max = (lll)(2*k + l) * v / k;
        ll num = n - (lll)(k + l) * v;
        if (num < 0) break;
        ll u_cn = num / (l - k);
        if (u_cn < u_max) u_max = u_cn;
        if (u_min <= u_max) count += u_max - u_min + 1;
    }
    return count;
}

ll f_first_loop_only(ll n, int checkParity) {
    ll L = (ll)sqrtl(1.5 * n);
    ll result = 0;
    for (ll l = 1; l <= L; l++) {
        for (ll k = 1; k < l; k++) {
            if (!checkParity || ((k % 2 == 0) && (l % 2 == 0)))
                result += count_pq_case1(k, l, n);
            else if ((k % 2 == 0) || (l % 2 == 0))
                result += count_pq_case2(k, l, n);
            else
                result += count_pq_case3(k, l, n);
        }
    }
    return result;
}

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
            int p = spf[i]; int q = i / p;
            if (q % p == 0) mobius[i] = 0;
            else mobius[i] = -mobius[q];
        }
    }
    free(spf);
}

int main() {
    compute_mobius(N);
    
    // Just measure g=1 case
    total_iters = 0;
    ll result_g1 = f_first_loop_only(N, 1);
    printf("g=1 first loop only: result=%lld, iters=%lld\n", result_g1, total_iters);
    
    // Measure all g
    total_iters = 0;
    ll ans = 0;
    for (int g = 1; g <= N; g++) {
        if (mobius[g] != 0) {
            // Just first loop
            ans += mobius[g] * f_first_loop_only(N / g, g % 2 == 1);
        }
    }
    printf("All g first loop: total_iters=%lld\n", total_iters);
    
    free(mobius);
    return 0;
}
