/*
 * Project Euler Problem 616: Creative numbers
 *
 * Find the sum of all creative numbers n <= 10^12.
 * A number a^b with a>=2, b>=2 is creative unless both a and b are prime.
 * Special case: 16 = 2^4 = 4^2, and 4^2 has non-prime base, but 16 is
 * NOT creative (need to discard it).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

typedef long long ll;

#define LIMIT 1000000  /* sqrt(10^12) = 10^6 */

static char is_prime_arr[LIMIT + 1];

static void sieve(void) {
    memset(is_prime_arr, 1, sizeof(is_prime_arr));
    is_prime_arr[0] = is_prime_arr[1] = 0;
    for (int i = 2; (ll)i * i <= LIMIT; i++) {
        if (is_prime_arr[i]) {
            for (int j = i * i; j <= LIMIT; j += i)
                is_prime_arr[j] = 0;
        }
    }
}

/* Hash set for storing numbers up to 10^12 */
#define HASH_SIZE (1 << 22)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct Node {
    ll val;
    struct Node *next;
} Node;

static Node *htable[HASH_SIZE];
static Node node_pool[10000000];
static int pool_idx = 0;

static int hash_contains(ll v) {
    unsigned h = (unsigned)((v ^ (v >> 17)) * 0x9E3779B97F4A7C15ULL) & HASH_MASK;
    for (Node *n = htable[h]; n; n = n->next)
        if (n->val == v) return 1;
    return 0;
}

static void hash_insert(ll v) {
    unsigned h = (unsigned)((v ^ (v >> 17)) * 0x9E3779B97F4A7C15ULL) & HASH_MASK;
    for (Node *n = htable[h]; n; n = n->next)
        if (n->val == v) return;
    Node *n = &node_pool[pool_idx++];
    n->val = v;
    n->next = htable[h];
    htable[h] = n;
}

int main(void) {
    ll N = 1000000000000LL;

    sieve();
    memset(htable, 0, sizeof(htable));

    /* For each base a >= 2, exponent b >= 2, if not (a prime AND b prime), add a^b */
    for (ll a = 2; a * a <= N; a++) {
        ll power = a * a;  /* a^2 */
        int b = 2;
        while (power <= N) {
            if (!(is_prime_arr[a] && b < LIMIT && is_prime_arr[b])) {
                /* Need to check b is prime: b can be large but a^b <= N means b <= ~40 */
                int b_is_prime = 0;
                if (b <= LIMIT) b_is_prime = is_prime_arr[b];
                else {
                    /* b won't be > LIMIT for a >= 2 since a^b <= 10^12 */
                    b_is_prime = 0;
                }
                if (!(is_prime_arr[a] && b_is_prime)) {
                    hash_insert(power);
                }
            }
            /* Check for overflow before multiplying */
            if (power > N / a) break;
            power *= a;
            b++;
        }
    }

    /* Remove 16 */
    /* We need to un-insert 16. Since our hash_insert prevents duplicates,
       we need to mark it. Let's just compute sum and subtract 16 if present. */
    ll sum = 0;
    for (int i = 0; i < HASH_SIZE; i++) {
        for (Node *n = htable[i]; n; n = n->next) {
            if (n->val != 16)
                sum += n->val;
        }
    }

    printf("%lld\n", sum);
    return 0;
}
