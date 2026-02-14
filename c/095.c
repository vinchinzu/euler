#include <stdio.h>

#define LIMIT 1000000

static int sum_of_divisors[LIMIT + 1];

void compute_divisor_sums(void) {
    for (int i = 0; i <= LIMIT; i++) sum_of_divisors[i] = 0;
    for (int i = 1; i <= LIMIT; i++)
        for (int j = 2 * i; j <= LIMIT; j += i)
            sum_of_divisors[j] += i;
}

int main(void) {
    compute_divisor_sums();

    int longest_chain_length = 0;
    int smallest_member = 0;

    for (int start = 2; start <= LIMIT; start++) {
        /* Follow the chain, tracking up to a maximum length */
        int chain[100];
        int chain_len = 0;
        int current = start;
        int found_cycle = 0;

        while (chain_len < 100) {
            if (current > LIMIT || current <= 0) break;

            /* Check if we've seen this number before in the chain */
            int seen = 0;
            for (int i = 0; i < chain_len; i++) {
                if (chain[i] == current) {
                    seen = 1;
                    break;
                }
            }
            if (seen) break;

            chain[chain_len++] = current;
            current = sum_of_divisors[current];
        }

        /* Check if we formed a cycle back to start */
        if (current == start && chain_len > 1) {
            if (chain_len > longest_chain_length) {
                longest_chain_length = chain_len;
                smallest_member = start;
                for (int i = 0; i < chain_len; i++)
                    if (chain[i] < smallest_member)
                        smallest_member = chain[i];
            }
        }
    }

    printf("%d\n", smallest_member);
    return 0;
}
