/*
 * Project Euler Problem 325 - Stone Game II
 *
 * S(N) = sum_{y=1}^N sum_{x=ceil(y/phi)}^{y-1} (x+y) mod M, M = 7^10, N = 10^16.
 *
 * Formula: S(N) = 3*(sum y^2 - sum y)/2 - (S02 + S01)/2 - S11
 * where S01 = sum floor(k*phi_inv), S02 = sum floor(k*phi_inv)^2,
 *       S11 = sum k*floor(k*phi_inv), all for k=1..N.
 *
 * Recursive O(log N) computation using Beatty/lattice-point identities.
 */
#include <stdio.h>
#include <math.h>
#include <string.h>

typedef long long ll;
typedef __int128 i128;
typedef unsigned long long ull;

static ll M;
static ll inv2, inv6;

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1; base %= mod; if (base < 0) base += mod;
    while (exp > 0) { if (exp & 1) result = (i128)result * base % mod; base = (i128)base * base % mod; exp >>= 1; }
    return result;
}

ull isqrt_ull(i128 n) {
    if (n <= 1) return (ull)n;
    i128 x = (i128)sqrtl((long double)n);
    while (x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return (ull)x;
}

ll floor_phi_inv(ll n) {
    if (n <= 0) return 0;
    i128 n2 = (i128)n * n;
    ull s = isqrt_ull(5 * n2);
    return (ll)((s - n) / 2);
}

ll floor_phi(ll n) {
    if (n <= 0) return 0;
    i128 n2 = (i128)n * n;
    ull s = isqrt_ull(5 * n2);
    return (ll)((s + n) / 2);
}

ll sum_k_mod(ll n) {
    ll nm = ((n % M) + M) % M;
    ll np1 = (((n + 1) % M) + M) % M;
    return (i128)nm * np1 % M * inv2 % M;
}

ll sum_k2_mod(ll n) {
    ll nm = ((n % M) + M) % M;
    ll np1 = (((n + 1) % M) + M) % M;
    ll n2p1 = (((2 * nm + 1) % M) + M) % M;
    return (i128)nm * np1 % M * n2p1 % M * inv6 % M;
}

typedef struct { ll s01, s02, s11; } Sums;

/* Memoization: since the recursion depth is O(log N) ~ 110, and each level
 * can call itself for m and m-1, the total number of distinct calls is bounded.
 * Use simple linear memo. */
#define MEMO_SIZE 500
ll memo_keys[MEMO_SIZE];
Sums memo_vals[MEMO_SIZE];
int memo_count = 0;

int memo_find(ll n) {
    for (int i = 0; i < memo_count; i++)
        if (memo_keys[i] == n) return i;
    return -1;
}

void memo_store(ll n, Sums s) {
    memo_keys[memo_count] = n;
    memo_vals[memo_count] = s;
    memo_count++;
}

/* Compute S01, S02, S11 for sum_{k=1}^n floor(k*phi_inv)^a * k^b */
Sums compute(ll n) {
    Sums zero = {0, 0, 0};
    if (n <= 0) return zero;

    int idx = memo_find(n);
    if (idx >= 0) return memo_vals[idx];

    ll m = floor_phi_inv(n);
    if (m == 0) {
        memo_store(n, zero);
        return zero;
    }

    Sums sm = compute(m);
    Sums sm1 = compute(m - 1);

    ll p = floor_phi(m);
    ll q = n - p;
    if (q < 0) { q = 0; p = n; }

    ll v_mod = ((n % M) + M) % M;
    ll m_mod = ((m % M) + M) % M;
    ll m1_mod = (((m - 1) % M) + M) % M;
    ll p_mod = ((p % M) + M) % M;
    ll q_mod = ((q % M) + M) % M;

    /* S01(n) = n*m - m*(m+1)/2 - S01(m) */
    ll t1 = (i128)v_mod * m_mod % M;
    ll t2 = (i128)m_mod * ((m_mod + 1) % M) % M * inv2 % M;
    ll S01 = ((t1 - t2 - sm.s01) % M + M) % M;

    /* S02: split into S02_A (k=1..p) + m^2*q (k=p+1..n)
     * S02_A = (m-1)^2*p - 2*S11(m-1,phi) + S01(m-1,phi)
     * S11(m-1,phi) = (m-1)m(2m-1)/6 + S11(m-1,phi_inv)
     * S01(m-1,phi) = (m-1)m/2 + S01(m-1,phi_inv)
     */
    ll sk2m1 = (i128)m1_mod * ((m1_mod + 1) % M) % M * ((2 * m1_mod + 1) % M) % M * inv6 % M;
    ll skm1 = (i128)m1_mod * ((m1_mod + 1) % M) % M * inv2 % M;

    /* S11(m-1, phi) */
    ll s11_phi_m1 = (sk2m1 + sm1.s11) % M;
    /* S01(m-1, phi) */
    ll s01_phi_m1 = (skm1 + sm1.s01) % M;

    ll S02_A = ((i128)m1_mod * m1_mod % M * p_mod % M
               + M * 4LL
               - 2 * s11_phi_m1 % M
               + s01_phi_m1) % M;

    ll S02 = (S02_A + (i128)m_mod * m_mod % M * q_mod % M) % M;

    /* S11: split into S11_A (k=1..p) + m * sum_{k=p+1}^n k
     * S11_A = (1/2)*((m-1)*p^2 - S02(m-1,phi) + S01_A)
     * S02(m-1,phi) = sk2m1 + 2*S11(m-1,phi_inv) + S02(m-1,phi_inv)
     * S01_A = S01 - m*q
     */
    ll S01_A = (S01 + M - (i128)m_mod * q_mod % M) % M;
    ll s02_phi_m1 = (sk2m1 + 2 * sm1.s11 % M + sm1.s02 + M) % M;

    ll S11_A = ((i128)m1_mod * p_mod % M * p_mod % M
               + M * 4LL
               - s02_phi_m1
               + S01_A) % M;
    S11_A = (i128)S11_A * inv2 % M;

    ll sum_k_n_v = sum_k_mod(n);
    ll sum_k_p_v = sum_k_mod(p);
    ll sum_tail = (sum_k_n_v + M - sum_k_p_v) % M;
    ll S11 = (S11_A + (i128)m_mod * sum_tail % M) % M;

    Sums result = {S01, S02, S11};
    memo_store(n, result);
    return result;
}

int main(void) {
    ll N_val = 10000000000000000LL;

    M = 1;
    for (int i = 0; i < 10; i++) M *= 7;
    ll phi_M = M / 7 * 6;
    inv2 = mod_pow(2, phi_M - 1, M);
    inv6 = mod_pow(6, phi_M - 1, M);

    Sums s = compute(N_val);

    ll sum_y = sum_k_mod(N_val);
    ll sum_y2 = sum_k2_mod(N_val);

    ll part1 = (i128)3 * ((sum_y2 + M - sum_y) % M) % M * inv2 % M;
    ll part2 = (i128)((s.s02 + s.s01) % M) * inv2 % M;
    ll part3 = s.s11;

    ll ans = ((part1 + M * 2LL - part2 - part3) % M + M) % M;

    printf("%lld\n", ans);
    return 0;
}
