/*
 * Project Euler 846 - Graph Potency
 *
 * Build graph on "allowed" numbers (1, 2, odd prime powers, 2*odd prime powers),
 * connect u,v if x^2+1 = u*v for some x. Find 2-core, biconnected components,
 * enumerate all simple cycles, sum their vertex sums = "potency".
 *
 * F(10^6).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MAXN 1000001
#define MAX_ALLOWED 200000
#define MAX_EDGES 2000000

static int min_prime[MAXN];
static int primes[80000], nprimes;

/* Allowed numbers */
static int allowed[MAX_ALLOWED], nallowed;
static char is_allowed[MAXN]; /* set for allowed numbers */

/* Adjacency list */
static int adj_head[MAX_ALLOWED]; /* head index into edge list */
static int adj_next[MAX_EDGES * 2];
static int adj_to[MAX_EDGES * 2];
static int adj_count;

/* Map from value to index in allowed */
static int val_to_idx[MAXN];

static void sieve(int n) {
    memset(min_prime, 0, sizeof(int) * (n + 1));
    nprimes = 0;
    for (int i = 2; i <= n; i++) {
        if (min_prime[i] == 0) {
            min_prime[i] = i;
            primes[nprimes++] = i;
            for (long long j = (long long)i * i; j <= n; j += i)
                if (min_prime[(int)j] == 0)
                    min_prime[(int)j] = i;
        }
    }
}

/* sqrt(-1) mod p for p=1 mod 4 */
static int memo_roots[MAXN];
static char memo_roots_set[MAXN];

static int get_sqrt_neg1(int p) {
    if (p == 2) return 1;
    if (p % 4 != 1) return -1;
    if (memo_roots_set[p]) return memo_roots[p];

    int g = 2;
    while (1) {
        long long t = 1;
        int exp = (p - 1) / 2;
        long long base = g;
        int e = exp;
        while (e > 0) {
            if (e & 1) t = t * base % p;
            base = base * base % p;
            e >>= 1;
        }
        if (t == p - 1) break;
        g++;
    }
    /* r = g^((p-1)/4) mod p */
    long long r = 1, base = g;
    int e = (p - 1) / 4;
    while (e > 0) {
        if (e & 1) r = r * base % p;
        base = base * base % p;
        e >>= 1;
    }
    memo_roots[p] = (int)r;
    memo_roots_set[p] = 1;
    return (int)r;
}

static void add_edge(int u_idx, int v_idx) {
    int e1 = adj_count++;
    adj_to[e1] = v_idx;
    adj_next[e1] = adj_head[u_idx];
    adj_head[u_idx] = e1;

    int e2 = adj_count++;
    adj_to[e2] = u_idx;
    adj_next[e2] = adj_head[v_idx];
    adj_head[v_idx] = e2;
}

/* 2-core degree tracking */
static int degree[MAX_ALLOWED];
static char active[MAX_ALLOWED];

/* Biconnected components */
static int depth_arr[MAX_ALLOWED], low_arr[MAX_ALLOWED], parent_arr[MAX_ALLOWED];
static char visited[MAX_ALLOWED];

/* Edge stack for BCC */
static int estack_u[MAX_EDGES], estack_v[MAX_EDGES], estack_top;

/* Blocks */
#define MAX_BLOCKS 50000
static int block_nodes[MAX_EDGES]; /* flattened */
static int block_start[MAX_BLOCKS], block_size_arr[MAX_BLOCKS];
static int nblocks;

/* Sub-adjacency for cycle enumeration within a block */
static int sub_adj_head[5000];
static int sub_adj_next[20000];
static int sub_adj_to[20000];
static int sub_adj_count;

static int blk_nodes[5000];
static int blk_node_map[MAXN]; /* value -> index in blk_nodes */

/* DFS for cycle enumeration */
static long long blk_potency;
static int blk_path_vis[5000];

static void dfs_blk(int start_idx, int u_idx, long long current_sum, int length, int *nodes, int nn) {
    blk_path_vis[u_idx] = 1;
    int u_val = nodes[u_idx];

    for (int e = sub_adj_head[u_idx]; e != -1; e = sub_adj_next[e]) {
        int v_idx = sub_adj_to[e];
        if (v_idx == start_idx) {
            if (length >= 3)
                blk_potency += current_sum;
        } else if (v_idx > start_idx) {
            if (!blk_path_vis[v_idx])
                dfs_blk(start_idx, v_idx, current_sum + nodes[v_idx], length + 1, nodes, nn);
        }
    }
    blk_path_vis[u_idx] = 0;
}

static int cmp_int(const void *a, const void *b) { return *(const int *)a - *(const int *)b; }

