/*
 * Project Euler 428 - Necklace of Circles
 *
 * T(N) = S3 + S4 + S6 where each Si counts necklace triplets for k=3,4,6.
 * Uses: Mertens function, quotient grouping, Lucy DP for pi_1,
 * DFS over 1mod3-smooth numbers.
 * Translated from python/428.py.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;

#define NVAL 1000000000LL

static int isqrt_ll(ll n) {
    ll x = (ll)sqrtl((long double)n);
    while (x * x > n) x--;
    while ((x+1)*(x+1) <= n) x++;
    return (int)x;
}

/* Sieve limit and arrays */
static int sieve_limit;
static signed char *mu_arr;
static ll *mu_prefix;
static int *primes_all;
static int n_primes_all;

/* Mertens cache using hash map */
#define HASH_SIZE (1 << 20)
#define HASH_MASK (HASH_SIZE - 1)
typedef struct { ll key; ll val; int used; } HEntry;
static HEntry mertens_ht[HASH_SIZE];

static void sieve_mu(int limit) {
    sieve_limit = limit;
    mu_arr = (signed char *)calloc(limit + 1, sizeof(signed char));
    mu_arr[1] = 1;
    char *is_comp = (char *)calloc(limit + 1, 1);

    primes_all = (int *)malloc((limit / 5 + 100) * sizeof(int));
    n_primes_all = 0;

    for (int i = 2; i <= limit; i++) {
        if (!is_comp[i]) {
            primes_all[n_primes_all++] = i;
            mu_arr[i] = -1;
        }
        for (int j = 0; j < n_primes_all; j++) {
            ll v = (ll)primes_all[j] * i;
            if (v > limit) break;
            is_comp[(int)v] = 1;
            if (i % primes_all[j] == 0) {
                mu_arr[(int)v] = 0;
                break;
            }
            mu_arr[(int)v] = -mu_arr[i];
        }
    }
    free(is_comp);

    mu_prefix = (ll *)malloc((limit + 1) * sizeof(ll));
    mu_prefix[0] = 0;
    for (int i = 1; i <= limit; i++)
        mu_prefix[i] = mu_prefix[i-1] + mu_arr[i];
}

static ll mertens(ll n) {
    if (n <= sieve_limit) return mu_prefix[n];
    int h = (int)((n * 0x9E3779B97F4A7C15ULL) >> 44) & HASH_MASK;
    for (int i = 0; i < HASH_SIZE; i++) {
        int idx = (h + i) & HASH_MASK;
        if (!mertens_ht[idx].used) break;
        if (mertens_ht[idx].key == n) return mertens_ht[idx].val;
    }
    ll s = 0;
    ll d = 2;
    while (d <= n) {
        ll q = n / d;
        ll d_max = n / q;
        s += (d_max - d + 1) * mertens(q);
        d = d_max + 1;
    }
    ll result = 1 - s;
    /* Store in hash */
    h = (int)((n * 0x9E3779B97F4A7C15ULL) >> 44) & HASH_MASK;
    for (int i = 0; i < HASH_SIZE; i++) {
        int idx = (h + i) & HASH_MASK;
        if (!mertens_ht[idx].used) {
            mertens_ht[idx].key = n;
            mertens_ht[idx].val = result;
            mertens_ht[idx].used = 1;
            break;
        }
    }
    return result;
}

/* Q(X) = #{squarefree n <= X} */
static ll Q_val(ll X) {
    if (X <= 0) return 0;
    ll s = 0;
    int sq = isqrt_ll(X);
    for (int k = 1; k <= sq; k++) {
        if (mu_arr[k] != 0)
            s += mu_arr[k] * (X / ((ll)k * k));
    }
    return s;
}

/* F(X) = sum_{d=1}^X 2^omega(d), via hyperbola */
#define F_HASH_SIZE (1 << 19)
#define F_HASH_MASK (F_HASH_SIZE - 1)
typedef struct { ll key; ll val; int used; } FEntry;
static FEntry F_ht[F_HASH_SIZE];

