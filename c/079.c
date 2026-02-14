/* Project Euler 079 - Passcode derivation
 * Reads from ../data/keylog.txt or data/0079_keylog.txt */
#include <stdio.h>
#include <string.h>

#define MAX_NODES 10

int main(void) {
    FILE *f = fopen("../data/keylog.txt", "r");
    if (!f) f = fopen("data/keylog.txt", "r");
    if (!f) f = fopen("../data/0079_keylog.txt", "r");
    if (!f) f = fopen("data/0079_keylog.txt", "r");
    if (!f) {
        fprintf(stderr, "Cannot open keylog file\n");
        return 1;
    }

    /* Track which digits exist and adjacency */
    int node_exists[10];
    int adj[10][10]; /* adj[a][b] = 1 means a must come before b */
    int in_degree[10];
    memset(node_exists, 0, sizeof(node_exists));
    memset(adj, 0, sizeof(adj));
    memset(in_degree, 0, sizeof(in_degree));

    char line[16];
    while (fgets(line, sizeof(line), f)) {
        if (strlen(line) < 3) continue;
        int d1 = line[0] - '0';
        int d2 = line[1] - '0';
        int d3 = line[2] - '0';

        node_exists[d1] = 1;
        node_exists[d2] = 1;
        node_exists[d3] = 1;

        if (!adj[d1][d2]) { adj[d1][d2] = 1; in_degree[d2]++; }
        if (!adj[d1][d3]) { adj[d1][d3] = 1; in_degree[d3]++; }
        if (!adj[d2][d3]) { adj[d2][d3] = 1; in_degree[d3]++; }
    }
    fclose(f);

    /* Count nodes */
    int num_nodes = 0;
    for (int i = 0; i < 10; i++) {
        if (node_exists[i]) num_nodes++;
    }

    /* Topological sort (Kahn's algorithm) */
    int queue[10];
    int qfront = 0, qback = 0;

    /* Initialize queue with nodes having in_degree 0 */
    for (int i = 0; i < 10; i++) {
        if (node_exists[i] && in_degree[i] == 0) {
            queue[qback++] = i;
        }
    }

    char passcode[11];
    int plen = 0;

    while (qfront < qback) {
        /* Pick the smallest from queue for deterministic order */
        int min_idx = qfront;
        for (int i = qfront + 1; i < qback; i++) {
            if (queue[i] < queue[min_idx]) min_idx = i;
        }
        /* Swap to front */
        int tmp = queue[qfront];
        queue[qfront] = queue[min_idx];
        queue[min_idx] = tmp;

        int u = queue[qfront++];
        passcode[plen++] = '0' + u;

        for (int v = 0; v < 10; v++) {
            if (adj[u][v]) {
                in_degree[v]--;
                if (in_degree[v] == 0) {
                    queue[qback++] = v;
                }
            }
        }
    }
    passcode[plen] = '\0';

    if (plen == num_nodes) {
        printf("%s\n", passcode);
    } else {
        fprintf(stderr, "Error: cycle detected\n");
        return 1;
    }

    return 0;
}
