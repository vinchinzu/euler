/*
 * Project Euler Problem 325 - Stone Game II
 *
 * S(N) = sum of (x+y) over losing positions (x,y) with 1 <= y <= N, x < y.
 * The losing positions in the Wythoff game are (floor(k*phi), floor(k*phi^2))
 * for k >= 1, where phi = (1+sqrt(5))/2.
 *
 * Equivalently, a position (a,b) with a < b is losing iff a = floor(k*phi_inv*(b))
 * for some k, where the Beatty complement structure applies.
 *
 * The losing positions are: a_k = floor(k * phi), b_k = a_k + k = floor(k * phi^2)
 * where phi = (1+sqrt(5))/2.
 *
 * S(N) = sum_{k=1}^{K} (a_k + b_k) where b_k <= N.
 * Since b_k = a_k + k, we have a_k + b_k = 2*a_k + k.
 * a_k = floor(k * phi), so a_k + b_k = 2*floor(k*phi) + k.
 *
 * We need b_k <= N, i.e., floor(k*phi^2) <= N.
 * phi^2 = phi + 1, so floor(k*(phi+1)) <= N, i.e., k + floor(k*phi) <= N.
 *
 * Let K be the largest k with b_k <= N.
 * K = floor(N / phi^2) = floor(N * phi_inv^2) where phi_inv = (sqrt(5)-1)/2.
 * Actually phi^2 = phi + 1 = (3+sqrt(5))/2, so 1/phi^2 = (3-sqrt(5))/2 = phi_inv^2.
 *
 * S(N) = sum_{k=1}^{K} (2*floor(k*phi) + k) = 2 * sum floor(k*phi) + K*(K+1)/2
 *
 * We need sum_{k=1}^{K} floor(k*phi) mod M where M = 7^10.
 *
 * Use the identity: floor(k*phi) = k + floor(k*phi_inv) where phi_inv = phi - 1 = (sqrt(5)-1)/2.
 * So sum floor(k*phi) = K*(K+1)/2 + sum floor(k*phi_inv).
 *
 * sum_{k=1}^{K} floor(k*phi_inv) can be computed in O(log K) using the
 * Euclidean-like recursion for floor sums with the golden ratio.
 *
 * Key identity (Beatty):
 * sum_{k=1}^n floor(k*alpha) = n*m - m*(m+1)/2 - sum_{j=1}^m floor(j/alpha)
 * where m = floor(n*alpha), 1/alpha = phi for alpha = phi_inv.
 * But floor(j*phi) = j + floor(j*phi_inv), so:
 * sum_{j=1}^m floor(j*phi) = m*(m+1)/2 + sum_{j=1}^m floor(j*phi_inv)
 *
 * Therefore: sum_{k=1}^n floor(k*phi_inv) = n*m - m*(m+1)/2 - m*(m+1)/2 - sum_{j=1}^m floor(j*phi_inv)
 *                                          = n*m - m*(m+1) - sum_{j=1}^m floor(j*phi_inv)
 *
 * Wait, let me redo: Beatty identity for irrational alpha with 1/alpha = beta:
 * sum_{k=1}^n floor(k*alpha) + sum_{j=1}^m floor(j*beta) = n*m
 * where m = floor(n*alpha).
 *
 * Here alpha = phi_inv = (sqrt(5)-1)/2, beta = 1/phi_inv = phi = (1+sqrt(5))/2.
 * sum_{k=1}^n floor(k*phi_inv) + sum_{j=1}^m floor(j*phi) = n*m
 * where m = floor(n*phi_inv).
 *
 * And floor(j*phi) = j + floor(j*phi_inv), so:
 * sum floor(j*phi) = m*(m+1)/2 + sum_{j=1}^m floor(j*phi_inv)
 *
 * Substituting:
 * F(n) + m*(m+1)/2 + F(m) = n*m
 * where F(n) = sum_{k=1}^n floor(k*phi_inv).
 *
 * So: F(n) = n*m - m*(m+1)/2 - F(m) where m = floor(n*phi_inv).
 *
 * Since m < n (phi_inv < 1), this recurses with decreasing argument.
 * Depth is O(log n) due to golden ratio.
 *
 * All computation mod M = 7^10.
 */
#include <stdio.h>
#include <math.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 i128;

