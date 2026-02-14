/* Project Euler 186: Connectedness of a network. */
#include <stdio.h>

#define MOD 1000000
#define TOTAL_USERS MOD
#define TARGET (TOTAL_USERS * 99 / 100)
#define PRIME_MINISTER 524287

static int parent[TOTAL_USERS];
static int sz[TOTAL_USERS];

static int find(int x) {
    while (parent[x] != x) {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    return x;
}

static int unite(int a, int b) {
    int ra = find(a), rb = find(b);
    if (ra == rb) return ra;
    if (sz[ra] < sz[rb]) {
        parent[ra] = rb;
        sz[rb] += sz[ra];
        return rb;
    } else {
        parent[rb] = ra;
        sz[ra] += sz[rb];
        return ra;
    }
}

/* Lagged Fibonacci generator */
static int buf[55];
static int lfg_k;

static void lfg_init(void) {
    for (int k = 1; k <= 55; k++) {
        long long kk = k;
        long long val = (100003LL - 200003LL * kk + 300007LL * kk * kk * kk) % MOD;
        if (val < 0) val += MOD;
        buf[k - 1] = (int)val;
    }
    lfg_k = 1;
}

static int lfg_next(void) {
    int value;
    if (lfg_k <= 55) {
        value = buf[lfg_k - 1];
    } else {
        value = (buf[(lfg_k - 24 - 1) % 55] + buf[(lfg_k - 55 - 1) % 55]) % MOD;
        buf[(lfg_k - 1) % 55] = value;
    }
    lfg_k++;
    return value;
}

int main(void) {
    for (int i = 0; i < TOTAL_USERS; i++) {
        parent[i] = i;
        sz[i] = 1;
    }
    lfg_init();

    int successful_calls = 0;

    while (1) {
        int caller = lfg_next();
        int called = lfg_next();
        if (caller == called) continue;

        successful_calls++;
        unite(caller, called);

        int pm_root = find(PRIME_MINISTER);
        if (sz[pm_root] >= TARGET) break;
    }

    printf("%d\n", successful_calls);
    return 0;
}
