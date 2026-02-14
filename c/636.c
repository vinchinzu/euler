/*
 * Project Euler 636: Restricted Factorisations
 *
 * F(n) = number of ways to write n = a * b1^2 * b2^2 * c1^3 * c2^3 * c3^3 * d1^4...
 * with all 10 bases distinct. Find F(10^6!) mod 10^9+7.
 *
 * Approach:
 * 1. Enumerate partitions of the 10 positions (1 of type-1, 2 of type-2,
 *    3 of type-3, 4 of type-4) into groups.
 * 2. Each partition gives a "jump profile" with a coefficient via inclusion-exclusion.
 * 3. For each jump profile, coin-change DP for each prime exponent in N!.
 *
 * This C version inlines both the Python preprocessing and the C DP.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define N_FACT 1000000
#define MOD 1000000007LL

ll power(ll base, ll exp, ll mod) {
    ll r = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) r = (lll)r * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return r;
}

/* Group composition: (n1, n2, n3, n4) - how many of each type in a group */
typedef struct { int n[4]; } GC;

/* Partition: list of group compositions */
#define MAX_GROUPS 10
typedef struct {
    GC groups[MAX_GROUPS];
    int ngroups;
} Partition;

/* Jump profile: sorted tuple of jump values + coefficient */
#define MAX_PROFILES 200
typedef struct {
    int jumps[MAX_GROUPS];
    int njumps;
    ll coeff;
} Profile;

Profile profiles[MAX_PROFILES];
int nprofiles = 0;

/* Enumerate all valid group compositions */
GC group_comps[60];
int n_gc = 0;

void init_group_comps() {
    for (int n1 = 0; n1 <= 1; n1++)
        for (int n2 = 0; n2 <= 2; n2++)
            for (int n3 = 0; n3 <= 3; n3++)
                for (int n4 = 0; n4 <= 4; n4++)
                    if (n1 + n2 + n3 + n4 > 0) {
                        group_comps[n_gc].n[0] = n1;
                        group_comps[n_gc].n[1] = n2;
                        group_comps[n_gc].n[2] = n3;
                        group_comps[n_gc].n[3] = n4;
                        n_gc++;
                    }
}

ll factorial_val(int n) {
    ll r = 1;
    for (int i = 2; i <= n; i++) r *= i;
    return r;
}

/* Sort int array */
void isort(int *a, int n) {
    for (int i = 1; i < n; i++) {
        int t = a[i];
        int j = i - 1;
        while (j >= 0 && a[j] > t) { a[j+1] = a[j]; j--; }
        a[j+1] = t;
    }
}

/* Compare two int arrays */
int iarr_cmp(int *a, int *b, int n) {
    for (int i = 0; i < n; i++) {
        if (a[i] < b[i]) return -1;
        if (a[i] > b[i]) return 1;
    }
    return 0;
}

/* Compare two GC */
int gc_cmp(GC *a, GC *b) {
    for (int i = 0; i < 4; i++) {
        if (a->n[i] < b->n[i]) return -1;
        if (a->n[i] > b->n[i]) return 1;
    }
    return 0;
}

/* Sort partition groups for canonical form */
void sort_partition(Partition *p) {
    /* Bubble sort (small) */
    for (int i = 0; i < p->ngroups - 1; i++)
        for (int j = i + 1; j < p->ngroups; j++)
            if (gc_cmp(&p->groups[i], &p->groups[j]) > 0) {
                GC tmp = p->groups[i];
                p->groups[i] = p->groups[j];
                p->groups[j] = tmp;
            }
}

/* Collected partitions for dedup */
#define MAX_PARTS 2000
Partition all_parts[MAX_PARTS];
int n_all_parts = 0;

int part_eq(Partition *a, Partition *b) {
    if (a->ngroups != b->ngroups) return 0;
    for (int i = 0; i < a->ngroups; i++)
        if (gc_cmp(&a->groups[i], &b->groups[i]) != 0) return 0;
    return 1;
}

