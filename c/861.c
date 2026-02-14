#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/*
 * Project Euler 861 - Bi-unitary divisors
 *
 * Find sum_{k=2}^{10} Q_k(10^12).
 * Uses prime sieve, Lucy DP for pi(x), signature enumeration, and backtracking.
 */

typedef long long ll;

#define N_LIMIT 1000000000000LL
#define SIEVE_LIM 1000001

static int prime_list[80000];
static int num_primes = 0;
static ll S_small[SIEVE_LIM + 1];
static ll S_large[SIEVE_LIM + 1];
static ll isqrt_val;

void sieve_primes(void) {
    char *is_prime = calloc(SIEVE_LIM + 1, 1);
    for (int i = 2; i <= SIEVE_LIM; i++) is_prime[i] = 1;
    for (int p = 2; (ll)p * p <= SIEVE_LIM; p++) {
        if (is_prime[p]) {
            for (int i = p * p; i <= SIEVE_LIM; i += p)
                is_prime[i] = 0;
        }
    }
    for (int p = 2; p <= SIEVE_LIM; p++) {
        if (is_prime[p]) prime_list[num_primes++] = p;
    }
    free(is_prime);
}

void compute_pi(ll N) {
    isqrt_val = (ll)sqrt((double)N);
    while ((isqrt_val + 1) * (isqrt_val + 1) <= N) isqrt_val++;
    while (isqrt_val > 0 && isqrt_val * isqrt_val > N) isqrt_val--;

    for (ll v = 0; v <= isqrt_val; v++) S_small[v] = v - 1;
    for (ll k = 1; k <= isqrt_val; k++) S_large[k] = (N / k) - 1;

    for (int pi = 0; pi < num_primes; pi++) {
        ll p = prime_list[pi];
        if (p > isqrt_val) break;
        ll p2 = p * p;
        if (p2 > N) break;
        ll sp_1 = S_small[p - 1];
        ll k_limit = N / p2;
        if (k_limit > isqrt_val) k_limit = isqrt_val;
        for (ll k = 1; k <= k_limit; k++) {
            ll target = (N / k) / p;
            ll s_target = (target <= isqrt_val) ? S_small[target] : S_large[k * p];
            S_large[k] -= (s_target - sp_1);
        }
        for (ll v = isqrt_val; v >= p2; v--) {
            S_small[v] -= (S_small[v / p] - sp_1);
        }
    }
}

ll get_pi(ll x) {
    if (x <= 1) return 0;
    if (x <= isqrt_val) return S_small[x];
    return S_large[N_LIMIT / x];
}

ll integer_root(ll n, int k) {
    if (k == 1) return n;
    if (n <= 1) return n;
    ll lo = 1, hi = (ll)pow((double)n, 1.0 / k) + 2;
    ll ans = 1;
    while (lo <= hi) {
        ll mid = lo + (hi - lo) / 2;
        ll p = 1;
        int over = 0;
        for (int i = 0; i < k; i++) {
            if (p > n / mid) { over = 1; break; }
            p *= mid;
        }
        if (!over && p <= n) { ans = mid; lo = mid + 1; }
        else hi = mid - 1;
    }
    return ans;
}

/* Factor partitions */
#define MAX_PARTS 20
typedef struct { int parts[MAX_PARTS]; int len; } Partition;
Partition partitions[10000];
int n_partitions;

void get_factor_partitions(int target, int count, int min_val, int *current, int depth) {
    if (count == 1) {
        if (target >= min_val) {
            Partition *p = &partitions[n_partitions];
            for (int i = 0; i < depth; i++) p->parts[i] = current[i];
            p->parts[depth] = target;
            p->len = depth + 1;
            n_partitions++;
        }
        return;
    }
    for (int i = min_val; i <= target; i++) {
        if (target % i == 0) {
            current[depth] = i;
            get_factor_partitions(target / i, count - 1, i, current, depth + 1);
        }
    }
}

/* Signature storage */
#define MAX_SIGS 5000
typedef struct { int exps[MAX_PARTS]; int len; } Signature;
Signature sigs[MAX_SIGS];
int n_sigs;

int sig_cmp(const void *a, const void *b) {
    const Signature *sa = a, *sb = b;
    if (sa->len != sb->len) return sa->len - sb->len;
    for (int i = 0; i < sa->len; i++) {
        if (sa->exps[i] != sb->exps[i]) return sa->exps[i] - sb->exps[i];
    }
    return 0;
}

void sort_sig(int *arr, int len) {
    for (int i = 0; i < len - 1; i++)
        for (int j = i + 1; j < len; j++)
            if (arr[j] < arr[i]) { int t = arr[i]; arr[i] = arr[j]; arr[j] = t; }
}

