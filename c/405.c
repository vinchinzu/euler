/*
 * Project Euler Problem 405: A rectangular tiling.
 *
 * f(n) counts 4-way meeting points in tiling T(n).
 * Recurrence: f(n) = 5*f(n-1) - 2*f(n-2) - 8*f(n-3) + 6
 * with f(1)=0, f(2)=2, f(3)=16.
 *
 * We need f(10^(10^18)) mod 17^7. Use matrix exponentiation with
 * period reduction for the enormous exponent.
 */
#include <stdio.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define DIM 4

typedef struct { ll a[DIM][DIM]; } Mat;

static Mat mat_mul(Mat A, Mat B, ll mod) {
    Mat C;
    memset(&C, 0, sizeof(C));
    for (int i = 0; i < DIM; i++)
        for (int k = 0; k < DIM; k++) {
            if (A.a[i][k] == 0) continue;
            for (int j = 0; j < DIM; j++)
                C.a[i][j] = (C.a[i][j] + (lll)A.a[i][k] * B.a[k][j]) % mod;
        }
    return C;
}

static Mat mat_pow(Mat M, ll p, ll mod) {
    Mat R;
    memset(&R, 0, sizeof(R));
    for (int i = 0; i < DIM; i++) R.a[i][i] = 1;
    Mat B = M;
    while (p > 0) {
        if (p & 1) R = mat_mul(R, B, mod);
        B = mat_mul(B, B, mod);
        p >>= 1;
    }
    return R;
}

static ll pow_mod(ll base, ll exp, ll mod) {
    ll r = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) r = (lll)r * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return r;
}

static ll mod_inv(ll a, ll m) {
    /* Extended GCD */
    ll old_r = a, r = m, old_s = 1, s = 0;
    while (r) {
        ll q = old_r / r;
        ll tmp = r; r = old_r - q * r; old_r = tmp;
        tmp = s; s = old_s - q * s; old_s = tmp;
    }
    return ((old_s % m) + m) % m;
}

int main(void) {
    ll MOD = 1;
    for (int i = 0; i < 7; i++) MOD *= 17;  /* 17^7 = 410338673 */

    /* Matrix for recurrence f(n) = 5*f(n-1) - 2*f(n-2) - 8*f(n-3) + 6 */
    /* State: [f(n), f(n-1), f(n-2), 1] */
    /* M = [[5,-2,-8,6],[1,0,0,0],[0,1,0,0],[0,0,0,1]] */

    /* Period of sequence mod 17 is 8 */
    /* Period mod 17^7 divides 8 * 17^6 = 193100552 */
    ll period = 8;
    for (int i = 0; i < 6; i++) period *= 17;  /* 193100552 */

    /* Compute 10^(10^18) mod period */
    /* period = 8 * 17^6. CRT: compute mod 8 and mod 17^6 separately. */

    /* 10^(10^18) mod 8: 10 = 2 (mod 8), 2^3 = 0 mod 8 */
    /* For k >= 3: 10^k = 0 (mod 8). 10^18 >= 3. */
    ll r8 = 0;

    /* 10^(10^18) mod 17^6: use Euler's theorem */
    /* phi(17^6) = 17^5 * 16 */
    ll phi_17_6 = 16;
    for (int i = 0; i < 5; i++) phi_17_6 *= 17;  /* 22717712 */

    ll seventeen_6 = 1;
    for (int i = 0; i < 6; i++) seventeen_6 *= 17;  /* 24137569 */

    /* Need 10^18 mod phi(17^6) */
    ll exp_mod = pow_mod(10, 18, phi_17_6);
    ll r17_6 = pow_mod(10, exp_mod, seventeen_6);

    /* CRT: x = 0 (mod 8), x = r17_6 (mod 17^6) */
    ll inv8 = mod_inv(8, seventeen_6);
    ll k = (lll)r17_6 * inv8 % seventeen_6;
    ll n_mod_period = (8 * k) % period;

    if (n_mod_period < 3) n_mod_period += period;

    ll steps = n_mod_period - 3;

    Mat M;
    memset(&M, 0, sizeof(M));
    M.a[0][0] = 5; M.a[0][1] = MOD - 2; M.a[0][2] = MOD - 8; M.a[0][3] = 6;
    M.a[1][0] = 1;
    M.a[2][1] = 1;
    M.a[3][3] = 1;

    Mat Mn = mat_pow(M, steps, MOD);
    ll state[4] = {16, 2, 0, 1};
    ll result = 0;
    for (int j = 0; j < 4; j++)
        result = (result + (lll)Mn.a[0][j] * state[j]) % MOD;

    printf("%lld\n", result);
    return 0;
}
