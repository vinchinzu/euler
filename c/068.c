/* Project Euler 068 - Magic 5-gon ring */
#include <stdio.h>
#include <string.h>

static int perm[10];
static int used[11]; /* used[1..10] */
static char best[20]; /* best 16-digit string found */
static int found_any;

static void check_solution(void) {
    /* outer nodes: perm[0..4], inner nodes: perm[5..9] */
    int *outer = perm;
    int *inner = perm + 5;

    /* 10 must be in outer nodes for 16-digit string */
    int has_10 = 0;
    for (int i = 0; i < 5; i++) {
        if (outer[i] == 10) { has_10 = 1; break; }
    }
    if (!has_10) return;

    int target = outer[0] + inner[0] + inner[1];
    if (outer[1] + inner[1] + inner[2] != target) return;
    if (outer[2] + inner[2] + inner[3] != target) return;
    if (outer[3] + inner[3] + inner[4] != target) return;
    if (outer[4] + inner[4] + inner[0] != target) return;

    /* Find the starting index (minimum outer node) */
    int start = 0;
    for (int i = 1; i < 5; i++) {
        if (outer[i] < outer[start]) start = i;
    }

    /* Build candidate string */
    char candidate[20];
    int pos = 0;
    for (int i = 0; i < 5; i++) {
        int idx = (start + i) % 5;
        pos += sprintf(candidate + pos, "%d", outer[idx]);
        pos += sprintf(candidate + pos, "%d", inner[idx]);
        pos += sprintf(candidate + pos, "%d", inner[(idx + 1) % 5]);
    }

    if ((int)strlen(candidate) != 16) return;

    if (!found_any || strcmp(candidate, best) > 0) {
        strcpy(best, candidate);
        found_any = 1;
    }
}

static void generate(int depth) {
    if (depth == 10) {
        check_solution();
        return;
    }
    for (int v = 1; v <= 10; v++) {
        if (!used[v]) {
            used[v] = 1;
            perm[depth] = v;
            generate(depth + 1);
            used[v] = 0;
        }
    }
}

int main(void) {
    memset(used, 0, sizeof(used));
    found_any = 0;
    generate(0);
    printf("%s\n", best);
    return 0;
}
