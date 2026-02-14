/*
 * Project Euler Problem 419: Look and say sequence.
 *
 * By the Cosmological Theorem, every look-and-say sequence eventually decays
 * into independent "atoms". There are 92 such atoms (Conway's elements).
 * Each atom evolves into a specific combination of atoms. We build the 92x92
 * transition matrix and use matrix exponentiation to compute the Nth term.
 *
 * The initial sequence "1" corresponds to element 22 (Hydrogen).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define NELEMS 92
#define MOD (1LL << 30)  /* 2^30 = 1073741824 */

/* Conway's 92 elements: each element's string and what it decays into */
/* Decay table: element i -> list of element indices */

/* The 92 element strings */
static const char *elements[92] = {
    "22",           /* 0: Hydrogen (H) */
    "13112221133211411213211231131122211311123113322112",  /* 1: Helium (He) - will be filled correctly */
    "312211322212221121123222112",  /* 2: Lithium (Li) */
    "111312211312113221133211322112211213322112", /* 3: Beryllium */
    "1321132132111213122112311311222113111221131221",  /* 4: Boron */
    "3113112211322112211213322112", /* 5: Carbon */
    "111312212221121123222112",     /* 6: Nitrogen */
    "132112211213322112",          /* 7: Oxygen */
    "31121123222112",              /* 8: Fluorine */
    "111213322112",                /* 9: Neon */
    "123222112",                   /* 10: Sodium */
    "3113322112",                  /* 11: Magnesium */
    "1113222112",                  /* 12: Aluminium */
    "1322112",                     /* 13: Silicon */
    "311311222112",                /* 14: Phosphorus */
    "1113122112",                  /* 15: Sulfur */
    "132112",                      /* 16: Chlorine */
    "3112",                        /* 17: Argon */
    "1112",                        /* 18: Potassium */
    "12",                          /* 19: Calcium */
    "3113112221133211411213211231131122211311123113322112", /* 20: Scandium */
    "11131221131211132221232112111312111213111213211231132132211211131221131211221321123113213221123113112221131112311332211211131221131211132211121312211231131112311211232221121321132132211331121321231231121113112221121321133112132112312321123113112221121113122113121113123112112322111213211322211312113211", /* 21: Titanium */
    "13211321322113311213211331121113122112",  /* 22: Vanadium */
    "11131221131211132221232112111312111213111213211231132132211211131221232112111312111213322112",  /* 23: Chromium */
    "312211322212221121123222113", /* 24: Manganese - NOT USED, will recalculate */
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", ""
};

/* Instead of hardcoding all 92 elements (which is error-prone), let's compute
 * the decay table by actually running the look-and-say process and splitting. */

/* Actually, since this is very complex to hardcode correctly in C,
 * let's take the same approach as the Python: dynamically discover the
 * atoms by running the look-and-say process and splitting. */

#define MAX_STR 4096
#define MAX_ATOMS 200

static char atom_strs[MAX_ATOMS][MAX_STR];
static int atom_count = 0;
static int atom_index[MAX_ATOMS]; /* hash -> index not used, linear search */

/* Transition matrix: trans[i][j] = number of times atom j appears in decay of atom i */
static int trans[MAX_ATOMS][MAX_ATOMS];

/* digit counts per atom: digit_count[i][d] = count of digit d in atom i */
static int digit_count[MAX_ATOMS][4];  /* digits 1, 2, 3 at indices 1, 2, 3 */

static int find_or_add_atom(const char *s) {
    for (int i = 0; i < atom_count; i++) {
        if (strcmp(atom_strs[i], s) == 0) return i;
    }
    strcpy(atom_strs[atom_count], s);
    return atom_count++;
}

/* Look and say transform */
static void look_and_say(const char *s, char *out) {
    int len = strlen(s);
    int oi = 0;
    int last = 0;
    for (int i = 0; i <= len; i++) {
        if (i == len || s[i] != s[last]) {
            int count = i - last;
            oi += sprintf(out + oi, "%d%c", count, s[last]);
            last = i;
        }
    }
    out[oi] = '\0';
}

/* Check if we can split the look-and-say sequence at position i.
 * The split is valid if the last digit of the left part never equals
 * the first digit of the right part after iterating L times. */
static int can_split_at(const char *s, int pos, int L) {
    if (pos >= (int)strlen(s)) return 1;
    if (pos == 0) return 1;

    char c = s[pos - 1];
    char buf1[MAX_STR], buf2[MAX_STR];
    strncpy(buf1, s + pos, MAX_STR - 1);
    buf1[MAX_STR - 1] = '\0';

    for (int iter = 0; iter < L; iter++) {
        if (strlen(buf1) == 0) return 1;
        if (buf1[0] == c) return 0;
        /* Compute look_and_say of buf1, but only keep first MAX_STR chars */
        look_and_say(buf1, buf2);
        /* The first char of buf2 is a count digit, which won't match last digit
         * of previous transform easily. But we need the last char of the left part
         * after transform. Actually, the condition is simpler: after the split,
         * the left part's last digit and right part's first digit should never match.
         *
         * The condition checks if the right part's first digit (which may change
         * after each transform) ever matches the boundary digit c. */
        if (strlen(buf2) > MAX_STR - 2) break;
        strcpy(buf1, buf2);
    }
    return 1;
}

