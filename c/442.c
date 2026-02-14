/*
 * Project Euler 442 - Eleven-free integers
 *
 * Find the N-th positive "eleven-free" integer (no power of 11 except 1
 * as a substring). Digit-by-digit search with memoization.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define K 11
#define B 10
#define MAX_DIGITS 20
#define MAX_POWERS 20

typedef long long ll;
typedef unsigned long long ull;

/* Powers of 11 as strings */
static char power_strs[MAX_POWERS][30];
static int power_lens[MAX_POWERS];
static int num_powers;

/* Hash table for memoization: key = (relevant_prefix_index, num_remaining_digits) */
/* We encode relevant prefix as an index into a table of known prefixes */
#define HASH_SIZE 1000003

typedef struct {
    int rp_len;       /* length of relevant prefix */
    char rp[20];      /* relevant prefix string */
    int nrd;          /* num_remaining_digits */
    ll val;
    int used;
} CacheEntry;

static CacheEntry cache[HASH_SIZE];

static ull hash_key(const char *rp, int rp_len, int nrd) {
    ull h = (ull)nrd * 31337;
    for (int i = 0; i < rp_len; i++)
        h = h * 131 + (unsigned char)rp[i];
    return h % HASH_SIZE;
}

static int cache_get(const char *rp, int rp_len, int nrd, ll *val) {
    ull h = hash_key(rp, rp_len, nrd);
    for (int i = 0; i < HASH_SIZE; i++) {
        ull idx = (h + i) % HASH_SIZE;
        if (!cache[idx].used) return 0;
        if (cache[idx].nrd == nrd && cache[idx].rp_len == rp_len &&
            memcmp(cache[idx].rp, rp, rp_len) == 0) {
            *val = cache[idx].val;
            return 1;
        }
    }
    return 0;
}

static void cache_set(const char *rp, int rp_len, int nrd, ll val) {
    ull h = hash_key(rp, rp_len, nrd);
    for (int i = 0; i < HASH_SIZE; i++) {
        ull idx = (h + i) % HASH_SIZE;
        if (!cache[idx].used) {
            cache[idx].rp_len = rp_len;
            memcpy(cache[idx].rp, rp, rp_len);
            cache[idx].nrd = nrd;
            cache[idx].val = val;
            cache[idx].used = 1;
            return;
        }
        if (cache[idx].nrd == nrd && cache[idx].rp_len == rp_len &&
            memcmp(cache[idx].rp, rp, rp_len) == 0) {
            cache[idx].val = val;
            return;
        }
    }
}

/* Check if string 'hay' of length 'hlen' ends with string 'needle' of length 'nlen' */
static int ends_with(const char *hay, int hlen, const char *needle, int nlen) {
    if (nlen > hlen) return 0;
    return memcmp(hay + hlen - nlen, needle, nlen) == 0;
}

static ll num_eleven_frees(const char *prefix, int prefix_len, int num_remaining) {
    /* Find relevant prefix */
    char relevant[30] = "";
    int rel_len = 0;

    for (int pi = 0; pi < num_powers; pi++) {
        /* Check if prefix contains this power as substring -> dead end */
        if (pi > 0 && power_lens[pi] <= prefix_len) {
            /* Check if prefix ends with this power string */
            /* Actually need to check ALL substrings, but the algorithm only checks endswith */
            if (ends_with(prefix, prefix_len, power_strs[pi], power_lens[pi])) {
                return 0;
            }
        }
        /* Check all proper prefixes of this power string */
        for (int tlen = power_lens[pi] - 1; tlen > rel_len; tlen--) {
            if (ends_with(prefix, prefix_len, power_strs[pi], tlen)) {
                rel_len = tlen;
                memcpy(relevant, power_strs[pi], tlen);
            }
        }
    }

    if (num_remaining == 0) return 1;

    ll cached;
    if (cache_get(relevant, rel_len, num_remaining, &cached))
        return cached;

    ll res = 0;
    char new_prefix[50];
    memcpy(new_prefix, prefix, prefix_len);
    for (int d = 0; d < B; d++) {
        new_prefix[prefix_len] = '0' + d;
        res += num_eleven_frees(new_prefix, prefix_len + 1, num_remaining - 1);
    }

    cache_set(relevant, rel_len, num_remaining, res);
    return res;
}

int main(void) {
    ll N_val = 1000000000000000000LL; /* 10^18 */

    /* Precompute powers of 11 as strings */
    num_powers = 0;
    ll pw = 1;
    while (pw <= (ll)9e18) {
        int len = 0;
        char buf[30];
        ll tmp = pw;
        if (tmp == 0) { buf[0] = '0'; len = 1; }
        else {
            while (tmp > 0) { buf[len++] = '0' + (int)(tmp % 10); tmp /= 10; }
        }
        /* Reverse */
        for (int i = 0; i < len / 2; i++) {
            char c = buf[i]; buf[i] = buf[len - 1 - i]; buf[len - 1 - i] = c;
        }
        memcpy(power_strs[num_powers], buf, len);
        power_lens[num_powers] = len;
        num_powers++;
        if (pw > (ll)9e18 / K) break;
        pw *= K;
    }

    int L = MAX_DIGITS;

    ll ans = 0;
    ll n = N_val;
    memset(cache, 0, sizeof(cache));

    for (int num_remaining = L; num_remaining >= 0; num_remaining--) {
        for (int d = 0; d < B; d++) {
            /* Build prefix string from ans * B + d */
            ll val = ans * B + d;
            char prefix[30];
            int plen = 0;
            if (val == 0) { prefix[0] = '0'; plen = 1; }
            else {
                ll tmp = val;
                while (tmp > 0) { prefix[plen++] = '0' + (int)(tmp % 10); tmp /= 10; }
                for (int i = 0; i < plen / 2; i++) {
                    char c = prefix[i]; prefix[i] = prefix[plen - 1 - i]; prefix[plen - 1 - i] = c;
                }
            }

            ll count = num_eleven_frees(prefix, plen, num_remaining);
            if (count > n) {
                ans = ans * B + d;
                break;
            }
            n -= count;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
