/*
 * Project Euler 679 - Free Farea
 *
 * Count strings of length 30 using "AEFR" containing exactly one of
 * each of "FREE", "FARE", "AREA", "REEF".
 *
 * Memoized DP: state = (position, last 4 chars prefix, bitmask of found keywords).
 * 4 chars from "AEFR" -> 256 possible prefixes, 16 keyword masks, 31 positions.
 */
#include <stdio.h>
#include <string.h>

#define N 30
#define NCHARS 4
#define PREFIX_STATES 256  /* 4^4 */
#define MASK_STATES 16     /* 2^4 */

/* Characters: A=0, E=1, F=2, R=3 */
static const char CHARS[] = "AEFR";
static const char *KEYWORDS[] = {"FREE", "FARE", "AREA", "REEF"};
static const int NKEYWORDS = 4;

/* Encode a 4-char prefix as an integer (base 4) */
/* prefix[0] is oldest, prefix[3] is newest */
static int encode_prefix(const char *p, int len) {
    int v = 0;
    for (int i = 0; i < len; i++) {
        int c = 0;
        switch (p[i]) {
            case 'A': c = 0; break;
            case 'E': c = 1; break;
            case 'F': c = 2; break;
            case 'R': c = 3; break;
        }
        v = v * 4 + c;
    }
    /* Pad with zeros for shorter prefixes */
    for (int i = len; i < 4; i++) v = v * 4;
    return v;
}

/* DP table: dp[pos][prefix][mask] */
/* pos: 0..30, prefix: 0..255, mask: 0..15 */
static long long dp[N + 1][PREFIX_STATES][MASK_STATES];
static char computed[N + 1][PREFIX_STATES][MASK_STATES];

/* Precomputed: for each prefix state, which keyword bits are found */
/* And for each (prefix, char) -> new_prefix, new_keyword_bit */
static int next_prefix[PREFIX_STATES][NCHARS]; /* new prefix after adding char */
static int keyword_match[PREFIX_STATES];        /* bitmask of keywords matching this prefix */

static char prefix_str[PREFIX_STATES][5]; /* decoded prefix strings */

static void decode_prefix(int v, char *out) {
    for (int i = 3; i >= 0; i--) {
        out[i] = CHARS[v % 4];
        v /= 4;
    }
    out[4] = '\0';
}

static void init_tables(void) {
    for (int p = 0; p < PREFIX_STATES; p++) {
        decode_prefix(p, prefix_str[p]);

        /* Check which keywords match this 4-char prefix */
        keyword_match[p] = 0;
        for (int k = 0; k < NKEYWORDS; k++) {
            if (strncmp(prefix_str[p], KEYWORDS[k], 4) == 0)
                keyword_match[p] |= (1 << k);
        }

        /* For each character, compute next prefix (shift left, add char) */
        for (int c = 0; c < NCHARS; c++) {
            char new_p[5];
            new_p[0] = prefix_str[p][1];
            new_p[1] = prefix_str[p][2];
            new_p[2] = prefix_str[p][3];
            new_p[3] = CHARS[c];
            new_p[4] = '\0';
            next_prefix[p][c] = encode_prefix(new_p, 4);
        }
    }
}

/* However, the Python solution uses variable-length prefixes up to 4.
 * For positions 0-3, the prefix is shorter than 4. We need to handle this.
 * Let's use a different encoding: track position and build prefix up to length 4.
 *
 * Actually, let's just simulate the Python approach more closely.
 * We'll track the last 4 characters (or fewer at the start).
 * For simplicity, encode the state as the last min(pos, 4) characters.
 *
 * Better approach: pad the initial prefix with a dummy char not in CHARS.
 * Use 5 values: A=0, E=1, F=2, R=3, NONE=4. Base 5, 5^4 = 625 states.
 */

#define BASE 5
#define PREFIX_STATES_V2 625

static int next_prefix_v2[PREFIX_STATES_V2][NCHARS];
static int keyword_match_v2[PREFIX_STATES_V2];

static char prefix_str_v2[PREFIX_STATES_V2][5];
static const char CHARS_V2[] = "AEFR_";

static void decode_prefix_v2(int v, char *out) {
    for (int i = 3; i >= 0; i--) {
        out[i] = CHARS_V2[v % BASE];
        v /= BASE;
    }
    out[4] = '\0';
}

static void init_tables_v2(void) {
    for (int p = 0; p < PREFIX_STATES_V2; p++) {
        decode_prefix_v2(p, prefix_str_v2[p]);

        keyword_match_v2[p] = 0;
        for (int k = 0; k < NKEYWORDS; k++) {
            if (strncmp(prefix_str_v2[p], KEYWORDS[k], 4) == 0)
                keyword_match_v2[p] |= (1 << k);
        }

        for (int c = 0; c < NCHARS; c++) {
            char new_p[5];
            new_p[0] = prefix_str_v2[p][1];
            new_p[1] = prefix_str_v2[p][2];
            new_p[2] = prefix_str_v2[p][3];
            new_p[3] = CHARS[c];
            new_p[4] = '\0';
            /* Encode */
            int v = 0;
            for (int i = 0; i < 4; i++) {
                int ch = 4; /* NONE */
                for (int j = 0; j < 4; j++) {
                    if (new_p[i] == CHARS[j]) { ch = j; break; }
                }
                v = v * BASE + ch;
            }
            next_prefix_v2[p][c] = v;
        }
    }
}

static long long dp2[N + 1][PREFIX_STATES_V2][MASK_STATES];

static long long solve_dp(void) {
    memset(dp2, 0, sizeof(dp2));

    /* Initial state: position 0, prefix = "____" (all NONE = 4444 in base 5) */
    int init_prefix = 4 * 125 + 4 * 25 + 4 * 5 + 4; /* 4*5^3 + 4*5^2 + 4*5 + 4 = 624 */
    dp2[0][init_prefix][0] = 1;

    for (int pos = 0; pos < N; pos++) {
        for (int p = 0; p < PREFIX_STATES_V2; p++) {
            for (int mask = 0; mask < MASK_STATES; mask++) {
                if (dp2[pos][p][mask] == 0) continue;
                long long cnt = dp2[pos][p][mask];

                for (int c = 0; c < NCHARS; c++) {
                    int np = next_prefix_v2[p][c];
                    int kw = keyword_match_v2[np];
                    int new_mask = mask | kw;

                    /* If any keyword is found that was already found, skip */
                    if (kw & mask) continue; /* duplicate keyword */

                    dp2[pos + 1][np][new_mask] += cnt;
                }
            }
        }
    }

    /* Sum all states at position N with mask = 0xF (all 4 keywords found) */
    long long result = 0;
    for (int p = 0; p < PREFIX_STATES_V2; p++) {
        result += dp2[N][p][0xF];
    }
    return result;
}

int main() {
    init_tables_v2();
    long long ans = solve_dp();
    printf("%lld\n", ans);
    return 0;
}
