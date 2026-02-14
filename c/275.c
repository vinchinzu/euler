/*
 * Project Euler 275: Balanced Sculptures
 *
 * Find the number of 18-polyominoes balanced at x=0 with plinth at (0,0)
 * and all other blocks at y>0.
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define N 18
#define MAX_CC 300000

static const int DX[] = {1, -1, 0, 0};
static const int DY[] = {0, 0, 1, -1};

static int cand_x[4 * N];
static int cand_y[4 * N];
static int invalid[2 * N + 1][N + 2];
static long column_counts[2 * N + 1];

static long num_sculptures(int start, int end, long num_tiles) {
    if (num_tiles == 0)
        return 1;
    long res = 0;
    for (int i = start; i < end; i++) {
        int cx = cand_x[i];
        int cy = cand_y[i];
        int ci = cx + N;
        if (column_counts[ci] == 0)
            continue;
        column_counts[ci]--;
        int new_end = end;
        for (int d = 0; d < 4; d++) {
            int nx = cx + DX[d];
            int ny = cy + DY[d];
            int ni = nx + N;
            if (ny > 0 && !invalid[ni][ny]) {
                cand_x[new_end] = nx;
                cand_y[new_end] = ny;
                new_end++;
                invalid[ni][ny] = 1;
            }
        }
        res += num_sculptures(i + 1, new_end, num_tiles - 1);
        column_counts[ci]++;
        for (int j = end; j < new_end; j++)
            invalid[cand_x[j] + N][cand_y[j]] = 0;
    }
    return res;
}

static int cc_data[MAX_CC][N];
static int cc_len[MAX_CC];
static int num_cc = 0;

static void find_column_counts(int num_tiles, int *current, int cur_len) {
    if (num_tiles > 0) {
        memcpy(cc_data[num_cc], current, cur_len * sizeof(int));
        cc_len[num_cc] = cur_len;
        num_cc++;
    }
    for (int i = 1; num_tiles + i <= N; i++) {
        current[cur_len] = i;
        find_column_counts(num_tiles + i, current, cur_len + 1);
    }
}

#define HASH_SIZE 65536

typedef struct {
    int c0;
    int moment;
} Key;

typedef struct Node {
    Key key;
    int *indices;
    int count;
    int capacity;
    struct Node *next;
} Node;

static Node *hash_table[HASH_SIZE];

static unsigned int hash_key(Key k) {
    return (unsigned int)(k.c0 * 31 + k.moment) & (HASH_SIZE - 1);
}

static Node *find_or_create(Key k) {
    unsigned int h = hash_key(k);
    for (Node *n = hash_table[h]; n; n = n->next) {
        if (n->key.c0 == k.c0 && n->key.moment == k.moment)
            return n;
    }
    Node *n = (Node *)malloc(sizeof(Node));
    n->key = k;
    n->capacity = 16;
    n->indices = (int *)malloc(n->capacity * sizeof(int));
    n->count = 0;
    n->next = hash_table[h];
    hash_table[h] = n;
    return n;
}

static void add_to_group(Node *n, int idx) {
    if (n->count >= n->capacity) {
        n->capacity *= 2;
        n->indices = (int *)realloc(n->indices, n->capacity * sizeof(int));
    }
    n->indices[n->count++] = idx;
}

static int cc_sum(int idx) {
    int s = 0;
    for (int i = 0; i < cc_len[idx]; i++)
        s += cc_data[idx][i];
    return s;
}

static int cc_moment(int idx) {
    int m = 0;
    for (int i = 0; i < cc_len[idx]; i++)
        m += i * cc_data[idx][i];
    return m;
}

int main(void) {
    int current[N];
    find_column_counts(0, current, 0);

    memset(hash_table, 0, sizeof(hash_table));
    for (int i = 0; i < num_cc; i++) {
        Key k;
        k.c0 = cc_data[i][0];
        k.moment = cc_moment(i);
        Node *n = find_or_create(k);
        add_to_group(n, i);
    }

    long long ans = 0;

    for (int h = 0; h < HASH_SIZE; h++) {
        for (Node *node = hash_table[h]; node; node = node->next) {
            int num_middle = node->key.c0;
            int group_size = node->count;
            int *group = node->indices;

            for (int i1 = 0; i1 < group_size; i1++) {
                int idx1 = group[i1];
                int size1 = cc_sum(idx1);
                int size2 = N - size1 + num_middle;

                for (int i2 = i1; i2 < group_size; i2++) {
                    int idx2 = group[i2];
                    if (cc_sum(idx2) != size2)
                        continue;

                    cand_x[0] = 0;
                    cand_y[0] = 1;
                    memset(invalid, 0, sizeof(invalid));
                    invalid[N][1] = 1;
                    memset(column_counts, 0, sizeof(column_counts));

                    for (int i = 0; i < cc_len[idx1]; i++)
                        column_counts[i + N] = cc_data[idx1][i];

                    int reversible = (idx1 == idx2);

                    if (reversible) {
                        ans += num_sculptures(0, 1, size1);
                    }

                    for (int i = 1; i < cc_len[idx2]; i++)
                        column_counts[N - i] = cc_data[idx2][i];

                    long count = num_sculptures(0, 1, N);

                    if (reversible) {
                        ans += count;
                    } else {
                        ans += 2 * count;
                    }
                }
            }
        }
    }

    ans /= 2;
    printf("%lld\n", ans);
    return 0;
}
