/*
 * Project Euler 426 - Box-Ball System
 *
 * Sum of squares of final soliton lengths.
 * Extracted from embedded C in python/426.py.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INITIAL_CAP (12000000)
#define STACK_CAP (6000000)

typedef struct {
    int start;
    int i;
    long long diff;
    int phase;
    long long res;
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

    stack = (Frame *)malloc((size_t)STACK_CAP * sizeof(Frame));
    stack_top = 0;

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

            if (start + 1 == lengths_len) {
                return_val = (long long)lengths[start] * lengths[start];
                stack_top--;
                continue;
            }

            while (1) {
                int i = f->i;
                f->diff += lengths[i];

                if (i + 1 == lengths_len) {
                    ensure_lengths_cap(lengths_len + 1);
                    lengths[lengths_len] = (int)f->diff;
                    lengths_len++;

                    f->start = start + 1;
                    f->i = start + 1;
                    f->diff = 0;
                    f->phase = 0;
                    f->res = 0;
                    break;
                } else if (f->diff <= lengths[i + 1]) {
                    f->phase = 1;

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
            f->res = return_val;

            int i = f->i;
            lengths_len = i + 1;
            ensure_lengths_cap(lengths_len + 1);
            lengths[lengths_len] = (int)f->diff;
            lengths_len++;

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
            return_val = f->res + return_val;
            stack_top--;
        }
    }

    printf("%lld\n", return_val);

    free(lengths);
    free(stack);
    return 0;
}
