/*
 * Project Euler Problem 541: Divisibility of Harmonic Numbers.
 *
 * Find the largest n such that the nth harmonic number H_n has a denominator
 * not divisible by P=137 when reduced to lowest terms.
 *
 * Uses p-adic approach: H_n mod P^e, with Lagrange interpolation for
 * sum of powers, and BFS over digits base P.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 i128;

#define P 137

/* Modular exponentiation */
static ll pow_mod(ll base, ll exp, ll mod) {
    if (mod <= 0) return 0;
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* Extended GCD for modular inverse */
static ll mod_inv(ll a, ll m) {
    if (m == 1) return 0;
    ll t = 0, new_t = 1;
    ll r = m, new_r = ((a % m) + m) % m;
    while (new_r != 0) {
        ll q = r / new_r;
        ll tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;
        tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }
    if (t < 0) t += m;
    return t;
}

static ll mod_pos(ll a, ll m) {
    return ((a % m) + m) % m;
}

/* Integer power */
static ll pow_int(ll base, int exp) {
    ll result = 1;
    for (int i = 0; i < exp; i++) result *= base;
    return result;
}

/* Sum of powers: sum_{j=1}^{limit} j^exp mod mod_val
 * Uses Lagrange interpolation at exp+2 points
 */
static ll sum_powers(ll limit, int exp, ll mod_val) {
    if (limit == 0) return 0;

    int n = exp + 2;  /* number of sample points needed */
    ll sum_pows = 0;
    ll result = 0;

    for (int j = 1; j <= n; j++) {
        sum_pows = mod_pos(sum_pows + pow_mod(j, exp, mod_val), mod_val);
        ll res = sum_pows;
        if ((exp + j) % 2 != 0)
            res = mod_pos(-res, mod_val);

        /* Compute denominator = product of |j - k| for k in 1..n, k!=j */
        ll denom = 1;
        for (int k = 1; k < j; k++)
            denom = (i128)denom * k % mod_val;
        for (int k = 1; k <= n - j; k++)
            denom = (i128)denom * k % mod_val;

        ll inv_denom = mod_inv(denom, mod_val);
        res = (i128)res * inv_denom % mod_val;

        /* Multiply by product of (limit - k) for k in 1..n, k!=j */
        for (int k = 1; k <= n; k++) {
            if (k != j) {
                res = (i128)res * mod_pos(limit - k, mod_val) % mod_val;
            }
        }
        result = mod_pos(result + res, mod_val);
    }
    return result;
}

/* Hash table for H cache */
#define HT_SIZE (1 << 18)
#define HT_MASK (HT_SIZE - 1)

typedef struct {
    ll n;
    int e;
    ll val;
    int used;
} HEntry;

static HEntry htable[1 << 18];

static void ht_init(void) {
    memset(htable, 0, sizeof(htable));
}

static unsigned int ht_hash(ll n, int e) {
    ull h = (ull)n * 2654435761ULL ^ (ull)e * 40503ULL;
    return (unsigned int)(h & HT_MASK);
}

static int ht_get(ll n, int e, ll *val) {
    unsigned int idx = ht_hash(n, e);
    for (int i = 0; i < HT_SIZE; i++) {
        unsigned int pos = (idx + i) & HT_MASK;
        if (!htable[pos].used) return 0;
        if (htable[pos].n == n && htable[pos].e == e) {
            *val = htable[pos].val;
            return 1;
        }
    }
    return 0;
}

static void ht_set(ll n, int e, ll val) {
    unsigned int idx = ht_hash(n, e);
    for (int i = 0; i < HT_SIZE; i++) {
        unsigned int pos = (idx + i) & HT_MASK;
        if (!htable[pos].used || (htable[pos].n == n && htable[pos].e == e)) {
            htable[pos].n = n;
            htable[pos].e = e;
            htable[pos].val = val;
            htable[pos].used = 1;
            return;
        }
    }
}

/* Compute H_n mod P^e. Returns -1 if H_n is not a p-adic integer. */
static ll H(ll n, int e) {
    if (n == 0) return 0;

    ll cached;
    if (ht_get(n, e, &cached)) return cached;

    ll H_val = H(n / P, e + 1);
    if (H_val == -1 || H_val % P != 0) {
        ht_set(n, e, -1);
        return -1;
    }

    H_val /= P;
    ll pe = pow_int(P, e);

    for (int r = 1; r < P; r++) {
        ll L = (n % P + r < P) ? n / P : n / P + 1;
        ll r_inv = mod_inv(r, pe);
        for (int k = 0; k < e; k++) {
            ll term = (i128)r_inv * pow_mod((i128)P * r_inv % pe, k, pe) % pe;
            term = (i128)term * sum_powers(L, k, pe) % pe;
            H_val = mod_pos(H_val - term, pe);
        }
    }

    ll result = mod_pos(H_val, pe);
    ht_set(n, e, result);
    return result;
}

/* BFS queue */
#define QMAX 200000
static ll queue[QMAX];

int main(void) {
    ht_init();

    int head = 0, tail = 0;
    queue[tail++] = 0;
    ll ans = 0;

    while (head < tail) {
        ll n = queue[head++];
        if (H(n * P, 0) == 0) {
            int start = (n == 0) ? 1 : 0;
            for (int i = start; i < P; i++) {
                if (tail < QMAX)
                    queue[tail++] = n * P + i;
            }
        }
        ans = n;
    }

    printf("%lld\n", ans);
    return 0;
}