static ll F_val(ll X) {
    if (X <= 0) return 0;
    int h = (int)((X * 0x9E3779B97F4A7C15ULL) >> 45) & F_HASH_MASK;
    for (int i = 0; i < F_HASH_SIZE; i++) {
        int idx = (h + i) & F_HASH_MASK;
        if (!F_ht[idx].used) break;
        if (F_ht[idx].key == X) return F_ht[idx].val;
    }
    int sX = isqrt_ll(X);
    ll result = 0;
    for (int d = 1; d <= sX; d++) {
        if (mu_arr[d] != 0)
            result += X / d;
    }
    ll max_q = X / (sX + 1);
    for (ll q = 1; q <= max_q; q++) {
        result += q * (Q_val(X / q) - Q_val(X / (q + 1)));
    }
    h = (int)((X * 0x9E3779B97F4A7C15ULL) >> 45) & F_HASH_MASK;
    for (int i = 0; i < F_HASH_SIZE; i++) {
        int idx = (h + i) & F_HASH_MASK;
        if (!F_ht[idx].used) {
            F_ht[idx].key = X;
            F_ht[idx].val = result;
            F_ht[idx].used = 1;
            break;
        }
    }
    return result;
}

/* T(X) = sum_{b=1}^X tau(b^2) */
#define T_HASH_SIZE (1 << 19)
#define T_HASH_MASK (T_HASH_SIZE - 1)
static FEntry T_ht[T_HASH_SIZE];

static ll T_c(ll X) {
    if (X <= 0) return 0;
    int h = (int)((X * 0x517CC1B727220A95ULL) >> 45) & T_HASH_MASK;
    for (int i = 0; i < T_HASH_SIZE; i++) {
        int idx = (h + i) & T_HASH_MASK;
        if (!T_ht[idx].used) break;
        if (T_ht[idx].key == X) return T_ht[idx].val;
    }
    ll result = 0;
    ll d = 1;
    while (d <= X) {
        ll q = X / d;
        ll d_max = X / q;
        result += q * (F_val(d_max) - F_val(d - 1));
        d = d_max + 1;
    }
    h = (int)((X * 0x517CC1B727220A95ULL) >> 45) & T_HASH_MASK;
    for (int i = 0; i < T_HASH_SIZE; i++) {
        int idx = (h + i) & T_HASH_MASK;
        if (!T_ht[idx].used) {
            T_ht[idx].key = X;
            T_ht[idx].val = result;
            T_ht[idx].used = 1;
            break;
        }
    }
    return result;
}

/* T_odd(X) = sum_{m odd, m<=X} tau(m^2) */
#define TO_HASH_SIZE (1 << 18)
#define TO_HASH_MASK (TO_HASH_SIZE - 1)
static FEntry TO_ht[TO_HASH_SIZE];

static ll T_odd(ll X) {
    if (X <= 0) return 0;
    int h = (int)((X * 0xA24BAED4963EE407ULL) >> 46) & TO_HASH_MASK;
    for (int i = 0; i < TO_HASH_SIZE; i++) {
        int idx = (h + i) & TO_HASH_MASK;
        if (!TO_ht[idx].used) break;
        if (TO_ht[idx].key == X) return TO_ht[idx].val;
    }
    ll result = T_c(X);
    int a = 1;
    ll pw = 2;
    while (pw <= X) {
        result -= (2 * a + 1) * T_odd(X / pw);
        a++;
        pw *= 2;
    }
    h = (int)((X * 0xA24BAED4963EE407ULL) >> 46) & TO_HASH_MASK;
    for (int i = 0; i < TO_HASH_SIZE; i++) {
        int idx = (h + i) & TO_HASH_MASK;
        if (!TO_ht[idx].used) {
            TO_ht[idx].key = X;
            TO_ht[idx].val = result;
            TO_ht[idx].used = 1;
            break;
        }
    }
    return result;
}

/* T_on3(X) = sum_{m odd, gcd(m,3)=1, m<=X} tau(m^2) */
#define TON3_HASH_SIZE (1 << 18)
#define TON3_HASH_MASK (TON3_HASH_SIZE - 1)
static FEntry TON3_ht[TON3_HASH_SIZE];

