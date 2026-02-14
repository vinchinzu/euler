/*
 * Project Euler 480 - The Last Question
 *
 * From the phrase "thereisasyetinsufficientdataforameaningfulanswer",
 * consider all words formed by selecting at most 15 letters (respecting counts).
 * Words are listed alphabetically.
 *
 * Given P(w) = position, find
 * W(P(legionary) + P(calorimeters) - P(annihilate) + P(orchestrated) - P(fluttering)).
 *
 * Use rational number DP (numerator/denominator as __int128) for exact counting.
 */

#include <stdio.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define K 15
#define MAX_LETTERS 20

/* Letter frequencies from "thereisasyetinsufficientdataforameaningfulanswer" */
/* Sorted letters: a, c, d, e, f, g, h, i, l, m, n, r, s, t, u, w, y */
static const char letters[] = "acdefghilmnorstuwy";
static int n_letters;
static int freq_list[MAX_LETTERS];

static ll fact[K + 1];

ll gcd_ll(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

/* Rational number: num/den */
typedef struct { lll num, den; } Rat;

Rat rat_new(lll n, lll d) {
    Rat r;
    if (d < 0) { n = -n; d = -d; }
    if (n == 0) { r.num = 0; r.den = 1; return r; }
    lll g = n < 0 ? -n : n;
    lll b = d;
    while (b) { lll t = b; b = g % b; g = t; }
    r.num = n / g;
    r.den = d / g;
    return r;
}

Rat rat_add(Rat a, Rat b) {
    return rat_new(a.num * b.den + b.num * a.den, a.den * b.den);
}

Rat rat_mul(Rat a, Rat b) {
    return rat_new(a.num * b.num, a.den * b.den);
}

Rat rat_zero(void) { Rat r = {0, 1}; return r; }
Rat rat_one(void) { Rat r = {1, 1}; return r; }

/* Count words of length 1..max_len formable from available frequencies */
ll count_words(int *avail, int max_len) {
    if (max_len <= 0) return 0;

    Rat dp[K + 1];
    Rat new_dp[K + 1];
    for (int j = 0; j <= max_len; j++) dp[j] = rat_zero();
    dp[0] = rat_one();

    for (int i = 0; i < n_letters; i++) {
        int fi = avail[i];
        if (fi == 0) continue;
        for (int j = 0; j <= max_len; j++) new_dp[j] = rat_zero();
        for (int j = 0; j <= max_len; j++) {
            if (dp[j].num == 0) continue;
            int lim = fi < (max_len - j) ? fi : (max_len - j);
            for (int c = 0; c <= lim; c++) {
                Rat inv_c_fact = rat_new(1, fact[c]);
                new_dp[j + c] = rat_add(new_dp[j + c], rat_mul(dp[j], inv_c_fact));
            }
        }
        for (int j = 0; j <= max_len; j++) dp[j] = new_dp[j];
    }

    ll total = 0;
    for (int L = 1; L <= max_len; L++) {
        /* fact[L] * dp[L] should be an integer */
        Rat val = rat_mul(dp[L], rat_new(fact[L], 1));
        total += (ll)val.num;
    }
    return total;
}

int letter_to_idx(char c) {
    for (int i = 0; i < n_letters; i++)
        if (letters[i] == c) return i;
    return -1;
}

ll P(const char *word) {
    int len = strlen(word);
    ll pos = 0;
    int avail[MAX_LETTERS];
    for (int i = 0; i < n_letters; i++) avail[i] = freq_list[i];

    for (int i = 0; i < len; i++) {
        int ci = letter_to_idx(word[i]);
        for (int li = 0; li < ci; li++) {
            if (avail[li] > 0) {
                avail[li]--;
                pos += 1 + count_words(avail, K - i - 1);
                avail[li]++;
            }
        }
        avail[ci]--;
        if (i < len - 1) {
            pos += 1;  /* count this prefix as a shorter word */
        }
    }
    pos += 1;  /* the word itself */
    return pos;
}

void W(ll p, char *result) {
    int avail[MAX_LETTERS];
    for (int i = 0; i < n_letters; i++) avail[i] = freq_list[i];
    int rlen = 0;

    for (int depth = 0; depth < K; depth++) {
        int found = 0;
        for (int li = 0; li < n_letters; li++) {
            if (avail[li] > 0) {
                avail[li]--;
                ll cnt = 1 + count_words(avail, K - depth - 1);
                if (p <= cnt) {
                    result[rlen++] = letters[li];
                    if (p == 1) {
                        result[rlen] = '\0';
                        return;
                    }
                    p -= 1;
                    found = 1;
                    break;
                }
                p -= cnt;
                avail[li]++;
            }
        }
        if (!found) break;
    }
    result[rlen] = '\0';
}

void init_freqs(void) {
    const char *S = "thereisasyetinsufficientdataforameaningfulanswer";
    int counts[256];
    memset(counts, 0, sizeof(counts));
    for (int i = 0; S[i]; i++) counts[(unsigned char)S[i]]++;

    n_letters = strlen(letters);
    for (int i = 0; i < n_letters; i++) {
        freq_list[i] = counts[(unsigned char)letters[i]];
    }
}

int main(void) {
    /* Precompute factorials */
    fact[0] = 1;
    for (int i = 1; i <= K; i++) fact[i] = fact[i - 1] * i;

    init_freqs();

    ll total_pos = 0;
    total_pos += P("legionary");
    total_pos += P("calorimeters");
    total_pos -= P("annihilate");
    total_pos += P("orchestrated");
    total_pos -= P("fluttering");

    char answer[20];
    W(total_pos, answer);
    printf("%s\n", answer);
    return 0;
}
