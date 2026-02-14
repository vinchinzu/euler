/*
 * Project Euler 640: Shut the Box
 *
 * Bob has cards 1..12. Each turn rolls two 6-sided dice (x,y), must toggle
 * exactly one card from {x, y, x+y}. Optimal strategy minimizes expected
 * turns to flip all cards.
 *
 * Value iteration with Gauss-Seidel updates over 4096 states.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <math.h>

#define ND 6
#define CARDS 12
#define GOAL ((1 << CARDS) - 1)
#define NUM_STATES (1 << CARDS)

double E[NUM_STATES];

int main() {
    int s, x, y, c, iteration;
    double total, best, val, new_val, max_change;

    for (s = 0; s < NUM_STATES; s++) {
        E[s] = (s == GOAL) ? 0.0 : 100.0;
    }

    for (iteration = 0; iteration < 1000000; iteration++) {
        max_change = 0.0;
        for (s = 0; s < NUM_STATES; s++) {
            if (s == GOAL) continue;

            total = 0.0;
            for (x = 1; x <= ND; x++) {
                for (y = 1; y <= ND; y++) {
                    int opts[3] = {x, y, x + y};
                    best = 1e18;
                    for (int i = 0; i < 3; i++) {
                        c = opts[i];
                        if (c >= 1 && c <= CARDS) {
                            val = E[s ^ (1 << (c - 1))];
                            if (val < best) best = val;
                        }
                    }
                    total += best;
                }
            }

            new_val = 1.0 + total / (ND * ND);
            double change = fabs(new_val - E[s]);
            if (change > max_change) max_change = change;
            E[s] = new_val;
        }

        if (max_change < 1e-12) break;
    }

    printf("%.6f\n", E[0]);
    return 0;
}