int main(void) {
    int N_val = 1000000;
    sieve(N_val);
    memset(memo_roots_set, 0, sizeof(memo_roots_set));

    /* Build allowed set */
    memset(is_allowed, 0, sizeof(is_allowed));
    nallowed = 0;
    allowed[nallowed++] = 1; is_allowed[1] = 1;
    allowed[nallowed++] = 2; is_allowed[2] = 1;

    for (int i = 0; i < nprimes; i++) {
        int p = primes[i];
        if (p == 2) continue;
        long long pk = p;
        while (pk <= N_val) {
            if (!is_allowed[(int)pk]) { allowed[nallowed++] = (int)pk; is_allowed[(int)pk] = 1; }
            if (2 * pk <= N_val) {
                if (!is_allowed[(int)(2*pk)]) { allowed[nallowed++] = (int)(2*pk); is_allowed[(int)(2*pk)] = 1; }
            }
            pk *= p;
        }
    }

    qsort(allowed, nallowed, sizeof(int), cmp_int);
    memset(val_to_idx, -1, sizeof(val_to_idx));
    for (int i = 0; i < nallowed; i++)
        val_to_idx[allowed[i]] = i;

    /* Build graph edges */
    memset(adj_head, -1, sizeof(int) * nallowed);
    adj_count = 0;

    for (int ai = 0; ai < nallowed; ai++) {
        int u = allowed[ai];
        int roots[4], nroots = 0;

        if (u == 1) {
            roots[nroots++] = 0;
        } else if (u == 2) {
            roots[nroots++] = 1;
        } else {
            int temp = u;
            if (temp % 2 == 0) temp /= 2;
            int p = min_prime[temp];
            if (p % 4 == 3) continue;
            int r = get_sqrt_neg1(p);
            if (r < 0) continue;

            /* Hensel lift r to mod temp (p^k) */
            long long cur_r = r;
            long long cur_mod = p;
            while (cur_mod < temp) {
                /* inv(2*cur_r) mod p */
                long long inv2r = 1;
                {
                    long long base2 = (2 * cur_r) % p, e2 = p - 2, rr = 1;
                    while (e2 > 0) { if (e2 & 1) rr = rr * base2 % p; base2 = base2 * base2 % p; e2 >>= 1; }
                    inv2r = rr;
                }
                long long val = (cur_r * cur_r + 1) / cur_mod;
                long long diff = (val % p * inv2r) % p;
                cur_r = cur_r - diff * cur_mod;
                cur_mod *= p;
                cur_r = ((cur_r % cur_mod) + cur_mod) % cur_mod;
            }

            if (u % 2 == 0) {
                if ((int)(cur_r % 2) == 0)
                    roots[nroots++] = (int)(cur_r + temp);
                else
                    roots[nroots++] = (int)cur_r;
                if (roots[0] * 2 != u)
                    roots[nroots++] = u - roots[0];
            } else {
                roots[nroots++] = (int)cur_r;
                roots[nroots++] = u - (int)cur_r;
            }
        }

        long long limit = (long long)sqrt((double)u * N_val);
        /* Correct limit */
        while ((limit + 1) * (limit + 1) <= (long long)u * N_val) limit++;

        if (u == 1) {
            for (long long x = 0; x <= limit; x++) {
                long long v = x * x + 1;
                if (v > u && v <= N_val && is_allowed[(int)v]) {
                    int ui = val_to_idx[u], vi = val_to_idx[(int)v];
                    if (ui >= 0 && vi >= 0)
                        add_edge(ui, vi);
                }
            }
        } else {
            for (int ri = 0; ri < nroots; ri++) {
                int r = roots[ri];
                long long start = r;
                if (start == 0) start += u;
                for (long long x = start; x <= limit; x += u) {
                    long long val = x * x + 1;
                    long long v = val / u;
                    if (v > u && v <= N_val && is_allowed[(int)v]) {
                        int ui = val_to_idx[u], vi = val_to_idx[(int)v];
                        if (ui >= 0 && vi >= 0)
                            add_edge(ui, vi);
                    }
                }
            }
        }
    }

    /* 2-core: compute degrees and iteratively remove degree < 2 */
    memset(degree, 0, sizeof(int) * nallowed);
    for (int i = 0; i < nallowed; i++) active[i] = 1;

    /* Build degree from adjacency list - but we have duplicate edges (both directions) */
    /* Actually add_edge adds 2 directed edges per undirected edge, but we might add
       the same undirected edge multiple times. Need to deduplicate. */
    /* This is getting complex. Let me use a simpler approach: rebuild adjacency
       using sorted neighbor sets. */

    /* Actually, the Python uses sets for adj[u], so duplicates are removed.
       Let me do the same: for each node, collect unique neighbors. */
    /* Rebuild clean adjacency */
    {
        /* For each node, collect neighbors from current adj list, deduplicate, sort */
        static int *neighbors[MAX_ALLOWED];
        static int neighbor_count[MAX_ALLOWED];

        for (int i = 0; i < nallowed; i++) {
            /* Count */
            int cnt = 0;
            for (int e = adj_head[i]; e != -1; e = adj_next[e]) cnt++;
            neighbors[i] = (int *)malloc(cnt * sizeof(int));
            cnt = 0;
            for (int e = adj_head[i]; e != -1; e = adj_next[e])
                neighbors[i][cnt++] = adj_to[e];
            /* Sort and deduplicate */
            qsort(neighbors[i], cnt, sizeof(int), cmp_int);
            int uc = 0;
            for (int j = 0; j < cnt; j++)
                if (j == 0 || neighbors[i][j] != neighbors[i][j-1])
                    neighbors[i][uc++] = neighbors[i][j];
            neighbor_count[i] = uc;
            degree[i] = uc;
        }

        /* Iterative 2-core peeling */
        int changed = 1;
        while (changed) {
            changed = 0;
            for (int i = 0; i < nallowed; i++) {
                if (!active[i]) continue;
                if (degree[i] < 2) {
                    active[i] = 0;
                    changed = 1;
                    for (int j = 0; j < neighbor_count[i]; j++) {
                        int nb = neighbors[i][j];
                        if (active[nb]) degree[nb]--;
                    }
                }
            }
        }

        /* Rebuild adjacency for 2-core nodes */
        memset(adj_head, -1, sizeof(int) * nallowed);
        adj_count = 0;

        for (int i = 0; i < nallowed; i++) {
            if (!active[i]) continue;
            for (int j = 0; j < neighbor_count[i]; j++) {
                int nb = neighbors[i][j];
                if (active[nb] && nb > i) {
                    add_edge(i, nb);
                }
            }
        }

        for (int i = 0; i < nallowed; i++) free(neighbors[i]);
    }

    /* BCC: find biconnected components */
    /* Use iterative DFS for BCC to avoid stack overflow */
    memset(visited, 0, sizeof(char) * nallowed);
    nblocks = 0;
    estack_top = 0;

    /* Simplified approach: since blocks are needed for cycle enumeration,
       and the 2-core has all nodes with degree >= 2, each connected component
       of the 2-core IS a biconnected component (or close to it).

       Actually, let me try a simpler approach: for each connected component,
       enumerate cycles directly. The 2-core components should be small enough. */

    /* Find connected components of 2-core */
    static int comp_id[MAX_ALLOWED];
    memset(comp_id, -1, sizeof(int) * nallowed);
    int ncomps = 0;

    static int queue[MAX_ALLOWED];
    for (int i = 0; i < nallowed; i++) {
        if (!active[i] || comp_id[i] >= 0) continue;
        int head = 0, tail = 0;
        queue[tail++] = i;
        comp_id[i] = ncomps;
        while (head < tail) {
            int u = queue[head++];
            for (int e = adj_head[u]; e != -1; e = adj_next[e]) {
                int v = adj_to[e];
                if (comp_id[v] < 0) {
                    comp_id[v] = ncomps;
                    queue[tail++] = v;
                }
            }
        }
        ncomps++;
    }

    /* For each component, enumerate cycles */
    long long total_potency = 0;

    for (int ci = 0; ci < ncomps; ci++) {
        /* Collect nodes */
        int nn = 0;
        for (int i = 0; i < nallowed; i++) {
            if (comp_id[i] == ci)
                blk_nodes[nn++] = i;
        }
        if (nn < 3) continue;

        /* Build sub-adjacency */
        memset(sub_adj_head, -1, sizeof(int) * nn);
        sub_adj_count = 0;

        /* Map node index to local index */
        for (int i = 0; i < nn; i++)
            blk_node_map[blk_nodes[i]] = i;

        for (int i = 0; i < nn; i++) {
            int u = blk_nodes[i];
            for (int e = adj_head[u]; e != -1; e = adj_next[e]) {
                int v = adj_to[e];
                int vi = blk_node_map[v];
                int eidx = sub_adj_count++;
                sub_adj_to[eidx] = vi;
                sub_adj_next[eidx] = sub_adj_head[i];
                sub_adj_head[i] = eidx;
            }
        }

        /* Map local indices back to values */
        static int node_vals[5000];
        for (int i = 0; i < nn; i++)
            node_vals[i] = allowed[blk_nodes[i]];

        /* Enumerate cycles */
        blk_potency = 0;
        memset(blk_path_vis, 0, sizeof(int) * nn);

        for (int i = 0; i < nn; i++)
            dfs_blk(i, i, node_vals[i], 1, node_vals, nn);

        total_potency += blk_potency / 2;
    }

    printf("%lld\n", total_potency);
    return 0;
}
