/*
 * Project Euler 145 - Reversible numbers below one billion
 *
 * Count n where n + reverse(n) has only odd digits.
 * Uses recursive digit-pair approach.
 */
#include <stdio.h>
#include <stdbool.h>

static bool do_backward(int *psums, int init_c, int np) {
    int c = init_c;
    for (int i = np - 1; i >= 0; i--) {
        int s = psums[i] + c;
        if ((s % 10) % 2 == 0) return false;
        c = s / 10;
    }
    return true;
}

static int rec(int layer, int cin, int *psums, int np, int is_odd) {
    if (layer == np) {
        int c = 0;
        if (!is_odd) {
            c = do_backward(psums, cin, np) ? 1 : 0;
        } else {
            int minm = (np == 0) ? 1 : 0;
            for (int m = minm; m < 10; m++) {
                int sm = 2 * m + cin;
                if ((sm % 10) % 2 == 1) {
                    int com = sm / 10;
                    if (do_backward(psums, com, np)) c++;
                }
            }
        }
        return c;
    }

    int mina = (layer == 0) ? 1 : 0;
    int minb = (layer == 0) ? 1 : 0;
    int cc = 0;
    for (int a = mina; a < 10; a++) {
        for (int b = minb; b < 10; b++) {
            int psu = a + b;
            int sf = psu + cin;
            if ((sf % 10) % 2 == 1) {
                int cou = sf / 10;
                psums[layer] = psu;
                cc += rec(layer + 1, cou, psums, np, is_odd);
            }
        }
    }
    return cc;
}

static int count_for_len(int length) {
    int np = length / 2;
    int is_odd = length % 2;
    int psums[10]; /* max np = 4 for length 9 */
    return rec(0, 0, psums, np, is_odd);
}

int main(void) {
    int total = 0;
    for (int length = 1; length <= 9; length++) {
        total += count_for_len(length);
    }
    printf("%d\n", total);
    return 0;
}
