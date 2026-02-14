/* Project Euler Problem 121: Disc game prize fund.
 *
 * Dynamic programming to count winning weights, then compute prize fund.
 * Prize = denominator / winning_weight, where denominator = (TURNS+1)!
 */
#include <stdio.h>
#include <string.h>

#define TURNS 15

int main(void) {
    /* weights[b] = number of ways to get exactly b blue draws */
    long long weights[TURNS + 1];
    long long next_weights[TURNS + 1];
    memset(weights, 0, sizeof(weights));
    weights[0] = 1;

    for (int turn = 1; turn <= TURNS; turn++) {
        memset(next_weights, 0, sizeof(next_weights));
        for (int blue = 0; blue < turn; blue++) {
            long long w = weights[blue];
            if (w == 0) continue;
            next_weights[blue] += w * turn;      /* draw red */
            next_weights[blue + 1] += w;          /* draw blue */
        }
        memcpy(weights, next_weights, sizeof(weights));
    }

    long long winning_weight = 0;
    for (int blue = TURNS / 2 + 1; blue <= TURNS; blue++) {
        winning_weight += weights[blue];
    }

    /* denominator = product(2..TURNS+1) = (TURNS+1)! */
    long long denominator = 1;
    for (int i = 2; i <= TURNS + 1; i++) {
        denominator *= i;
    }

    printf("%lld\n", denominator / winning_weight);
    return 0;
}
