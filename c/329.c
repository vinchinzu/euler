/*
 * Project Euler Problem 329 - Prime Frog
 *
 * A frog jumps on squares 1..500. It starts uniformly at random on one
 * square, then at each step jumps to a random adjacent square (at endpoints
 * it must move inward). On each square it croaks 'P' (prime) with prob 2/3
 * if prime, 1/3 if not, and 'N' (not prime) with the complementary prob.
 *
 * Given the sequence PPPPNNPPPNPPNPN (length 15), compute the probability.
 * Answer as a reduced fraction.
 *
 * We use exact integer arithmetic with GCD reduction.
 */
#include <stdio.h>
#include <string.h>

#define NSQUARES 500
#define SEQLEN 15

typedef __int128 i128;

/* Sieve of Eratosthenes up to NSQUARES */
int is_prime[NSQUARES + 1];

void sieve(void) {
    memset(is_prime, 0, sizeof(is_prime));
    for (int i = 2; i <= NSQUARES; i++) is_prime[i] = 1;
    for (int i = 2; i * i <= NSQUARES; i++)
        if (is_prime[i])
            for (int j = i * i; j <= NSQUARES; j += i)
                is_prime[j] = 0;
}

i128 gcd128(i128 a, i128 b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { i128 t = b; b = a % b; a = t; }
    return a;
}

/* dp[i] = probability of producing the observed sequence starting from step t
 * while on square i. We work with numerators (denominator is a power of 3*2=6). */

/* Actually, let's track numerator and denominator separately.
 * At each step, probability branches by 1/2 (neighbor choice) and 2/3 or 1/3 (croak).
 * So denominator multiplies by 6 at each step, and by 500 for the initial position.
 *
 * Total denominator = 500 * 6^15 = 500 * 470184984576 */

/* Work with numerator arrays. Denominator = 3^SEQLEN for the croak part,
 * 2^(SEQLEN-1) for the movement part (first step has no movement, but we need
 * transitions for SEQLEN-1 steps), plus 500 for uniform start.
 *
 * Actually: Let prob[i] = probability of generating the sequence given frog starts
 * on square i at step 0.
 *
 * Transition: from square i, next square is i-1 or i+1 with prob 1/2 each
 * (except endpoints: square 1 -> square 2 with prob 1, square 500 -> 499 with prob 1).
 *
 * Croak probability: if square i is prime and sequence[t]='P': 2/3, else 1/3.
 *
 * We process from the last character backward.
 * dp[t][i] = probability of matching sequence[t..SEQLEN-1] starting from square i at step t.
 *
 * Base: dp[SEQLEN][i] = 1 for all i.
 * Step: dp[t][i] = croak_prob(i, seq[t]) * sum over neighbors j of (transition_prob * dp[t+1][j])
 *
 * We use rational arithmetic with common denominator.
 * At each step, denominator multiplies by 6 (3 for croak, 2 for movement).
 * Exception: endpoints only have 1 neighbor, so movement is deterministic (denom 1 not 2).
 * This complicates having a single denominator.
 *
 * Alternative: Use integer numerators with denominator = 3^SEQLEN * 2^(SEQLEN-1) * ... no.
 *
 * Simplest approach: Use fractions with a large common denominator.
 * At each backward step:
 * - Multiply croak probability (2/3 or 1/3).
 * - Average over neighbors (divide by 2 for interior, or just take the one neighbor for endpoints).
 *
 * Let's use numerator/denominator = num[i] / (3^t * 2^movement_denom).
 * Actually this gets messy with endpoints.
 *
 * Cleaner: work in double-precision for verification, but compute exactly.
 * Use numerator array where all values share denominator = 3^15 * 500 * 2^13.
 * Wait, endpoints mess up the 2 factor.
 *
 * Simplest exact approach: Track numerator and a common denominator.
 * Process backward. At step t:
 * - new_num[i] = croak_num(i,seq[t]) * sum_neighbor_num(i) / croak_denom
 *   where sum_neighbor_num for interior = (dp[i-1] + dp[i+1]), denom *= 2
 *   for endpoint = dp[only_neighbor], denom *= 1
 *
 * To keep a single denominator, multiply all by 2 at each step (interior is natural,
 * endpoint gets doubled). Then endpoint contribution = 2 * dp[only_neighbor].
 *
 * So at each backward step: common_denom *= 6.
 * new_num[i] = croak_numerator * movement_sum
 * where movement_sum = dp[i-1]+dp[i+1] for interior, = 2*dp[only_neighbor] for endpoint.
 * croak_numerator = 2 if (is_prime[i] and seq[t]=='P') or (!is_prime[i] and seq[t]=='N'), else 1.
 * This accounts for the 2/3 vs 1/3 with the factor of 3 absorbed into denom.
 *
 * Initial: dp[i] = 1 for all i. Common denominator = 1.
 * After SEQLEN backward steps: common denominator = 6^SEQLEN.
 * Total probability = sum(dp[i]) / (500 * common_denom).
 */

