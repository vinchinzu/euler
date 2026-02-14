/*
 * Project Euler Problem 551: Sum of digits sequence.
 * Find a_N if a_0 = 1 and a_n = a_{n-1} + sumDigits(a_{n-1}).
 * N = 10^15.
 *
 * Uses memoized "jump" approach: for a given (r, m, sum_q),
 * precompute how many steps and how much a increases before
 * r + da >= m. This allows logarithmic-depth recursion.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

#define B 10

/* Hash table for jump cache */
#define HT_SIZE (1 << 20)
#define HT_MASK (HT_SIZE - 1)

typedef struct {
    ll r;
    ll m;
    int sum_q;
    ll di;
    ll da;
    int used;
} JEntry;

static JEntry jcache[1 << 20];

static unsigned int j_hash(ll r, ll m, int sum_q) {
    unsigned long long h = (unsigned long long)r * 2654435761ULL;
    h ^= (unsigned long long)m * 40503ULL;
    h ^= (unsigned long long)sum_q * 2246822519ULL;
    return (unsigned int)(h & HT_MASK);
}

static int j_get(ll r, ll m, int sum_q, ll *di, ll *da) {
    unsigned int idx = j_hash(r, m, sum_q);
    for (int i = 0; i < 512; i++) {
        unsigned int pos = (idx + i) & HT_MASK;
        if (!jcache[pos].used) return 0;
        if (jcache[pos].r == r && jcache[pos].m == m && jcache[pos].sum_q == sum_q) {
            *di = jcache[pos].di;
            *da = jcache[pos].da;
            return 1;
        }
    }
    return 0;
}

static void j_set(ll r, ll m, int sum_q, ll di, ll da) {
    unsigned int idx = j_hash(r, m, sum_q);
    for (int i = 0; i < 512; i++) {
        unsigned int pos = (idx + i) & HT_MASK;
        if (!jcache[pos].used || (jcache[pos].r == r && jcache[pos].m == m && jcache[pos].sum_q == sum_q)) {
            jcache[pos].r = r;
            jcache[pos].m = m;
            jcache[pos].sum_q = sum_q;
            jcache[pos].di = di;
            jcache[pos].da = da;
            jcache[pos].used = 1;
            return;
        }
    }
}

static int sum_digits(ll n) {
    int s = 0;
    while (n > 0) { s += (int)(n % 10); n /= 10; }
    return s;
}

/* B^3 = 1000 */
#define CB_B 1000

static void get_jump(ll r, ll m, int sum_q, ll *out_di, ll *out_da) {
    ll di_c, da_c;
    if (j_get(r, m, sum_q, &di_c, &da_c)) {
        *out_di = di_c;
        *out_da = da_c;
        return;
    }

    ll di = 0, da = 0;
    while (r + da < m) {
        if (m <= CB_B) {
            di++;
            da += sum_q + sum_digits(r + da);
        } else {
            ll sub_r = (r + da) % (m / B);
            int sub_sq = sum_q + sum_digits((r + da) / (m / B));
            ll sub_di, sub_da;
            get_jump(sub_r, m / B, sub_sq, &sub_di, &sub_da);
            di += sub_di;
            da += sub_da;
        }
    }

    j_set(r, m, sum_q, di, da);
    *out_di = di;
    *out_da = da;
}

int main(void) {
    ll N_val = 1000000000000000LL; /* 10^15 */
    memset(jcache, 0, sizeof(jcache));

    ll di = 0;
    ll ans = 1;
    ll m = N_val;

    while (m >= 1) {
        while (1) {
            ll j_di, j_da;
            get_jump(ans % m, m, sum_digits(ans / m), &j_di, &j_da);
            if (di + j_di >= N_val) break;
            di += j_di;
            ans += j_da;
        }
        m /= B;
    }

    printf("%lld\n", ans);
    return 0;
}
