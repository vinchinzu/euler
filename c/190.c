/* Project Euler 190: Maximising a weighted product.
   P(m) = prod_{k=1}^{m} (2k/(m+1))^k, floor it, sum for m=2..15.
   We use long double arithmetic for sufficient precision. */
#include <stdio.h>
#include <math.h>

int main(void) {
    long long total = 0;
    for (int m = 2; m <= 15; m++) {
        long double log_val = 0.0L;
        for (int k = 1; k <= m; k++) {
            log_val += k * logl(2.0L * k / (m + 1));
        }
        long long floored = (long long)expl(log_val);
        /* Check if we undershoot by 1 due to floating point */
        long double check = expl(log_val);
        if (check - floored > 0.9999999L) floored++;
        total += floored;
    }
    printf("%lld\n", total);
    return 0;
}
