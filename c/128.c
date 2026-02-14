/* Project Euler Problem 128: Hexagonal tile differences.
 *
 * Find the 2000th tile n for which PD(n) = 3.
 * Only two positions per ring can have PD=3: type A and type B.
 * Type A: n = 3r^2 - 3r + 2, needs 6r-1, 6r+1, 12r+5 all prime.
 * Type B: n = 3r^2 + 3r + 1, needs 6r-1, 6r+5, 12r-7 all prime (r >= 2).
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>

#define MAX_RING 70000
#define MAX_PRIME (12 * MAX_RING + 5)

int main(void) {
    /* Build sieve */
    bool *sieve = calloc(MAX_PRIME + 1, sizeof(bool));
    if (!sieve) return 1;
    for (int i = 2; i <= MAX_PRIME; i++) sieve[i] = true;
    for (int i = 2; (long long)i * i <= MAX_PRIME; i++) {
        if (sieve[i]) {
            for (int j = i * i; j <= MAX_PRIME; j += i)
                sieve[j] = false;
        }
    }

    int target = 2000;
    int count = 2; /* tiles 1 and 2 both have PD=3 */
    if (target <= 2) {
        printf("%d\n", target == 1 ? 1 : 2);
        free(sieve);
        return 0;
    }

    for (int r = 2; r < MAX_RING; r++) {
        /* Type A: n = 3r^2 - 3r + 2 */
        if (sieve[6 * r - 1] && sieve[6 * r + 1] && sieve[12 * r + 5]) {
            count++;
            if (count == target) {
                long long n = 3LL * r * r - 3 * r + 2;
                printf("%lld\n", n);
                free(sieve);
                return 0;
            }
        }

        /* Type B: n = 3r^2 + 3r + 1 */
        if (sieve[6 * r - 1] && sieve[6 * r + 5] && sieve[12 * r - 7]) {
            count++;
            if (count == target) {
                long long n = 3LL * r * r + 3 * r + 1;
                printf("%lld\n", n);
                free(sieve);
                return 0;
            }
        }
    }

    free(sieve);
    return 1;
}
