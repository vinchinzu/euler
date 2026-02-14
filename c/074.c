/* Project Euler 074 - Digit factorial chains */
#include <stdio.h>
#include <string.h>

#define LIMIT 1000000
#define CACHE_SIZE 3000000

static const int FACTORIALS[] = {1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880};

static int cache[CACHE_SIZE];

static int sum_digit_factorials(int n) {
    int sum = 0;
    if (n == 0) return FACTORIALS[0];
    while (n > 0) {
        sum += FACTORIALS[n % 10];
        n /= 10;
    }
    return sum;
}

static int get_chain_length(int start_node) {
    if (start_node < CACHE_SIZE && cache[start_node] != 0) {
        return cache[start_node];
    }

    int path[100];
    int path_len = 0;
    int current = start_node;

    while (1) {
        if (current < CACHE_SIZE && cache[current] != 0) {
            /* Hit a cached node */
            int length_from_current = cache[current];
            for (int j = path_len - 1; j >= 0; j--) {
                int len = (path_len - j) + length_from_current;
                if (path[j] < CACHE_SIZE) {
                    cache[path[j]] = len;
                }
            }
            break;
        }

        /* Check if we've hit a cycle in our current path */
        int found_in_path = -1;
        for (int i = 0; i < path_len; i++) {
            if (path[i] == current) {
                found_in_path = i;
                break;
            }
        }

        if (found_in_path >= 0) {
            int cycle_len = path_len - found_in_path;

            /* Nodes in the cycle */
            for (int i = found_in_path; i < path_len; i++) {
                if (path[i] < CACHE_SIZE) {
                    cache[path[i]] = cycle_len;
                }
            }

            /* Nodes leading to the cycle */
            for (int i = 0; i < found_in_path; i++) {
                int len = (found_in_path - i) + cycle_len;
                if (path[i] < CACHE_SIZE) {
                    cache[path[i]] = len;
                }
            }
            break;
        }

        path[path_len++] = current;
        current = sum_digit_factorials(current);
    }

    if (start_node < CACHE_SIZE) {
        return cache[start_node];
    }
    /* Fallback: shouldn't happen for our range */
    return 0;
}

int main(void) {
    memset(cache, 0, sizeof(cache));
    int count = 0;

    for (int i = 1; i < LIMIT; i++) {
        if (get_chain_length(i) == 60) {
            count++;
        }
    }

    printf("%d\n", count);
    return 0;
}
