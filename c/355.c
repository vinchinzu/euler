/* Project Euler 355 - Maximal coprime subset sum
 *
 * Find the largest sum of a mutually coprime subset of {1, ..., 200000}.
 * Base set = {1} + all primes > sqrt(N). For each small prime p, assign it
 * to one base element q, replacing q with max(q*p^k <= N). This is a
 * max-weight bipartite assignment problem (Hungarian algorithm).
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 200000
#define SQRT_N 448  /* floor(sqrt(200000)) = 447, but use 448 for safety */

static char is_prime_arr[N + 1];
static int all_primes[20000];
static int nall_primes;

void sieve(void) {
    memset(is_prime_arr, 1, sizeof(is_prime_arr));
    is_prime_arr[0] = is_prime_arr[1] = 0;
    for (int i = 2; (long long)i * i <= N; i++) {
        if (is_prime_arr[i]) {
            for (int j = i * i; j <= N; j += i)
                is_prime_arr[j] = 0;
        }
    }
    nall_primes = 0;
    for (int i = 2; i <= N; i++)
        if (is_prime_arr[i]) all_primes[nall_primes++] = i;
}

/* Hungarian algorithm for max-weight assignment (rows <= cols) */
#define INF_COST 1000000000000LL

typedef long long ll;

/* small_primes are rows, base_elements are columns */
static int small_primes[100];
static int n_small;
static int base_elements[20000];
static int n_base;
static int gains_flat[100 * 18000]; /* row-major, n_small x n_base */

#define GAINS(i,j) gains_flat[(i) * n_base + (j)]

ll hungarian(void) {
    int row_count = n_small;
    int col_count = n_base;

    ll *u = (ll*)calloc(row_count + 1, sizeof(ll));
    ll *v = (ll*)calloc(col_count + 1, sizeof(ll));
    int *p = (int*)calloc(col_count + 1, sizeof(int));
    int *way = (int*)calloc(col_count + 1, sizeof(int));
    ll *minv = (ll*)malloc((col_count + 1) * sizeof(ll));
    char *used = (char*)malloc((col_count + 1) * sizeof(char));

    for (int i = 1; i <= row_count; i++) {
        p[0] = i;
        int j0 = 0;
        for (int j = 0; j <= col_count; j++) { minv[j] = INF_COST; used[j] = 0; }

        while (1) {
            used[j0] = 1;
            int i0 = p[j0];
            ll delta = INF_COST;
            int j1 = 0;

            for (int j = 1; j <= col_count; j++) {
                if (used[j]) continue;
                ll cur = -(ll)GAINS(i0 - 1, j - 1) - u[i0] - v[j];
                if (cur < minv[j]) {
                    minv[j] = cur;
                    way[j] = j0;
                }
                if (minv[j] < delta) {
                    delta = minv[j];
                    j1 = j;
                }
            }

            for (int j = 0; j <= col_count; j++) {
                if (used[j]) {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }

            j0 = j1;
            if (p[j0] == 0) break;
        }

        while (1) {
            int j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if (j0 == 0) break;
        }
    }

    /* Recover assignment and sum original weights */
    ll total = 0;
    for (int j = 1; j <= col_count; j++) {
        if (p[j] != 0) {
            total += GAINS(p[j] - 1, j - 1);
        }
    }

    free(u); free(v); free(p); free(way); free(minv); free(used);
    return total;
}

int main(void) {
    sieve();

    /* Build small primes and base elements */
    n_small = 0;
    n_base = 0;
    base_elements[n_base++] = 1;
    for (int i = 0; i < nall_primes; i++) {
        if (all_primes[i] <= SQRT_N)
            small_primes[n_small++] = all_primes[i];
        else
            base_elements[n_base++] = all_primes[i];
    }

    ll base_sum = 0;
    for (int i = 0; i < n_base; i++)
        base_sum += base_elements[i];

    /* Build gains matrix */
    for (int i = 0; i < n_small; i++) {
        int p = small_primes[i];
        for (int j = 0; j < n_base; j++) {
            int q = base_elements[j];
            long long combined = q;
            while (combined * p <= N) combined *= p;
            GAINS(i, j) = (int)(combined - (q == 1 ? 0 : q));
        }
    }

    ll assignment_gain = hungarian();
    printf("%lld\n", base_sum + assignment_gain);
    return 0;
}
