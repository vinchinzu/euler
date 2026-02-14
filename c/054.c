#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

int card_value(char c) {
    if (c >= '2' && c <= '9') return c - '0';
    if (c == 'T') return 10;
    if (c == 'J') return 11;
    if (c == 'Q') return 12;
    if (c == 'K') return 13;
    if (c == 'A') return 14;
    return 0;
}

int cmp_desc(const void *a, const void *b) {
    return *(const int *)b - *(const int *)a;
}

int cmp_pair_desc(const void *a, const void *b) {
    const int *pa = (const int *)a;
    const int *pb = (const int *)b;
    /* Sort by count descending, then value descending */
    if (pb[0] != pa[0]) return pb[0] - pa[0];
    return pb[1] - pa[1];
}

/* rank array: rank[0] = category, rank[1..] = tiebreakers. Returns length. */
int hand_rank(const char cards[5][4], int *rank) {
    int values[5];
    char suits[5];

    for (int i = 0; i < 5; i++) {
        values[i] = card_value(cards[i][0]);
        suits[i] = cards[i][1];
    }

    qsort(values, 5, sizeof(int), cmp_desc);

    /* Count frequencies */
    int counts[15] = {0};
    for (int i = 0; i < 5; i++) counts[values[i]]++;

    /* Build count_pairs: [count, value] sorted by count desc, then value desc */
    int count_pairs[15][2];
    int npairs = 0;
    for (int v = 14; v >= 2; v--) {
        if (counts[v] > 0) {
            count_pairs[npairs][0] = counts[v];
            count_pairs[npairs][1] = v;
            npairs++;
        }
    }
    qsort(count_pairs, npairs, sizeof(int) * 2, cmp_pair_desc);

    bool is_flush = true;
    for (int i = 1; i < 5; i++) {
        if (suits[i] != suits[0]) { is_flush = false; break; }
    }

    bool is_straight = true;
    for (int i = 1; i < 5; i++) {
        if (values[i] != values[0] - i) { is_straight = false; break; }
    }

    bool is_wheel = (values[0] == 14 && values[1] == 5 && values[2] == 4 && values[3] == 3 && values[4] == 2);

    int straight_high = 0;
    if (is_straight || is_wheel) {
        straight_high = is_wheel ? 5 : values[0];
    }

    int len = 0;

    if ((is_straight || is_wheel) && is_flush) {
        rank[len++] = 8; rank[len++] = straight_high;
    } else if (count_pairs[0][0] == 4) {
        rank[len++] = 7; rank[len++] = count_pairs[0][1]; rank[len++] = count_pairs[1][1];
    } else if (count_pairs[0][0] == 3 && count_pairs[1][0] == 2) {
        rank[len++] = 6; rank[len++] = count_pairs[0][1]; rank[len++] = count_pairs[1][1];
    } else if (is_flush) {
        rank[len++] = 5;
        for (int i = 0; i < 5; i++) rank[len++] = values[i];
    } else if (is_straight || is_wheel) {
        rank[len++] = 4; rank[len++] = straight_high;
    } else if (count_pairs[0][0] == 3) {
        rank[len++] = 3;
        rank[len++] = count_pairs[0][1];
        rank[len++] = count_pairs[1][1];
        rank[len++] = count_pairs[2][1];
    } else if (count_pairs[0][0] == 2 && count_pairs[1][0] == 2) {
        int high_pair = count_pairs[0][1] > count_pairs[1][1] ? count_pairs[0][1] : count_pairs[1][1];
        int low_pair = count_pairs[0][1] < count_pairs[1][1] ? count_pairs[0][1] : count_pairs[1][1];
        rank[len++] = 2; rank[len++] = high_pair; rank[len++] = low_pair; rank[len++] = count_pairs[2][1];
    } else if (count_pairs[0][0] == 2) {
        rank[len++] = 1;
        rank[len++] = count_pairs[0][1];
        rank[len++] = count_pairs[1][1];
        rank[len++] = count_pairs[2][1];
        rank[len++] = count_pairs[3][1];
    } else {
        rank[len++] = 0;
        for (int i = 0; i < 5; i++) rank[len++] = values[i];
    }

    return len;
}

int main(void) {
    FILE *f = fopen("../data/poker.txt", "r");
    if (!f) {
        fprintf(stderr, "Error: cannot open ../data/poker.txt\n");
        return 1;
    }

    int count = 0;
    char line[256];

    while (fgets(line, sizeof(line), f)) {
        /* Parse 10 cards */
        char cards_all[10][4];
        int nc = 0;
        char *p = line;
        while (*p && nc < 10) {
            while (*p == ' ' || *p == '\n' || *p == '\r') p++;
            if (*p == '\0') break;
            int ci = 0;
            while (*p && *p != ' ' && *p != '\n' && *p != '\r') {
                cards_all[nc][ci++] = *p++;
            }
            cards_all[nc][ci] = '\0';
            nc++;
        }
        if (nc != 10) continue;

        char hand1[5][4], hand2[5][4];
        for (int i = 0; i < 5; i++) {
            strcpy(hand1[i], cards_all[i]);
            strcpy(hand2[i], cards_all[i + 5]);
        }

        int rank1[16], rank2[16];
        int len1 = hand_rank(hand1, rank1);
        int len2 = hand_rank(hand2, rank2);

        bool p1_wins = false;
        int minlen = len1 < len2 ? len1 : len2;
        for (int i = 0; i < minlen; i++) {
            if (rank1[i] > rank2[i]) { p1_wins = true; break; }
            if (rank1[i] < rank2[i]) break;
        }

        if (p1_wins) count++;
    }

    fclose(f);
    printf("%d\n", count);
    return 0;
}
