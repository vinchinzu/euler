/*
 * Project Euler 680 - Yarra Gnisrever
 *
 * Array [0..N-1], K reverse operations using Fibonacci indices.
 * Compute R = sum i*A[i] mod 10^9.
 *
 * Uses a treap-like implicit tree with lazy reversal and
 * arithmetic-progression leaves.
 * N=10^18, K=10^6, M=10^9.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define M_VAL 1000000000LL
#define MAX_NODES 20000000

/* Each node represents a range of the array.
 * Leaf: len elements forming an arithmetic progression: first, first+diff, ..., first+diff*(len-1)
 * Internal: left child, right child
 * diff field: for internal, 1 = normal, -1 = reversed
 */
typedef struct {
    ll len;
    ll first;
    int diff;   /* +1 or -1 for both leaf and internal */
    int left;   /* child index, -1 for leaf */
    int right;  /* child index, -1 for leaf */
} Node;

static Node nodes[MAX_NODES];
static int num_nodes = 0;

static int new_leaf(ll len, ll first, int diff) {
    int id = num_nodes++;
    nodes[id].len = len;
    nodes[id].first = first;
    nodes[id].diff = diff;
    nodes[id].left = -1;
    nodes[id].right = -1;
    return id;
}

static int new_internal(int left, int right) {
    int id = num_nodes++;
    nodes[id].len = nodes[left].len + nodes[right].len;
    nodes[id].first = 0;
    nodes[id].diff = 1;
    nodes[id].left = left;
    nodes[id].right = right;
    return id;
}

static void node_reverse(int id) {
    if (nodes[id].left == -1) {
        /* Leaf: first -> first + diff*(len-1), diff -> -diff */
        nodes[id].first += (ll)nodes[id].diff * (nodes[id].len - 1);
    }
    nodes[id].diff *= -1;
}

static void canonicalize(int id) {
    if (nodes[id].left != -1 && nodes[id].diff == -1) {
        int left = nodes[id].left;
        int right = nodes[id].right;
        /* Swap and reverse children */
        node_reverse(right);
        node_reverse(left);
        nodes[id].left = right;
        nodes[id].right = left;
        nodes[id].diff = 1;
    }
}

static void ensure_cut_at(int id, ll index);
static void do_reverse(int id, ll start, ll end);

static void ensure_cut_at(int id, ll index) {
    canonicalize(id);
    if (nodes[id].left == -1) {
        /* Split leaf into [0, index) and [index, len) */
        int l = new_leaf(index, nodes[id].first, nodes[id].diff);
        int r = new_leaf(nodes[id].len - index,
                         nodes[id].first + (ll)nodes[id].diff * index,
                         nodes[id].diff);
        nodes[id].left = l;
        nodes[id].right = r;
        nodes[id].diff = 1;
    } else if (index < nodes[nodes[id].left].len) {
        ensure_cut_at(nodes[id].left, index);
        int ll_node = nodes[nodes[id].left].left;
        int lr_node = nodes[nodes[id].left].right;
        int old_right = nodes[id].right;
        nodes[id].left = ll_node;
        nodes[id].right = new_internal(lr_node, old_right);
    } else if (index > nodes[nodes[id].left].len) {
        ensure_cut_at(nodes[id].right, index - nodes[nodes[id].left].len);
        int rl_node = nodes[nodes[id].right].left;
        int rr_node = nodes[nodes[id].right].right;
        int old_left = nodes[id].left;
        nodes[id].right = rr_node;
        nodes[id].left = new_internal(old_left, rl_node);
    }
    /* else index == left.len, already cut */
}

