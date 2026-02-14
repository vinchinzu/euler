#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

/* State: (num_a, num_b, num_unvisited) where num_c = 2K - num_a - num_b */
/* num_a: 0..2K, num_b: 0..2K-num_a, num_unvisited: 0..1 */
/* Constraint: num_a + num_b <= 2K */

#define K 10
#define FK (2*K)
#define MAX_STATES 1000

int n_states;
int state_a[MAX_STATES], state_b[MAX_STATES], state_u[MAX_STATES];
int state_idx[FK+1][FK+1][2];  /* [num_a][num_b][num_unvisited] -> index */

void init_states() {
    n_states = 0;
    memset(state_idx, -1, sizeof(state_idx));
    for (int a = 0; a <= FK; a++)
        for (int b = 0; b <= FK - a; b++)
            for (int u = 0; u < 2; u++) {
                state_a[n_states] = a;
                state_b[n_states] = b;
                state_u[n_states] = u;
                state_idx[a][b][u] = n_states;
                n_states++;
            }
}

/* Multinomial coefficient n! / (a! b! c!) where a+b+c = n */
ll multinomial(int n, int a, int b, int c) {
    if (a < 0 || b < 0 || c < 0 || a+b+c != n) return 0;
    /* C(n,a) * C(n-a,b) */
    ll r = 1;
    for (int i = 0; i < a; i++) r = r * (n - i) / (i + 1);
    for (int i = 0; i < b; i++) r = r * (n - a - i) / (i + 1);
    return r;
}

/* Matrix operations mod p */
typedef struct {
    ll *data;
    int n;
} Matrix;

Matrix mat_alloc(int n) {
    Matrix m;
    m.n = n;
    m.data = (ll *)calloc((size_t)n * n, sizeof(ll));
    return m;
}

void mat_free(Matrix *m) {
    free(m->data);
}

void mat_identity(Matrix *m) {
    memset(m->data, 0, (size_t)m->n * m->n * sizeof(ll));
    for (int i = 0; i < m->n; i++)
        m->data[i * m->n + i] = 1;
}

void mat_mul(Matrix *res, const Matrix *a, const Matrix *b, ll mod) {
    int n = a->n;
    memset(res->data, 0, (size_t)n * n * sizeof(ll));
    for (int i = 0; i < n; i++) {
        for (int k = 0; k < n; k++) {
            ll aik = a->data[i * n + k];
            if (aik == 0) continue;
            ll *rrow = &res->data[i * n];
            ll *brow = &b->data[k * n];
            for (int j = 0; j < n; j++)
                rrow[j] += aik * brow[j];
        }
        /* Reduce row mod */
        for (int j = 0; j < n; j++)
            res->data[i * n + j] %= mod;
    }
}

void mat_pow(Matrix *res, Matrix *base, ll exp, ll mod) {
    int n = base->n;
    mat_identity(res);
    Matrix tmp = mat_alloc(n);

    while (exp > 0) {
        if (exp & 1) {
            mat_mul(&tmp, res, base, mod);
            ll *t = res->data; res->data = tmp.data; tmp.data = t;
        }
        mat_mul(&tmp, base, base, mod);
        ll *t = base->data; base->data = tmp.data; tmp.data = t;
        exp >>= 1;
    }

    mat_free(&tmp);
}

/* Build transition matrix */
void build_matrix(Matrix *A) {
    int n = n_states;
    memset(A->data, 0, (size_t)n * n * sizeof(ll));

    for (int i = 0; i < n_states; i++) {
        int a = state_a[i], b = state_b[i], c = FK - a - b, u = state_u[i];

        int new_u = u + (a == 0 ? 1 : 0);
        if (new_u > 1) continue;

        for (int j1 = 0; j1 <= a; j1++) {
            for (int j2 = 0; j2 <= a - j1; j2++) {
                int j3 = a - j1 - j2;
                int new_a = b + j1;
                int new_b = c + j2;
                int new_c = j3;
                if (new_a + new_b + new_c != FK) continue;
                if (new_a > FK || new_b > FK) continue;

                int j = state_idx[new_a][new_b][new_u];
                if (j < 0) continue;

                ll coeff = multinomial(a, j1, j2, j3);
                A->data[j * n + i] += coeff;
            }
        }
    }
}

ll extended_gcd(ll a, ll b, ll *x, ll *y) {
    if (a == 0) { *x = 0; *y = 1; return b; }
    ll x1, y1;
    ll g = extended_gcd(b % a, a, &x1, &y1);
    *x = y1 - (b / a) * x1;
    *y = x1;
    return g;
}

ll mod_inverse(ll a, ll m) {
    ll x, y;
    extended_gcd(a, m, &x, &y);
    return ((x % m) + m) % m;
}

ll crt(ll r1, ll m1, ll r2, ll m2) {
    ll M = m1 * m2;
    ll M1 = m2, M2 = m1;
    ll inv1 = mod_inverse(M1, m1);
    ll inv2 = mod_inverse(M2, m2);
    return ((r1 * M1 % M * inv1 % M) + (r2 * M2 % M * inv2 % M)) % M;
}

int main() {
    ll N = 1000000000000LL;  /* 10^12 */
    ll M1 = 512;   /* 2^9 */
    ll M2 = 1953125; /* 5^9 */

    init_states();

    int start = state_idx[FK][0][0];
    int end0 = state_idx[FK][0][0];
    int end1 = state_idx[FK][0][1];

    /* Compute modulo M1 */
    Matrix A1 = mat_alloc(n_states);
    Matrix R1 = mat_alloc(n_states);
    build_matrix(&A1);
    /* Reduce A1 mod M1 */
    for (int i = 0; i < n_states * n_states; i++)
        A1.data[i] %= M1;
    mat_pow(&R1, &A1, N - 1, M1);
    ll r1 = (R1.data[end0 * n_states + start] + R1.data[end1 * n_states + start]) % M1;

    /* Compute modulo M2 */
    Matrix A2 = mat_alloc(n_states);
    Matrix R2 = mat_alloc(n_states);
    build_matrix(&A2);
    for (int i = 0; i < n_states * n_states; i++)
        A2.data[i] %= M2;
    mat_pow(&R2, &A2, N - 1, M2);
    ll r2 = (R2.data[end0 * n_states + start] + R2.data[end1 * n_states + start]) % M2;

    ll result = crt(r1, M1, r2, M2);
    printf("%lld\n", result);

    mat_free(&A1); mat_free(&R1);
    mat_free(&A2); mat_free(&R2);
    return 0;
}
