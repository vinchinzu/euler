/*
 * Project Euler Problem 362 - Squarefree factors.
 *
 * Fsf(n) is the number of ways to factor n into square-free factors > 1.
 * Find sum of Fsf(k) for k=1 to N=10^10.
 *
 * Algorithm:
 * - Precompute Mobius function and square-free count for all quotient values N//k
 * - Use iterative stack-based search over products of square-free factors
 * - For each partial product, add count of valid last factors
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;

#define N_VAL 10000000000LL

static int L;  /* floor(sqrt(N)) */
static signed char *mu;
static char *is_prime_arr;
static char *is_square_free;
static ll *sf_cumul;

/* Square-free numbers list */
static int *square_frees;
static int num_sf;

static void compute_sieve(void) {
    L = (int)sqrt((double)N_VAL);

    mu = (signed char *)calloc(L + 1, sizeof(signed char));
    is_prime_arr = (char *)calloc(L + 1, sizeof(char));
    is_square_free = (char *)calloc(L + 1, sizeof(char));
    sf_cumul = (ll *)calloc(L + 2, sizeof(ll));

    for (int i = 0; i <= L; i++) {
        mu[i] = 1;
        is_prime_arr[i] = 1;
        is_square_free[i] = 1;
    }
    is_prime_arr[0] = is_prime_arr[1] = 0;
    is_square_free[0] = is_square_free[1] = 0;

    for (int i = 2; (ll)i * i <= L; i++) {
        if (is_prime_arr[i]) {
            for (int j = i * i; j <= L; j += i)
                is_prime_arr[j] = 0;
            ll sq = (ll)i * i;
            for (ll j = sq; j <= L; j += sq) {
                mu[j] = 0;
                is_square_free[j] = 0;
            }
        }
    }

    for (int i = 2; i <= L; i++) {
        if (mu[i] == 0) continue;
        if (is_prime_arr[i]) {
            for (int j = i; j <= L; j += i)
                mu[j] = -mu[j];
        }
    }

    /* Build square-free list and cumulative count */
    num_sf = 0;
    for (int i = 2; i <= L; i++) {
        sf_cumul[i] = sf_cumul[i - 1] + (is_square_free[i] ? 1 : 0);
        if (is_square_free[i]) num_sf++;
    }

    square_frees = (int *)malloc(num_sf * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= L; i++)
        if (is_square_free[i])
            square_frees[idx++] = i;
}

static ll count_square_free(ll x) {
    if (x < 1) return 0;
    ll total = 0;
    ll d = 1;
    while (d * d <= x) {
        if (mu[d] != 0)
            total += (ll)mu[d] * (x / (d * d));
        d++;
    }
    return total;
}

static ll num_square_free_up_to(ll x) {
    if (x < 2) return 0;
    if (x <= L) return sf_cumul[x];
    return count_square_free(x) - 1; /* exclude 1 */
}

/* Hash map for quotient values */
#define HM_SIZE (1 << 18)
#define HM_MASK (HM_SIZE - 1)

typedef struct {
    ll key;
    ll value;
    char occupied;
} QEntry;

static QEntry qmap[HM_SIZE];

static unsigned int qhash(ll key) {
    unsigned long long k = (unsigned long long)key;
    k ^= k >> 33;
    k *= 0xff51afd7ed558ccdULL;
    k ^= k >> 33;
    return (unsigned int)(k & HM_MASK);
}

static void qmap_put(ll key, ll value) {
    unsigned int idx = qhash(key);
    for (int probe = 0; probe < HM_SIZE; probe++) {
        unsigned int i = (idx + probe) & HM_MASK;
        if (!qmap[i].occupied) {
            qmap[i].key = key;
            qmap[i].value = value;
            qmap[i].occupied = 1;
            return;
        }
        if (qmap[i].key == key) return;
    }
}

static ll qmap_get(ll key) {
    unsigned int idx = qhash(key);
    for (int probe = 0; probe < HM_SIZE; probe++) {
        unsigned int i = (idx + probe) & HM_MASK;
        if (!qmap[i].occupied) return -1;
        if (qmap[i].key == key) return qmap[i].value;
    }
    return -1;
}

static ll get_num_sf(ll x) {
    if (x < 2) return 0;
    if (x <= L) return sf_cumul[x];
    ll v = qmap_get(x);
    if (v >= 0) return v;
    v = num_square_free_up_to(x);
    qmap_put(x, v);
    return v;
}

static int find_last_index(int start_index, ll limit) {
    if (start_index >= num_sf) return start_index - 1;
    if (square_frees[start_index] > limit) return start_index - 1;
    int lo = start_index, hi = num_sf - 1;
    while (lo < hi) {
        int mid = (lo + hi + 1) / 2;
        if (square_frees[mid] <= limit) lo = mid;
        else hi = mid - 1;
    }
    return lo;
}

/* Stack for iterative DFS */
#define STACK_SIZE 10000000
typedef struct {
    int prev_index;
    ll prod;
} StackEntry;

static StackEntry *stack;
static int stack_top;

int main(void) {
    compute_sieve();
    memset(qmap, 0, sizeof(qmap));

    /* Precompute quotient values */
    for (int k = 1; k <= L; k++) {
        ll q = N_VAL / k;
        if (qmap_get(q) < 0)
            qmap_put(q, num_square_free_up_to(q));
    }

    stack = (StackEntry *)malloc(STACK_SIZE * sizeof(StackEntry));
    stack_top = 0;
    stack[stack_top].prev_index = 0;
    stack[stack_top].prod = 1;
    stack_top++;

    ll ans = 0;

    while (stack_top > 0) {
        stack_top--;
        int prev_index = stack[stack_top].prev_index;
        ll prod = stack[stack_top].prod;

        ll max_last = N_VAL / prod;
        ll min_sf = (prev_index < num_sf) ? square_frees[prev_index] : 2;

        if (max_last >= min_sf) {
            ll contrib = get_num_sf(max_last) - get_num_sf(min_sf - 1);
            ans += contrib;
        }

        ll max_sf_for_next = (ll)sqrt((double)(N_VAL / prod));
        if (max_sf_for_next < 2) continue;

        int last_valid = find_last_index(prev_index, max_sf_for_next);
        if (last_valid < prev_index) continue;

        for (int index = last_valid; index >= prev_index; index--) {
            ll sf = square_frees[index];
            ll new_prod = prod * sf;
            if (new_prod * sf > N_VAL) continue;
            if (stack_top >= STACK_SIZE) {
                fprintf(stderr, "Stack overflow!\n");
                exit(1);
            }
            stack[stack_top].prev_index = index;
            stack[stack_top].prod = new_prod;
            stack_top++;
        }
    }

    printf("%lld\n", ans);

    free(mu);
    free(is_prime_arr);
    free(is_square_free);
    free(sf_cumul);
    free(square_frees);
    free(stack);
    return 0;
}
