/* Project Euler 196: Prime triplets.
   For each target row, sieve 5 rows (row-2..row+2) and find primes
   that are part of a "prime triplet" (chain of 3 adjacent primes). */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

static long long tr(long long n) {
    return n * (n + 1) / 2;
}

static long long sum_good_primes(long long row) {
    long long start = tr(row - 3) + 1;
    long long end = tr(row + 2);

    /* Odd-only segmented sieve */
    long long base = start;
    if (base % 2 == 0) base++;
    long long sieve_len = (end - base) / 2 + 1;

    long long sieve_limit = (long long)sqrt((double)end) + 1;

    /* Small primes sieve */
    char *small_sieve = calloc(sieve_limit + 1, 1);
    small_sieve[0] = small_sieve[1] = 1;
    for (long long i = 2; i * i <= sieve_limit; i++) {
        if (!small_sieve[i]) {
            for (long long j = i * i; j <= sieve_limit; j += i)
                small_sieve[j] = 1;
        }
    }

    /* Main sieve - odd numbers from base to end */
    char *is_composite = calloc(sieve_len, 1);

    /* Mark even prime: 2 divides nothing in odd-only sieve */
    for (long long p = 3; p <= sieve_limit; p += 2) {
        if (small_sieve[p]) continue;
        long long first = p * p;
        if (first < base) {
            long long k = (base + p - 1) / p;
            if (k % 2 == 0) k++;
            first = p * k;
        }
        if (first > end) continue;
        long long idx = (first - base) / 2;
        for (long long i = idx; i < sieve_len; i += p) {
            is_composite[i] = 1;
        }
    }

    /* Helper: is num prime? */
    #define IS_PRIME(num) ( \
        (num) < 2 ? 0 : \
        (num) == 2 ? 1 : \
        ((num) % 2 == 0) ? 0 : \
        (((num) >= base && (num) <= end) ? !is_composite[((num) - base) / 2] : 0) \
    )

    /* Build 5 rows: row-2..row+2, indexed 0..4 */
    long long row_starts[5];
    long long row_lens[5];
    for (int i = 0; i < 5; i++) {
        row_starts[i] = tr(row - 3 + i) + 1;
        row_lens[i] = row - 2 + i;
    }

    /* Build primality arrays for each row */
    char *is_p[5];
    for (int ri = 0; ri < 5; ri++) {
        is_p[ri] = calloc(row_lens[ri], 1);
        for (long long j = 0; j < row_lens[ri]; j++) {
            long long num = row_starts[ri] + j;
            if (IS_PRIME(num)) is_p[ri][j] = 1;
        }
    }

    /* Directions for adjacency in triangular grid */
    static const int dirs[6][2] = {{-1,-1},{-1,0},{-1,1},{1,-1},{1,0},{1,1}};

    /* Compute isCentral for rows 1, 2, 3 (a prime with >= 2 prime neighbours) */
    char *central[5];
    for (int ri = 0; ri < 5; ri++) central[ri] = NULL;
    for (int ri = 1; ri <= 3; ri++) {
        central[ri] = calloc(row_lens[ri], 1);
        for (long long j = 0; j < row_lens[ri]; j++) {
            if (!is_p[ri][j]) continue;
            int count = 0;
            for (int d = 0; d < 6; d++) {
                int ni = ri + dirs[d][0];
                long long nj = j + dirs[d][1];
                if (ni >= 0 && ni < 5 && nj >= 0 && nj < row_lens[ni]) {
                    if (is_p[ni][nj]) {
                        count++;
                        if (count >= 2) break;
                    }
                }
            }
            if (count >= 2) central[ri][j] = 1;
        }
    }

    /* Sum good primes in target row (ri=2) */
    long long total = 0;
    int target_ri = 2;
    long long target_start = row_starts[target_ri];
    long long target_len = row_lens[target_ri];

    for (long long j = 0; j < target_len; j++) {
        if (!is_p[target_ri][j]) continue;
        int good = central[target_ri][j];
        if (!good) {
            for (int d = 0; d < 6; d++) {
                int ni = target_ri + dirs[d][0];
                long long nj = j + dirs[d][1];
                if (ni >= 1 && ni <= 3 && nj >= 0 && nj < row_lens[ni]) {
                    if (central[ni][nj]) {
                        good = 1;
                        break;
                    }
                }
            }
        }
        if (good) total += target_start + j;
    }

    /* Cleanup */
    for (int ri = 0; ri < 5; ri++) {
        free(is_p[ri]);
        if (central[ri]) free(central[ri]);
    }
    free(small_sieve);
    free(is_composite);

    return total;
}

int main(void) {
    long long result = sum_good_primes(5678027) + sum_good_primes(7208785);
    printf("%lld\n", result);
    return 0;
}
