/*
 * Project Euler 641 - A Long Row of Dice
 * Count numbers <= 10^18 with exactly 6k divisors.
 * Uses Lucy hedgehog prime counting, recursive helper with prime enumeration.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define K 6

typedef long long ll;
typedef unsigned long long ull;

static ll N_VAL;

/* Lucy hedgehog prime counting */
static ll *V;
static ll *S;
static int V_len;

static int val_to_idx(ll v) {
    if (v <= 0) return -1;
    if (v <= V[V_len - 1]) {
        return V_len - (int)v;
    } else {
        int k = (int)(N_VAL / v);
        return k - 1;
    }
}

static void prime_count_init(ll n) {
    N_VAL = n;
    int r = (int)sqrt((double)n);
    while ((ll)(r+1)*(r+1) <= n) r++;
    while ((ll)r*r > n) r--;

    V_len = 0;
    int cap = 2 * r + 2;
    V = (ll *)malloc(cap * sizeof(ll));
    S = (ll *)malloc(cap * sizeof(ll));

    for (int i = 1; i <= r; i++) {
        V[V_len] = n / i;
        V_len++;
    }
    ll last = V[V_len - 1];
    for (ll v = last - 1; v >= 1; v--) {
        V[V_len] = v;
        V_len++;
    }

    for (int i = 0; i < V_len; i++) {
        S[i] = V[i] - 1;
    }

    for (int p = 2; p <= r; p++) {
        int pidx = val_to_idx(p);
        int pm1idx = val_to_idx(p - 1);
        if (S[pidx] <= S[pm1idx]) continue;
        ll sp = S[pm1idx];
        ll p2 = (ll)p * p;
        for (int i = 0; i < V_len; i++) {
            if (V[i] < p2) break;
            int vi_p_idx = val_to_idx(V[i] / p);
            S[i] -= S[vi_p_idx] - sp;
        }
    }
}

static ll get_prime_count(ll v) {
    if (v < 2) return 0;
    return S[val_to_idx(v)];
}

static int *primes;
static int nprimes;
static int *prime_counts_arr;
static int L;

static void sieve_small(int limit) {
    L = limit;
    char *is_p = (char *)calloc(limit + 1, 1);
    memset(is_p, 1, limit + 1);
    is_p[0] = is_p[1] = 0;
    int sq = (int)sqrt((double)limit);
    for (int i = 2; i <= sq; i++)
        if (is_p[i])
            for (int j = i*i; j <= limit; j += i)
                is_p[j] = 0;

    nprimes = 0;
    for (int i = 2; i <= limit; i++)
        if (is_p[i]) nprimes++;
    primes = (int *)malloc(nprimes * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= limit; i++)
        if (is_p[i]) primes[idx++] = i;

    prime_counts_arr = (int *)calloc(limit + 1, sizeof(int));
    int cnt = 0;
    for (int i = 0; i <= limit; i++) {
        if (i >= 2 && is_p[i]) cnt++;
        prime_counts_arr[i] = cnt;
    }
    free(is_p);
}

static ll nthrt(ll n, int r) {
    if (r == 1) return n;
    if (n <= 0) return 0;
    if (r == 2) {
        ll x = (ll)sqrt((double)n);
        while ((x+1)*(x+1) <= n) x++;
        while (x > 0 && x*x > n) x--;
        return x;
    }
    double dr = 1.0 / r;
    ll x = (ll)pow((double)n, dr);
    for (ll t = x + 2; t >= 0 && t >= x - 2; t--) {
        ll pw = 1;
        int ok = 1;
        for (int i = 0; i < r; i++) {
            if (pw > n / t + 1) { ok = 0; break; }
            pw *= t;
            if (pw > n) { ok = 0; break; }
        }
        if (ok && pw <= n) {
            ll pw2 = 1;
            int over = 0;
            for (int i = 0; i < r; i++) {
                if (pw2 > n / (t+1) + 1) { over = 1; break; }
                pw2 *= (t+1);
                if (pw2 > n) { over = 1; break; }
            }
            if (over) return t;
        }
    }
    return x;
}

static ll safe_pow(ll p, int e) {
    ll result = 1;
    for (int i = 0; i < e; i++) {
        if (result > 1000000000000000000LL / p) return -1;
        result *= p;
    }
    return result;
}

static ll ans;
static ll N_GLOBAL;

static void helper(int min_index, ll n, int num_divisors) {
    int e = (num_divisors % K == 1) ? K : K - 2;
    while (1) {
        int half_e = e / 2;
        ll bound = nthrt(N_GLOBAL / n, half_e);
        if (min_index >= nprimes || bound < primes[min_index])
            break;
        ll cnt;
        if (bound > L)
            cnt = get_prime_count(bound) - min_index;
        else
            cnt = prime_counts_arr[(int)bound] - min_index;
        ans += cnt;
        e += K;
    }

    for (int index = min_index; index < nprimes; index++) {
        ll p = primes[index];
        ll pk2 = safe_pow(p, K - 2);
        if (pk2 < 0 || n > N_GLOBAL / pk2)
            break;

        for (int start_e_idx = 0; start_e_idx < 2; start_e_idx++) {
            int se = (start_e_idx == 0) ? K - 2 : K;
            int ee = se;
            int half_se = ee / 2;
            ll ppow = safe_pow(p, half_se);
            if (ppow < 0 || n > N_GLOBAL / ppow) continue;
            ll new_n = n * ppow;
            while (new_n < N_GLOBAL) {
                helper(index + 1, new_n, num_divisors * (ee + 1));
                ee += K;
                ll pmul = safe_pow(p, K / 2);
                if (pmul < 0 || new_n > N_GLOBAL / pmul) break;
                new_n *= pmul;
            }
        }
    }
}

int main(void) {
    N_GLOBAL = 1000000000000000000LL; /* 10^18 */
    int L_val = (int)pow((double)N_GLOBAL, 0.4);
    while (safe_pow((ll)(L_val+1), 5) > 0 && safe_pow((ll)(L_val+1), 5) <= N_GLOBAL) L_val++;

    sieve_small(L_val);
    prime_count_init(N_GLOBAL);

    ans = 1;

    helper(0, 1, 1);
    printf("%lld\n", ans);

    free(V);
    free(S);
    free(primes);
    free(prime_counts_arr);
    return 0;
}
