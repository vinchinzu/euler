#!/usr/bin/env python3
"""Project Euler Problem 426: Box-Ball System.

Find the sum of squares of the final soliton lengths in a BBS.
Uses C for performance with iterative stack to avoid recursion limits.
"""
import os
import subprocess
import tempfile

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * The helper function computes the sum of squares of final soliton lengths.
 * Original recursive logic:
 *
 * helper(start, lengths):
 *   if start+1 == len(lengths): return lengths[start]^2
 *   diff = 0; i = start
 *   loop:
 *     diff += lengths[i]
 *     if i+1 == len(lengths):
 *       lengths.append(diff)
 *       return helper(start+1, lengths)
 *     elif diff <= lengths[i+1]:
 *       res = helper(i+2, lengths)
 *       truncate lengths to i+1, append diff
 *       return res + helper(start+1, lengths)
 *     diff -= lengths[i+1]
 *     i += 2
 *
 * We convert this to an iterative approach with an explicit stack.
 */

#define INITIAL_CAP (12000000)
#define STACK_CAP (6000000)

typedef struct {
    int start;
    int i;             /* current i in the while loop */
    long long diff;
    int phase;         /* 0 = just entered, 1 = returned from helper(i+2), 2 = returned from helper(start+1) */
    long long res;     /* result from helper(i+2) */
} Frame;

static int *lengths;
static int lengths_len;
static int lengths_cap;

static Frame *stack;
static int stack_top;

static void ensure_lengths_cap(int needed) {
    if (needed > lengths_cap) {
        while (lengths_cap < needed) lengths_cap *= 2;
        lengths = (int *)realloc(lengths, (size_t)lengths_cap * sizeof(int));
    }
}

int main() {
    int N = 10000000;

    /* Generate sequence: s_0 = 290797, s_{k+1} = s_k^2 mod 50515093, t_k = (s_k % 64) + 1 */
    lengths_cap = INITIAL_CAP;
    lengths = (int *)malloc((size_t)lengths_cap * sizeof(int));
    lengths_len = N + 1;

    {
        long long s = 290797;
        long long mod = 50515093;
        for (int idx = 0; idx <= N; idx++) {
            lengths[idx] = (int)(s % 64) + 1;
            s = (s * s) % mod;
        }
    }

    /* Iterative helper using explicit stack */
    stack = (Frame *)malloc((size_t)STACK_CAP * sizeof(Frame));
    stack_top = 0;

    /* Push initial frame: helper(0, lengths) */
    stack[0].start = 0;
    stack[0].i = 0;
    stack[0].diff = 0;
    stack[0].phase = 0;
    stack[0].res = 0;
    stack_top = 1;

    long long return_val = 0;

    while (stack_top > 0) {
        Frame *f = &stack[stack_top - 1];

        if (f->phase == 0) {
            int start = f->start;

            /* Base case */
            if (start + 1 == lengths_len) {
                return_val = (long long)lengths[start] * lengths[start];
                stack_top--;
                continue;
            }

            /* Continue the while-true loop */
            while (1) {
                int i = f->i;
                f->diff += lengths[i];

                if (i + 1 == lengths_len) {
                    /* lengths.append(diff) */
                    ensure_lengths_cap(lengths_len + 1);
                    lengths[lengths_len] = (int)f->diff;
                    lengths_len++;

                    /* tail call: helper(start+1, lengths) */
                    f->start = start + 1;
                    f->i = start + 1;
                    f->diff = 0;
                    f->phase = 0;
                    f->res = 0;
                    break;
                } else if (f->diff <= lengths[i + 1]) {
                    /* Need to call helper(i+2, lengths), then after return,
                       truncate and call helper(start+1, lengths) */
                    f->phase = 1;

                    /* Push new frame for helper(i+2, lengths) */
                    if (stack_top >= STACK_CAP) {
                        fprintf(stderr, "Stack overflow at %d\n", stack_top);
                        return 1;
                    }
                    Frame *nf = &stack[stack_top];
                    nf->start = i + 2;
                    nf->i = i + 2;
                    nf->diff = 0;
                    nf->phase = 0;
                    nf->res = 0;
                    stack_top++;
                    break;
                }

                f->diff -= lengths[i + 1];
                f->i = i + 2;
            }
        } else if (f->phase == 1) {
            /* Returned from helper(i+2, lengths) */
            f->res = return_val;

            /* Truncate lengths to i+1, append diff */
            int i = f->i;
            lengths_len = i + 1;
            ensure_lengths_cap(lengths_len + 1);
            lengths[lengths_len] = (int)f->diff;
            lengths_len++;

            /* Now call helper(start+1, lengths) */
            f->phase = 2;

            if (stack_top >= STACK_CAP) {
                fprintf(stderr, "Stack overflow at %d\n", stack_top);
                return 1;
            }
            int start = f->start;
            Frame *nf = &stack[stack_top];
            nf->start = start + 1;
            nf->i = start + 1;
            nf->diff = 0;
            nf->phase = 0;
            nf->res = 0;
            stack_top++;
        } else if (f->phase == 2) {
            /* Returned from helper(start+1, lengths) */
            return_val = f->res + return_val;
            stack_top--;
        }
    }

    printf("%lld\n", return_val);

    free(lengths);
    free(stack);
    return 0;
}
""";

def solve():
    tmpdir = tempfile.mkdtemp()
    c_file = os.path.join(tmpdir, "p426.c")
    exe_file = os.path.join(tmpdir, "p426")

    with open(c_file, "w") as f:
        f.write(C_CODE)

    subprocess.run(
        ["gcc", "-O2", "-o", exe_file, c_file, "-lm"],
        check=True, capture_output=True
    )

    result = subprocess.run(
        [exe_file], capture_output=True, text=True, check=True,
        timeout=120
    )
    return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