/* M = 7^10 */
static ll M;
/* phi(M) = 7^10 - 7^9 = 7^9 * 6 */
static ll phi_M;

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1)
            result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* isqrt for __int128 */
ull isqrt_ull(i128 n) {
    if (n <= 1) return (ull)n;
    i128 x = (i128)sqrtl((long double)n);
    /* Adjust */
    while (x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return (ull)x;
}

/* floor(n * phi_inv) = floor(n * (sqrt(5)-1)/2) exactly using integer sqrt */
ll floor_phi_inv(ll n) {
    if (n <= 0) return 0;
    /* floor((sqrt(5*n^2) - n) / 2) */
    i128 n2 = (i128)n * n;
    ull s = isqrt_ull(5 * n2);
    /* Verify and adjust */
    while ((i128)s * s > 5 * n2) s--;
    while ((i128)(s + 1) * (s + 1) <= 5 * n2) s++;
    return (ll)((s - n) / 2);
}

/* floor(n * phi) = floor(n * (sqrt(5)+1)/2) exactly */
ll floor_phi(ll n) {
    if (n <= 0) return 0;
    i128 n2 = (i128)n * n;
    ull s = isqrt_ull(5 * n2);
    while ((i128)s * s > 5 * n2) s--;
    while ((i128)(s + 1) * (s + 1) <= 5 * n2) s++;
    return (ll)((s + n) / 2);
}

ll inv2;

ll sum_k_mod(ll n) {
    /* sum_{k=1}^n k mod M = n*(n+1)/2 mod M */
    ll nm = n % M;
    ll np1 = (n + 1) % M;
    return (i128)nm * np1 % M * inv2 % M;
}

/* F(n) = sum_{k=1}^n floor(k*phi_inv) mod M
 * Recursion: F(n) = n*m - m*(m+1)/2 - F(m) where m = floor(n*phi_inv) */

/* Use iterative approach to avoid stack overflow for large n */
ll compute_F(ll n) {
    if (n <= 0) return 0;

    /* Collect the chain of (n, m) values going down */
    /* n -> m = floor(n*phi_inv) -> m' = floor(m*phi_inv) -> ... -> 0 */
    /* At most O(log_phi n) ~ 110 steps for n ~ 10^16 */
    ll chain_n[200];
    ll chain_m[200];
    int depth = 0;

    ll cur = n;
    while (cur > 0) {
        ll m = floor_phi_inv(cur);
        chain_n[depth] = cur;
        chain_m[depth] = m;
        depth++;
        cur = m;
    }

    /* Now compute F bottom-up.
     * F(0) = 0.
     * F(chain_n[depth-1]) = chain_n[depth-1]*chain_m[depth-1] - chain_m[depth-1]*(chain_m[depth-1]+1)/2 - F(chain_m[depth-1])
     * But chain_m[depth-1] might not be 0... let's adjust.
     */

    /* Actually, the chain is: chain_n[0] = n, chain_m[0] = m0
     *                          chain_n[1] = m0, chain_m[1] = m1
     *                          ...
     *                          chain_n[d-1] has chain_m[d-1] possibly > 0 but chain_m[d-1] = floor(chain_n[d-1]*phi_inv).
     *                          If chain_m[d-1] = 0, we're done.
     *                          cur at the end is 0, so depth includes the step where m=0. Actually no.
     * Let's redo: cur starts at n, loop while cur > 0. Inside, m = floor(cur*phi_inv).
     * Store (cur, m). Then cur = m. If m = 0, the loop body executes one more time
     * storing (cur=something>0, m=0), then cur=0 and loop ends. */

    /* F(chain_n[depth-1]) uses F(chain_m[depth-1]). chain_m[depth-1] should be 0
     * (otherwise the loop would have continued). */
    /* Wait: loop condition is cur > 0. Last iteration: cur = chain_n[depth-1] > 0,
     * m = chain_m[depth-1] = floor(cur*phi_inv). After storing, cur = m. If m > 0,
     * loop continues. So the last stored entry has m such that floor(m*phi_inv) is checked
     * next iteration. Actually, let me just check: the loop continues while cur > 0.
     * So after the loop, cur = 0. The last stored m = chain_m[depth-1], and cur was set
     * to this m. Since loop ended, this m must be 0? No, wait. cur is set to m after
     * storing. Then the while checks cur > 0. So if m > 0, we continue. So the last
     * stored m is > 0 (the one before had m > 0 too). Actually let me trace:
     *
     * Initially cur = n > 0. Enter loop.
     *   m = floor(n*phi_inv). Store (n, m). cur = m.
     * If m > 0, loop continues.
     *   m' = floor(m*phi_inv). Store (m, m'). cur = m'.
     * ...
     * Eventually cur becomes 0 and loop ends.
     * The last stored entry has chain_m[depth-1] = 0? Not necessarily.
     * Example: if cur = 1, m = floor(phi_inv) = 0. Store (1, 0). cur = 0. Loop ends.
     * So chain_m[depth-1] = 0. Good.
     * Example: cur = 2, m = floor(2*0.618) = 1. Store (2, 1). cur = 1.
     * cur = 1, m = 0. Store (1, 0). cur = 0. Loop ends.
     * So yes, the last entry always has m = 0 (since floor(1*phi_inv) = 0 and
     * we only reach 0 from 1 or directly from larger).
     * Actually floor(1*phi_inv) = floor(0.618..) = 0. So any cur=1 gives m=0.
     * And cur cannot be smaller than 1 (since cur > 0 in the loop).
     * So chain_m[depth-1] = 0, and F(0) = 0. */

    /* Compute F from bottom to top */
    ll F = 0; /* F(chain_m[depth-1]) = F(0) = 0 */

    for (int i = depth - 1; i >= 0; i--) {
        ll ni = chain_n[i];
        ll mi = chain_m[i];
        /* F(ni) = ni*mi - mi*(mi+1)/2 - F(mi) */
        /* F(mi) is the current F value */
        ll ni_mod = ni % M;
        ll mi_mod = mi % M;
        ll term1 = (i128)ni_mod * mi_mod % M;
        ll term2 = (i128)mi_mod * ((mi_mod + 1) % M) % M * inv2 % M;
        ll new_F = ((term1 - term2 - F) % M + M) % M;
        F = new_F;
    }

    return F;
}

int main(void) {
    ll N_val = 10000000000000000LL; /* 10^16 */

    /* M = 7^10 */
    M = 1;
    for (int i = 0; i < 10; i++) M *= 7;

    /* phi(M) = 7^9 * 6 */
    phi_M = M / 7 * 6;

    inv2 = mod_pow(2, phi_M - 1, M);

    /* K = largest k with b_k = k + floor(k*phi) <= N */
    /* b_k = floor(k*phi^2) = floor(k*(phi+1)) = k + floor(k*phi) */
    /* We need k + floor(k*phi) <= N */
    /* phi^2 = (3+sqrt(5))/2, 1/phi^2 = (3-sqrt(5))/2 */
    /* K = floor(N / phi^2) = floor(N * (3-sqrt(5))/2) */

    /* Compute K exactly */
    /* K = floor(N * (3-sqrt(5))/2) */
    /* = floor((3*N - sqrt(5*N^2)) / 2) */
    i128 N2 = (i128)N_val * N_val;
    ull s5n = isqrt_ull(5 * N2);
    while ((i128)s5n * s5n > 5 * N2) s5n--;
    while ((i128)(s5n + 1) * (s5n + 1) <= 5 * N2) s5n++;

    ll K = (ll)((3LL * N_val - (ll)s5n) / 2);

    /* Verify: K + floor(K*phi) <= N and (K+1) + floor((K+1)*phi) > N */
    while (K + floor_phi(K) > N_val) K--;
    while (K + 1 + floor_phi(K + 1) <= N_val) K++;

    /* S(N) = sum_{k=1}^K (2*floor(k*phi) + k)
     *      = 2 * sum floor(k*phi) + K*(K+1)/2
     *      = 2 * (K*(K+1)/2 + F(K)) + K*(K+1)/2
     *      = 2*F(K) + K*(K+1) + K*(K+1)/2
     *      = 2*F(K) + 3*K*(K+1)/2
     * where F(K) = sum_{k=1}^K floor(k*phi_inv).
     */

    ll FK = compute_F(K);

    ll K_mod = K % M;
    ll Kp1_mod = (K + 1) % M;
    ll tri_K = (i128)K_mod * Kp1_mod % M * inv2 % M;

    ll ans = (2 * FK % M + 3 * tri_K % M) % M;
    ans = (ans + M) % M;

    printf("%lld\n", ans);
    return 0;
}