i128 dp_arr[NSQUARES + 2];  /* 1-indexed: dp_arr[1..500] */
i128 tmp[NSQUARES + 2];

int main(void) {
    sieve();

    const char *seq = "PPPPNNPPPNPPNPN";

    /* Initialize dp to 1 for all squares */
    for (int i = 1; i <= NSQUARES; i++) dp_arr[i] = 1;

    /* Process sequence backward */
    for (int t = SEQLEN - 1; t >= 0; t--) {
        char ch = seq[t];
        for (int i = 1; i <= NSQUARES; i++) {
            /* Movement sum (with factor of 2 for uniform treatment) */
            i128 msum;
            if (i == 1) {
                msum = 2 * dp_arr[2];
            } else if (i == NSQUARES) {
                msum = 2 * dp_arr[NSQUARES - 1];
            } else {
                msum = dp_arr[i - 1] + dp_arr[i + 1];
            }

            /* Croak factor: 2 if match, 1 if not */
            int match;
            if (ch == 'P')
                match = is_prime[i];
            else
                match = !is_prime[i];

            tmp[i] = msum * (match ? 2 : 1);
        }
        for (int i = 1; i <= NSQUARES; i++) dp_arr[i] = tmp[i];
    }

    /* Total numerator */
    i128 num = 0;
    for (int i = 1; i <= NSQUARES; i++) num += dp_arr[i];

    /* Denominator = 500 * 6^15 */
    i128 den = 500;
    for (int t = 0; t < SEQLEN; t++) den *= 6;

    /* Reduce fraction */
    i128 g = gcd128(num, den);
    num /= g;
    den /= g;

    /* Print as fraction */
    /* num and den should fit in long long after reduction */
    /* Actually they might be large. Let's print carefully. */
    /* Expected: 199740353/29386561536000 */

    /* Convert i128 to string */
    char buf_num[100], buf_den[100];
    int pos;

    /* Print numerator */
    i128 n_copy = num;
    pos = 0;
    if (n_copy == 0) {
        buf_num[pos++] = '0';
    } else {
        char rev[100];
        int rpos = 0;
        while (n_copy > 0) {
            rev[rpos++] = '0' + (int)(n_copy % 10);
            n_copy /= 10;
        }
        for (int i = rpos - 1; i >= 0; i--)
            buf_num[pos++] = rev[i];
    }
    buf_num[pos] = '\0';

    /* Print denominator */
    i128 d_copy = den;
    pos = 0;
    if (d_copy == 0) {
        buf_den[pos++] = '0';
    } else {
        char rev[100];
        int rpos = 0;
        while (d_copy > 0) {
            rev[rpos++] = '0' + (int)(d_copy % 10);
            d_copy /= 10;
        }
        for (int i = rpos - 1; i >= 0; i--)
            buf_den[pos++] = rev[i];
    }
    buf_den[pos] = '\0';

    printf("%s/%s\n", buf_num, buf_den);
    return 0;
}
