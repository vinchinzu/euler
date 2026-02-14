/*
 * Project Euler 673 - Beds and Desks
 *
 * Read bed/desk pairings from files, find connected components,
 * compute answer using component structure.
 *
 * Data files: python/0673_beds.txt and python/0673_desks.txt
 * (paths relative to the executable's parent directory)
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <libgen.h>

typedef long long ll;
typedef __int128 lll;

#define N_STUDENTS 500
#define MOD 999999937LL

/* Adjacency: beds[i] = partner of i via bed, -1 if none */
static int beds[N_STUDENTS];
static int desks[N_STUDENTS];

static ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

static ll factorial_mod(int n, ll mod) {
    ll result = 1;
    for (int i = 1; i <= n; i++) {
        result = result * i % mod;
    }
    return result;
}

/* Component info */
typedef struct {
    int num_students;
    int num_bed_pairings;
    int num_desk_pairings;
} Component;

static int read_graph(const char *filepath, int *graph) {
    FILE *f = fopen(filepath, "r");
    if (!f) {
        fprintf(stderr, "Cannot open %s\n", filepath);
        return -1;
    }
    int v1, v2;
    while (fscanf(f, "%d,%d", &v1, &v2) == 2) {
        v1--; v2--;
        graph[v1] = v2;
        graph[v2] = v1;
    }
    fclose(f);
    return 0;
}

int main(int argc, char *argv[]) {
    /* Determine data file paths relative to this source's location */
    /* We expect data files in ../python/ relative to c/ directory */
    char beds_path[1024], desks_path[1024];

    /* Try to find the data files */
    const char *paths_to_try[] = {
        "python/0673_beds.txt",
        "../python/0673_beds.txt",
        NULL
    };
    const char *beds_file = NULL;
    for (int i = 0; paths_to_try[i]; i++) {
        FILE *test = fopen(paths_to_try[i], "r");
        if (test) {
            fclose(test);
            beds_file = paths_to_try[i];
            break;
        }
    }

    if (!beds_file) {
        /* Try using argv[0] to find relative path */
        char *dir = dirname(strdup(argv[0]));
        snprintf(beds_path, sizeof(beds_path), "%s/../python/0673_beds.txt", dir);
        snprintf(desks_path, sizeof(desks_path), "%s/../python/0673_desks.txt", dir);
    } else {
        if (strncmp(beds_file, "python/", 7) == 0) {
            strcpy(beds_path, "python/0673_beds.txt");
            strcpy(desks_path, "python/0673_desks.txt");
        } else {
            strcpy(beds_path, "../python/0673_beds.txt");
            strcpy(desks_path, "../python/0673_desks.txt");
        }
    }

    for (int i = 0; i < N_STUDENTS; i++) {
        beds[i] = -1;
        desks[i] = -1;
    }

    if (read_graph(beds_path, beds) < 0) return 1;
    if (read_graph(desks_path, desks) < 0) return 1;

    /* Find connected components via DFS on union of both graphs */
    int visited[N_STUDENTS];
    memset(visited, 0, sizeof(visited));

    Component components[N_STUDENTS];
    int comp_count = 0;

    for (int i = 0; i < N_STUDENTS; i++) {
        if (visited[i]) continue;

        int ns = 0, nb = 0, nd = 0;
        /* DFS stack */
        int stack[N_STUDENTS];
        int sp = 0;
        stack[sp++] = i;

        while (sp > 0) {
            int v = stack[--sp];
            if (visited[v]) continue;
            visited[v] = 1;
            ns++;
            if (beds[v] >= 0) {
                nb++;
                if (!visited[beds[v]]) stack[sp++] = beds[v];
            }
            if (desks[v] >= 0) {
                nd++;
                if (!visited[desks[v]]) stack[sp++] = desks[v];
            }
        }

        components[comp_count].num_students = ns;
        components[comp_count].num_bed_pairings = nb / 2;
        components[comp_count].num_desk_pairings = nd / 2;
        comp_count++;
    }

    /* Count duplicates and compute answer */
    /* Sort components for grouping */
    /* Simple O(n^2) grouping since comp_count <= 500 */
    int used[N_STUDENTS];
    memset(used, 0, sizeof(used));

    ll ans = 1;
    for (int i = 0; i < comp_count; i++) {
        if (used[i]) continue;
        int count = 0;
        for (int j = i; j < comp_count; j++) {
            if (!used[j] &&
                components[j].num_students == components[i].num_students &&
                components[j].num_bed_pairings == components[i].num_bed_pairings &&
                components[j].num_desk_pairings == components[i].num_desk_pairings) {
                count++;
                used[j] = 1;
            }
        }

        Component *c = &components[i];
        if (c->num_bed_pairings + c->num_desk_pairings == c->num_students) {
            ans = (lll)ans * mod_pow(c->num_students, count, MOD) % MOD;
        } else if (c->num_students % 2 == 0) {
            ans = (lll)ans * mod_pow(2, count, MOD) % MOD;
        }
        ans = (lll)ans * factorial_mod(count, MOD) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}