int part_exists(Partition *p) {
    for (int i = 0; i < n_all_parts; i++)
        if (part_eq(&all_parts[i], p)) return 1;
    return 0;
}

/* Enumerate partitions of (1,2,3,4) into groups */
void enum_parts(int rem[4], int min_idx, GC cur[], int ncur) {
    if (rem[0] == 0 && rem[1] == 0 && rem[2] == 0 && rem[3] == 0) {
        Partition p;
        p.ngroups = ncur;
        for (int i = 0; i < ncur; i++) p.groups[i] = cur[i];
        sort_partition(&p);
        if (!part_exists(&p)) {
            all_parts[n_all_parts++] = p;
        }
        return;
    }
    for (int idx = min_idx; idx < n_gc; idx++) {
        GC *gc = &group_comps[idx];
        if (gc->n[0] <= rem[0] && gc->n[1] <= rem[1] &&
            gc->n[2] <= rem[2] && gc->n[3] <= rem[3]) {
            int new_rem[4] = {rem[0]-gc->n[0], rem[1]-gc->n[1],
                              rem[2]-gc->n[2], rem[3]-gc->n[3]};
            cur[ncur] = *gc;
            enum_parts(new_rem, idx, cur, ncur + 1);
        }
    }
}

/* Multinomial coefficient n! / (g1! * g2! * ...) */
ll multinomial(int n, int *groups, int ngroups) {
    ll r = factorial_val(n);
    for (int i = 0; i < ngroups; i++)
        r /= factorial_val(groups[i]);
    return r;
}

/* Build jump profiles from partitions */
void build_profiles() {
    /* Temporary storage for aggregating same-jump profiles */
    int jump_keys[MAX_PARTS][MAX_GROUPS];
    int jump_lens[MAX_PARTS];
    ll jump_coeffs[MAX_PARTS];
    int n_jump_keys = 0;

    for (int pi = 0; pi < n_all_parts; pi++) {
        Partition *p = &all_parts[pi];
        int m = p->ngroups;
        int sign = 1;
        for (int i = 0; i < 10 - m; i++) sign *= -1;

        /* bf = product of (sum(g)-1)! for each group */
        ll bf = 1;
        for (int i = 0; i < m; i++) {
            int s = p->groups[i].n[0] + p->groups[i].n[1] +
                    p->groups[i].n[2] + p->groups[i].n[3];
            bf *= factorial_val(s - 1);
        }

        /* ways = product of multinomials for each type */
        ll ways = 1;
        int tot[4] = {1, 2, 3, 4};
        for (int t = 0; t < 4; t++) {
            int gs[MAX_GROUPS];
            for (int i = 0; i < m; i++) gs[i] = p->groups[i].n[t];
            ways *= multinomial(tot[t], gs, m);
        }

        /* Divide by product of count! for each distinct group */
        /* Count occurrences of each distinct group */
        int counts[MAX_GROUPS];
        int distinct = 0;
        for (int i = 0; i < m; ) {
            int cnt = 1;
            while (i + cnt < m && gc_cmp(&p->groups[i], &p->groups[i + cnt]) == 0) cnt++;
            counts[distinct++] = cnt;
            i += cnt;
        }
        for (int i = 0; i < distinct; i++)
            ways /= factorial_val(counts[i]);

        ll coeff = sign * bf * ways;

        /* Compute jumps */
        int jumps[MAX_GROUPS];
        for (int i = 0; i < m; i++) {
            jumps[i] = p->groups[i].n[0] + 2 * p->groups[i].n[1] +
                        3 * p->groups[i].n[2] + 4 * p->groups[i].n[3];
        }
        isort(jumps, m);

        /* Find or create this jump key */
        int found = -1;
        for (int j = 0; j < n_jump_keys; j++) {
            if (jump_lens[j] == m && iarr_cmp(jump_keys[j], jumps, m) == 0) {
                found = j;
                break;
            }
        }
        if (found >= 0) {
            jump_coeffs[found] += coeff;
        } else {
            memcpy(jump_keys[n_jump_keys], jumps, m * sizeof(int));
            jump_lens[n_jump_keys] = m;
            jump_coeffs[n_jump_keys] = coeff;
            n_jump_keys++;
        }
    }

    /* Convert to profiles, skip zero coefficients */
    nprofiles = 0;
    for (int i = 0; i < n_jump_keys; i++) {
        if (jump_coeffs[i] == 0) continue;
        profiles[nprofiles].njumps = jump_lens[i];
        memcpy(profiles[nprofiles].jumps, jump_keys[i], jump_lens[i] * sizeof(int));
        profiles[nprofiles].coeff = jump_coeffs[i];
        nprofiles++;
    }
}

