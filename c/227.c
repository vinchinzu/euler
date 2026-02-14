/*
 * Project Euler Problem 227: The Chase
 *
 * 100 players in a circle, two opposite players have dice.
 * Find expected number of rounds until one player has both dice.
 * Uses Markov chain probability propagation.
 */
#include <stdio.h>

#define N 100
#define K 6
#define L 100000

int main(void) {
    double table[N / 2 + 1] = {0};
    table[N / 2] = 1.0;
    double ans = 0.0;
    double K2 = (double)(K * K);

    for (int rounds = 0; rounds < L; rounds++) {
        ans += rounds * table[0];
        double new_table[N / 2 + 1] = {0};

        for (int dist = 1; dist <= N / 2; dist++) {
            double t = table[dist];
            if (t == 0.0) continue;

            /* Both dice move towards each other */
            if (dist == 1)
                new_table[dist] += t / K2;
            else
                new_table[dist - 2] += t / K2;

            /* One die moves towards */
            new_table[dist - 1] += t * 2.0 * (K - 2) / K2;

            /* Stay same */
            new_table[dist] += t * (2.0 + (double)(K - 2) * (K - 2)) / K2;

            /* One die moves away */
            if (dist == N / 2)
                new_table[dist - 1] += t * 2.0 * (K - 2) / K2;
            else
                new_table[dist + 1] += t * 2.0 * (K - 2) / K2;

            /* Both dice move away */
            if (dist == N / 2)
                new_table[dist - 2] += t / K2;
            else if (dist == N / 2 - 1)
                new_table[dist] += t / K2;
            else
                new_table[dist + 2] += t / K2;
        }

        for (int i = 0; i <= N / 2; i++)
            table[i] = new_table[i];
    }

    printf("%.6f\n", ans);
    return 0;
}
