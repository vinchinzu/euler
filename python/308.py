"""Project Euler Problem 308 - An amazing prime-generating automaton.

Conway's PRIMEGAME: 14-fraction FRACTRAN program that generates primes.
We need the number of steps to reach the 10001st prime (2^p where p is prime).

Uses an embedded C state machine with 3 loop optimizations that reduce
~1.5*10^15 actual FRACTRAN steps to ~62 billion state machine iterations.

State machine encodes the FRACTRAN state n = 2^two * 3^three * 5^five * 7^seven
with 7 states corresponding to which "flag prime" (11,13,17,19,23,29) divides n.

The 14 fractions (numbered 1-14):
 1: 17/91   2: 78/85   3: 19/51   4: 23/38   5: 29/33
 6: 77/29   7: 95/23   8: 77/19   9: 1/17   10: 11/13
11: 13/11  12: 15/2   13: 1/7    14: 55/1
"""

from __future__ import annotations

import os
import subprocess
import tempfile

C_SOURCE = r"""
#include <stdio.h>
#include <stdint.h>

int main(void) {
    int64_t two = 1, three = 0, five = 0, seven = 0;
    int state = 0; /* S_ */
    int64_t steps = 0;
    int prime_count = 0;
    const int TARGET = 10001;

    while (1) {
        /* Prime detected when state=S_ with only 2^two remaining, two>1 */
        if (state == 0 && three == 0 && five == 0 && seven == 0 && two > 1) {
            prime_count++;
            if (prime_count >= TARGET) break;
        }

        switch (state) {
        case 0: /* S_ */
            if (two > 0) {
                two--; three++; five++;
                steps++;
            } else if (seven > 0) {
                seven--;
                steps++;
            } else {
                five++;
                state = 1; /* S11 */
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
                state = 2; /* S13 */
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
                state = 3; /* S17 */
                steps++;
            } else {
                state = 1; /* S11 */
                steps++;
            }
            break;

        case 3: /* S17 */
            if (five > 0) {
                five--;
                two++;
                three++;
                state = 2; /* S13 */
                steps++;
            } else if (three > 0) {
                three--;
                state = 4; /* S19 */
                steps++;
            } else {
                state = 0; /* S_ */
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
                state = 1; /* S11 */
                steps++;
            }
            break;
        }
    }

    printf("%lld\n", steps);
    return 0;
}
"""


def main() -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        c_path = os.path.join(tmpdir, "sol.c")
        bin_path = os.path.join(tmpdir, "sol")
        with open(c_path, "w") as f:
            f.write(C_SOURCE)
        subprocess.run(
            ["gcc", "-O3", "-march=native", "-o", bin_path, c_path], check=True
        )
        result = subprocess.run(
            [bin_path], capture_output=True, text=True, check=True, timeout=280
        )
    print(result.stdout.strip())


if __name__ == "__main__":
    main()
