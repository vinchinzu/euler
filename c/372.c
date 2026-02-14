/*
 * Project Euler Problem 372: Pencils of Rays
 *
 * R(M, N) = number of lattice points (x, y) with M < x <= N, M < y <= N
 * where floor(y^2 / x^2) is odd.
 *
 * Find R(2*10^6, 10^9).
 *
 * Algorithm: For each odd k, count lattice points (x,y) with floor(y^2/x^2) = k,
 * meaning k*x^2 <= y^2 < (k+1)*x^2, i.e., x*sqrt(k) <= y < x*sqrt(k+1).
 * Sum contributions using floor-sum-of-sqrt via continued fraction recursion.
 */

#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

static ll isqrt_ll(ll n) {
    if (n <= 0) return 0;
    ll x = (ll)sqrt((double)n);
    while (x > 0 && x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return x;
}

/* Compute floor(n * (sqrt(d) + P) / Q) exactly */
static ll exact_floor_n_alpha(ll n, ll d, ll P, ll Q) {
    if (n == 0) return 0;
    lll nd = (lll)n * n * d;
    ll S = isqrt_ll((ll)((double)n * sqrt((double)d)));
    /* Refine S so that S^2 <= n^2*d < (S+1)^2 */
    while ((lll)S * S > nd) S--;
    while ((lll)(S + 1) * (S + 1) <= nd) S++;

    lll total_int = (lll)n * P + S;
    ll q_div = (ll)(total_int / Q);
    lll r = total_int - (lll)q_div * Q;
    lll need = Q - r;
    if (need <= 0) return q_div + 1;
    lll rhs = S + need;
    if (nd >= rhs * rhs) return q_div + 1;
    return q_div;
}

/* Compute sum_{x=1}^n floor(x * sqrt(d)) using continued fraction recursion */
static ll sum_floor(ll n, ll d) {
    if (n <= 0) return 0;
    ll s = isqrt_ll(d);
    if (s * s == d) return s * n * (n + 1) / 2;

    ll base = s * n * (n + 1) / 2;
    ll P_cur = -s, Q_cur = 1;
    ll n_cur = n;
    int sign = 1;
    ll result = base;

    while (n_cur > 0) {
        ll m = exact_floor_n_alpha(n_cur, d, P_cur, Q_cur);
        if (m == 0) break;

        lll denom = (lll)d - (lll)P_cur * P_cur;
        ll Q_new = (ll)(denom / Q_cur);
        ll P_inv = -P_cur;

        ll a = exact_floor_n_alpha(1, d, P_inv, Q_new);

        if (sign > 0)
            result += n_cur * m - a * m * (m + 1) / 2;
        else
            result -= n_cur * m - a * m * (m + 1) / 2;

        P_cur = P_inv - a * Q_new;
        Q_cur = Q_new;
        n_cur = m;
        sign = -sign;
    }

    return result;
}

static ll floor_div_sqrt(ll num, ll den) {
    if (den == 0 || num <= 0) return 0;
    lll num2 = (lll)num * num;
    lll z2_den = num2 / den;
    ll z = isqrt_ll((ll)((double)num / sqrt((double)den)));
    while ((lll)(z + 1) * (z + 1) * den <= num2) z++;
    while ((lll)z * z * den > num2) z--;
    return z;
}

static ll sum_upper(ll ll_v, ll rr, ll N_val, ll d) {
    if (ll_v > rr) return 0;
    ll s = isqrt_ll(d);
    if (s * s == d) {
        ll x_split = (N_val + s) / s;
        ll res = 0;
        ll left_end = rr < x_split - 1 ? rr : x_split - 1;
        if (ll_v <= left_end) {
            ll cnt = left_end - ll_v + 1;
            ll sum_x = cnt * (ll_v + left_end) / 2;
            res += s * sum_x - cnt;
        }
        ll right_start = ll_v > x_split ? ll_v : x_split;
        if (right_start <= rr) {
            ll cnt = rr - right_start + 1;
            res += N_val * cnt;
        }
        return res;
    } else {
        ll x_split = floor_div_sqrt(N_val, d) + 1;
        ll left_end = rr < x_split - 1 ? rr : x_split - 1;
        ll res = 0;
        if (ll_v <= left_end) {
            res += sum_floor(left_end, d) - sum_floor(ll_v - 1, d);
        }
        ll right_start = ll_v > x_split ? ll_v : x_split;
        if (right_start <= rr) {
            ll cnt = rr - right_start + 1;
            res += N_val * cnt;
        }
        return res;
    }
}

static ll sum_lower_max(ll ll_v, ll rr, ll L_val, ll d) {
    if (ll_v > rr) return 0;
    ll s = isqrt_ll(d);
    if (s * s == d) {
        ll x_split = (L_val + s - 1) / s;
        ll res = 0;
        ll left_end = rr < x_split - 1 ? rr : x_split - 1;
        if (ll_v <= left_end) {
            ll cnt = left_end - ll_v + 1;
            res += L_val * cnt;
        }
        ll right_start = ll_v > x_split ? ll_v : x_split;
        if (right_start <= rr) {
            ll cnt = rr - right_start + 1;
            ll sum_x = cnt * (right_start + rr) / 2;
            res += s * sum_x;
        }
        return res;
    } else {
        ll x_split;
        if (L_val <= 1) {
            x_split = 1;
        } else {
            x_split = floor_div_sqrt(L_val - 1, d) + 1;
        }
        ll res = 0;
        ll left_end = rr < x_split - 1 ? rr : x_split - 1;
        if (ll_v <= left_end) {
            ll cnt = left_end - ll_v + 1;
            res += L_val * cnt;
        }
        ll right_start = ll_v > x_split ? ll_v : x_split;
        if (right_start <= rr) {
            ll cnt = rr - right_start + 1;
            ll sf = sum_floor(rr, d) - sum_floor(right_start - 1, d);
            res += sf + cnt;
        }
        return res;
    }
}

static ll compute_R(ll M, ll N_val) {
    ll L_val = M + 1;
    if (L_val > N_val) return 0;
    ll total = 0;
    ll max_ratio = N_val / L_val + 1;
    ll max_k = 2 * max_ratio * max_ratio;
    ll k = 1;
    while (k <= max_k) {
        ll x_lo = floor_div_sqrt(L_val - 1, k + 1) + 1;
        ll x_start = L_val > x_lo ? L_val : x_lo;
        ll x_end_bound = floor_div_sqrt(N_val, k);
        ll x_end = N_val < x_end_bound ? N_val : x_end_bound;

        if (x_start > x_end) {
            k += 2;
            continue;
        }

        ll su = sum_upper(x_start, x_end, N_val, k + 1);
        ll sl = sum_lower_max(x_start, x_end, L_val, k);
        ll num_x = x_end - x_start + 1;
        ll contrib = su - sl + num_x;
        if (contrib > 0) total += contrib;
        k += 2;
    }
    return total;
}

int main(void) {
    printf("%lld\n", compute_R(2000000, 1000000000));
    return 0;
}
