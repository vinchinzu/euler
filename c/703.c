/*
 * Project Euler Problem 703: Circular Logic III.
 *
 * Build a functional graph from f on N-bit boolean tuples, where f shifts
 * right and sets the high bit to b1 AND (b2 XOR b3). Count independent set
 * colorings on each connected component.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define NN 20
#define SZ (1 << NN)
#define MOD 1001001011LL

typedef long long ll;

int next_arr[SZ];
int *prevs[SZ];
int prev_count[SZ];
int prev_cap[SZ];
int used[SZ];

typedef struct {
    ll f, t;
} Result;

/* Stack-based DFS to avoid recursion overflow */
typedef struct {
    int ptr;
    int avoid;
    int child_idx;
    ll f_val, t_val;
} Frame;

Frame stack[SZ + 10];

Result helper_iterative(int start_ptr, int start_avoid) {
    int sp = 0;
    stack[0].ptr = start_ptr;
    stack[0].avoid = start_avoid;
    stack[0].child_idx = -1;
    stack[0].f_val = 1;
    stack[0].t_val = 1;

    while (sp >= 0) {
        Frame *fr = &stack[sp];
        int ptr = fr->ptr;

        if (fr->child_idx == -1) {
            used[ptr] = 1;
            if (ptr == fr->avoid) {
                /* Return Result(1, 0) */
                if (sp == 0) {
                    return (Result){1, 0};
                }
                sp--;
                Frame *parent = &stack[sp];
                Result child_res = {1, 0};
                parent->f_val = (parent->f_val * ((child_res.f + child_res.t) % MOD)) % MOD;
                parent->t_val = (parent->t_val * child_res.f) % MOD;
                parent->child_idx++;
                continue;
            }
            fr->child_idx = 0;
        }

        if (fr->child_idx < prev_count[ptr]) {
            int child = prevs[ptr][fr->child_idx];
            /* Push child */
            sp++;
            stack[sp].ptr = child;
            stack[sp].avoid = fr->avoid;
            stack[sp].child_idx = -1;
            stack[sp].f_val = 1;
            stack[sp].t_val = 1;
            continue;
        }

        /* All children processed */
        Result res = {fr->f_val % MOD, fr->t_val % MOD};
        if (sp == 0) {
            return res;
        }
        sp--;
        Frame *parent = &stack[sp];
        parent->f_val = (parent->f_val * ((res.f + res.t) % MOD)) % MOD;
        parent->t_val = (parent->t_val * res.f) % MOD;
        parent->child_idx++;
    }

    return (Result){1, 1}; /* Should not reach here */
}

void add_prev(int node, int prev) {
    if (prev_count[node] >= prev_cap[node]) {
        prev_cap[node] = prev_cap[node] ? prev_cap[node] * 2 : 4;
        prevs[node] = realloc(prevs[node], prev_cap[node] * sizeof(int));
    }
    prevs[node][prev_count[node]++] = prev;
}

int main() {
    /* Build next array */
    for (int i = 0; i < SZ; i++) {
        int shifted = i >> 1;
        int b1 = i & 1;
        int b2 = (i >> 1) & 1;
        int b3 = (i >> 2) & 1;
        int highest_bit = b1 & (b2 ^ b3);
        next_arr[i] = shifted + (highest_bit << (NN - 1));
    }

    /* Build reverse graph */
    memset(prev_count, 0, sizeof(prev_count));
    memset(prev_cap, 0, sizeof(prev_cap));
    memset(prevs, 0, sizeof(prevs));
    for (int i = 0; i < SZ; i++) {
        add_prev(next_arr[i], i);
    }

    memset(used, 0, sizeof(used));
    ll ans = 1;

    for (int i = 0; i < SZ; i++) {
        if (!used[i]) {
            /* Find root of cycle */
            int root = i;
            while (!used[root]) {
                used[root] = 1;
                root = next_arr[root];
            }

            /* Root is false: all subtrees independent */
            ll f_val = 1;
            for (int j = 0; j < prev_count[root]; j++) {
                Result res = helper_iterative(prevs[root][j], root);
                f_val = (f_val * ((res.f + res.t) % MOD)) % MOD;
            }

            /* Root is true: next[root] must be false */
            ll t_val;
            if (next_arr[root] == root) {
                t_val = 0;
            } else {
                Result r1 = helper_iterative(next_arr[root], root);
                Result r2 = helper_iterative(root, next_arr[root]);
                t_val = (r1.f * r2.t) % MOD;
            }

            ans = (ans * ((f_val + t_val) % MOD)) % MOD;
        }
    }

    printf("%lld\n", ans);

    for (int i = 0; i < SZ; i++) {
        if (prevs[i]) free(prevs[i]);
    }
    return 0;
}
