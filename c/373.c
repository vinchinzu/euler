#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N 10000000
#define MAX_SIG_LEN 8

/* ---- SPF sieve ---- */
static int spf[N + 1];

static void build_spf(void) {
    for (int i = 0; i <= N; i++) spf[i] = i;
    for (int i = 2; (long long)i * i <= N; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= N; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        }
    }
}

/* ---- Exponent signature ---- */
typedef struct {
    unsigned char exps[MAX_SIG_LEN];
    unsigned char len;
} Signature;

static void sort_exps(unsigned char *arr, int n) {
    for (int i = 1; i < n; i++) {
        unsigned char key = arr[i];
        int j = i - 1;
        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j--;
        }
        arr[j + 1] = key;
    }
}

static Signature get_signature(int n) {
    Signature sig;
    sig.len = 0;
    while (n > 1) {
        int p = spf[n];
        int e = 0;
        while (n % p == 0) {
            n /= p;
            e++;
        }
        if (p % 4 == 1) {
            sig.exps[sig.len++] = (unsigned char)e;
        }
    }
    sort_exps(sig.exps, sig.len);
    return sig;
}

/* ---- Hash map for signature -> count ---- */
#define HM_SIZE (1 << 18)
#define HM_MASK (HM_SIZE - 1)

typedef struct HMEntry {
    Signature key;
    int value;
    int occupied;
} HMEntry;

static HMEntry hashmap[HM_SIZE];

static unsigned int sig_hash(const Signature *s) {
    unsigned int h = 2166136261u;
    h ^= s->len;
    h *= 16777619u;
    for (int i = 0; i < s->len; i++) {
        h ^= s->exps[i];
        h *= 16777619u;
    }
    return h;
}

static int sig_equal(const Signature *a, const Signature *b) {
    if (a->len != b->len) return 0;
    for (int i = 0; i < a->len; i++) {
        if (a->exps[i] != b->exps[i]) return 0;
    }
    return 1;
}

static HMEntry* hm_find(const Signature *key) {
    unsigned int idx = sig_hash(key) & HM_MASK;
    for (int probe = 0; probe < HM_SIZE; probe++) {
        unsigned int i = (idx + probe) & HM_MASK;
        if (!hashmap[i].occupied) return NULL;
        if (sig_equal(&hashmap[i].key, key)) return &hashmap[i];
    }
    return NULL;
}

static void hm_insert(const Signature *key, int value) {
    unsigned int idx = sig_hash(key) & HM_MASK;
    for (int probe = 0; probe < HM_SIZE; probe++) {
        unsigned int i = (idx + probe) & HM_MASK;
        if (!hashmap[i].occupied) {
            hashmap[i].key = *key;
            hashmap[i].value = value;
            hashmap[i].occupied = 1;
            return;
        }
    }
    fprintf(stderr, "hashmap full!\n");
    exit(1);
}

/* ---- isqrt for __int128 ---- */
static long long isqrt128(__int128 n) {
    if (n <= 0) return 0;
    long long x = (long long)sqrt((double)n);
    while (x > 0 && (__int128)x * x > n) x--;
    while ((__int128)(x + 1) * (x + 1) <= n) x++;
    return x;
}

/* ---- Main computation ---- */
int main(void) {
    build_spf();
    memset(hashmap, 0, sizeof(hashmap));

    long long total = 0;

    for (int r = 1; r <= N; r++) {
        Signature sig = get_signature(r);

        HMEntry *entry = hm_find(&sig);
        if (entry) {
            total += (long long)entry->value * r;
            continue;
        }

        /* Find all x > 0 with x^2 + y^2 = r^2 */
        int sides[512];
        int n_sides = 0;
        long long r2 = (long long)r * r;

        for (int x = 1; x <= r; x++) {
            long long y2 = r2 - (long long)x * x;
            if (y2 < 0) break;
            long long y = (long long)sqrt((double)y2);
            while (y > 0 && y * y > y2) y--;
            while ((y + 1) * (y + 1) <= y2) y++;
            if (y * y == y2) {
                if (n_sides < 512) {
                    sides[n_sides++] = 2 * x;
                }
            }
        }

        int num_triangles = 0;
        for (int i = 0; i < n_sides; i++) {
            long long a = sides[i];
            for (int j = i; j < n_sides; j++) {
                long long b = sides[j];
                for (int k = j; k < n_sides; k++) {
                    long long c = sides[k];
                    if (a + b <= c) break;

                    long long s2 = a + b + c;
                    long long p1 = -a + b + c;
                    long long p2 = a - b + c;
                    long long p3 = a + b - c;

                    __int128 P = (__int128)s2 * p1 * p2 * p3;
                    long long Q = isqrt128(P);
                    if ((__int128)Q * Q != P) continue;

                    __int128 abc = (__int128)a * b * c;
                    __int128 rQ = (__int128)r * Q;
                    if (abc == rQ) {
                        num_triangles++;
                    }
                }
            }
        }

        hm_insert(&sig, num_triangles);
        total += (long long)num_triangles * r;
    }

    printf("%lld\n", total);
    return 0;
}
