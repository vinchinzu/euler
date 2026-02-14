/*
 * Project Euler Problem 169: Number of ways to express n as sum of powers of 2
 * (each power used at most twice).
 *
 * n = 10^25. Uses binary digit DP.
 */
#include <stdio.h>

/* 10^25 in binary is about 84 bits. We need to extract bits of 10^25.
 * 10^25 = (10^12)^2 * 10 = ... Let's compute it with __int128. */

typedef unsigned __int128 u128;

int main(void) {
    u128 n = 1;
    for (int i = 0; i < 25; i++) n *= 10;

    /* Extract binary digits (LSB first) */
    int bits[200];
    int nbits = 0;
    u128 temp = n;
    while (temp > 0) {
        bits[nbits++] = (int)(temp & 1);
        temp >>= 1;
    }

    /* DP: prev_ways[carry] = number of ways */
    long long prev[2] = {1, 0};

    for (int i = 0; i < nbits; i++) {
        long long next[2] = {0, 0};
        int bit = bits[i];
        for (int carry_in = 0; carry_in <= 1; carry_in++) {
            if (prev[carry_in] == 0) continue;
            for (int coeff = 0; coeff <= 2; coeff++) {
                int diff = coeff + carry_in - bit;
                if (diff == 0)
                    next[0] += prev[carry_in];
                else if (diff == 2)
                    next[1] += prev[carry_in];
            }
        }
        prev[0] = next[0];
        prev[1] = next[1];
    }

    printf("%lld\n", prev[0]);
    return 0;
}
