/*
 * Project Euler Problem 248: Numbers for which Euler's totient equals 13!
 *
 * Find the 150000th smallest n such that phi(n) = 13!.
 *
 * Strategy: enumerate all n whose totient divides 13! by trying each prime
 * p where (p-1) | 13!, building up candidate (prod, phi) pairs.
 * Then sort and pick the 150000th.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

/* 13! = 6227020800 */
#define KF 6227020800LL
#define TARGET_N 150000

/* Primes up to 13 for factoring 13! */
static int small_primes[] = {2, 3, 5, 7, 11, 13};
static int n_small_primes = 6;

/* Simple primality test */
static int is_prime(ll n) {
    if (n < 2) return 0;
    if (n == 2 || n == 3) return 1;
    if (n % 2 == 0 || n % 3 == 0) return 0;
    for (ll i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return 0;
    }
    return 1;
}

/* Get all divisors of KF */
static ll *divisors;
static int ndivisors;

static void get_divisors(ll n) {
    /* Factor n using small primes */
    ll temp = n;
    divisors = (ll*)malloc(100000 * sizeof(ll));
    divisors[0] = 1;
    ndivisors = 1;

    for (int pi = 0; pi < n_small_primes; pi++) {
        int p = small_primes[pi];
        if (temp % p == 0) {
            int sz = ndivisors;
            ll power = 1;
            while (temp % p == 0) {
                temp /= p;
                power *= p;
                for (int i = 0; i < sz; i++)
                    divisors[ndivisors++] = divisors[i] * power;
            }
        }
    }
    if (temp > 1) {
        int sz = ndivisors;
        for (int i = 0; i < sz; i++)
            divisors[ndivisors++] = divisors[i] * temp;
    }
}

/* Dynamic array of (prod, phi) pairs */
typedef struct { ll prod; ll phi; } Pair;
static Pair *nums;
static int nnums, cap_nums;

static void add_num(ll prod, ll phi) {
    if (nnums >= cap_nums) {
        cap_nums *= 2;
        nums = (Pair*)realloc(nums, cap_nums * sizeof(Pair));
    }
    nums[nnums].prod = prod;
    nums[nnums].phi = phi;
    nnums++;
}

static int cmp_ll(const void *a, const void *b) {
    ll x = *(const ll*)a, y = *(const ll*)b;
    return (x > y) - (x < y);
}

int main(void) {
    /* Build divisors of KF */
    get_divisors(KF);

    /* Initial: (1, 1) */
    cap_nums = 1000000;
    nums = (Pair*)malloc(cap_nums * sizeof(Pair));
    nnums = 0;
    add_num(1, 1);

    /* For each divisor d of KF, check if p = KF/d + 1 is prime */
    for (int di = 0; di < ndivisors; di++) {
        ll d = divisors[di];
        ll p = KF / d + 1;
        if (p < 2) continue;
        if (!is_prime(p)) continue;

        /* Try adding powers of p to existing numbers */
        int old_nnums = nnums;
        for (int ni = 0; ni < old_nnums; ni++) {
            ll prod = nums[ni].prod;
            ll phi = nums[ni].phi;
            ll pe = 1;
            while (1) {
                /* Check if phi * pe * (p-1) divides KF */
                lll new_phi = (lll)phi * pe * (p - 1);
                if (new_phi > KF) break;
                if (KF % (ll)new_phi != 0) break;
                add_num(prod * pe * p, (ll)new_phi);
                pe *= p;
            }
        }
    }

    /* Collect all prod where phi == KF */
    ll *valid = (ll*)malloc(nnums * sizeof(ll));
    int nvalid = 0;
    for (int i = 0; i < nnums; i++) {
        if (nums[i].phi == KF) {
            valid[nvalid++] = nums[i].prod;
        }
    }

    qsort(valid, nvalid, sizeof(ll), cmp_ll);

    printf("%lld\n", valid[TARGET_N - 1]);

    free(divisors);
    free(nums);
    free(valid);
    return 0;
}
