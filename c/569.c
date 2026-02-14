/*
 * Project Euler Problem 569: Mountain Range Peaks
 *
 * A mountain range with slopes of prime lengths, alternating up/down 45 degrees.
 * P(k) = number of peaks visible from peak k. Find sum P(k) for k=1..N.
 *
 * Use convex hull visibility with binary search on previously visible peaks.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 2500000
#define SIEVE_LIMIT (2 * N + 100)

/* Sieve of Eratosthenes */
static int primes[5500000];
static int nprimes;

static void sieve(int limit) {
    char *is_prime = (char *)calloc(limit + 1, 1);
    memset(is_prime, 1, limit + 1);
    is_prime[0] = is_prime[1] = 0;
    for (long long i = 2; i * i <= limit; i++)
        if (is_prime[i])
            for (long long j = i * i; j <= limit; j += i)
                is_prime[j] = 0;
    nprimes = 0;
    for (int i = 2; i <= limit; i++)
        if (is_prime[i])
            primes[nprimes++] = i;
    free(is_prime);
}

/* We need at least 2*N primes. Estimate limit. */
/* pi(x) ~ x/ln(x). For 5M primes, x ~ 90M should suffice. */
#define PRIME_SIEVE_LIMIT 90000000

typedef struct { long long x, y; } Point;

/* Store visible peaks as dynamic arrays */
/* To save memory, store them in a flat array with index pointers */
static int *vis_data;      /* flat storage of visible peak indices */
static int *vis_start;     /* start index in vis_data for each peak */
static int *vis_count;     /* number of visible peaks for each peak */
static int vis_total;

/* Cross product: (p2-p1) x (p3-p2) */
static long long cross(Point p1, Point p2, Point p3) {
    long long dx1 = p2.x - p1.x;
    long long dy1 = p2.y - p1.y;
    long long dx2 = p3.x - p2.x;
    long long dy2 = p3.y - p2.y;
    return dx1 * dy2 - dy1 * dx2;
}

int main(void) {
    sieve(PRIME_SIEVE_LIMIT);

    /* Ensure we have at least 2*N primes */
    if (nprimes < 2 * N) {
        fprintf(stderr, "Not enough primes: %d < %d\n", nprimes, 2 * N);
        return 1;
    }

    Point *peaks = (Point *)malloc(N * sizeof(Point));

    /* Allocate visible peak storage */
    /* Average ~8 visible peaks per peak => ~20M entries */
    int vis_cap = 30000000;
    vis_data = (int *)malloc(vis_cap * sizeof(int));
    vis_start = (int *)malloc(N * sizeof(int));
    vis_count = (int *)malloc(N * sizeof(int));
    vis_total = 0;

    long long x = 0, y = 0;
    long long ans = 0;

    for (int i = 0; i < N; i++) {
        x += primes[2 * i];
        y += primes[2 * i];
        peaks[i] = (Point){x, y};
        x += primes[2 * i + 1];
        y -= primes[2 * i + 1];

        /* Build visible peaks list */
        vis_start[i] = vis_total;
        int count = 0;
        int j = i - 1;

        while (j >= 0) {
            /* Ensure capacity */
            if (vis_total + count + 1 >= vis_cap) {
                vis_cap *= 2;
                vis_data = (int *)realloc(vis_data, vis_cap * sizeof(int));
            }
            vis_data[vis_total + count] = j;
            count++;

            if (j == 0) break;

            /* Binary search on previous visible peaks */
            int *prev_vis = vis_data + vis_start[j];
            int prev_count = vis_count[j];
            Point mid = peaks[j];
            Point base = peaks[i];

            int left = 0, right = prev_count;
            while (left < right) {
                int mid_idx = (left + right) / 2;
                int peak_idx = prev_vis[mid_idx];
                long long turn_val = cross(peaks[peak_idx], mid, base);
                if (turn_val < 0)
                    left = mid_idx + 1;
                else
                    right = mid_idx;
            }

            if (left >= prev_count) break;
            j = prev_vis[left];
        }

        vis_count[i] = count;
        vis_total += count;
        ans += count;
    }

    printf("%lld\n", ans);

    free(peaks);
    free(vis_data);
    free(vis_start);
    free(vis_count);
    return 0;
}
