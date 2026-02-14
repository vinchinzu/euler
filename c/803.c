/*
 * Project Euler Problem 803: Pseudorandom Sequence.
 *
 * LCG: a_{n+1} = (25214903917 * a_n + 11) mod 2^48
 * b_n = floor(a_n / 2^16) mod 52
 * Map: 0-25 -> a-z, 26-51 -> A-Z
 *
 * Find index of first occurrence of "LuckyText" given sequence starts with "PuzzleOne".
 */
#include <stdio.h>
#include <string.h>
#include <stdint.h>

typedef unsigned long long ull;

#define MOD (1ULL << 48)
#define MASK (MOD - 1)
#define L (1 << 16)
#define MULT 25214903917ULL
#define INC 11ULL

static int char_to_code(char c) {
    if (c >= 'a' && c <= 'z') return c - 'a';
    return c - 'A' + 26;
}

static ull next_val(ull a) {
    return (MULT * a + INC) & MASK;
}

static int find_r(int *codes, int len) {
    for (int r = 0; r < L; r++) {
        ull a = r;
        int good = 1;
        for (int i = 1; i < len; i++) {
            a = next_val(a % L);
            if (((a / L + codes[i - 1] - codes[i]) % 4 + 4) % 4 != 0) {
                good = 0;
                break;
            }
        }
        if (good) return r;
    }
    return -1;
}

static int is_substring(ull a, int *codes, int len) {
    for (int i = 0; i < len; i++) {
        if ((int)((a / L) % 52) != codes[i]) return 0;
        a = next_val(a);
    }
    return 1;
}

int main(void) {
    const char *S_str = "PuzzleOne";
    const char *T_str = "LuckyText";
    int S[9], T[9];
    int slen = (int)strlen(S_str);
    int tlen = (int)strlen(T_str);

    for (int i = 0; i < slen; i++) S[i] = char_to_code(S_str[i]);
    for (int i = 0; i < tlen; i++) T[i] = char_to_code(T_str[i]);

    /* Find starting value a such that sequence starts with S */
    int r_s = find_r(S, slen);
    ull a = (ull)S[0] * L + r_s;
    while (!is_substring(a, S, slen)) {
        a += 52 * L;
    }

    /* Find remainder for T */
    int r_t = find_r(T, tlen);
    long long ans = 0;
    while ((int)(a % L) != r_t) {
        a = next_val(a);
        ans++;
    }

    /* Compute coefficients for a_{n+L} */
    ull c0 = 0, c1 = 1;
    for (int i = 0; i < L; i++) c0 = next_val(c0);
    for (int i = 0; i < L; i++) c1 = next_val(c1);

    /* c1 - c0 is the multiplier, c0 is the addend for the step-by-L map */
    ull step_mult = (c1 - c0) & MASK;
    ull step_add = c0;

    /* Find substring T */
    while (!is_substring(a, T, tlen)) {
        a = (step_mult * a + step_add) & MASK;
        ans += L;
    }

    printf("%lld\n", ans);
    return 0;
}
