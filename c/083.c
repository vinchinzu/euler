#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

#define SIZE 80
#define MAXNODES (SIZE * SIZE)

int matrix[SIZE][SIZE];
long long dist[SIZE][SIZE];

/* Simple priority queue using a binary heap */
typedef struct {
    long long cost;
    int r, c;
} Node;

Node heap[MAXNODES + 1];
int heap_size = 0;

void heap_push(long long cost, int r, int c) {
    int i = ++heap_size;
    heap[i].cost = cost; heap[i].r = r; heap[i].c = c;
    while (i > 1 && heap[i].cost < heap[i/2].cost) {
        Node tmp = heap[i]; heap[i] = heap[i/2]; heap[i/2] = tmp;
        i /= 2;
    }
}

Node heap_pop(void) {
    Node top = heap[1];
    heap[1] = heap[heap_size--];
    int i = 1;
    while (1) {
        int smallest = i;
        int l = 2*i, r = 2*i+1;
        if (l <= heap_size && heap[l].cost < heap[smallest].cost) smallest = l;
        if (r <= heap_size && heap[r].cost < heap[smallest].cost) smallest = r;
        if (smallest == i) break;
        Node tmp = heap[i]; heap[i] = heap[smallest]; heap[smallest] = tmp;
        i = smallest;
    }
    return top;
}

void load_matrix(const char *filename) {
    FILE *f = fopen(filename, "r");
    if (!f) { fprintf(stderr, "Cannot open %s\n", filename); exit(1); }
    char line[2048];
    int row = 0;
    while (row < SIZE && fgets(line, sizeof(line), f)) {
        int col = 0;
        char *tok = strtok(line, ",\n\r");
        while (tok && col < SIZE) {
            matrix[row][col] = atoi(tok);
            tok = strtok(NULL, ",\n\r");
            col++;
        }
        row++;
    }
    fclose(f);
}

int main(void) {
    load_matrix("../data/matrix.txt");

    for (int i = 0; i < SIZE; i++)
        for (int j = 0; j < SIZE; j++)
            dist[i][j] = LLONG_MAX;

    dist[0][0] = matrix[0][0];
    heap_push(dist[0][0], 0, 0);

    int dr[] = {0, 1, 0, -1};
    int dc[] = {1, 0, -1, 0};

    while (heap_size > 0) {
        Node cur = heap_pop();
        if (cur.cost > dist[cur.r][cur.c]) continue;

        for (int d = 0; d < 4; d++) {
            int nr = cur.r + dr[d];
            int nc = cur.c + dc[d];
            if (nr >= 0 && nr < SIZE && nc >= 0 && nc < SIZE) {
                long long new_cost = cur.cost + matrix[nr][nc];
                if (new_cost < dist[nr][nc]) {
                    dist[nr][nc] = new_cost;
                    heap_push(new_cost, nr, nc);
                }
            }
        }
    }

    printf("%lld\n", dist[SIZE-1][SIZE-1]);
    return 0;
}
