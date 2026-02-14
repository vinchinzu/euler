#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>

#define MAX_WORDS 3000
#define MAX_WORD_LEN 20

char words[MAX_WORDS][MAX_WORD_LEN];
int nwords = 0;

void load_words(const char *filename) {
    FILE *f = fopen(filename, "r");
    if (!f) { fprintf(stderr, "Cannot open %s\n", filename); exit(1); }

    int ch;
    char word[MAX_WORD_LEN];
    int wlen = 0;
    bool in_word = false;

    while ((ch = fgetc(f)) != EOF) {
        if (ch == '"') {
            if (in_word && wlen > 0) {
                word[wlen] = '\0';
                strcpy(words[nwords++], word);
                wlen = 0;
            }
            in_word = !in_word;
        } else if (in_word && ch != ',' && ch != ' ' && ch != '\n' && ch != '\r') {
            if (wlen < MAX_WORD_LEN - 1)
                word[wlen++] = (char)ch;
        }
    }
    fclose(f);
}

void get_signature(const char *s, char *sig) {
    strcpy(sig, s);
    int len = (int)strlen(sig);
    /* Simple bubble sort for short strings */
    for (int i = 0; i < len - 1; i++)
        for (int j = i + 1; j < len; j++)
            if (sig[i] > sig[j]) {
                char t = sig[i]; sig[i] = sig[j]; sig[j] = t;
            }
}

/* Check if n is a perfect square */
bool is_square(long long n) {
    if (n < 0) return false;
    long long r = (long long)sqrt((double)n);
    if (r * r == n) return true;
    r++;
    if (r * r == n) return true;
    r -= 2;
    if (r >= 0 && r * r == n) return true;
    return false;
}

/* Precompute squares by digit count */
#define MAX_DIGITS 15
long long *squares_by_len[MAX_DIGITS + 1];
int nsquares_by_len[MAX_DIGITS + 1];

void precompute_squares(int max_len) {
    for (int d = 1; d <= max_len; d++) {
        /* Count squares with d digits */
        long long lo = 1;
        for (int i = 1; i < d; i++) lo *= 10;
        if (d == 1) lo = 0; /* include single-digit squares starting from 1 */
        long long hi = lo * 10 - 1;
        if (d == 1) { lo = 1; hi = 9; }

        long long start = (long long)sqrt((double)lo);
        if (start * start < lo) start++;
        long long end = (long long)sqrt((double)hi);

        int cnt = (int)(end - start + 1);
        if (cnt < 0) cnt = 0;
        squares_by_len[d] = (long long *)malloc(cnt * sizeof(long long));
        nsquares_by_len[d] = 0;

        for (long long n = start; n <= end; n++)
            squares_by_len[d][nsquares_by_len[d]++] = n * n;
    }
}

int digit_count(long long n) {
    int c = 0;
    while (n > 0) { c++; n /= 10; }
    return c;
}

void long_to_str(long long n, char *buf, int len) {
    for (int i = len - 1; i >= 0; i--) {
        buf[i] = '0' + (int)(n % 10);
        n /= 10;
    }
    buf[len] = '\0';
}

int main(void) {
    load_words("../data/words.txt");

    int max_len = 0;
    for (int i = 0; i < nwords; i++) {
        int l = (int)strlen(words[i]);
        if (l > max_len) max_len = l;
    }

    precompute_squares(max_len);

    /* Group words by signature to find anagrams */
    /* Simple O(n^2) approach since nwords < 3000 */
    long long max_square = 0;

    for (int i = 0; i < nwords; i++) {
        for (int j = i + 1; j < nwords; j++) {
            int len1 = (int)strlen(words[i]);
            int len2 = (int)strlen(words[j]);
            if (len1 != len2) continue;

            /* Check if anagrams */
            char sig1[MAX_WORD_LEN], sig2[MAX_WORD_LEN];
            get_signature(words[i], sig1);
            get_signature(words[j], sig2);
            if (strcmp(sig1, sig2) != 0) continue;

            int wlen = len1;
            /* Try mapping words[i] to each square */
            for (int si = 0; si < nsquares_by_len[wlen]; si++) {
                long long square1 = squares_by_len[wlen][si];
                char s1[MAX_WORD_LEN];
                long_to_str(square1, s1, wlen);

                /* Build letter->digit mapping */
                char mapping[26];
                memset(mapping, 0, sizeof(mapping));
                char reverse_map[10]; /* digit -> letter */
                memset(reverse_map, 0, sizeof(reverse_map));
                bool valid = true;

                for (int k = 0; k < wlen; k++) {
                    int letter = words[i][k] - 'A';
                    char digit = s1[k];

                    if (mapping[letter]) {
                        if (mapping[letter] != digit) { valid = false; break; }
                    } else {
                        if (reverse_map[digit - '0']) {
                            if (reverse_map[digit - '0'] != words[i][k]) { valid = false; break; }
                        }
                        mapping[letter] = digit;
                        reverse_map[digit - '0'] = words[i][k];
                    }
                }
                if (!valid) continue;

                /* Apply mapping to words[j] */
                char s2[MAX_WORD_LEN];
                bool valid2 = true;
                for (int k = 0; k < wlen; k++) {
                    int letter = words[j][k] - 'A';
                    if (!mapping[letter]) { valid2 = false; break; }
                    s2[k] = mapping[letter];
                }
                s2[wlen] = '\0';
                if (!valid2) continue;

                /* Check for leading zero */
                if (wlen > 1 && s2[0] == '0') continue;

                /* Convert s2 to number and check if square */
                long long square2 = 0;
                for (int k = 0; k < wlen; k++)
                    square2 = square2 * 10 + (s2[k] - '0');

                if (is_square(square2)) {
                    if (square1 > max_square) max_square = square1;
                    if (square2 > max_square) max_square = square2;
                }
            }
        }
    }

    printf("%lld\n", max_square);
    return 0;

    /* Cleanup not strictly necessary */
}
