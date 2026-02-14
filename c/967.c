/* Project Euler 967 - B-trivisible integers
 * F(10^18, 120) using DFT with cube roots of unity.
 * Exact integer arithmetic using Z[omega] representation.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef __int128 lll;

int main(void) {
    ll N = 1000000000000000000LL;

    /* Primes <= 120, excluding 3 (which contributes 0 to h) */
    int small_primes[] = {2,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113};
    int nsp = 29;
    /* mod3 class of each prime */
    int mod3[29];
    for (int i = 0; i < nsp; i++) mod3[i] = small_primes[i] % 3;

    /* Represent h in Z[omega] as (a, b) meaning a + b*omega where omega^2+omega+1=0.
     * Multiplication: (a+b*w)(c+d*w) = (ac-bd) + (ad+bc-bd)*w
     * omega-1 = -1 + 1*omega → (-1, 1)
     * omega^2-1 = -omega-1-1 = -2 - omega → (-2, -1)
     * Real part: Re(a+b*omega) = a + b*Re(omega) = a - b/2
     * 2*Re = 2a - b
     */

    /* For j=1:
     * p mod 3 = 1: factor = omega^1 - 1 = (-1, 1)
     * p mod 3 = 2: factor = omega^2 - 1 = (-2, -1)
     */

    /* Meet in the middle: split 29 primes into first 15 and last 14 */
    int half1 = 15, half2 = nsp - half1;
    int n1 = 1 << half1;
    int n2 = 1 << half2;

    ll *d1_arr = (ll *)malloc(n1 * sizeof(ll));
    ll *h1a_arr = (ll *)malloc(n1 * sizeof(ll));
    ll *h1b_arr = (ll *)malloc(n1 * sizeof(ll));

    for (int mask = 0; mask < n1; mask++) {
        ll d = 1;
        ll ha = 1, hb = 0; /* h = 1 + 0*omega */
        int overflow = 0;
        for (int i = 0; i < half1; i++) {
            if (mask & (1 << i)) {
                if (d > N / small_primes[i]) { overflow = 1; break; }
                d *= small_primes[i];
                ll fa, fb;
                if (mod3[i] == 1) { fa = -1; fb = 1; }
                else { fa = -2; fb = -1; }
                ll new_ha = ha * fa - hb * fb;
                ll new_hb = ha * fb + hb * fa - hb * fb;
                ha = new_ha;
                hb = new_hb;
            }
        }
        if (overflow) { d1_arr[mask] = N + 1; }
        else { d1_arr[mask] = d; }
        h1a_arr[mask] = ha;
        h1b_arr[mask] = hb;
    }

    lll total_2re = 0; /* accumulate sum of (2*ha - hb) * floor(N/d) */

    for (int mask2 = 0; mask2 < n2; mask2++) {
        ll d2 = 1;
        ll h2a = 1, h2b = 0;
        int overflow2 = 0;
        for (int i = 0; i < half2; i++) {
            if (mask2 & (1 << i)) {
                int pi = half1 + i;
                if (d2 > N / small_primes[pi]) { overflow2 = 1; break; }
                d2 *= small_primes[pi];
                ll fa, fb;
                if (mod3[pi] == 1) { fa = -1; fb = 1; }
                else { fa = -2; fb = -1; }
                ll new_ha = h2a * fa - h2b * fb;
                ll new_hb = h2a * fb + h2b * fa - h2b * fb;
                h2a = new_ha;
                h2b = new_hb;
            }
        }
        if (overflow2) continue;

        for (int mask1 = 0; mask1 < n1; mask1++) {
            ll d1 = d1_arr[mask1];
            if (d1 > N / d2) continue;

            ll dd = d1 * d2;
            ll nd = N / dd;
            if (nd == 0) continue;

            /* h = h1 * h2 in Z[omega] */
            /* (a1+b1*w)(a2+b2*w) = (a1*a2-b1*b2) + (a1*b2+b1*a2-b1*b2)*w */
            ll ha = h1a_arr[mask1];
            ll hb = h1b_arr[mask1];
            /* Full h = (ha,hb) * (h2a, h2b) */
            lll ra = (lll)ha * h2a - (lll)hb * h2b;
            lll rb = (lll)ha * h2b + (lll)hb * h2a - (lll)hb * h2b;
            lll two_re = 2 * ra - rb;

            total_2re += two_re * nd;
        }
    }

    /* F = (N + 2*Re(S1)) / 3 = (N + total_2re) / 3 */
    /* But we computed 2*Re, and F = (N + sum(2*Re*floor)) / 3 */
    /* Wait: F = (N + 2*Re(S1)) / 3 where S1 = sum h(d)*floor(N/d).
     * 2*Re(S1) = sum (2*Re(h(d))) * floor(N/d) = sum (2a-b) * floor(N/d) = total_2re.
     * So F = (N + total_2re) / 3.
     */
    lll F = ((lll)N + total_2re) / 3;
    ll answer = (ll)F;
    printf("%lld\n", answer);

    free(d1_arr);
    free(h1a_arr);
    free(h1b_arr);
    return 0;
}