void generate_signatures(int k) {
    n_sigs = 0;
    int r = 1;
    while (1) {
        ll power_of_2 = 1LL << (r - 1);
        if (power_of_2 > k) break;
        if (k % (int)power_of_2 == 0) {
            int target = k / (int)power_of_2;
            n_partitions = 0;
            int current[MAX_PARTS];
            get_factor_partitions(target, r, 1, current, 0);

            for (int pi = 0; pi < n_partitions; pi++) {
                int n_choices = 1 << r;
                for (int mask = 0; mask < n_choices; mask++) {
                    int a[MAX_PARTS];
                    for (int i = 0; i < r; i++) {
                        if ((mask >> i) & 1) a[i] = 2 * partitions[pi].parts[i];
                        else a[i] = 2 * partitions[pi].parts[i] - 1;
                    }
                    sort_sig(a, r);
                    /* Check if duplicate */
                    int dup = 0;
                    for (int s = 0; s < n_sigs; s++) {
                        if (sigs[s].len != r) continue;
                        int same = 1;
                        for (int i = 0; i < r; i++) {
                            if (sigs[s].exps[i] != a[i]) { same = 0; break; }
                        }
                        if (same) { dup = 1; break; }
                    }
                    if (!dup) {
                        for (int i = 0; i < r; i++) sigs[n_sigs].exps[i] = a[i];
                        sigs[n_sigs].len = r;
                        n_sigs++;
                    }
                }
            }
        }
        r++;
    }
}

/* Backtracking count */
typedef struct { int exp; int count; } Group;

ll used_primes[20];
int n_used;

ll backtrack_inner(int group_idx, int remain, ll current_prod,
                   Group *groups, int n_groups, int min_p_idx);

ll backtrack(int group_idx, ll current_prod,
             Group *groups, int n_groups) {
    if (group_idx == n_groups) return 1;
    return backtrack_inner(group_idx, groups[group_idx].count, current_prod,
                           groups, n_groups, 0);
}

ll backtrack_inner(int group_idx, int remain, ll current_prod,
                   Group *groups, int n_groups, int min_p_idx) {
    int exp = groups[group_idx].exp;

    if (remain == 0) {
        return backtrack(group_idx + 1, current_prod, groups, n_groups);
    }

    ll total = 0;

    /* Last prime optimization */
    if (group_idx == n_groups - 1 && remain == 1) {
        ll rem = N_LIMIT / current_prod;
        ll limit_p = integer_root(rem, exp);
        if (limit_p < 2) return 0;

        ll lower_bound_val;
        if (min_p_idx < num_primes) lower_bound_val = prime_list[min_p_idx];
        else lower_bound_val = prime_list[num_primes - 1] + 1;

        if (limit_p < lower_bound_val) return 0;

        ll valid_count = get_pi(limit_p) - get_pi(lower_bound_val - 1);

        for (int u = 0; u < n_used; u++) {
            if (used_primes[u] >= lower_bound_val && used_primes[u] <= limit_p)
                valid_count--;
        }
        return valid_count;
    }

    for (int i = min_p_idx; i < num_primes; i++) {
        ll p = prime_list[i];

        int collision = 0;
        for (int u = 0; u < n_used; u++) {
            if (used_primes[u] == p) { collision = 1; break; }
        }
        if (collision) continue;

        ll p_pow = 1;
        int over = 0;
        for (int k = 0; k < exp; k++) {
            if (p_pow > N_LIMIT / p) { over = 1; break; }
            p_pow *= p;
        }
        if (over) break;

        if (current_prod > N_LIMIT / p_pow) break;
        ll next_prod = current_prod * p_pow;
        if (next_prod > N_LIMIT) break;

        used_primes[n_used++] = p;
        total += backtrack_inner(group_idx, remain - 1, next_prod,
                                 groups, n_groups, i + 1);
        n_used--;
    }
    return total;
}

int main(void) {
    sieve_primes();
    compute_pi(N_LIMIT);

    ll total_sum = 0;

    for (int k = 2; k <= 10; k++) {
        generate_signatures(k);

        ll q_k = 0;
        for (int s = 0; s < n_sigs; s++) {
            /* Group exponents */
            int exp_counts[100];
            int exp_vals[100];
            int n_g = 0;
            /* Count occurrences of each exponent */
            for (int i = 0; i < sigs[s].len; i++) {
                int e = sigs[s].exps[i];
                int found = -1;
                for (int g = 0; g < n_g; g++) {
                    if (exp_vals[g] == e) { found = g; break; }
                }
                if (found >= 0) exp_counts[found]++;
                else { exp_vals[n_g] = e; exp_counts[n_g] = 1; n_g++; }
            }

            /* Sort by exponent descending */
            Group groups[20];
            for (int g = 0; g < n_g; g++) {
                groups[g].exp = exp_vals[g];
                groups[g].count = exp_counts[g];
            }
            for (int i = 0; i < n_g - 1; i++)
                for (int j = i + 1; j < n_g; j++)
                    if (groups[j].exp > groups[i].exp) {
                        Group tmp = groups[i]; groups[i] = groups[j]; groups[j] = tmp;
                    }

            n_used = 0;
            ll count = backtrack(0, 1, groups, n_g);
            q_k += count;
        }
        total_sum += q_k;
    }

    printf("%lld\n", total_sum);
    return 0;
}
