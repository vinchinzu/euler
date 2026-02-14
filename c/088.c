#include <stdio.h>
#include <string.h>

#define K_MAX 12000
#define ULIMIT (2 * K_MAX)

int min_n[K_MAX + 1];

int imin(int a, int b) { return a < b ? a : b; }

void search(int prod, int sum, int num_factors, int start) {
    int k = prod - sum + num_factors;
    if (k >= 2 && k <= K_MAX)
        min_n[k] = imin(min_n[k], prod);

    for (int i = start; i <= ULIMIT / prod; i++)
        search(prod * i, sum + i, num_factors + 1, i);
}

int main(void) {
    for (int i = 0; i <= K_MAX; i++)
        min_n[i] = ULIMIT;

    search(1, 0, 0, 2);

    /* Collect unique values and sum them */
    /* Use a bit array since values are <= ULIMIT */
    unsigned char seen[ULIMIT / 8 + 2];
    memset(seen, 0, sizeof(seen));

    long long total = 0;
    for (int k = 2; k <= K_MAX; k++) {
        int v = min_n[k];
        int byte_idx = v >> 3;
        int bit_idx = v & 7;
        if (!((seen[byte_idx] >> bit_idx) & 1)) {
            seen[byte_idx] |= (1 << bit_idx);
            total += v;
        }
    }

    printf("%lld\n", total);
    return 0;
}