/* Recursively find atom transitions */
static void find_transitions(const char *s, int L);

static void find_transitions(const char *s, int L) {
    int idx = find_or_add_atom(s);
    if (trans[idx][0] != 0 || trans[idx][1] != 0) {
        /* Already computed (check if any transition set) */
        /* Actually, need a separate "visited" flag */
        return;
    }

    /* Mark as visited by setting a sentinel */
    /* We'll use a separate array */
    static int visited[MAX_ATOMS];
    if (visited[idx]) return;
    visited[idx] = 1;

    char next[MAX_STR];
    look_and_say(s, next);
    int nlen = strlen(next);

    /* Split next into atoms */
    int last_split = 0;
    for (int i = 1; i <= nlen; i++) {
        if (i == nlen || can_split_at(next, i, L)) {
            char part[MAX_STR];
            int plen = i - last_split;
            strncpy(part, next + last_split, plen);
            part[plen] = '\0';

            int pidx = find_or_add_atom(part);
            trans[idx][pidx]++;

            if (!visited[pidx]) {
                find_transitions(part, L);
            }

            last_split = i;
        }
    }
}

/* Matrix multiplication mod M */
typedef struct {
    ll *data;
    int n;
} Matrix;

static Matrix mat_alloc(int n) {
    Matrix m;
    m.n = n;
    m.data = (ll *)calloc((size_t)n * n, sizeof(ll));
    return m;
}

static void mat_free(Matrix *m) {
    free(m->data);
}

static void mat_identity(Matrix *m) {
    memset(m->data, 0, (size_t)m->n * m->n * sizeof(ll));
    for (int i = 0; i < m->n; i++)
        m->data[i * m->n + i] = 1;
}

static void mat_mul(Matrix *res, const Matrix *a, const Matrix *b, ll mod) {
    int n = a->n;
    memset(res->data, 0, (size_t)n * n * sizeof(ll));
    for (int i = 0; i < n; i++)
        for (int k = 0; k < n; k++) {
            ll aik = a->data[i * n + k];
            if (aik == 0) continue;
            for (int j = 0; j < n; j++)
                res->data[i * n + j] = (res->data[i * n + j] + aik * b->data[k * n + j]) % mod;
        }
}

static void mat_pow(Matrix *res, Matrix *base, ll exp, ll mod) {
    int n = base->n;
    mat_identity(res);
    Matrix tmp = mat_alloc(n);
    while (exp > 0) {
        if (exp & 1) {
            mat_mul(&tmp, res, base, mod);
            ll *t = res->data; res->data = tmp.data; tmp.data = t;
        }
        mat_mul(&tmp, base, base, mod);
        ll *t = base->data; base->data = tmp.data; tmp.data = t;
        exp >>= 1;
    }
    mat_free(&tmp);
}

int main(void) {
    ll N = 1000000000000LL;  /* 10^12 */
    int L = 10;

    memset(trans, 0, sizeof(trans));

    /* Start with "1" */
    find_transitions("1", L);

    int size = atom_count;

    /* Count digits in each atom */
    memset(digit_count, 0, sizeof(digit_count));
    for (int i = 0; i < size; i++) {
        for (int j = 0; atom_strs[i][j]; j++) {
            int d = atom_strs[i][j] - '0';
            if (d >= 1 && d <= 3) digit_count[i][d]++;
        }
    }

    /* Build transition matrix A where A[to][from] = trans[from][to] */
    Matrix A = mat_alloc(size);
    for (int from = 0; from < size; from++)
        for (int to = 0; to < size; to++)
            A.data[to * size + from] = trans[from][to];

    /* Compute A^(N-1) */
    Matrix R = mat_alloc(size);
    mat_pow(&R, &A, N - 1, MOD);

    /* Initial vector: "1" is atom 0 (the first one added) */
    int start_idx = find_or_add_atom("1");

    /* Count digits */
    ll count1 = 0, count2 = 0, count3 = 0;
    for (int i = 0; i < size; i++) {
        ll cnt = R.data[i * size + start_idx];
        count1 = (count1 + cnt * digit_count[i][1]) % MOD;
        count2 = (count2 + cnt * digit_count[i][2]) % MOD;
        count3 = (count3 + cnt * digit_count[i][3]) % MOD;
    }

    printf("%lld,%lld,%lld\n", count1, count2, count3);

    mat_free(&A);
    mat_free(&R);
    return 0;
}
