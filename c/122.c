/* Project Euler Problem 122: Efficient exponentiation.
 *
 * Find sum of m(k) for k=1..200, where m(k) is minimum multiplications
 * to compute n^k using addition chains.
 * Uses iterative deepening DFS (IDDFS) on addition chains.
 */
#include <stdio.h>
#include <string.h>

#define LIMIT 200

int m[LIMIT + 1];  /* m[k] = minimum multiplications for n^k */
int chain[100];    /* current addition chain */

static void dfs(int depth, int max_depth, int chain_len) {
    int top = chain[chain_len - 1];

    if (depth > max_depth) return;
    if (top > LIMIT) return;

    /* Update m[top] if this is better */
    if (depth < m[top]) {
        m[top] = depth;
    }

    /* Pruning: even if we double at every step, can we reach anything useful? */
    /* Max reachable = top * 2^(max_depth - depth) */
    {
        long long max_reach = top;
        for (int i = 0; i < max_depth - depth; i++) {
            max_reach *= 2;
            if (max_reach > LIMIT) break;
        }
        /* If max_reach is still less than smallest unset m[], we might skip,
           but this is hard to check efficiently. Just rely on depth limit. */
    }

    if (depth == max_depth) return;

    /* Try all pairs from the chain (use star chains: always include top) */
    for (int i = chain_len - 1; i >= 0; i--) {
        int new_val = top + chain[i];
        if (new_val > LIMIT) continue;

        chain[chain_len] = new_val;
        dfs(depth + 1, max_depth, chain_len + 1);
    }
}

int main(void) {
    /* Initialize m[] to large values */
    for (int i = 0; i <= LIMIT; i++) m[i] = 999;
    m[1] = 0;

    chain[0] = 1;

    /* IDDFS: increase max depth */
    for (int max_depth = 1; max_depth <= 11; max_depth++) {
        dfs(0, max_depth, 1);
    }

    int total = 0;
    for (int k = 1; k <= LIMIT; k++) {
        total += m[k];
    }

    printf("%d\n", total);
    return 0;
}
