/*
 * Project Euler 031 - Coin Sums
 * How many ways can 2 pounds be made using any number of coins?
 */
#include <stdio.h>

int main(void) {
    const int TARGET = 200;
    int coins[] = {1, 2, 5, 10, 20, 50, 100, 200};
    int num_coins = 8;

    long long ways[201] = {0};
    ways[0] = 1;

    for (int c = 0; c < num_coins; c++) {
        for (int amount = coins[c]; amount <= TARGET; amount++) {
            ways[amount] += ways[amount - coins[c]];
        }
    }

    printf("%lld\n", ways[TARGET]);
    return 0;
}