static void do_reverse(int id, ll start, ll end) {
    canonicalize(id);
    if (nodes[id].left == -1) {
        /* Split leaf into [0, start), [start, end) reversed, [end, len) */
        int left_part = new_leaf(start, nodes[id].first, nodes[id].diff);
        int mid_part = new_leaf(end - start,
                                nodes[id].first + (ll)nodes[id].diff * (end - 1),
                                -nodes[id].diff);
        int right_part = new_leaf(nodes[id].len - end,
                                  nodes[id].first + (ll)nodes[id].diff * end,
                                  nodes[id].diff);
        int right_combined = new_internal(mid_part, right_part);
        nodes[id].left = left_part;
        nodes[id].right = right_combined;
        nodes[id].diff = 1;
    } else {
        ll left_len = nodes[nodes[id].left].len;
        if (end <= left_len) {
            do_reverse(nodes[id].left, start, end);
        } else if (start >= left_len) {
            do_reverse(nodes[id].right, start - left_len, end - left_len);
        } else {
            /* Spans both children */
            ensure_cut_at(nodes[id].left, start);
            ensure_cut_at(nodes[id].right, end - nodes[nodes[id].left].len);

            int left_right = nodes[nodes[id].left].right;
            int right_left = nodes[nodes[id].right].left;

            node_reverse(left_right);
            node_reverse(right_left);

            nodes[nodes[id].left].right = right_left;
            nodes[nodes[id].right].left = left_right;

            nodes[nodes[id].left].len = nodes[nodes[nodes[id].left].left].len +
                                        nodes[nodes[nodes[id].left].right].len;
            nodes[nodes[id].right].len = nodes[nodes[nodes[id].right].left].len +
                                         nodes[nodes[nodes[id].right].right].len;
        }
    }
}

static ll sum_powers_1(ll n) {
    /* Sum of 1 + 2 + ... + n mod M */
    n = n % (2 * M_VAL);
    return (n * (n + 1) / 2) % M_VAL;
}

static ll sum_powers_2(ll n) {
    /* Sum of 1^2 + 2^2 + ... + n^2 mod M */
    n = n % (6 * M_VAL);
    return (n * (n + 1) % (6 * M_VAL) * (2 * n + 1) / 6) % M_VAL;
}

static ll compute_R(int id, ll start) {
    canonicalize(id);
    if (nodes[id].len == 0) return 0;
    if (nodes[id].left == -1) {
        /* Leaf: sum_{i=0}^{len-1} (start+i) * (first + diff*i)
         * = len*start*first + (start*diff + first)*sum(i) + diff*sum(i^2)
         */
        ll len = nodes[id].len;
        ll first = nodes[id].first;
        int diff = nodes[id].diff;

        ll term1 = (start % M_VAL) * (first % M_VAL) % M_VAL * (len % M_VAL) % M_VAL;
        term1 = ((term1 % M_VAL) + M_VAL) % M_VAL;

        ll coef = ((start * diff + first) % M_VAL + M_VAL) % M_VAL;
        ll term2 = coef * sum_powers_1(len - 1) % M_VAL;

        ll term3 = (ll)diff * sum_powers_2(len - 1) % M_VAL;
        term3 = (term3 + M_VAL) % M_VAL;

        return (term1 + term2 + term3) % M_VAL;
    }
    ll left_r = compute_R(nodes[id].left, start);
    ll right_r = compute_R(nodes[id].right, start + nodes[nodes[id].left].len);
    return (left_r + right_r) % M_VAL;
}

int main() {
    ll N = 1000000000000000000LL; /* 10^18 */
    int K = 1000000; /* 10^6 */

    /* Precompute Fibonacci numbers mod N */
    ll *F = (ll *)calloc(2 * K + 2, sizeof(ll));
    F[1] = 1; F[2] = 1;
    for (int i = 3; i <= 2 * K; i++) {
        F[i] = (F[i - 2] + F[i - 1]) % N;
    }

    /* Create initial tree */
    int root = new_leaf(N, 0, 1);

    for (int i = 1; i <= K; i++) {
        ll s = F[2 * i - 1];
        ll t = F[2 * i];
        if (s < t) {
            do_reverse(root, s, t + 1);
        } else {
            do_reverse(root, t, s + 1);
        }
    }

    ll ans = compute_R(root, 0);
    printf("%lld\n", ans);

    free(F);
    return 0;
}
