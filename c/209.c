/*
 * Project Euler 209: Circular Logic
 *
 * Find the number of boolean functions tau(a,b,c,d,e,f) such that
 * tau AND tau∘F = 0, where F(a,b,c,d,e,f) = (b,c,d,e,f, a XOR (b AND c)).
 *
 * The key insight: the 64 input values form cycles under F.
 * For each cycle of length n, the number of valid assignments is Lucas(n+2)
 * (L(1)=1, L(2)=3, L(n)=L(n-1)+L(n-2)).
 * The answer is the product over all cycles.
 */
#include <stdio.h>

int main(void) {
    int K = 6;
    int total = 1 << K;  /* 64 */

    /* Lucas-like sequence: good[1]=1, good[2]=3, good[n]=good[n-1]+good[n-2] */
    long long good[65];
    good[1] = 1;
    good[2] = 3;
    for (int i = 3; i < 65; i++)
        good[i] = good[i-1] + good[i-2];

    int seen[64] = {0};
    long long ans = 1;

    for (int bits = 0; bits < total; bits++) {
        if (seen[bits]) continue;

        int cur = bits;
        int cycle_len = 0;
        while (!seen[cur]) {
            seen[cur] = 1;
            /* Apply F: (a,b,c,d,e,f) -> (b,c,d,e,f, a^(b&c)) */
            int a = (cur >> 5) & 1;
            int b = (cur >> 4) & 1;
            int c = (cur >> 3) & 1;
            int d = (cur >> 2) & 1;
            int e = (cur >> 1) & 1;
            int f = cur & 1;
            int new_f = a ^ (b & c);
            cur = (b << 5) | (c << 4) | (d << 3) | (e << 2) | (f << 1) | new_f;
            cycle_len++;
        }

        if (cycle_len > 0)
            ans *= good[cycle_len];
    }

    printf("%lld\n", ans);
    return 0;
}