static ll T_on3(ll X) {
    if (X <= 0) return 0;
    int h = (int)((X * 0xC96C5795D7870F42ULL) >> 46) & TON3_HASH_MASK;
    for (int i = 0; i < TON3_HASH_SIZE; i++) {
        int idx = (h + i) & TON3_HASH_MASK;
        if (!TON3_ht[idx].used) break;
        if (TON3_ht[idx].key == X) return TON3_ht[idx].val;
    }
    ll result = T_odd(X);
    int c = 1;
    ll pw = 3;
    while (pw <= X) {
        result -= (2 * c + 1) * T_on3(X / pw);
        c++;
        pw *= 3;
    }
    h = (int)((X * 0xC96C5795D7870F42ULL) >> 46) & TON3_HASH_MASK;
    for (int i = 0; i < TON3_HASH_SIZE; i++) {
        int idx = (h + i) & TON3_HASH_MASK;
        if (!TON3_ht[idx].used) {
            TON3_ht[idx].key = X;
            TON3_ht[idx].val = result;
            TON3_ht[idx].used = 1;
            break;
        }
    }
    return result;
}

/* L(X) = sum_{j=1}^{isqrt(X)} mertens(X / j^2) */
#define L_HASH_SIZE (1 << 17)
#define L_HASH_MASK (L_HASH_SIZE - 1)
static FEntry L_ht[L_HASH_SIZE];

static ll L_val(ll X) {
    if (X <= 0) return 0;
    int h = (int)((X * 0xD1B54A32D192ED03ULL) >> 47) & L_HASH_MASK;
    for (int i = 0; i < L_HASH_SIZE; i++) {
        int idx = (h + i) & L_HASH_MASK;
        if (!L_ht[idx].used) break;
        if (L_ht[idx].key == X) return L_ht[idx].val;
    }
    ll total = 0;
    int sq = isqrt_ll(X);
    for (int j = 1; j <= sq; j++)
        total += mertens(X / ((ll)j * j));
    h = (int)((X * 0xD1B54A32D192ED03ULL) >> 47) & L_HASH_MASK;
    for (int i = 0; i < L_HASH_SIZE; i++) {
        int idx = (h + i) & L_HASH_MASK;
        if (!L_ht[idx].used) {
            L_ht[idx].key = X;
            L_ht[idx].val = total;
            L_ht[idx].used = 1;
            break;
        }
    }
    return total;
}

static ll L3(ll X) {
    if (X <= 0) return 0;
    return L_val(X) + L_val(X / 3);
}

/* Lucy DP arrays */
static ll *small_pi1, *big_pi1, *small_pi2, *big_pi2;
static int sqrtN_val;

static ll pi1(ll V) {
    if (V < 2) return 0;
    if (V <= sqrtN_val) return small_pi1[V];
    return big_pi1[NVAL / V];
}

/* DFS for S6_chi */
static ll sum_G;
static int *primes_1mod3;
static int n_primes_1mod3;

static void dfs_g(int idx, ll d_val, ll b_val, ll last_prime) {
    sum_G += b_val * L3(NVAL / d_val);

    ll upper_p = NVAL / d_val;
    ll lower_p = last_prime > sqrtN_val ? last_prime : sqrtN_val;

    if (upper_p > lower_p) {
        ll large_sum = 0;
        ll p = lower_p + 1;
        while (p <= upper_p) {
            ll q = NVAL / (d_val * p);
            ll p_range_hi;
            if (q > 0)
                p_range_hi = upper_p < NVAL / (d_val * q) ? upper_p : NVAL / (d_val * q);
            else
                p_range_hi = upper_p;

            ll p_range_lo;
            if (q < upper_p)
                p_range_lo = lower_p + 1 > NVAL / (d_val * (q + 1)) + 1 ? lower_p + 1 : NVAL / (d_val * (q + 1)) + 1;
            else
                p_range_lo = lower_p + 1;

            ll cnt = pi1(p_range_hi) - pi1(p_range_lo - 1);
            if (cnt > 0)
                large_sum += cnt * L3(q);

            p = p_range_hi + 1;
        }
        sum_G += 4 * b_val * large_sum;
    }

    for (int i = idx; i < n_primes_1mod3; i++) {
        int pr = primes_1mod3[i];
        if (pr > sqrtN_val) break;
        if (d_val * pr > NVAL) break;
        ll pk = pr;
        int k = 1;
        while (d_val * pk <= NVAL) {
            dfs_g(i + 1, d_val * pk, b_val * (4 * k), pr);
            k++;
            pk *= pr;
        }
    }
}

