/*
 * Project Euler Problem 316: Numbers in decimal expansions
 *
 * g(s) = expected position of first occurrence of s's digits in random decimal.
 * Sum g(floor(10^16/n)) for n=2..999999.
 *
 * g(s) = sum of 10^i where prefix of length i equals suffix of length i, minus len(s).
 * Uses __int128 for large power of 10 values.
 */
#include <stdio.h>
#include <string.h>

typedef unsigned __int128 u128;
typedef long long ll;

int main(void) {
    ll N = 10000000000000000LL; /* 10^16 */
    int K = 999999;

    /* Precompute powers of 10 up to 18 */
    u128 pow10[20];
    pow10[0] = 1;
    for (int i = 1; i < 20; i++) pow10[i] = pow10[i-1] * 10;

    u128 total = 0;

    for (int n = 2; n <= K; n++) {
        ll val = N / n;

        /* Convert val to string of digits */
        char s[20];
        int L = 0;
        {
            ll v = val;
            if (v == 0) { s[0] = '0'; L = 1; }
            else {
                char tmp[20];
                int tl = 0;
                while (v > 0) { tmp[tl++] = '0' + (int)(v % 10); v /= 10; }
                for (int i = 0; i < tl; i++) s[i] = tmp[tl - 1 - i];
                L = tl;
            }
            s[L] = '\0';
        }

        /* For each i from 0 to L, check if prefix of length i == suffix of length i */
        for (int i = 0; i <= L; i++) {
            /* prefix s[0..i-1] == suffix s[L-i..L-1] */
            int match = 1;
            for (int j = 0; j < i; j++) {
                if (s[j] != s[L - i + j]) { match = 0; break; }
            }
            if (match) {
                total += pow10[i];
            }
        }
        total -= L;
    }

    /* Print u128 */
    /* total fits in a long long since answer is ~5.4*10^17 */
    ll result = (ll)total;
    printf("%lld\n", result);
    return 0;
}