/* Prime sieve */
char *is_prime_arr;
void prime_sieve(int n) {
    is_prime_arr = calloc(n + 1, 1);
    for (int i = 2; i <= n; i++) is_prime_arr[i] = 1;
    for (int i = 2; (ll)i * i <= n; i++)
        if (is_prime_arr[i])
            for (int j = i * i; j <= n; j += i)
                is_prime_arr[j] = 0;
}

int main() {
    init_group_comps();

    GC cur[MAX_GROUPS];
    int rem[4] = {1, 2, 3, 4};
    enum_parts(rem, 0, cur, 0);

    build_profiles();

    /* Compute prime exponents in N_FACT! */
    prime_sieve(N_FACT);

    /* Count exponents and their multiplicities */
    int *exp_list = malloc(N_FACT * sizeof(int));
    int n_primes = 0;
    int max_e = 0;

    for (int p = 2; p <= N_FACT; p++) {
        if (!is_prime_arr[p]) continue;
        int e = 0;
        ll pk = p;
        while (pk <= N_FACT) {
            e += N_FACT / (int)pk;
            if (pk > N_FACT / p) break;
            pk *= p;
        }
        exp_list[n_primes++] = e;
        if (e > max_e) max_e = e;
    }

    /* Count multiplicities of each exponent value */
    int *exp_count = calloc(max_e + 1, sizeof(int));
    for (int i = 0; i < n_primes; i++)
        exp_count[exp_list[i]]++;

    /* Build distinct exponents and multiplicities */
    int *distinct_exps = malloc((max_e + 1) * sizeof(int));
    int *mults = malloc((max_e + 1) * sizeof(int));
    int n_distinct = 0;
    for (int e = 1; e <= max_e; e++) {
        if (exp_count[e] > 0) {
            distinct_exps[n_distinct] = e;
            mults[n_distinct] = exp_count[e];
            n_distinct++;
        }
    }

    /* For each profile, compute coin-change DP and evaluate product */
    unsigned int *dp = malloc((max_e + 1) * sizeof(unsigned int));
    unsigned int M_val = (unsigned int)MOD;
    ll answer = 0;

    for (int pi = 0; pi < nprofiles; pi++) {
        memset(dp, 0, (max_e + 1) * sizeof(unsigned int));
        dp[0] = 1;

        for (int ci = 0; ci < profiles[pi].njumps; ci++) {
            int c = profiles[pi].jumps[ci];
            for (int i = c; i <= max_e; i++) {
                unsigned int v = dp[i] + dp[i - c];
                dp[i] = v >= M_val ? v - M_val : v;
            }
        }

        ll prod = 1;
        for (int ei = 0; ei < n_distinct; ei++) {
            ll val = dp[distinct_exps[ei]];
            prod = (lll)prod * power(val, mults[ei], MOD) % MOD;
        }

        ll c = profiles[pi].coeff % MOD;
        if (c < 0) c += MOD;
        answer = (answer + (lll)c * prod) % MOD;
    }

    /* Divide by 1!*2!*3!*4! = 1*2*6*24 = 288 */
    ll div_val = 288;
    answer = (lll)answer * power(div_val, MOD - 2, MOD) % MOD;

    printf("%lld\n", answer);

    free(dp);
    free(exp_list);
    free(exp_count);
    free(distinct_exps);
    free(mults);
    free(is_prime_arr);
    return 0;
}
