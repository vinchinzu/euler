/*
 * Project Euler Problem 718: Unreachable Numbers.
 *
 * Find the sum of all positive integers that cannot be expressed as
 * A*a + B*b + C*c for positive integers a, b, c, where
 * A = 17^6, B = 19^6, C = 23^6.
 *
 * Uses two queues (for +B and +C) to enumerate reachable residues mod A.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

ll ncr2(ll n) {
    /* C(n, 2) = n*(n-1)/2 */
    if (n < 2) return 0;
    return n * (n - 1) / 2;
}

int main() {
    /* Compute A, B, C */
    ll a = 1, b = 1, c = 1;
    for (int i = 0; i < 6; i++) { a *= 17; b *= 19; c *= 23; }

    /* a = 17^6 = 24137569 */
    char *visited = calloc(a, 1);

    /* Two queues */
    int q1_cap = (int)a + 1;
    int q2_cap = (int)a + 1;
    ll *q1 = malloc(q1_cap * sizeof(ll));
    ll *q2 = malloc(q2_cap * sizeof(ll));
    int q1_head = 0, q1_tail = 0;
    int q2_head = 0, q2_tail = 0;

    ll val = a + b + c;
    ll ans = 0;

    while (1) {
        visited[val % a] = 1;
        ll q = val / a;
        ll r = val % a;
        /* Sum of unreachable multiples: a * C(q, 2) + r * q */
        ll term = ((lll)(ncr2(q) % MOD) * (a % MOD) % MOD + (lll)(r % MOD) * (q % MOD) % MOD) % MOD;
        ans = (ans + term) % MOD;

        ll next_val1 = val + b;
        ll next_val2 = val + c;

        if (!visited[next_val1 % a]) {
            q1[q1_tail++] = next_val1;
        }
        if (!visited[next_val2 % a]) {
            q2[q2_tail++] = next_val2;
        }

        if (q1_head >= q1_tail && q2_head >= q2_tail) break;

        ll v1 = (q1_head < q1_tail) ? q1[q1_head] : -1;
        ll v2 = (q2_head < q2_tail) ? q2[q2_head] : -1;

        if (v1 < 0) val = v2;
        else if (v2 < 0) val = v1;
        else val = v1 < v2 ? v1 : v2;

        if (q1_head < q1_tail && val == q1[q1_head]) q1_head++;
        if (q2_head < q2_tail && val == q2[q2_head]) q2_head++;
    }

    printf("%lld\n", ans);

    free(visited);
    free(q1);
    free(q2);
    return 0;
}
