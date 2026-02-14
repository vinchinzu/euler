/*
 * Project Euler 656 - Palindromic sequences
 * H_100(sqrt(beta)) for non-square beta <= 1000.
 * Uses continued fraction of sqrt(D).
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;

#define M_VAL 1000000000000000LL /* 10^15 */

static int is_square(int n) {
    int r = (int)sqrt((double)n);
    while (r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return r * r == n;
}

static ll H(int beta, int K) {
    int a0 = (int)sqrt((double)beta);
    if (a0 * a0 == beta) return 0;

    /* Generate CF and convergent denominators, collecting palindrome lengths */
    ll ns[110];
    int ns_len = 0;
    ns[ns_len++] = 0;

    ll qs[200]; /* convergent denominators mod M */
    int qs_len = 0;
    qs[qs_len++] = 1;
    qs[qs_len++] = 0;

    int P = a0;
    int Q = beta - a0 * a0;
    int cf_idx = 0; /* 0 = a0 already processed */

    /* Process a0 */
    {
        int a = a0;
        if (qs_len % 2 == 1) { /* odd index: add palindromes */
            for (int rep = 0; rep < a && ns_len <= K; rep++) {
                ns[ns_len] = (ns[ns_len - 1] + qs[qs_len - 1]) % M_VAL;
                ns_len++;
            }
        }
        ll new_q = (qs[qs_len - 1] * a + qs[qs_len - 2]) % M_VAL;
        qs[qs_len++] = new_q;
    }

    /* Continue with periodic part */
    while (ns_len <= K && Q != 0) {
        int a = (a0 + P) / Q;
        if (qs_len % 2 == 1) {
            for (int rep = 0; rep < a && ns_len <= K; rep++) {
                ns[ns_len] = (ns[ns_len - 1] + qs[qs_len - 1]) % M_VAL;
                ns_len++;
            }
        }
        ll new_q = (qs[qs_len - 1] * a + qs[qs_len - 2]) % M_VAL;
        qs[qs_len++] = new_q;

        P = a * Q - P;
        Q = (beta - P * P) / Q;
        if (Q == 0) break;
    }

    ll result = 0;
    int limit = K + 1;
    if (limit > ns_len) limit = ns_len;
    for (int i = 1; i < limit; i++) {
        result = (result + ns[i]) % M_VAL;
    }
    return result;
}

int main(void) {
    int N = 1000;
    int K = 100;

    ll ans = 0;
    for (int beta = 1; beta <= N; beta++) {
        if (is_square(beta)) continue;
        ans = (ans + H(beta, K)) % M_VAL;
    }

    printf("%lld\n", ans);
    return 0;
}
