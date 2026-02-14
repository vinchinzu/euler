/*
 * Project Euler 674 - Solving I-equations
 *
 * Parse I-expressions from file, find least simultaneous values
 * for all pairs.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;

#define MOD_VAL 1000000000LL
#define MAX_NODES 100000
#define MAX_VARS 100
#define MAX_EQUALITIES 500
#define MAX_LINES 200

/* I-expression node */
typedef struct {
    int var;       /* variable index, -1 if not a variable */
    int left;      /* index of left child, -1 if leaf */
    int right;     /* index of right child, -1 if leaf */
    ull var_mask;  /* bitmask of variables used */
} INode;

static INode nodes[MAX_NODES];
static int num_nodes = 0;

/* Variable name table */
static char var_names[MAX_VARS][16];
static int num_vars = 0;

static int find_or_add_var(const char *name, int len) {
    for (int i = 0; i < num_vars; i++) {
        if ((int)strlen(var_names[i]) == len && strncmp(var_names[i], name, len) == 0)
            return i;
    }
    strncpy(var_names[num_vars], name, len);
    var_names[num_vars][len] = '\0';
    return num_vars++;
}

static int new_node_var(int var) {
    int id = num_nodes++;
    nodes[id].var = var;
    nodes[id].left = -1;
    nodes[id].right = -1;
    nodes[id].var_mask = 1ULL << var;
    return id;
}

static int new_node_i(int left, int right) {
    int id = num_nodes++;
    nodes[id].var = -1;
    nodes[id].left = left;
    nodes[id].right = right;
    nodes[id].var_mask = nodes[left].var_mask | nodes[right].var_mask;
    return id;
}

/* Parse I-expression from string */
static int parse(const char *s, int *pos) {
    if (s[*pos] == 'I') {
        (*pos) += 2; /* skip "I(" */
        int left = parse(s, pos);
        (*pos)++; /* skip "," */
        int right = parse(s, pos);
        (*pos)++; /* skip ")" */
        return new_node_i(left, right);
    } else {
        int start = *pos;
        while (s[*pos] && s[*pos] >= 'a' && s[*pos] <= 'z') (*pos)++;
        int var = find_or_add_var(s + start, *pos - start);
        return new_node_var(var);
    }
}

/* Equality: expr = var */
typedef struct {
    int expr;  /* node index */
    int var;   /* variable index */
} Equality;

static Equality equalities[MAX_EQUALITIES];
static int num_equalities;

static void add_equality(int expr, int var) {
    equalities[num_equalities].expr = expr;
    equalities[num_equalities].var = var;
    num_equalities++;
}

/* Build equalities from two expressions */
static void helper(int e1, int e2) {
    if (nodes[e1].var >= 0 && nodes[e2].var >= 0) {
        if (nodes[e1].var < nodes[e2].var)
            add_equality(e1, nodes[e2].var);
        else if (nodes[e1].var > nodes[e2].var)
            add_equality(e2, nodes[e1].var);
    } else if (nodes[e1].var >= 0) {
        add_equality(e2, nodes[e1].var);
    } else if (nodes[e2].var >= 0) {
        add_equality(e1, nodes[e2].var);
    } else {
        helper(nodes[e1].left, nodes[e2].left);
        helper(nodes[e1].right, nodes[e2].right);
    }
}

/* Evaluate expression given variable values */
static ll evaluate(int node, ll *values) {
    if (nodes[node].var >= 0) {
        return values[nodes[node].var];
    }
    ll x = evaluate(nodes[node].left, values);
    ll y = evaluate(nodes[node].right, values);
    ll sum = 1 + x + y;
    return ((sum % MOD_VAL) * (sum % MOD_VAL) % MOD_VAL + y - x % MOD_VAL + MOD_VAL) % MOD_VAL;
}

static ll least_simultaneous_value(int e1, int e2) {
    num_equalities = 0;
    helper(e1, e2);

    /* Topological sort: find equalities where variable doesn't appear on RHS */
    Equality evaluations[MAX_EQUALITIES];
    int num_evaluations = 0;

    int active[MAX_EQUALITIES];
    for (int i = 0; i < num_equalities; i++) active[i] = 1;

    int total = num_equalities;
    while (total > 0) {
        int found = -1;
        for (int i = 0; i < num_equalities; i++) {
            if (!active[i]) continue;
            int var = equalities[i].var;
            /* Check if this var appears in any other active equality's expr */
            int appears = 0;
            for (int j = 0; j < num_equalities; j++) {
                if (!active[j]) continue;
                if (nodes[equalities[j].expr].var_mask & (1ULL << var)) {
                    appears = 1;
                    break;
                }
            }
            if (!appears) {
                found = i;
                break;
            }
        }

        if (found < 0) return 0; /* Circular dependency */

        int target_var = equalities[found].var;

        /* Collect all equalities with this variable */
        Equality good[MAX_EQUALITIES];
        int ngood = 0;
        for (int i = 0; i < num_equalities; i++) {
            if (active[i] && equalities[i].var == target_var) {
                good[ngood++] = equalities[i];
                active[i] = 0;
                total--;
            }
        }

        evaluations[num_evaluations++] = good[0];

        /* Add new equalities for remaining pairs */
        for (int k = 1; k < ngood; k++) {
            int old_num = num_equalities;
            helper(good[0].expr, good[k].expr);
            /* Mark new equalities as active */
            for (int i = old_num; i < num_equalities; i++) {
                active[i] = 1;
                total++;
            }
        }
    }

    /* Evaluate in reverse order */
    ll values[MAX_VARS];
    memset(values, 0, sizeof(values));

    for (int i = num_evaluations - 1; i >= 0; i--) {
        values[evaluations[i].var] = evaluate(evaluations[i].expr, values);
    }

    return evaluate(e1, values);
}

int main(int argc, char *argv[]) {
    /* Find data file */
    const char *paths[] = {
        "python/0674_i_expressions.txt",
        "../python/0674_i_expressions.txt",
        NULL
    };
    FILE *f = NULL;
    char fpath[1024];
    for (int i = 0; paths[i]; i++) {
        f = fopen(paths[i], "r");
        if (f) { strcpy(fpath, paths[i]); break; }
    }
    if (!f && argc > 0) {
        /* Try relative to argv[0] */
        char *tmp = strdup(argv[0]);
        char *dir = tmp;
        char *last_slash = strrchr(dir, '/');
        if (last_slash) *last_slash = '\0';
        else dir = (char *)".";
        snprintf(fpath, sizeof(fpath), "%s/../python/0674_i_expressions.txt", dir);
        f = fopen(fpath, "r");
        free(tmp);
    }
    if (!f) {
        fprintf(stderr, "Cannot open data file\n");
        return 1;
    }

    /* Read expressions */
    int exprs[MAX_LINES];
    int num_exprs = 0;
    char line[100000];

    while (fgets(line, sizeof(line), f)) {
        int len = strlen(line);
        while (len > 0 && (line[len-1] == '\n' || line[len-1] == '\r')) line[--len] = '\0';
        if (len == 0) continue;

        /* Reset var table for each pair computation - actually we need global vars */
        int pos = 0;
        exprs[num_exprs++] = parse(line, &pos);
    }
    fclose(f);

    ll ans = 0;
    for (int i = 0; i < num_exprs; i++) {
        for (int j = i + 1; j < num_exprs; j++) {
            ans += least_simultaneous_value(exprs[i], exprs[j]);
        }
    }
    ans %= MOD_VAL;

    printf("%lld\n", ans);
    return 0;
}
