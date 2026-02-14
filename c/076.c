/* Project Euler 076 - Counting summations */
#include <stdio.h>

int main(void) {
    int target = 100;
    long long ways[101] = {0};
    ways[0] = 1;

    for (int part = 1; part < target; part++) {
        for (int s = part; s <= target; s++) {
            ways[s] += ways[s - part];
        }
    }

    printf("%lld\n", ways[target]);
    return 0;
}
