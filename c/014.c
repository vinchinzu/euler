#include <stdio.h>

/* Iterative Collatz length to avoid stack overflow */
int main(void) {
    /* Use a cache for numbers < 1000000 */
    #define LIMIT 1000000
    static unsigned int cache[LIMIT];
    cache[1] = 1;

    unsigned long long max_length = 0;
    unsigned long long starting = 1;

    for (unsigned long long i = 2; i < LIMIT; i++) {
        unsigned long long n = i;
        unsigned int len = 0;
        while (n >= (unsigned long long)LIMIT || cache[n] == 0) {
            len++;
            if (n % 2 == 0)
                n = n / 2;
            else
                n = 3 * n + 1;
        }
        len += cache[n];
        cache[i] = len;
        if (len > max_length) {
            max_length = len;
            starting = i;
        }
    }
    printf("%llu\n", starting);
    return 0;
}
