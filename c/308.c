/*
 * Project Euler Problem 308: An amazing prime-generating automaton
 *
 * Conway's PRIMEGAME FRACTRAN program. Count steps to reach 10001st prime power of 2.
 * Uses optimized state machine with loop shortcuts.
 *
 * State machine encodes n = 2^two * 3^three * 5^five * 7^seven with 5 states.
 * All loops that can be batched are optimized away.
 */
#include <stdio.h>
#include <stdint.h>

int main(void) {
    int64_t two = 1, three = 0, five = 0, seven = 0;
    int state = 0;
    int64_t steps = 0;
    int prime_count = 0;
    const int TARGET = 10001;

    while (1) {
        if (state == 0 && three == 0 && five == 0 && seven == 0 && two > 1) {
            prime_count++;
            if (prime_count >= TARGET) break;
        }

        switch (state) {
        case 0: /* S_ */
            if (two > 0) {
                /* Optimization: batch all two iterations at once.
                 * Each step: two--, three++, five++, steps++
                 * seven doesn't change in this branch. */
                steps += two;
                three += two;
                five += two;
                two = 0;
            } else if (seven > 0) {
                seven--;
                steps++;
            } else {
                five++;
                state = 1;
                steps++;
            }
            break;

        case 1: /* S11 */
            if (three > 0) {
                /* Optimized S11<->S29 loop */
                steps += 2 * three;
                seven += three;
                three = 0;
            } else {
                state = 2;
                steps++;
            }
            break;

        case 2: /* S13 */
            if (five > 0 && seven > 0) {
                /* Optimized S13<->S17 loop */
                int64_t min_val = five < seven ? five : seven;
                steps += 2 * min_val;
                two += min_val;
                three += min_val;
                five -= min_val;
                seven -= min_val;
            }
            if (seven > 0) {
                seven--;
                state = 3;
                steps++;
            } else {
                state = 1;
                steps++;
            }
            break;

        case 3: /* S17 */
            if (five > 0) {
                five--;
                two++;
                three++;
                state = 2;
                steps++;
            } else if (three > 0) {
                three--;
                state = 4;
                steps++;
            } else {
                state = 0;
                steps++;
            }
            break;

        case 4: /* S19 */
            if (two > 0) {
                /* Optimized S19<->S23 loop */
                steps += 2 * two;
                five += two;
                two = 0;
            } else {
                seven++;
                state = 1;
                steps++;
            }
            break;
        }
    }

    printf("%lld\n", steps);
    return 0;
}