int main(void) {
    ll N = NVAL;
    sqrtN_val = isqrt_ll(N);

    int cbrt = (int)round(pow((double)N, 1.0/3.0));
    while ((ll)(cbrt+1)*(cbrt+1)*(cbrt+1) <= N) cbrt++;
    while ((ll)cbrt*cbrt*cbrt > N) cbrt--;

    int sl = cbrt * cbrt;
    if (sl < sqrtN_val + 1) sl = sqrtN_val + 1;

    sieve_mu(sl);

    /* Pre-fill mertens cache */
    memset(mertens_ht, 0, sizeof(mertens_ht));
    {
        ll d = 1;
        while (d <= N) {
            mertens(N / d);
            d = N / (N / d) + 1;
        }
    }

    /* Precompute F for needed T arguments */
    memset(F_ht, 0, sizeof(F_ht));
    memset(T_ht, 0, sizeof(T_ht));
    memset(TO_ht, 0, sizeof(TO_ht));
    memset(TON3_ht, 0, sizeof(TON3_ht));
    memset(L_ht, 0, sizeof(L_ht));

    /* Precompute needed F values */
    ll needed_T[3000];
    int n_needed = 0;
    for (int a = 0; a < 61; a++) {
        ll pw2 = 1LL << a;
        if (pw2 > N) break;
        for (int c = 0; c < 40; c++) {
            ll pw3 = 1;
            for (int cc = 0; cc < c; cc++) pw3 *= 3;
            ll pw = pw2 * pw3;
            if (pw > N) break;
            needed_T[n_needed++] = N / pw;
        }
    }

    /* Sort and deduplicate */
    for (int i = 0; i < n_needed; i++)
        for (int j = i + 1; j < n_needed; j++)
            if (needed_T[i] > needed_T[j]) { ll t = needed_T[i]; needed_T[i] = needed_T[j]; needed_T[j] = t; }
    int unique = 0;
    for (int i = 0; i < n_needed; i++)
        if (i == 0 || needed_T[i] != needed_T[i-1])
            needed_T[unique++] = needed_T[i];
    n_needed = unique;

    /* Precompute F for quotient values of each needed_T */
    for (int ti = 0; ti < n_needed; ti++) {
        ll X = needed_T[ti];
        ll dd = 1;
        while (dd <= X) {
            ll qq = X / dd;
            F_val(qq);
            dd = X / qq + 1;
        }
    }

    /* S4 = sum_{b=1}^N tau(2*b^2) */
    ll S4 = 0;
    {
        int a = 0;
        ll pw = 1;
        while (pw <= N) {
            S4 += (2 * a + 2) * T_odd(N / pw);
            a++;
            pw *= 2;
        }
    }

    /* S3 = sum_{b=1}^N tau(12*b^2) */
    ll S3 = 0;
    {
        int a = 0;
        ll pw2 = 1;
        while (pw2 <= N) {
            int c = 0;
            ll pw3 = 1;
            while (pw2 * pw3 <= N) {
                S3 += (ll)(2 * a + 3) * (2 * c + 2) * T_on3(N / (pw2 * pw3));
                c++;
                pw3 *= 3;
            }
            a++;
            pw2 *= 2;
        }
    }

    /* S6_div3 */
    ll S6_div3 = 0;
    {
        int v = 1;
        ll pw3 = 3;
        while (pw3 <= N) {
            int a = 0;
            ll pw2 = 1;
            while (pw2 * pw3 <= N) {
                S6_div3 += (ll)(2 * v - 1) * (2 * a + 3) * T_on3(N / (pw2 * pw3));
                a++;
                pw2 *= 2;
            }
            v++;
            pw3 *= 3;
        }
    }

    /* S6_tau */
    ll S6_tau = 0;
    {
        int a = 0;
        ll pw = 1;
        while (pw <= N) {
            S6_tau += (2 * a + 3) * T_on3(N / pw);
            a++;
            pw *= 2;
        }
    }

    /* Lucy DP for pi_1 */
    small_pi1 = (ll *)calloc(sqrtN_val + 1, sizeof(ll));
    big_pi1 = (ll *)calloc(sqrtN_val + 2, sizeof(ll));
    small_pi2 = (ll *)calloc(sqrtN_val + 1, sizeof(ll));
    big_pi2 = (ll *)calloc(sqrtN_val + 2, sizeof(ll));

    for (int v = 1; v <= sqrtN_val; v++) {
        small_pi1[v] = (v + 2) / 3 - 1;
        small_pi2[v] = (v + 1) / 3;
    }
    for (int k = 1; k <= sqrtN_val; k++) {
        ll V = N / k;
        big_pi1[k] = (V + 2) / 3 - 1;
        big_pi2[k] = (V + 1) / 3;
    }

    /* Collect quotients descending */
    ll *quotients_desc = (ll *)malloc((2 * sqrtN_val + 10) * sizeof(ll));
    int n_quot = 0;
    {
        ll d = 1;
        while (d <= N) {
            quotients_desc[n_quot++] = N / d;
            d = N / (N / d) + 1;
        }
    }
    /* Sort descending */
    for (int i = 0; i < n_quot / 2; i++) {
        ll t = quotients_desc[i];
        quotients_desc[i] = quotients_desc[n_quot - 1 - i];
        quotients_desc[n_quot - 1 - i] = t;
    }

    /* Primes up to sqrtN for Lucy DP */
    int *primes_small = NULL;
    int n_primes_small = 0;
    for (int i = 0; i < n_primes_all; i++) {
        if (primes_all[i] <= sqrtN_val) n_primes_small++;
        else break;
    }
    primes_small = primes_all; /* first n_primes_small are <= sqrtN */

    for (int pi = 0; pi < n_primes_small; pi++) {
        int p = primes_small[pi];
        if (p == 3) continue;
        ll pp = (ll)p * p;
        ll p1 = small_pi1[p - 1];
        ll p2 = small_pi2[p - 1];

        for (int qi = 0; qi < n_quot; qi++) {
            ll V = quotients_desc[qi];
            if (V < pp) break;
            ll Vp = V / p;
            ll c1 = Vp <= sqrtN_val ? small_pi1[Vp] : big_pi1[N / Vp];
            ll c2 = Vp <= sqrtN_val ? small_pi2[Vp] : big_pi2[N / Vp];

            if (p % 3 == 1) {
                if (V <= sqrtN_val) {
                    small_pi1[V] -= c1 - p1;
                    small_pi2[V] -= c2 - p2;
                } else {
                    int k = (int)(N / V);
                    big_pi1[k] -= c1 - p1;
                    big_pi2[k] -= c2 - p2;
                }
            } else {
                if (V <= sqrtN_val) {
                    ll old1 = small_pi1[V];
                    ll old2 = small_pi2[V];
                    small_pi1[V] = old1 - (c2 - p2);
                    small_pi2[V] = old2 - (c1 - p1);
                } else {
                    int k = (int)(N / V);
                    ll old1 = big_pi1[k];
                    ll old2 = big_pi2[k];
                    big_pi1[k] = old1 - (c2 - p2);
                    big_pi2[k] = old2 - (c1 - p1);
                }
            }
        }
    }

    /* Collect primes 1 mod 3 up to sqrtN */
    primes_1mod3 = (int *)malloc((n_primes_all + 1) * sizeof(int));
    n_primes_1mod3 = 0;
    for (int i = 0; i < n_primes_all; i++)
        if (primes_all[i] % 3 == 1)
            primes_1mod3[n_primes_1mod3++] = primes_all[i];

    /* DFS */
    sum_G = 0;
    dfs_g(0, 1, 1, 0);

    ll S6_chi = -sum_G;
    ll S6 = S6_div3 + (S6_tau + S6_chi) / 2;

    printf("%lld\n", S3 + S4 + S6);

    free(mu_arr);
    free(mu_prefix);
    free(primes_all);
    free(small_pi1);
    free(big_pi1);
    free(small_pi2);
    free(big_pi2);
    free(quotients_desc);
    free(primes_1mod3);

    return 0;
}
