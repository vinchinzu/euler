/*
 * Project Euler 631: Constrained Permutations
 *
 * Count permutations of (1..n) avoiding pattern 1243 with at most K=40
 * descents. Sum over n=0..N where N=10^18.
 *
 * Uses memoized DP. For n > K+1, the count stabilizes.
 */
#include <stdio.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define K_VAL 40
#define MOD 1000000007LL
/* max n we compute individually = K+2 = 42 */
#define MAX_N 42

/* Cache for num_permutations(n, k, min_val, rise) */
/* n: 0..42, k: 0..40, min_val: 0..42, rise: 0..42 */
/* That's 43*41*43*43 = ~3.2M entries */
#define DIM_N (MAX_N + 1)
#define DIM_K (K_VAL + 1)
#define DIM_M (MAX_N + 1)
#define DIM_R (MAX_N + 1)

static ll cache[DIM_N][DIM_K][DIM_M][DIM_R];
static char computed[DIM_N][DIM_K][DIM_M][DIM_R];

ll num_permutations(int n, int k, int min_val, int rise) {
    if (n == 0) return 1;
    if (k < 0) return 0;
    if (n < DIM_N && k < DIM_K && min_val < DIM_M && rise < DIM_R) {
        if (computed[n][k][min_val][rise])
            return cache[n][k][min_val][rise];
    }

    ll result = 0;
    int limit = (n < rise) ? n : rise;
    for (int nv = 1; nv <= limit; nv++) {
        if (nv - 1 > k) continue;

        int next_rise = rise;
        if (nv < next_rise) {
            if (nv >= min_val) {
                next_rise = nv;
            } else {
                next_rise -= 1;
            }
        }

        int new_min = (nv < min_val) ? nv : min_val;
        result = (result + num_permutations(n - 1, k - (nv - 1), new_min, next_rise)) % MOD;
    }

    if (n < DIM_N && k < DIM_K && min_val < DIM_M && rise < DIM_R) {
        cache[n][k][min_val][rise] = result;
        computed[n][k][min_val][rise] = 1;
    }
    return result;
}

int main(void) {
    ll N = 1000000000000000000LL; /* 10^18 */
    memset(computed, 0, sizeof(computed));

    ll ans = 0;
    for (int n = 0; n <= K_VAL + 1; n++) {
        ans = (ans + num_permutations(n, K_VAL, n, n)) % MOD;
    }

    if (N > K_VAL + 1) {
        ll base_count = num_permutations(K_VAL + 2, K_VAL, K_VAL + 2, K_VAL + 2);
        ll remaining = (N - K_VAL - 1) % MOD;
        ans = (ans + (lll)remaining * base_count % MOD) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}
