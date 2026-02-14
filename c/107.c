/* Project Euler Problem 107: Minimal Network (MST via Kruskal) */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 50

static int parent[MAXN], rnk[MAXN];

static void dsu_init(int n) {
    for (int i = 0; i < n; i++) { parent[i] = i; rnk[i] = 0; }
}

static int dsu_find(int x) {
    while (parent[x] != x) { parent[x] = parent[parent[x]]; x = parent[x]; }
    return x;
}

static int dsu_union(int x, int y) {
    int px = dsu_find(x), py = dsu_find(y);
    if (px == py) return 0;
    if (rnk[px] < rnk[py]) parent[px] = py;
    else if (rnk[px] > rnk[py]) parent[py] = px;
    else { parent[py] = px; rnk[px]++; }
    return 1;
}

typedef struct { int w, u, v; } Edge;

static int cmp_edge(const void *a, const void *b) {
    return ((Edge*)a)->w - ((Edge*)b)->w;
}

int main(void) {
    FILE *f = fopen("data/network.txt", "r");
    if (!f) f = fopen("../data/network.txt", "r");
    if (!f) { printf("0\n"); return 1; }

    char line[4096];
    int n = 0;
    Edge edges[2000];
    int edge_count = 0;

    while (fgets(line, sizeof(line), f) && n < MAXN) {
        char *tok = strtok(line, ",\n\r");
        int col = 0;
        while (tok) {
            /* Trim spaces */
            while (*tok == ' ') tok++;
            if (*tok != '-' || (tok[1] >= '0' && tok[1] <= '9')) {
                if (*tok != '-' || tok[1] != '\0') {
                    int w = atoi(tok);
                    if (w > 0 && n < col) {
                        /* Only add edge once (n < col ensures u < v) */
                        edges[edge_count].w = w;
                        edges[edge_count].u = n;
                        edges[edge_count].v = col;
                        edge_count++;
                    }
                }
            }
            tok = strtok(NULL, ",\n\r");
            col++;
        }
        n++;
    }
    fclose(f);

    int total_weight = 0;
    for (int i = 0; i < edge_count; i++)
        total_weight += edges[i].w;

    qsort(edges, edge_count, sizeof(Edge), cmp_edge);

    dsu_init(n);
    int mst_weight = 0, mst_edges = 0;
    for (int i = 0; i < edge_count && mst_edges < n - 1; i++) {
        if (dsu_union(edges[i].u, edges[i].v)) {
            mst_weight += edges[i].w;
            mst_edges++;
        }
    }

    printf("%d\n", total_weight - mst_weight);
    return 0;
}
