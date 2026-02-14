/* Project Euler 721: High Powers of Irrational Numbers
 * Matrix exponentiation for (ceil(sqrt(a)) + sqrt(a))^(a^2).
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

#define N 5000000
#define M 999999937LL

int is_sq(ll n) {
    ll r = (ll)sqrt((double)n);
    while (r * r < n) r++;
    while (r * r > n) r--;
    return r * r == n;
}

void mat_mult(ll *A, ll *B, ll *C) {
    C[0] = ((lll)A[0] * B[0] + (lll)A[1] * B[2]) % M;
    C[1] = ((lll)A[0] * B[1] + (lll)A[1] * B[3]) % M;
    C[2] = ((lll)A[2] * B[0] + (lll)A[3] * B[2]) % M;
    C[3] = ((lll)A[2] * B[1] + (lll)A[3] * B[3]) % M;
}

void mat_pow(ll *mat, ll exp, ll *result) {
    result[0] = 1; result[1] = 0; result[2] = 0; result[3] = 1;
    ll base[4] = {mat[0] % M, mat[1] % M, mat[2] % M, mat[3] % M};
    ll temp[4];

    while (exp > 0) {
        if (exp & 1) {
            mat_mult(result, base, temp);
            result[0] = temp[0]; result[1] = temp[1];
            result[2] = temp[2]; result[3] = temp[3];
        }
        mat_mult(base, base, temp);
        base[0] = temp[0]; base[1] = temp[1];
        base[2] = temp[2]; base[3] = temp[3];
        exp >>= 1;
    }
}

ll f(ll a, ll n) {
    ll c = (ll)ceil(sqrt((double)a));
    ll mat[4] = {c, a, 1, c};
    ll result[4];
    mat_pow(mat, n, result);
    ll s = (2 * result[0]) % M;
    if (!is_sq(a)) {
        s = (s - 1 + M) % M;
    }
    return s;
}

int main() {
    ll ans = 0;
    for (ll a = 1; a <= N; a++) {
        ans = (ans + f(a, a * a)) % M;
    }
    printf("%lld\n", ans);
    return 0;
}
