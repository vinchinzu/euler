/* Project Euler Problem 125: Palindromic sums.
 *
 * Find sum of all numbers < 10^8 that are palindromic and expressible
 * as sum of consecutive squares.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>

#define MAX_SUM_LIMIT 100000000LL

static bool is_palindrome(long long num) {
    char buf[20];
    int len = snprintf(buf, sizeof(buf), "%lld", num);
    for (int i = 0; i < len / 2; i++) {
        if (buf[i] != buf[len - 1 - i]) return false;
    }
    return true;
}

int main(void) {
    /* Use a hash set to store unique palindromic sums.
     * Since values < 10^8, we can use a simple bit array. */
    /* 10^8 bits = 12.5 MB, acceptable */
    int nbytes = (int)((MAX_SUM_LIMIT + 7) / 8);
    unsigned char *seen = calloc(nbytes, 1);
    if (!seen) return 1;

    int i_upper = (int)sqrt((double)MAX_SUM_LIMIT / 2.0) + 1;

    for (int i = 1; i <= i_upper; i++) {
        long long sum = (long long)i * i;
        for (int j = i + 1; ; j++) {
            sum += (long long)j * j;
            if (sum >= MAX_SUM_LIMIT) break;
            if (is_palindrome(sum)) {
                seen[sum / 8] |= (1 << (sum % 8));
            }
        }
    }

    long long total = 0;
    for (long long n = 1; n < MAX_SUM_LIMIT; n++) {
        if (seen[n / 8] & (1 << (n % 8))) {
            total += n;
        }
    }

    printf("%lld\n", total);
    free(seen);
    return 0;
}
