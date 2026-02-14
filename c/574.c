/*
 * Project Euler Problem 574: Verifying Primes
 *
 * V(p) = smallest A in triplet (A,B,q) such that A>=B>0, gcd(A,B)=1,
 * AB divisible by every prime < q, p < q^2, and p=A+B or p=A-B.
 * Sum V(p) for all primes p < 3800.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

static int is_prime_arr[4000];
static int primes[600];
static int nprimes;

static void sieve(int limit) {
    for (int i = 0; i <= limit; i++) is_prime_arr[i] = 1;
    is_prime_arr[0] = is_prime_arr[1] = 0;
    for (int i = 2; i * i <= limit; i++)
        if (is_prime_arr[i])
            for (int j = i * i; j <= limit; j += i)
                is_prime_arr[j] = 0;
    nprimes = 0;
    for (int i = 2; i <= limit; i++)
        if (is_prime_arr[i])
            primes[nprimes++] = i;
}

static long long gcd_ll(long long a, long long b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

static void ext_gcd_ll(long long a, long long b, long long *g, long long *x, long long *y) {
    if (b == 0) { *g = a; *x = 1; *y = 0; return; }
    long long g1, x1, y1;
    ext_gcd_ll(b, a % b, &g1, &x1, &y1);
    *g = g1;
    *x = y1;
    *y = x1 - (a / b) * y1;
}

/* Get primes up to limit into a local array */
static int get_primes_up_to(int limit, int *out) {
    int cnt = 0;
    for (int i = 0; i < nprimes && primes[i] <= limit; i++)
        out[cnt++] = primes[i];
    return cnt;
}

static long long V(int p) {
    int sq = (int)sqrt((double)p);
    int q_primes[20];
    int nq = get_primes_up_to(sq, q_primes);
    if (nq == 0) return 0;

    /* Product of all primes up to sqrt(p) */
    long long product = 1;
    for (int i = 0; i < nq; i++) product *= q_primes[i];

    /* Try A < p case: p = A + B, A >= B > 0, so A >= (p+1)/2 */
    for (long long A = (p + 1) / 2; A < p; A++) {
        long long B = p - A;
        if (gcd_ll(A, B) != 1) continue;
        if ((A * B) % product == 0) return A;
    }

    /* Try A > p case: p = A - B */
    /* For each subset of q_primes, split into c0 (divides A) and c1 (divides B) */
    /* Solve c0*x - c1*y = p => c0*x = p (mod c1) */
    /* Then A = c0 * x, and we check all primes divide A*B */
    int num_subsets = 1 << nq;
    long long best = -1;

    for (int subset = 0; subset < num_subsets; subset++) {
        long long c0 = 1, c1 = 1;
        for (int i = 0; i < nq; i++) {
            if ((subset >> i) & 1) c0 *= q_primes[i];
            else c1 *= q_primes[i];
        }

        /* Solve c0 * x ≡ p (mod c1) */
        long long g, x0, y0;
        ext_gcd_ll(c0, c1, &g, &x0, &y0);
        if (p % g != 0) continue;
        long long x_sol = ((x0 % c1) * ((p / g) % c1)) % c1;
        if (x_sol < 0) x_sol += c1;

        /* A = c0 * (x_sol + k * c1) for k = 0, 1, 2, ... */
        /* B = A - p, need B > 0 => A > p => c0 * (x_sol + k * c1) > p */
        long long base_A = c0 * x_sol;
        long long step = c0 * c1;  /* = product */
        if (step == 0) continue;

        /* Find smallest k such that base_A + k * step > p */
        long long k_start = 0;
        if (base_A <= p) {
            k_start = (p - base_A) / step + 1;
        }

        for (long long k = k_start; k < k_start + 2000; k++) {
            long long A = base_A + k * step;
            if (A <= p) continue;
            long long B = A - p;
            if (gcd_ll(A, B) != 1) continue;
            /* Check that product divides A * B */
            /* By construction, c0 | A and c1 | B, so c0*c1 = product | A*B */
            /* But we need ALL primes < q to divide A*B, which is product */
            if ((A * B) % product == 0) {
                if (best == -1 || A < best) best = A;
                break;  /* smallest A for this subset */
            }
        }
    }

    return best > 0 ? best : 0;
}

int main(void) {
    sieve(3800);

    long long ans = 0;
    for (int i = 0; i < nprimes && primes[i] < 3800; i++) {
        ans += V(primes[i]);
    }

    printf("%lld\n", ans);
    return 0;
}
