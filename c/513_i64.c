#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;

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
            for (ll j = (ll)i * i; j <= limit; j += i)
                if (spf[j] == j) spf[j] = i;
        } else {
            int p = spf[i]; int q = i / p;
            mobius[i] = (q % p == 0) ? 0 : -mobius[q];
        }
    }
    free(spf);
}

static inline ll ceil_div(ll a, ll b) { return (a + b - 1) / b; }

ll count_pq_case1(ll k, ll l, ll n) {
    ll count = 0;
    ll kl3 = 3*k+l, kl_sum = k+l;
    for (ll p = 1; ; p++) {
        ll q_min = (p >= l) ? p + 1 : l;
        ll q_bc = ceil_div(kl3 * p, kl_sum);
        if (q_bc > q_min) q_min = q_bc;
        ll q_max = l * p / k;
        ll q_cn = (n + k * p) / l;
        if (q_cn < q_max) q_max = q_cn;
        if (q_min > q_max) { if (q_cn < q_bc) break; continue; }
        count += q_max - q_min + 1;
    }
    return count;
}

ll count_pq_case2(ll k, ll l, ll n) {
    ll l_half = (l + 1) / 2, n_half = n / 2;
    ll count = 0;
    ll kl3 = 3*k+l, kl_sum = k+l;
    for (ll p = 1; ; p++) {
        ll q_min = (p >= l_half) ? p + 1 : l_half;
        ll q_bc = ceil_div(kl3 * p, kl_sum);
        if (q_bc > q_min) q_min = q_bc;
        ll q_max = l * p / k;
        ll q_cn = (n_half + k * p) / l;
        if (q_cn < q_max) q_max = q_cn;
        if (q_min > q_max) { if (q_cn < q_bc) break; continue; }
        count += q_max - q_min + 1;
    }
    return count;
}

ll count_pq_case3(ll k, ll l, ll n) {
    ll count = 0;
    ll kl_sum = k+l, lk_diff = l-k, kl2 = 2*k+l;
    for (ll v = 1; ; v++) {
        ll u_min = v + 1;
        if (l - v > u_min) u_min = l - v;
        ll u_from_y = ceil_div(kl_sum * v, lk_diff);
        if (u_from_y > u_min) u_min = u_from_y;
        ll u_max = kl2 * v / k;
        ll num = n - kl_sum * v;
        if (num < 0) break;
        ll u_cn = num / lk_diff;
        if (u_cn < u_max) u_max = u_cn;
        if (u_min <= u_max) count += u_max - u_min + 1;
    }
    return count;
}

ll count_kl_case1(ll p, ll q, ll n, ll L) {
    ll l_base = (q > L) ? q : L;
    ll count = 0;
    ll p3q = 3*p-q, qp = q-p;
    for (ll k = 1; ; k++) {
        ll l_min = l_base + 1;
        if (k + 1 > l_min) l_min = k + 1;
        ll l_from_y = ceil_div(q * k, p);
        if (l_from_y > l_min) l_min = l_from_y;
        if (p3q > 0) {
            ll l_from_bc = ceil_div(p3q * k, qp);
            if (l_from_bc > l_min) l_min = l_from_bc;
        }
        ll l_max = (n + p * k) / q;
        if (l_min > l_max) break;
        count += l_max - l_min + 1;
    }
    return count;
}

ll count_kl_case2(ll p, ll q, ll n, ll L) {
    ll q_half = q / 2, L_half = L / 2;
    ll l_base = (q_half > L_half) ? q_half : L_half;
    ll n_half = n / 2;
    ll count = 0;
    ll p3q = 3*p-q, qp = q-p;
    for (ll k = 1; ; k++) {
        ll l_min = l_base + 1;
        if (k + 1 > l_min) l_min = k + 1;
        ll l_from_y = ceil_div(q * k, p);
        if (l_from_y > l_min) l_min = l_from_y;
        if (p3q > 0) {
            ll l_from_bc = ceil_div(p3q * k, qp);
            if (l_from_bc > l_min) l_min = l_from_bc;
        }
        ll l_max = (n_half + p * k) / q;
        if (l_min > l_max) break;
        count += l_max - l_min + 1;
    }
    return count;
}

ll count_kl_case3(ll p, ll q, ll n, ll L) {
    ll count = count_kl_case2(p, q, n, L);
    ll b_min_q = (q + 1) / 2, b_min_L = (L + 1) / 2;
    ll b_base = (b_min_q > b_min_L) ? b_min_q : b_min_L;
    ll n_adj = n - q + p;
    if (n_adj < 0) return count;
    ll p3q = 3*p-q, qp = q-p, p2 = 2*p, q2 = 2*q;
    for (ll a = 0; ; a++) {
        ll b_min = b_base;
        if (a + 1 > b_min) b_min = a + 1;
        ll b_from_y_num = q2 * a + q - p;
        if (b_from_y_num > 0) {
            ll b_from_y = ceil_div(b_from_y_num, p2);
            if (b_from_y > b_min) b_min = b_from_y;
        }
        if (p3q > 0) {
            ll b_from_bc_num = p3q * a + p2 - q;
            if (b_from_bc_num > 0) {
                ll b_from_bc = ceil_div(b_from_bc_num, qp);
                if (b_from_bc > b_min) b_min = b_from_bc;
            }
        }
        ll b_max_num = p2 * a + n_adj;
        if (b_max_num < 0) break;
        ll b_max = b_max_num / q2;
        if (b_min <= b_max) count += b_max - b_min + 1;
        if (b_max < b_base) break;
    }
    return count;
}

ll f(ll n, int checkParity) {
    ll L = (ll)sqrtl(1.5 * n);
    ll result = 0;
    for (ll l = 1; l <= L; l++)
        for (ll k = 1; k < l; k++)
            if (!checkParity || ((k%2==0) && (l%2==0)))
                result += count_pq_case1(k, l, n);
            else if ((k%2==0) || (l%2==0))
                result += count_pq_case2(k, l, n);
            else
                result += count_pq_case3(k, l, n);
    for (ll q = 1; q <= L; q++)
        for (ll p = 1; p < q; p++)
            if (!checkParity || ((p%2==0) && (q%2==0)))
                result += count_kl_case1(p, q, n, L);
            else if ((p%2==0) || (q%2==0))
                result += count_kl_case2(p, q, n, L);
            else
                result += count_kl_case3(p, q, n, L);
    return result;
}

int main() {
    compute_mobius(N);
    ll ans = 0;
    for (int g = 1; g <= N; g++)
        if (mobius[g] != 0)
            ans += mobius[g] * f(N / g, g % 2 == 1);
    printf("%lld\n", ans);
    free(mobius);
    return 0;
}
