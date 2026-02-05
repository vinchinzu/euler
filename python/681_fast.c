/*
 * C implementation for Project Euler Problem 681 - Optimized
 * Using prime factorization for efficient divisor generation
 */

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdint.h>
#include <string.h>

#define MAX_PRIMES 100
#define MAX_DIVISORS 100000

typedef struct {
    int primes[MAX_PRIMES];
    int powers[MAX_PRIMES];
    int count;
} PrimeFactors;

typedef struct {
    int64_t *data;
    int count;
    int capacity;
} DivisorList;

// Get prime factorization of n
PrimeFactors get_prime_factors(int n) {
    PrimeFactors pf;
    pf.count = 0;

    int d = 2;
    while (d * d <= n) {
        if (n % d == 0) {
            pf.primes[pf.count] = d;
            pf.powers[pf.count] = 0;
            while (n % d == 0) {
                pf.powers[pf.count]++;
                n /= d;
            }
            pf.count++;
        }
        d++;
    }
    if (n > 1) {
        pf.primes[pf.count] = n;
        pf.powers[pf.count] = 1;
        pf.count++;
    }

    return pf;
}

// Compare function for qsort
int compare_int64(const void *a, const void *b) {
    int64_t diff = *(int64_t*)a - *(int64_t*)b;
    return (diff > 0) - (diff < 0);
}

// Generate all divisors from prime factorization
DivisorList get_divisors_from_factors(PrimeFactors pf) {
    DivisorList dl;
    dl.capacity = MAX_DIVISORS;
    dl.data = (int64_t*)malloc(dl.capacity * sizeof(int64_t));
    dl.count = 1;
    dl.data[0] = 1;

    for (int i = 0; i < pf.count; i++) {
        int prime = pf.primes[i];
        int power = pf.powers[i];
        int old_count = dl.count;

        int64_t prime_pow = 1;
        for (int p = 0; p <= power; p++) {
            for (int j = 0; j < old_count; j++) {
                if (p > 0) {
                    dl.data[dl.count++] = dl.data[j] * prime_pow;
                }
            }
            prime_pow *= prime;
        }
    }

    // Sort divisors
    qsort(dl.data, dl.count, sizeof(int64_t), compare_int64);

    return dl;
}

// Binary search for smallest index where data[index] >= target
int binary_search_ge(int64_t *data, int count, int64_t target) {
    int left = 0, right = count - 1;
    int result = count;

    while (left <= right) {
        int mid = (left + right) / 2;
        if (data[mid] >= target) {
            result = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    return result;
}

int64_t solve_681(int N) {
    int64_t total = 0;

    for (int K = 1; K <= N; K++) {
        // Get prime factorization of K
        PrimeFactors pf = get_prime_factors(K);

        // Double all powers to get K²
        for (int i = 0; i < pf.count; i++) {
            pf.powers[i] *= 2;
        }

        // Generate divisors of K²
        DivisorList divs = get_divisors_from_factors(pf);
        int64_t K_squared = (int64_t)K * K;

        // Iterate over divisor pairs (d, c)
        for (int di = 0; di < divs.count; di++) {
            int64_t d = divs.data[di];

            // Early termination: d^4 > K²
            if (d * d * d * d > K_squared) {
                break;
            }

            for (int ci = di; ci < divs.count; ci++) {
                int64_t c = divs.data[ci];

                // Early termination: d * c^3 > K²
                if (d * c * c * c > K_squared) {
                    break;
                }

                // Calculate ab = K² / (c * d)
                int64_t ab = K_squared / (c * d);
                if (c * d * ab != K_squared) {
                    continue;
                }

                // Find starting index for a >= ceil(sqrt(ab))
                int64_t sqrt_ab = (int64_t)sqrt((double)ab);
                int64_t min_a = (sqrt_ab * sqrt_ab == ab) ? sqrt_ab : sqrt_ab + 1;
                int start_ai = binary_search_ge(divs.data, divs.count, min_a);

                for (int ai = start_ai; ai < divs.count; ai++) {
                    int64_t a = divs.data[ai];

                    if (ab % a != 0) {
                        continue;
                    }

                    int64_t b = ab / a;
                    int64_t sum = a + b + c + d;

                    // Check constraints
                    if (b < c || sum <= 2 * a) {
                        break;
                    }

                    // Check parity
                    if (sum % 2 == 0) {
                        total += sum;
                    }
                }
            }
        }

        free(divs.data);

        // Progress indicator
        if (K % 10000 == 0) {
            fprintf(stderr, "Progress: K = %d/%d, total = %lld\n", K, N, total);
        }
    }

    return total;
}

int main() {
    // Test cases
    int64_t result_10 = solve_681(10);
    fprintf(stderr, "SP(10) = %lld (expected 186)\n", result_10);
    if (result_10 != 186) {
        fprintf(stderr, "ERROR: Test failed!\n");
        return 1;
    }

    int64_t result_100 = solve_681(100);
    fprintf(stderr, "SP(100) = %lld (expected 23238)\n", result_100);
    if (result_100 != 23238) {
        fprintf(stderr, "ERROR: Test failed!\n");
        return 1;
    }

    // Main problem
    fprintf(stderr, "Computing SP(1000000)...\n");
    int64_t result = solve_681(1000000);
    printf("%lld\n", result);

    return 0;
}
